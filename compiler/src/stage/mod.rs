//! This package contains various compile stages
//! A stage takes project, and some input, and maybe some context data like defined macros.
//! A stage outputs something usable by the following stage.

pub mod file_contents_cache;
pub mod compile_module;
pub mod code_cache;

pub mod file_preload;
pub mod parse;
pub mod preprocess;
