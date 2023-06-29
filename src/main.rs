use std::{error::Error, ffi::c_int, io, process::exit};

fn io_err<E: Into<Box<dyn Error + Send + Sync>>>(err: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err.into())
}

fn print_help() {
    println!("{}", include_str!("usage.txt"));
}

fn run() -> io::Result<()> {
    let args: Vec<_> = std::env::args_os().collect();

    let Some(arg_os) = args.get(1) else {
        print_help();
        return Ok(());
    };

    let arg = arg_os
        .to_str()
        .ok_or_else(|| io_err("argument is not valid utf-8."))?;

    if arg == "help" || arg == "--help" {
        print_help();
        return Ok(());
    }

    let exit_code: c_int = arg.parse::<c_int>().map_err(io_err)?;

    if exit_code < 128 {
        return Err(io_err("argument is smaller than 128."));
    }

    let signo = exit_code - 128;

    let signal = signal_hook::low_level::signal_name(signo)
        .ok_or_else(|| io_err(format!("{signo} is not a valid signal number.")))?;

    println!("{signal}");

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("which-sig: {err}");
        exit(1);
    }
}
