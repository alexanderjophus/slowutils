mod colours;
mod flags;
mod train;

use std::io::{stdout, Error};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::{thread, time};

use clap::{arg, command, Parser, ValueEnum};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{size, Clear, ClearType, DisableLineWrap, EnableLineWrap},
};

use train::{COAL_COLLECTION, LOCO_COLLECTION, WHEEL_COLLECTION};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of carriages to display
    #[arg(short, long, default_value_t = 1)]
    count: usize,
    /// Displays the trains as pride flags
    #[arg(short, long, default_value = "auto")]
    flag: FlagChoice,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum FlagChoice {
    Auto,
    Pride,
    Trans,
}

const LOCO_LENGTH: u16 = 54;
const COAL_LENGTH: u16 = 30;

fn main() -> Result<(), Error> {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;

    let args = Args::parse();

    let (width, height) = size().expect("Unable to get terminal size");
    let (colours, _flag_length) = parse_flag(&args.flag);

    execute!(stdout(), Hide, DisableLineWrap).expect("Unable to hide cursor");

    train_animation(width, height, args.count, colours).expect("Unable to animate train");

    execute!(stdout(), Clear(ClearType::All)).expect("Unable to clear screen");
    execute!(stdout(), Show, EnableLineWrap).expect("Unable to show cursor");
    Ok(())
}

fn train_animation(
    width: u16,
    height: u16,
    carriage_count: usize,
    colours: Vec<Color>,
) -> Result<(), Error> {
    let offset = (carriage_count + 1) * 30;
    let mut loco_finished = false;
    let mut carriages_started_to_finish = 0;
    let mut carriages_finished = 0;
    for mut cursor_position_orig in (0..(width + offset as u16)).rev() {
        cursor_position_orig = cursor_position_orig - offset as u16;
        if cursor_position_orig == u16::MAX {
            loco_finished = true;
        }
        let cursor_position_default = match loco_finished {
            true => 0,
            false => cursor_position_orig as u16,
        };
        execute!(stdout(), Clear(ClearType::All)).expect("Unable to clear screen");

        for i in 0..10 {
            if i < 7 {
                let loco_print = match loco_finished {
                    true if u16::MAX - cursor_position_orig <= 54 => {
                        LOCO_COLLECTION[i as usize]
                            .split_at((u16::MAX - cursor_position_orig).into())
                            .1
                    }
                    false => LOCO_COLLECTION[i as usize],
                    _ => "",
                };

                execute!(stdout(), MoveTo(cursor_position_default, height - 10 + i))
                    .expect("Unable to move cursor");
                execute!(stdout(), Print(loco_print)).expect("Unable to print");
            } else {
                let wheel_print = match loco_finished {
                    true if u16::MAX - cursor_position_orig <= 54 => {
                        WHEEL_COLLECTION[i as usize - 7][(cursor_position_orig % 3) as usize]
                            .split_at((u16::MAX - cursor_position_orig).into())
                            .1
                    }
                    false => WHEEL_COLLECTION[i as usize - 7][(cursor_position_orig % 3) as usize],
                    _ => "",
                };
                execute!(stdout(), MoveTo(cursor_position_default, height - 10 + i))
                    .expect("Unable to move cursor");
                execute!(stdout(), Print(wheel_print)).expect("Unable to print");
            }
        }

        for carriage_index in 0..carriage_count {
            let idx = cursor_position_orig + LOCO_LENGTH + ((carriage_index as u16) * COAL_LENGTH);
            if idx == u16::MAX {
                carriages_started_to_finish += 1
            }
            if idx + COAL_LENGTH == u16::MAX {
                carriages_finished += 1
            }
            let carriage_cursor = match carriages_finished > carriage_index {
                true => 0,
                false if idx + COAL_LENGTH < idx => 0,
                _ => idx,
            };
            if carriages_finished > carriage_index && idx + COAL_LENGTH == u16::MAX {
                continue;
            }

            for i in 0..10 {
                execute!(
                    stdout(),
                    MoveTo(carriage_cursor, height - 10 + i),
                    SetForegroundColor(colours[carriage_index as usize]), // modulo flag_length
                    Print(get_carriage_printable(
                        COAL_COLLECTION[i as usize],
                        carriage_index,
                        cursor_position_orig,
                        carriages_started_to_finish > carriage_index
                    )),
                    ResetColor,
                )
                .expect("Unable to print");
            }
        }

        thread::sleep(time::Duration::from_millis(60));
    }
    Ok(())
}

fn parse_flag(flag: &FlagChoice) -> (Vec<Color>, usize) {
    match flag {
        FlagChoice::Pride => (flags::PRIDE.to_vec(), flags::PRIDE.len()),
        FlagChoice::Trans => (flags::TRANS.to_vec(), flags::TRANS.len()),
        FlagChoice::Auto => (flags::PRIDE.to_vec(), flags::PRIDE.len()),
    }
}

fn get_carriage_printable(
    orig_str: &str,
    index: usize,
    cursor_position_orig: u16,
    reached_start: bool,
) -> &str {
    let thing = u16::MAX - (cursor_position_orig + LOCO_LENGTH + ((index as u16) * COAL_LENGTH));
    match reached_start {
        true if thing <= COAL_LENGTH => orig_str.split_at(thing.into()).1,
        false => orig_str,
        _ => "",
    }
}
