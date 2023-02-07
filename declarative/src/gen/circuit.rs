use crate::dsl::FieldValue;
use crate::dsl::circuit::{CircuitConfig};
use crate::dsl::block_instance::{BlockInstance};
use crate::util::context::GenerationContext;
use super::traits::CodeGenerator;
use super::resolver::NameResolver;
use genco::prelude::{rust, quote};
use lazy_static::lazy_static;
#[allow(unused_imports)]
use super::comments::{Comment, DocComment};
use convert_case::{Case, Casing};

#[allow(non_snake_case)]
pub struct Imports {
  pub DS: rust::Import,
  pub ds_core: rust::Import,
  pub req_storage: rust::Import,
  pub req_peripherals: rust::Import,
  pub OnceCell: rust::Import,
}

lazy_static!(
  pub static ref IMPORTS: Imports = Imports {
    DS: rust::import("flowmbed_dynsys::core", "DynamicalSystem"),
    ds_core: rust::import("flowmbed_dynsys", "core").with_alias("ds_core"),
    req_storage: rust::import("flowmbed_dynsys::core", "RequiresStorage"),
    req_peripherals: rust::import("flowmbed_dynsys::core", "RequirePeripherals"),
    OnceCell: rust::import("flowmbed_dynsys::util::containers", "OnceCell"),
  };
);

pub struct CircuitGenerator<'a> {
  pub resolver: &'a dyn NameResolver,
  pub circuit: &'a CircuitConfig,
}

