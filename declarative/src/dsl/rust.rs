use std::collections::HashMap;

use crate::util::QualifiedPath;
use genco::{tokens, Tokens};
use genco::prelude::{Rust, rust};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructTypeRef {
  pub qpath: QualifiedPath,
  pub mutable: bool,
  pub lifetime: Option<String>,
}

impl tokens::FormatInto<Rust> for StructTypeRef {
  fn format_into(self, tokens: &mut Tokens<Rust>) {
      tokens.append(tokens::static_literal("&"));

      if let Some(lt) = self.lifetime {
        tokens.append(tokens::static_literal("'"));
        tokens.append(&lt);
        tokens.space();
      }

      if self.mutable {
        tokens.append(tokens::static_literal("mut"));
        tokens.space();
      }

      tokens.append(self.qpath.to_string());
      // if !self.gats.is_empty() {
      //   tokens.append(tokens::static_literal("<"));
      //   let mut initial = true;
      //   for (k,v) in self.gats {
      //     if initial {
      //       initial = false
      //     } else {
      //       tokens.append(tokens::static_literal(", "));
      //     }
      //     tokens.append(k);
      //     tokens.append(tokens::static_literal(" = "));
      //     tokens.append(v.to_string());
      //   }
      //   tokens.append(tokens::static_literal(">"));
      // }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraitTypeRef {
  pub qpath: QualifiedPath,
  pub mutable: bool,
  pub lifetime: Option<String>,
  pub gats: HashMap<String, QualifiedPath>
}

impl tokens::FormatInto<Rust> for TraitTypeRef {
  fn format_into(self, tokens: &mut Tokens<Rust>) {
    let dyn_ref = rust::import("flowmbed_dynsys::core", "DynRef");
    let dyn_ref_mut = rust::import("flowmbed_dynsys::core", "DynRefMut");
    // Owned (on the heap) or Referenced
    if self.lifetime.is_some() {
      if self.mutable {
        tokens.append(dyn_ref_mut);
      } else {
        tokens.append(dyn_ref);
      }  
    } else {
      tokens.append(tokens::static_literal("Box"));
    }

    tokens.append(tokens::static_literal("<"));
    // Lifetime
    if let Some(lt) = &self.lifetime {
      tokens.append(tokens::static_literal("'"));
      tokens.append(lt);
      tokens.append(tokens::static_literal(","));
      tokens.space();
    }
    tokens.append(tokens::static_literal("dyn"));
    tokens.space();

    tokens.append(self.qpath.to_string());
    if !self.gats.is_empty() {
      tokens.append(tokens::static_literal("<"));
      let mut initial = true;
      for (k,v) in self.gats {
        if initial {
          initial = false
        } else {
          tokens.append(tokens::static_literal(", "));
        }
        tokens.append(k);
        tokens.append(tokens::static_literal(" = "));
        tokens.append(v.to_string());
      }
      tokens.append(tokens::static_literal(">"));
    }

    tokens.append(tokens::static_literal(">"));

  }
}

pub enum RustTypeRef {
  Struct(StructTypeRef), 
  Trait(TraitTypeRef)
}

impl tokens::FormatInto<Rust> for RustTypeRef {
  fn format_into(self, tokens: &mut Tokens<Rust>) {
      match self {
        RustTypeRef::Struct(x) => x.format_into(tokens),
        RustTypeRef::Trait(x) => x.format_into(tokens),
    }
  }
}