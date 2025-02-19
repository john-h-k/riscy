use std::{error::Error, process::ExitCode};

use clap::Parser;
use core::Core;
use load::{LoadedElf, Segment};

mod core;
mod instruction;
mod load;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,

    #[arg(short, long)]
    entrypoint: Option<u64>,

    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let args = Args::parse();

    eprintln!("running {}...", args.file);

    let loaded = LoadedElf::load(&args.file)?;
    eprintln!("loaded elf with base 0x{:x}", loaded.base);

    let mut core = Core::new(loaded, args.entrypoint, args.debug);
    let info = core.run();

    Ok(ExitCode::from(info.return_code as u8))
}
