extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;
// use crate::*;

use std::error::Error;
use std::result;
use std::fs;
use std::path::PathBuf;

use kdl::{KdlDocument, KdlValue};
use miette::Result;
use miette::IntoDiagnostic;

pub fn test_parse_kdl(kdl_file_path: &PathBuf) -> Result<()> {
  let kdl_doc_str: String = fs::read_to_string(kdl_file_path).into_diagnostic()?;
  let doc: KdlDocument = kdl_doc_str.parse()?;
  println!("✓ parsed as KDL: {kdl_file_path:?}");
  Ok(())
}
