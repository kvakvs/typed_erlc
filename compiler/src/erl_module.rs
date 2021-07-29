//! Defines an Erlang module ready to be compiled

use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use pest::Parser;

use crate::funarity::FunArity;
use crate::project::compiler_opts::CompilerOpts;
use crate::syntaxtree::erl::erl_ast::{ErlAst};
use crate::typing::unifier::Unifier;
use crate::erl_error::{ErlError, ErlResult};
use crate::project::source_file::SourceFile;
use crate::syntaxtree::erl::erl_parser;
use std::ops::Deref;
use crate::syntaxtree::erl::node::literal_node::LiteralNode;
use std::collections::hash_map::Entry;

/// Erlang Module consists of
/// - List of forms: attributes, and Erlang functions
/// - Compiler options used to produce this module
pub struct ErlModule {
  /// Options used to build this module. Possibly just a ref to the main project's options
  pub compiler_options: Arc<CompilerOpts>,
  /// Module name atom, as a string
  pub name_atom: String,
  /// The file we're processing AND the file contents (owned by SourceFile)
  pub source_file: Arc<SourceFile>,
  /// AST tree of the module
  pub ast: Rc<ErlAst>,
  /// Type inference and typechecking engine, builds on the parsed AST
  pub unifier: Unifier,
  /// Function definitions of the module
  pub fun_table: HashMap<FunArity, Rc<ErlAst>>,
}


impl Default for ErlModule {
  fn default() -> Self {
    Self {
      compiler_options: Default::default(),
      name_atom: "".to_string(),
      source_file: Arc::new(SourceFile::default()),
      ast: Rc::new(ErlAst::Empty),
      unifier: Unifier::default(),
      fun_table: Default::default(),
    }
  }
}

impl Debug for ErlModule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ErlModule({})", self.name_atom)
  }
}

impl ErlModule {
  /// Create a new empty module
  pub fn new(opt: Arc<CompilerOpts>, source_file: Arc<SourceFile>) -> Self {
    Self {
      compiler_options: opt,
      name_atom: String::new(), // filled during parse
      source_file,
      ast: Rc::new(ErlAst::Empty), // filled after parse
      fun_table: Default::default(), // filled during parse
      unifier: Unifier::default(), // filled after parse
    }
  }

  /// Create a new empty module without compiler options and a hardcoded testing name
  pub fn new_testing() -> Self {
    Self {
      compiler_options: Arc::new(Default::default()),
      name_atom: String::new(), // filled during parse
      source_file: Arc::new(SourceFile::default()),
      ast: Rc::new(ErlAst::Empty), // filled after parse
      unifier: Unifier::default(), // filled after parse
      fun_table: Default::default(), // filled during parse
    }
  }

  /// Parse self.source_file
  pub fn parse_and_unify(&mut self) -> ErlResult<()> {
    let sf = self.source_file.clone(); // lure the borrow checker to letting us use text
    self.parse_and_unify_str(erl_parser::Rule::module, &sf.text)
  }

  /// Create a dummy sourcefile and parse it starting with the given parser rule.
  /// This updates the self.fun_table and self.ast
  pub fn parse_and_unify_str(&mut self, rule: erl_parser::Rule, input: &str) -> ErlResult<()> {
    let parse_output = match erl_parser::ErlParser::parse(rule, input) {
      Ok(mut root) => root.next().unwrap(),
      Err(bad) => {
        assert!(false, "Parse failed {}", bad);
        return Err(ErlError::from(bad));
      }
    };

    self.source_file = SourceFile::new(&PathBuf::from("<test>"), String::from(""));
    self.ast = self.postprocess_ast(self.to_ast_single_node(parse_output)?)?
        .unwrap();
    self.unifier = Unifier::new(&self.ast).unwrap();
    Ok(())
  }

  /// Create a dummy sourcefile and parse ANY given parser rule, do not call the unifier.
  /// This updates only self.ast
  pub fn parse_str(&mut self, rule: erl_parser::Rule, input: &str) -> ErlResult<()> {
    let parse_output = match erl_parser::ErlParser::parse(rule, input) {
      Ok(mut root) => root.next().unwrap(),
      Err(bad) => {
        assert!(false, "Parse str failed {}", bad);
        return Err(ErlError::from(bad));
      }
    };

    self.source_file = SourceFile::new(&PathBuf::from("<test>"), String::from(""));
    self.ast = self.postprocess_ast(self.to_ast_single_node(parse_output)?)?
        .unwrap();
    Ok(())
  }

  /// For an expression check whether it is a constant expression, and whether it points to some
  /// known function in this module. Arity is provided as the expression might be just an atom.
  pub fn find_function_expr_arity(&mut self, expr: &Rc<ErlAst>, arity: usize) -> Option<Rc<ErlAst>> {
    if let ErlAst::Lit(lit) = expr.deref() {
      // A single atom points to a possible existing function of `arity` in the current module
      if let LiteralNode::Atom(a) = lit {
        let fa = FunArity{ name: a.clone(), arity };

        match self.fun_table.entry(fa) {
          Entry::Occupied(e) => Some(e.get()), // found!
          Entry::Vacant(_) => None, // not found
        }
      }
    }
    None
  }
}
