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
    count: u16,
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

const LOCO_LENGTH: i16 = 54;
const COAL_LENGTH: i16 = 30;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let (width, height) = size().expect("Unable to get terminal size");
    let colours = parse_flag(&args.flag);

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;

    execute!(stdout(), Hide, DisableLineWrap).expect("Unable to hide cursor");

    train_animation(width as i16, height as i16, args.count as i16, colours)
        .expect("Unable to animate train");

    execute!(stdout(), Clear(ClearType::All)).expect("Unable to clear screen");
    execute!(stdout(), Show, EnableLineWrap).expect("Unable to show cursor");
    Ok(())
}

fn train_animation(
    width: i16,
    height: i16,
    carriage_count: i16,
    colours: Vec<Color>,
) -> Result<(), Error> {
    let offset = (carriage_count + 1) * 30;
    let mut loco_finished = false;
    let mut carriages_started_to_finish = 0;
    let mut carriages_finished = 0;
    // for 10 .. 0
    for mut cursor_position_orig in (0..(width + offset)).rev() {
        // for 6 .. -4
        cursor_position_orig = cursor_position_orig - offset;
        if cursor_position_orig < 0 {
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
                    true if cursor_position_orig < 0
                        && cursor_position_orig.abs() < LOCO_LENGTH =>
                    {
                        LOCO_COLLECTION[i as usize]
                            .split_at(cursor_position_orig.abs() as usize)
                            .1
                    }
                    false => LOCO_COLLECTION[i as usize],
                    _ => "",
                };

                execute!(
                    stdout(),
                    MoveTo(cursor_position_default, (height - 10 + i) as u16),
                )
                .expect("Unable to move cursor");
                execute!(stdout(), Print(loco_print)).expect("Unable to print");
            } else {
                let wheel_print = match loco_finished {
                    true if cursor_position_orig < 0
                        && cursor_position_orig.abs() < LOCO_LENGTH =>
                    {
                        WHEEL_COLLECTION[i as usize - 7][(cursor_position_orig % 3).abs() as usize]
                            .split_at(cursor_position_orig.abs() as usize)
                            .1
                    }
                    false => WHEEL_COLLECTION[i as usize - 7][(cursor_position_orig % 3) as usize],
                    _ => "",
                };
                execute!(
                    stdout(),
                    MoveTo(cursor_position_default, (height - 10 + i) as u16),
                )
                .expect("Unable to move cursor");
                execute!(stdout(), Print(wheel_print)).expect("Unable to print");
            }
        }

        for carriage_index in 0..carriage_count {
            let idx = cursor_position_orig + LOCO_LENGTH + (carriage_index * COAL_LENGTH);
            if idx == 0 {
                carriages_started_to_finish += 1;
            }
            if idx == -COAL_LENGTH {
                carriages_finished += 1;
            }
            let carriage_cursor = match carriages_started_to_finish > carriage_index {
                true => 0,
                false => idx,
            };

            for i in 0..10 {
                let carriage = if carriages_finished > carriage_index {
                    ""
                } else {
                    get_carriage_printable(COAL_COLLECTION[i as usize], idx)
                };

                execute!(
                    stdout(),
                    MoveTo(carriage_cursor as u16, (height - 10) as u16 + i),
                    SetForegroundColor(colours[i as usize]),
                    Print(carriage),
                    ResetColor,
                )
                .expect("Unable to print");
            }
        }

        thread::sleep(time::Duration::from_millis(50));
    }
    Ok(())
}

fn parse_flag(flag: &FlagChoice) -> Vec<Color> {
    match flag {
        FlagChoice::Pride => flags::PRIDE.to_vec(),
        FlagChoice::Trans => flags::TRANS.to_vec(),
        FlagChoice::Auto => flags::PRIDE.to_vec(),
    }
}

fn get_carriage_printable(orig_str: &str, relative_position: i16) -> &str {
    match relative_position {
        x if x <= 0 && x.abs() < orig_str.len() as i16 => orig_str.split_at(x.abs() as usize).1,
        _ => orig_str,
    }
}
