// use crate::*;

#[allow(unused_imports)]
use bpaf	::{*, long as l, short as s, positional as pos}; // short names to allow starting builders
use super::bpaf_ext::*;
use std::path::PathBuf;

#[derive(Debug,Clone)] pub struct Opt {pub paths:Vec<PathBuf>,}

// use owo_colors::OwoColorize;
pub fn options() -> OptionParser<Opt> {
  let paths	= pos::<PathBuf>("PATH").some("Expecting paths to dir/file(s)… (run with -h for help)");
  construct!(Opt {paths}).to_options()
    .version(env!("CARGO_PKG_VERSION"))
    .descr("Quick & dirty check of KDL files for invalid syntax (only v2 is supported)")
    // .header("")
    // .footer(&*format!("(only v2 is supported)"))
    .with_usage(|doc| {let mut u = Doc::default();/*u.emphasis("Use");u.text(": ");*/
      u.lit(env!("CARGO_BIN_NAME"));u.text(" ");u.doc(&doc);
      u
    })
}
