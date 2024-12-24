extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
// use ::h         	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;
// use crate::*;

use std::fs;
use std::path::PathBuf;

use kdl::KdlDocument;
use miette::Result;
use miette::IntoDiagnostic;

pub mod bpaf_ext;
pub mod cli_opt;
use cli_opt::*;

static _dbg:i8 = 1;
/// Quick and dirty way to disable blocks of debug-level code, use `if _d(1) {}` to do something only if global _dbg ≥ 1
pub fn _d(lvl:i8) -> bool {_dbg >= lvl}

pub fn main_cli() -> Result<()> {
  let opt = options().run();
  for kdl_file_path in &opt.paths {test_parse_kdl(kdl_file_path)?;}
  Ok(())
}

pub fn test_parse_kdl(kdl_file_path: &PathBuf) -> Result<()> {
  let kdl_doc_str: String = fs::read_to_string(kdl_file_path).into_diagnostic()?;
  let _doc: KdlDocument = kdl_doc_str.parse()?;
  println!("✓ parsed as KDL: {kdl_file_path:?}");
  Ok(())
}
