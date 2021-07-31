//! Defines a type variable, a unique numbered unnamed variable used in the Erlang code typing
use std::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;
use std::fmt::Formatter;

/// A type variable for not-yet-inferred types or generic types
/// Contains a name, and the type inferred so far (starts with Any)
#[derive(Clone, PartialEq, Hash, Eq)]
pub struct TypeVar(usize);

lazy_static! {
    /// Counter to create unique TypeVar names
    static ref TYPEVAR_NUM: AtomicUsize = AtomicUsize::new(0);
    static ref SUBSCRIPT_NUMERICS: Vec<char> = vec!['₀','₁','₂','₃','₄','₅','₆','₇','₈','₉'];
}

impl TypeVar {
  // fn subscript(n: usize) -> String {
  //   format!("{}", n).drain(..)
  //       .map(|c| SUBSCRIPT_NUMERICS[c as usize - 48]) // guarantee the input is 0..9
  //       .collect()
  // }

}

impl std::fmt::Display for TypeVar {
  /// Format typevar as a nice string (sigma 𝞼 + number)
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "𝜎{}", self.0)
  }
}

impl Default for TypeVar {
  /// Create a new type variable with unique integer id (guarded by atomic usize)
  fn default() -> Self {
    let new_id = TYPEVAR_NUM.fetch_add(1, Ordering::Acquire);
    Self(new_id)
  }
}
