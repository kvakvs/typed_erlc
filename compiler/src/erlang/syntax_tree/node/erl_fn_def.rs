//! Define a ErlFnDef struct for a new function AST node
use std::sync::{Arc, RwLock};
use crate::erl_error::ErlResult;
use crate::erlang::syntax_tree::node::erl_fn_clause::ErlFnClause;
use crate::mfarity::MFArity;
use crate::source_loc::SourceLoc;
use crate::typing::erl_type::ErlType;
use crate::typing::fn_clause_type::FnClauseType;
use crate::typing::fn_type::FnType;
use crate::typing::scope::Scope;

/// AST node which declares a new function. Contains function clauses. Names and arities on
/// all clauses must be equal and same as the function name.
#[derive(Debug)]
pub struct ErlFnDef {
  /// Source file pointer
  pub location: SourceLoc,
  /// Function name and arity, must be same for each clause (checked on clause insertion).
  /// Always local (`MFArity::module` is `None`)
  pub funarity: MFArity,
  /// Function clauses (non-empty vec)
  pub clauses: Vec<ErlFnClause>,
}

impl ErlFnDef {
  /// Create a new function definition AST node. Argument types vector is initialized with unions of
  /// all argument types.
  pub fn new(location: SourceLoc, funarity: MFArity, clauses: Vec<ErlFnClause>) -> Self {
    assert!(!clauses.is_empty(), "Cannot construct a function definition without clauses");
    Self {
      location,
      funarity,
      clauses,
    }
  }

  /// Produce `ErlType` for this function definition, with all clauses and their return types
  pub fn synthesize_function_type(&self, _scope: &RwLock<Scope>) -> ErlResult<Arc<ErlType>> {
    let clauses_r: ErlResult<Vec<FnClauseType>> = self.clauses.iter()
        .map(|fnc| fnc.synthesize_clause_type(&fnc.scope))
        .collect();
    let clauses = clauses_r?;

    let fn_type = FnType::new(self.funarity.arity, &clauses);
    let synthesized_t = ErlType::Fn(fn_type.into()).into();
    Ok(synthesized_t)
  }

  /// Produce a function return type, as union of all clauses returns
  pub fn synthesize_return_type(&self, _scope: &RwLock<Scope>) -> ErlResult<Arc<ErlType>> {
    // TODO: Filter out incompatible clauses
    let clauses_ret: ErlResult<Vec<Arc<ErlType>>> = self.clauses.iter()
        .map(|fnc| fnc.synthesize_clause_return_type(&fnc.scope))
        .collect();
    let synthesized_t = ErlType::new_union(&clauses_ret?);
    Ok(synthesized_t)
  }
}
