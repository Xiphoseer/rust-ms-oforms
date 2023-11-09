use std::{
    io::{self, BufReader, Read},
    path::PathBuf,
};

use argh::FromArgs;
use ms_oforms::{
    controls::command_button::parse_command_button,
    properties::{FormEmbeddedActiveXControl, FormEmbeddedActiveXControlCached},
    OFormsFile,
};
use nom::error::VerboseError;

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
    let mut f = oforms.root_form()?;
    println!("{:#?}", f.form_control());
    let mut iter = f.site_iter();
    while let Some((ctrl, _depth, _control)) = iter.next() {
        let mut s = iter.site_stream()?;
        let mut buf = Vec::with_capacity(s.limit() as usize);
        s.read_to_end(&mut buf)?;
        println!("{:?} {}", ctrl, buf.len());

        if let FormEmbeddedActiveXControl::ControlCached(
            FormEmbeddedActiveXControlCached::CommandButton,
        ) = ctrl
        {
            let (_, btn) = parse_command_button::<VerboseError<_>>(&buf).unwrap();
            println!("{:?}", btn);
        }
    }
    Ok(())
}
