use std::{
    io::{self, BufReader},
    path::PathBuf,
};

use argh::FromArgs;
use ms_oforms::OFormsFile;

#[derive(FromArgs)]
/// Parse a VB form
struct Options {
    #[argh(positional)]
    /// a filename
    file: PathBuf,

    #[argh(option)]
    /// name of the form (e.g. `frmFoo`)
    form: PathBuf,
}

fn main() -> io::Result<()> {
    let opts: Options = argh::from_env();
    let file = std::fs::File::open(opts.file)?;
    let reader = BufReader::new(file);
    let mut oforms = OFormsFile::open_in(reader, opts.form)?;
    let c = oforms.root_comp_obj()?;
    println!("{:?}", c);
    let f = oforms.root_form()?;
    println!("{:#?}", f.form_control());
    Ok(())
}
