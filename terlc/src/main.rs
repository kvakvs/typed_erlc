extern crate compiler;

use compiler::project::ErlProject;
use compiler::project::conf::ProjectConf;
use compiler::erl_error::{ErlResult};
use std::process::exit;

fn main_do() -> ErlResult<()> {
  // Test default project from ""
  let default_project: ErlProject = ProjectConf::from_string("")?.into();
  println!("default {:?}", default_project);

  let mut project: ErlProject = ProjectConf::from_project_file("test_project/erlproject.toml")?.into();
  println!("{:?}", project);
  project.file_set = project.build_file_list().unwrap();

  ErlProject::compile(project)
}

fn main() {
  match main_do() {
    Ok(_) => {
      println!("...finished.");
      exit(0);
    }
    Err(e) => {
      println!("Error occured: {}", e);
      exit(e.process_exit_code())
    }
  }
}
