use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use void::Void;
// use log::*;

use serde::de::{self, Visitor, MapAccess};
use serde::{Deserializer, Deserialize};

#[allow(dead_code)]
fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}


pub fn map2tuple<'a, T : Clone>(m: HashMap<String, T>) -> anyhow::Result<(String, T)> {
  if m.len() != 1 {
    anyhow::bail!("Expected hash-map with a single key-value pair")
  } else {
    let k = m.keys().next().unwrap();
    let v = m[k].clone();
    Ok((k.to_owned(), v))
  }
}

pub fn to_array<const N: usize, I, O>(mut iter: I) -> anyhow::Result<[O; N]> 
where
  I: Iterator<Item = O>,
  O: Default + Copy + core::fmt::Debug
{
  let mut res: [O; N] = [Default::default(); N];
  let mut i: usize = 0;
  loop {
    match iter.next() {
      Some(x) => {
        if i <= N - 1 {
          res[i] = x
        } else {
          anyhow::bail!("Too many elements in the iterator, required only {}", N)
        }    
      },
      None => {
        if i == N {
          break
        } else {
          anyhow::bail!("Not enough elements in the iterator, required {}, actual {}", N, i)
        }
      }
    }
    i += 1;
  }

  Ok(res)
}

// pub fn handle_error<T: Clone>(res: &anyhow::Result<T>, or: T) -> T {
//   match res {
//     Ok(v) => v.clone(),
//     Err(e) => {
//       error!("{}", e);
//       or
//     }
//   }
// }