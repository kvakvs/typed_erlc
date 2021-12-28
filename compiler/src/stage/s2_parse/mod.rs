//! Parses Erlang source into AST

use crate::project::ErlProject;
use std::sync::{Arc, Mutex};
use crate::stage::file_contents_cache::FileContentsCache;
use crate::erl_error::{ErlResult};
use crate::stage::code_cache::CodeCache;
use crate::project::module::Module;


/// Parse stage
/// * Parse loaded ERL files as Erlang.
/// Returns: Collection of AST trees for all affected ERL modules
pub fn run(project: &mut ErlProject,
           contents_cache: Arc<Mutex<FileContentsCache>>) -> ErlResult<Arc<Mutex<CodeCache>>> {
  // let mut ast_cache = ErlAstCache::new_empty();
  let code_cache = CodeCache::default();

  let contents_cache_r = contents_cache.lock().unwrap();

  for (path, source_file) in &contents_cache_r.all_files {
    let path_s = path.to_string_lossy();

    // Take only .erl and .hrl files
    if path_s.ends_with(".erl") || path_s.ends_with(".hrl") {
      let compile_options = project.get_compiler_options_for(path);

      let _parsed = Module::new(compile_options, source_file.clone());

      // module.parse_and_unify_erlang()?;
      // unimplemented!("Parse and unify entrypoint is notimpl");
      // code_cache.items.insert(parsed.name.clone(),
      //                         Arc::new(RwLock::new(parsed)));
    }
  }

  println!("Compiler processed {} sources (.erl and .hrl)", code_cache.items.len());

  let result = Arc::new(Mutex::new(code_cache));
  Ok(result)
}
