use crate::dsl::circuit::CircuitConfig;
use crate::dsl::block::{BlockInstance, BlockInput, BlockOutput};
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

    // let create_fn = if block.parameters.is_empty() {
    //   quote!(new())
    // } else { 
    //   quote!()
    //  };
    
    let mut create_args  = vec![quote!(&mut builder)];
    block.peripherals.iter().for_each(|periph_ref|
      create_args.push(quote!(&mut $(peripherals_name).$(&periph_ref.0)))
    );
    let args = quote!($(for arg in create_args join (, ) => $arg));

    quote!($(&block.id): $(&self.block_library)::$(&block.kind)::$(self.create_fn(args, None)))
  }

  fn create_fn(&self, args: rust::Tokens, modifiers: Option<rust::Tokens>) -> rust::Tokens {
    if modifiers.is_none() {
      quote!(new($(args)))
    } else { 
      quote!(builder($(args))$(modifiers).into())
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
            $(for block in &self.circuit.blocks =>
              $(self.create_block(&block)),$['\r']
            )    
          };

          circuit
        }
      }
    })
  }
}