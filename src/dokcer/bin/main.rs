mod visualise;

use core::time;
use std::{
    io::stdout,
    sync::{atomic::AtomicBool, Arc},
    thread,
};

use clap::{arg, command, Parser};
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{size, Clear, ClearType, DisableLineWrap, EnableLineWrap},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Delay between characters in milliseconds
    #[arg(short, long, default_value = "Dockerfile")]
    file: String,
    /// Seed for the noise generator
    #[arg(short, long, default_value = "0")]
    seed: u32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let seed = match args.seed {
        0 => rand::random(),
        _ => args.seed,
    };

    let contents =
        std::fs::read_to_string(args.file).expect("Something went wrong reading the file");

    let (width, height) = size().expect("Unable to get terminal size");

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;

    execute!(stdout(), Hide, DisableLineWrap).expect("Unable to hide cursor");
    execute!(stdout(), Clear(ClearType::All)).expect("Unable to clear screen");

    contents.lines().for_each(|line| {
        if !line.trim().is_empty() || line.starts_with("#") {
            match get_instruction(line) {
                (InstructionType::From, rem) => {
                    visualise::from(seed, width as usize, height as usize, rem)
                }
                (x, _) => println!("unsupported instruction: {:?}", x),
            }
        }
    });

    thread::sleep(time::Duration::from_secs(5));

    execute!(stdout(), Clear(ClearType::All)).expect("Unable to clear screen");
    execute!(stdout(), Show, EnableLineWrap).expect("Unable to show cursor");

    Ok(())
}

fn get_instruction(line: &str) -> (InstructionType, String) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts[0] {
        "FROM" => (InstructionType::From, parts[1].to_string()),
        "RUN" => (InstructionType::Run, parts[1..].join("")),
        "CMD" => (InstructionType::Cmd, parts[1..].join("")),
        "LABEL" => (InstructionType::Label, parts[1..].join("")),
        "EXPOSE" => (InstructionType::Expose, parts[1..].join("")),
        "ENV" => (InstructionType::Env, parts[1..].join("")),
        "ADD" => (InstructionType::Add, parts[1..].join("")),
        "COPY" => (InstructionType::Copy, parts[1..].join("")),
        "ENTRYPOINT" => (InstructionType::Entrypoint, parts[1..].join("")),
        "VOLUME" => (InstructionType::Volume, parts[1..].join("")),
        "USER" => (InstructionType::User, parts[1..].join("")),
        "WORKDIR" => (InstructionType::Workdir, parts[1..].join("")),
        "ARG" => (InstructionType::Arg, parts[1..].join("")),
        "ONBUILD" => (InstructionType::Onbuild, parts[1..].join("")),
        "STOPSIGNAL" => (InstructionType::Stopsignal, parts[1..].join("")),
        "HEALTHCHECK" => (InstructionType::Healthcheck, parts[1..].join("")),
        "SHELL" => (InstructionType::Shell, parts[1..].join("")),
        _ => (InstructionType::From, parts[1..].join("")),
    }
}

#[derive(Debug)]
enum InstructionType {
    From,
    Run,
    Cmd,
    Label,
    Expose,
    Env,
    Add,
    Copy,
    Entrypoint,
    Volume,
    User,
    Workdir,
    Arg,
    Onbuild,
    Stopsignal,
    Healthcheck,
    Shell,
}
