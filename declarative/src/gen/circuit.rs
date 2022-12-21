use crate::dsl::circuit::CircuitConfig;
use crate::dsl::block::{BlockInstance, BlockInput, BlockOutput, Value};
use super::traits::CodeGenerator;
use genco::prelude::{rust, quote};

pub struct CircuitGenerator<'a> {
  pub circuit: &'a CircuitConfig,
  pub block_library: rust::Import,
  pub dynsys_core: rust::Import,
}

impl<'a> CircuitGenerator<'a> {
  pub fn new(circuit: &'a CircuitConfig) -> CircuitGenerator<'a> {
    CircuitGenerator {
      circuit,
      block_library: rust::import("flowmbed_dynsys", "block_library"),
      dynsys_core: rust::import("flowmbed_dynsys", "core")
    }
  }

  fn declare_block(&self, block: &BlockInstance) -> rust::Tokens {
    quote!($(&block.id): $(&self.block_library)::$(&block.kind)<'a>)
  }

  fn create_block(&self, block: &BlockInstance) -> rust::Tokens {
    
    let peripherals_name = "peripherals";
    
    let mut args  = vec![quote!(&mut builder)];
    block.peripherals.iter().for_each(|periph_ref|
      args.push(quote!(&mut $(peripherals_name).$(&periph_ref.0)))
    );

    let modifiers = block.parameters.iter().map(
      |(key, value)| {
        let value_str = match &value {
          &Value::Bool(x) => x.to_string(),
          &Value::Int(x) => x.to_string(),
          &Value::Float(x) => x.to_string(),
          &Value::String(x) => x.to_string(),
        };
        quote!(.$(key)($(value_str)))
      }
    ).collect::<Vec<_>>();

    quote!(
      $(&block.id): $(&self.block_library)::$(&block.kind)
        ::$(self.create_fn(args, modifiers)),
      )
  }

  fn create_fn(&self, args: Vec<rust::Tokens>, modifiers: Vec<rust::Tokens>) -> rust::Tokens {
    let arg_tokens = quote!($(for arg in args join (, ) => $arg));
    if modifiers.is_empty() {
      quote!(new($(arg_tokens)))
    } else {
      let modifier_tokens = quote!($(for modifier in modifiers join () => $modifier));
      quote!(builder($(arg_tokens))$(modifier_tokens).into())
     }
  }
}

impl<'a> CodeGenerator for CircuitGenerator<'a> {
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    let circuit_name = &format!("{}Blocks", "LedSystem");
    let peripherals_type = &format!("{}Peripherals", "LedSystem");

    Ok(quote!{
      struct $(circuit_name)<'a> {
        $(for block in &self.circuit.blocks =>
          $(self.declare_block(&block)),$['\r']
        )
      }
      
      impl<'a> $(circuit_name)<'a> {
        pub fn new<ST: $(&self.dynsys_core)::DefaultSystemStrorage>(
          storage: &'a ST, peripherals: &'a mut $(peripherals_type)
        ) -> $(circuit_name)<'a> {

          let mut builder = $(&self.dynsys_core)::SystemStorageBuilder::new(storage);

          let mut circuit = $(circuit_name) {
            $(for block in &self.circuit.blocks join () =>
              $(self.create_block(&block))
            )    
          };

          circuit
        }
      }
    })
  }
}