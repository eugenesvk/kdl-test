// use crate::*;

#[allow(unused_imports)]
use bpaf	::{*, long as l, short as s, positional as pos}; // short names to allow starting builders
use super::bpaf_ext::*;
use std::path::PathBuf;

#[derive(Debug,Clone)] pub struct Opt {pub fmt:usize, pub paths:Vec<PathBuf>}

// use owo_colors::OwoColorize;
pub fn options() -> OptionParser<Opt> {
  let paths	= pos::<PathBuf>("PATH").some("Expecting paths to dir/file(s)… (run with -h for help)");

  let fmt	= s('f').l("fmt"   ).h({let mut d = Doc::default();d.text("Print formatted version:\n ");
    d.lit("ff");d.text("  to print in compact debug format\n ");
    d.lit("fff");d.text(" to print in expanded debug format");
    d}).switch().many().guard(|x| x.len() <= 3, "> 3 formatting flag repetitions")
    .map(|x| if x[0] {x.len()}else{x.len()-1});

  construct!(Opt {fmt, paths}).to_options()
    .version(env!("CARGO_PKG_VERSION"))
    .descr("Quick & dirty check of KDL files for invalid syntax (only v2 is supported)")
    // .header("")
    // .footer(&*format!("(only v2 is supported)"))
    .with_usage(|doc| {let mut u = Doc::default();/*u.emphasis("Use");u.text(": ");*/
      u.lit(env!("CARGO_BIN_NAME"));u.text(" ");u.doc(&doc);
      u
    })
}
