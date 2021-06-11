use std::fmt::Debug;
use std::fmt;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::types::ArcRw;
use crate::project::ErlProject;
use std::sync::Arc;
use crate::project::source_file::SourceFile;
use crate::erl_parse::Span;

/// While preprocessing source, the text is parsed into these segments
/// We are only interested in attributes (macros, conditionals, etc), macro pastes via ?MACRO and
/// comments where macros cannot occur. The rest of the text is parsed unchanged into tokens.
/// Lifetime note: Parse input string must live at least as long as this is alive
#[derive(Clone)]
pub enum PpAstNode {
  /// A % line comment
  Comment(Span),
  /// Any text
  Text(Span),
  /// Any attribute even with 0 args, -if(), -define(NAME, xxxxx)
  Attr { name: String, body: Option<Span> },
  /// Paste macro tokens/text as is, use as: ?NAME
  PasteMacro { name: String, body: Option<Span> },
  /// Paste macro arguments as is, use as: ??PARAM
  StringifyMacroParam { name: String },
  /// Included file from HRL cache
  IncludedFile(Arc<PpAstTree>),
}

impl PpAstNode {
  pub fn trim(s: &str) -> &str {
    const CLAMP_LENGTH: usize = 40;
    let trimmed = s.trim();
    if trimmed.len() <= CLAMP_LENGTH { return trimmed; }
    &trimmed[..CLAMP_LENGTH - 1]
  }

  pub fn fmt(&self, source_file: &SourceFile) -> String {
    match self {
      Self::Comment(s) => format!("%({})", Self::trim(s.text(source_file))),
      Self::Text(s) => format!("T({})", Self::trim(s.text(source_file))),
      Self::Attr { name, body } => format!("Attr({}, {:?})", name, body),
      Self::PasteMacro { name, body } => format!("?{}({:?})", name, body),
      Self::StringifyMacroParam { name } => format!("??{}", name),
      Self::IncludedFile(include_rc) => format!("include<{}>", include_rc.source.file_name.display()),
    }
  }
}

/// Lifetime note: Parse input string must live at least as long as this is alive
pub struct PpAstTree {
  pub source: Arc<SourceFile>,
  /// The parsed preprocessor syntax tree ready for inclusion
  pub nodes: Vec<PpAstNode>,
}

impl PpAstTree {
  /// Take ownership on source text
  pub fn new(source_file: Arc<SourceFile>, nodes: Vec<PpAstNode>) -> Self {
    PpAstTree { nodes, source: source_file }
  }
}

/// Stores HRL files parsed into PpAst tokens ready to be included into other files.
/// Lifetime note: Cache must live at least as long as parse trees are alive
pub struct PpAstCache {
  pub syntax_trees: HashMap<PathBuf, Arc<PpAstTree>>,
}

impl PpAstCache {
  pub fn new() -> Self {
    Self { syntax_trees: HashMap::with_capacity(ErlProject::DEFAULT_CAPACITY / 4) }
  }
}
