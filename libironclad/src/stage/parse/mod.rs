//! Parses Erlang source into AST

use libironclad_erlang::erl_syntax::node::erl_binary_element::ValueWidth::Default;
use libironclad_erlang::error::ic_error::IcResult;
use libironclad_erlang::file_cache::FileCache;
use libironclad_erlang::project::module::ErlModule;
use libironclad_erlang::project::ErlProject;
use libironclad_erlang::stats::time_stats::{TimeStats, TimeStatsImpl};
use std::sync::RwLock;

/// Handles parsing loaded Erlang files in the project
pub struct ErlParseStage {}

impl ErlParseStage {
  /// Parse stage
  /// * Parse loaded ERL files as Erlang.
  /// Returns: Collection of AST trees for all affected ERL modules
  pub fn run(project: ErlProject, contents_cache: FileCache) -> IcResult<()> {
    let mut stage_time = TimeStatsImpl::default();

    if let Ok(contents_cache_r) = contents_cache.read() {
      for (path, source_file) in &contents_cache_r.all_files {
        let path_s = path.to_string_lossy();

        // Take only .erl and .hrl files
        if path_s.ends_with(".erl") || path_s.ends_with(".hrl") {
          let _compiler_opts = project.get_compiler_options_for(path);

          let mut file_time = TimeStatsImpl::default();
          let tok_stream = ErlModule::tokenize(
            &source_file.file_name,
            source_file.text.as_str(),
            Some(project.clone()),
          )?;

          file_time.stop_timer();
          // tok_stream.into_iter().for_each(|t| print!("{} ", t));
          print!("TOKENIZED {}: {}", path_s, file_time);

          // let mut parsed = ErlModule::from_module_source(
          //   &source_file.file_name,
          //   source_file.text.as_str(),
          //   Some(project.clone()),
          // )?;
          // parsed.compiler_options = compiler_opts;
        }
      }
    }

    stage_time.stop_timer();
    println!("PARSE stage: {}", stage_time);

    Ok(())
  }
}
