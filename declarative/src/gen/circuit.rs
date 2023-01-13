use crate::dsl::FieldValue;
use crate::dsl::circuit::{CircuitConfig, BlockConnection};
use crate::dsl::block_instance::{BlockInstance};
use super::traits::CodeGenerator;
use super::resolver::NameResolver;
use genco::prelude::{rust, quote};
use super::comments::{Comment, DocComment};

pub struct CircuitGenerator<'a> {
  pub resolver: &'a dyn NameResolver,
  pub circuit: &'a CircuitConfig,
  pub dynsys_core: rust::Import,
  pub i_sys: rust::Import,
  pub i_req_storage: rust::Import,
  pub i_req_peripherals: rust::Import,
}

impl<'a> CircuitGenerator<'a> {
  pub fn new(circuit: &'a CircuitConfig, resolver: &'a dyn NameResolver) -> CircuitGenerator<'a> {
    CircuitGenerator {
      resolver,
      circuit,
      dynsys_core: rust::import("flowmbed_dynsys", "core").with_alias("fds_core"),
      i_sys: rust::import("flowmbed_dynsys::core", "DynamicalSystem"),
      i_req_storage: rust::import("flowmbed_dynsys::core", "RequiresStorage"),
      i_req_peripherals: rust::import("flowmbed_dynsys::core", "RequirePeripherals")
    }
  }

  /// Generate a field in the circuit structure definition
  fn declare_block(&self, block: &BlockInstance) -> anyhow::Result<rust::Tokens> {
    Ok(quote!(
      $(&block.id): $(self.resolver.resolve_import(&block.kind)?)<'a>
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
    
    let mut args  = vec![quote!(&mut builder)];

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
        .$(periph_ref.0)(&mut $(peripherals_name).$(periph_ref.1))
      ))
    );


    Ok(quote!(
      $(&block.id): {$(self.resolver.resolve_import(&block.kind)?)
        ::$(create_fn(args, modifiers))},
    ))
  }

  /// Creates connection between two blocks
  fn connect_blocks(&self, connection: &BlockConnection) -> rust::Tokens {
    let to = &connection.to;
    let from = &connection.from;
    quote!(self.$(&to.block).$(&to.id).connect(&self.$(&from.block).$(&from.id)))
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
    Ok(quote!(.add($(self.resolver.resolve_import(&block.kind)?)::SIZE)))
  }

}

impl<'a> CodeGenerator for CircuitGenerator<'a> {
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    let circuit_name = &self.circuit.id;
    let peripherals_type = &format!("{}Peripherals", self.circuit.device);

    Ok(quote!{
      $(DocComment(["Declare circuit structure"]))
      struct $(circuit_name)<'a> {
        $(for block in &self.circuit.blocks =>
          $(self.declare_block(&block)?),$['\r']
        )
      }
      
      $(DocComment(["Implement circuit structure"]))
      impl<'a> $(circuit_name)<'a> {
        pub fn new<ST: $(&self.dynsys_core)::DefaultSystemStrorage>(
          storage: &'a ST, peripherals: &'a mut $(peripherals_type)
        ) -> anyhow::Result<$(circuit_name)<'a>> {
          use $(&self.dynsys_core)::BlockBuilder;

          let mut builder = $(&self.dynsys_core)::SystemStorageBuilder::new(storage);

          let mut circuit = $(circuit_name) {
            $(for block in &self.circuit.blocks =>
              $(self.create_block(&block)?)
            )    
          };

          circuit.connect()?;
          Ok(circuit)
        }

      }

      $(DocComment(["Implement DynamicalSystem protocol"]))
      impl<'a> $(&self.i_sys) for $(circuit_name)<'a> {
        fn connect(&mut self) -> anyhow::Result<()> {
          $(for connection in &self.circuit.connections => 
            $(self.connect_blocks(&connection))?;$['\r']
          )
          Ok(())
        }

        fn init(&mut self) -> anyhow::Result<()> {
          $(for block in &self.circuit.blocks =>
            $(self.block_init(&block))?;$['\r']
          )    
          Ok(())
        }

        fn step(&mut self, ssi: &$(&self.dynsys_core)::SystemStateInfo) -> anyhow::Result<()> {
          $(for block in &self.circuit.blocks =>
            $(self.block_step(&block))?;$['\r']
          )              
          Ok(())
        }
      }

      $(DocComment(["Implement RequirePeripherals protocol"]))
      impl<'a> $(&self.i_req_peripherals) for $(circuit_name)<'a> {
        type PeripheralsStruct = $(peripherals_type)<'a>;
      }

      $(DocComment(["Implement RequireStorage protocol"]))
      use const_default::ConstDefault;
      impl<'a> $(&self.i_req_storage) for $(circuit_name)<'a> {
        const SIZE: $(&self.dynsys_core)::StorageSize = 
          $(&self.dynsys_core)::StorageSize::DEFAULT$['\r']
            $(for block in &self.circuit.blocks =>
              $(self.block_add_size(&block)?)$['\r']
            )              
          ;
      }
      
    })
  }
}