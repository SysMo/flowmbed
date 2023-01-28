use genco::prelude::{rust, quote};
use std::collections::HashMap;
use crate::dsl::references::{PackageImport};
use crate::util::QualifiedPath;

pub trait NameResolver {
  fn resolve_import(&self, path: &str) -> anyhow::Result<rust::Tokens>;
}


pub struct NameResolverImpl {
  import_map: HashMap<String, QualifiedPath>
}

impl NameResolverImpl {
  pub fn new(imports: &Vec<PackageImport>) -> Self {
    let mut import_map: HashMap::<String, QualifiedPath> = HashMap::new();
    for pkg_coll in imports {
      for path in &pkg_coll.paths {
        let qualified_path = path.qpath.prepend(&pkg_coll.package);
        let name = path.alias.clone().unwrap_or(qualified_path.name().to_owned());
        import_map.insert(name, qualified_path);
      }
    }
    NameResolverImpl { import_map }
  }
}


// impl From<Vec<PackageImport>> for NameResolverImpl {
//   fn from(x: Vec<PackageImport>) -> Self {
//     let mut imports: HashMap::<String, QualifiedPath> = HashMap::new();
//     for pkg_coll in x {
//       for path in pkg_coll.paths {
//         let qualified_path = path.qpath.prepend(&pkg_coll.package);
//         let name = path.alias.unwrap_or(qualified_path.name());
//         imports.insert(name, qualified_path);
//       }
//     }
//     NameResolverImpl { imports }
//   }
// }

impl NameResolver for NameResolverImpl {
  fn resolve_import(&self, path: &str) -> anyhow::Result<rust::Tokens> {
    let path: QualifiedPath = path.to_owned().try_into().unwrap();
    use log::*;
    let import = self.import_map.get(path.root());
    match import {
      Some(pkg_imp) => {
        debug!("{} - {}", pkg_imp.to_string(), path.rest().unwrap().to_string());
        let imp = rust::import(pkg_imp.parent().unwrap().to_string(), pkg_imp.name())
          .with_alias(path.root());
        if path.len() > 1 {
          Ok(quote!($(imp)::$(path.rest().unwrap().to_string())))  
        } else {
          Ok(quote!($(imp)))
        }
      },
      None => anyhow::bail!("Path {} could not be resolved", path.to_string())
    }
  }
}