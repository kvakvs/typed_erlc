//! A variable with possibly missing name and unique typevar
#![cfg(coreast)]

use lazy_static::lazy_static;
use libironclad_erlsyntax::typing::erl_type::ErlType;
use libironclad_util::source_loc::SourceLoc;
use std::fmt::Formatter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[cfg(coreast)]
lazy_static! {
    /// Counter to create unique unique Var names
    static ref UNIQ_VAR_NUM: AtomicUsize = AtomicUsize::new(0);
}

/// Represents a variable with an optional name (`None` for generated variables), or a string name,
/// and a new unique type variable.
#[derive(Debug)]
#[cfg(coreast)]
pub struct Var {
  /// Source file pointer
  pub location: SourceLoc,
  /// Variable name, numbered unnamed variables are pre-formatted to strings, for simplicity
  pub name: String,
  /// Variable's deduced type, begins as `any()` and is narrowed down.
  pub ty: Arc<ErlType>,
}

#[cfg(coreast)]
impl Var {
  /// Creates a new prefixed variable with unique numbering
  pub fn new_unique(location: SourceLoc, prefix: &str) -> Self {
    let new_id = UNIQ_VAR_NUM.fetch_add(1, Ordering::Acquire);
    Self {
      location,
      name: format!("@{}{}", prefix, new_id),
      ty: ErlType::any(),
    }
  }

  /// A default guessed type for var is `any()` we will reiterate and make it more a narrow type at
  /// a later stage, as we learn more usage details.
  pub fn synthesize_type() -> Arc<ErlType> {
    ErlType::any()
  }
}

#[cfg(coreast)]
impl std::fmt::Display for Var {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Var({})", self.name)
  }
}
