use std::{error::Error, process::ExitCode};

use clap::Parser;
use core::{AlignedMemReader, Core32, MemReader, RunInfo, UnalignedMemReader};
use load::LoadedElf;

mod core;
mod instruction;
mod load;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,

    #[arg(short, long)]
    entrypoint: Option<u64>,

    #[arg(long)]
    assume_aligned: bool,

    #[arg(short, long, default_value = "16777215")]
    size: usize,

    #[arg(short, long)]
    debug: bool,
}

fn run_core32<Reader: MemReader<Idx = u32>>(
    elf: LoadedElf,
    entrypoint: Option<u64>,
    size: usize,
    debug: bool,
) -> RunInfo {
    let mut core = Core32::<Reader>::new(elf, entrypoint, size, debug);
    core.run()
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let args = Args::parse();

    eprintln!("running {}...", args.file);

    let loaded = LoadedElf::load(&args.file)?;
    eprintln!(
        "loaded elf with base {:#x}, entrypoint {:#x}",
        loaded.base, loaded.entrypoint
    );

    let info = if args.assume_aligned {
        run_core32::<AlignedMemReader<u32>>(loaded, args.entrypoint, args.size, args.debug)
    } else {
        run_core32::<UnalignedMemReader<u32>>(loaded, args.entrypoint, args.size, args.debug)
    };

    Ok(ExitCode::from(info.return_code as u8))
}
