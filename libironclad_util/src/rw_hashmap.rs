//! Collection of MFArity which is lockable

use crate::mfarity::MFArity;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::sync::RwLock;

/// Collection of fun-arities, protected by a `RwLock`
#[derive(Debug)]
pub struct RwHashMap<KeyType, ValType> {
  /// The lockable hashmap
  pub collection: RwLock<HashMap<KeyType, ValType>>,
}

impl<KeyType: Display, ValType: Display> Display for RwHashMap<KeyType, ValType> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if let Ok(mut r_collection) = self.collection.read() {
      for (k, v) in r_collection.iter() {
        writeln!(f, "{} = {}; ", k, v)?;
      }
      Ok(())
    } else {
      panic!("Can't lock RwHashMap to print")
    }
  }
}

impl<KeyType, ValType> Default for RwHashMap<KeyType, ValType> {
  fn default() -> Self {
    Self { collection: RwLock::new(HashMap::default()) }
  }
}

impl<KeyType: Eq + Hash, ValType: Clone> RwHashMap<KeyType, ValType> {
  /// Contained data length
  pub fn len(&self) -> usize {
    if let Ok(r_collection) = self.collection.read() {
      r_collection.len()
    } else {
      panic!("Can't lock RwHashMap for length check")
    }
  }

  /// Check whether an item exists
  pub fn contains(&self, key: &KeyType) -> bool {
    if let Ok(mut w_collection) = self.collection.write() {
      w_collection.contains_key(key)
    } else {
      panic!("Can't lock RwHashMap for reading")
    }
  }

  /// Inserts an item into a set
  pub fn add(&self, key: KeyType, item: ValType) {
    if let Ok(mut w_collection) = self.collection.write() {
      w_collection.insert(key, item);
    } else {
      panic!("Can't lock RwHashMap to insert a new one")
    }
  }

  /// Retrieve an item
  pub fn get(&self, key: &KeyType) -> Option<ValType> {
    if let Ok(mut r_collection) = self.collection.read() {
      r_collection.get(key).cloned()
    } else {
      panic!("Can't lock RwHashMap to insert a new one")
    }
  }
}
