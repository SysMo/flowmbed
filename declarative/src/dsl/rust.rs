use std::collections::HashMap;

use super::references::QualifiedPath;
use genco::{tokens, Tokens};
use genco::prelude::Rust;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeReference {
  pub qpath: QualifiedPath,
  pub mutable: bool,
  pub is_trait: bool,
  pub lifetime: Option<String>,
  pub gats: HashMap<String, QualifiedPath>
}

impl tokens::FormatInto<Rust> for TypeReference {
  fn format_into(self, tokens: &mut Tokens<Rust>) {
      // tokens.push();
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
      if self.is_trait {
        tokens.append(tokens::static_literal("dyn"));
        tokens.space();
      }
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
  }
}