impl<'a> CircuitGenerator<'a> {
  pub fn new(circuit: &'a CircuitConfig, resolver: &'a dyn NameResolver) -> CircuitGenerator<'a> {
    CircuitGenerator {
      resolver,
      circuit,
    }
  }

  /// Generate a field in the circuit structure definition
  fn declare_block(&self, block: &BlockInstance) -> anyhow::Result<rust::Tokens> {
    let mut gen_params: Vec<rust::Tokens> = vec![quote!('a)];
    block.structural.iter().for_each(|x|{
      gen_params.push(quote!($(x.as_text())))
    });
    let gen = quote!($(for p in gen_params join(, ) => $p));
    Ok(quote!(
      $(&block.id): $(self.resolver.resolve_import(&block.kind)?)<$gen>
    ))
  }

  /// Initialize a block field in the circuit structure
  fn create_block(&self, block: &BlockInstance) -> anyhow::Result<rust::Tokens> {
    
    /// Helper for the field initialization
    fn create_fn(args: Vec<rust::Tokens>, modifiers: Vec<rust::Tokens>) -> rust::Tokens {
      let arg_tokens = quote!($(for arg in args join (, ) => $arg));
      if modifiers.is_empty() {
        quote!(builder().build($(arg_tokens)))
      } else {
        let modifier_tokens = quote!($(for modifier in modifiers join () => $modifier));
        quote!(builder()$(modifier_tokens).build($(arg_tokens)))
      }
    }

    let peripherals_name = "peripherals";
    
    let args  = vec![quote!(&mut builder)];

    let mut modifiers = block.parameters.iter().map(
      |(key, value)| {
        let value_str = match &value {          
          &FieldValue::Bool(x) => x.to_string(),
          &FieldValue::Int(x) => x.to_string(),
          &FieldValue::Float(x) => x.to_string(),
          &FieldValue::String(x) => x.to_string(),
        };
        quote!(.$(key)($(value_str)))
      }
    ).collect::<Vec<_>>();

    block.peripherals.iter().for_each(|periph_ref|
      modifiers.push(quote!(
        .$(periph_ref.0)($(peripherals_name).$(periph_ref.1).mut_ref()?)
      ))
    );


    Ok(quote!(
      {$(self.resolver.resolve_import(&block.kind)?)
        ::$(create_fn(args, modifiers))}
    ))
  }

  /// Connect a block to its inputs
  fn block_connect(&self, block: &BlockInstance) -> rust::Tokens {
    quote!($(for (input, output) in &block.inputs =>
      self.$(&block.id).$(&input.input_id).connect(&self.$(&output.block_id).$(&output.output_id))?;$['\r']
    ))
  }

  /// Initialize a block
  fn block_init(&self, block: &BlockInstance) -> rust::Tokens {
    quote!(self.$(&block.id).init())
  }

  /// Perform a step for the block
  fn block_step(&self, block: &BlockInstance) -> rust::Tokens {
    quote!(self.$(&block.id).step(ssi))
  }

  /// Perform a step for the block
  fn block_add_size(&self, block: &BlockInstance) -> anyhow::Result<rust::Tokens> {
    let gen_params = block.structural.iter().map(|x|
      quote!($(x.as_text()))
    ).collect::<Vec<_>>();

    if gen_params.is_empty() {
      Ok(quote!(.add($(self.resolver.resolve_import(&block.kind)?)::SIZE)))
    } else {
      let gen = quote!($(for p in gen_params join(, ) => $p));
      Ok(quote!(.add($(self.resolver.resolve_import(&block.kind)?)::<$(gen)>::SIZE)))
    }

    
  }

}

impl<'a> CodeGenerator for CircuitGenerator<'a> {
  fn generate(&self, _: &dyn GenerationContext) -> anyhow::Result<rust::Tokens> {
    let ds_core = &IMPORTS.ds_core;
    let circuit_name = &self.circuit.id;

    let peripherals_struct = &format!("{}Peripherals",
      self.circuit.device.from_case(Case::Snake).to_case(Case::UpperCamel));

    Ok(quote!{
      $(DocComment(["Declare circuit structure"]))
      #[derive(const_default_derive::ConstDefault)]
      struct $(circuit_name)<'a> {
        $(for block in &self.circuit.blocks =>
          $(self.declare_block(&block)?),$['\r']
        )
      }
      
      $(DocComment(["Implement circuit structure"]))
      impl<'a> $(circuit_name)<'a> {
        pub fn init<ST: $(ds_core)::DefaultSystemStrorage>(
          circuit: &mut $(&IMPORTS.OnceCell)<$(circuit_name)<'a>>,
          storage: &'a ST, peripherals: &'a $(peripherals_struct)
        ) -> anyhow::Result<()> {
          use $(ds_core)::BlockBuilder;


          let mut builder = $(ds_core)::SystemStorageBuilder::new(storage);
          
          circuit.set($(circuit_name) {
            $(for block in &self.circuit.blocks =>
              $(&block.id):$(self.create_block(&block)?),$['\r']
            )    
          })?;

          circuit.get_mut()?.connect()?;
          Ok(())
        }

      }

      $(DocComment(["Implement DynamicalSystem protocol"]))
      impl<'a> $(&IMPORTS.DS)<'a> for $(circuit_name)<'a> {
        fn connect(&mut self) -> anyhow::Result<()> {
          $(for block in &self.circuit.blocks =>
            $(self.block_connect(&block))
          )
          Ok(())
        }

        fn init(&mut self) -> anyhow::Result<()> {
          $(for block in &self.circuit.blocks =>
            $(self.block_init(&block))?;$['\r']
          )    
          Ok(())
        }

        fn step(&mut self, ssi: &$(ds_core)::SystemStateInfo) -> anyhow::Result<()> {
          $(for block in &self.circuit.blocks =>
            $(self.block_step(&block))?;$['\r']
          )              
          Ok(())
        }
      }

      $(DocComment(["Implement RequirePeripherals protocol"]))
      impl<'a> $(&IMPORTS.req_peripherals) for $(circuit_name)<'a> {
        type PeripheralsStruct = $(peripherals_struct)<'a>;
      }

      $(DocComment(["Implement RequireStorage protocol"]))
      use const_default::ConstDefault;
      impl<'a> $(&IMPORTS.req_storage) for $(circuit_name)<'a> {
        const SIZE: $(ds_core)::StorageSize = 
          $(ds_core)::StorageSize::DEFAULT$['\r']
            $(for block in &self.circuit.blocks =>
              $(self.block_add_size(&block)?)$['\r']
            )              
          ;
      }
      
    })
  }
}
