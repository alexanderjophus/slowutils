use std::{io::stdout, thread};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, SetBackgroundColor},
    terminal::{Clear, ClearType},
};
use itertools::Itertools;
use noise::{utils::*, Abs, Perlin};

pub fn visualise(seed: u32, swidth: usize, sheight: usize, image: String) {
    // generate ascii map
    //
    // "spin" around globe
    //
    // slow down on given image
    let perlin = Perlin::new(seed);

    let (width, height) = (swidth * 3, sheight - 1);

    // iter so that a map of 12 by 3 is collected as a vec of vecs
    let mut map = PlaneMapBuilder::new(Abs::new(perlin))
        .set_size(width, height)
        .build()
        .iter()
        .map(|ns| match ns {
            0.0..=0.2 => ' ',
            0.2..=0.4 => '.',
            0.4..=0.6 => 'o',
            0.6..=0.8 => 'O',
            0.8..=1.0 => '@',
            _ => unreachable!(),
        })
        .chunks(width)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect::<Vec<String>>();

    // insert the image as a string in the middle of the rightmost view of the map
    map[height / 2].replace_range(
        width - ((swidth + image.len()) / 2)..width - ((swidth - image.len()) / 2),
        &image,
    );
    execute!(stdout(), MoveTo(0, 0), Print(format!("FROM: {}", image))).expect("Unable to print");

    // for width to scroll left to right
    for x in 0..width - swidth {
        for (y, row) in map.iter().enumerate() {
            execute!(
                stdout(),
                MoveTo(0, y as u16 + 1),
                Print(&row[x..x + swidth]),
            )
            .unwrap();
        }
        thread::sleep(std::time::Duration::from_millis(50));
    }
    // animate a bounding box around the center of the screen
    for i in 0..image.len() + 2 {
        execute!(
            stdout(),
            SetBackgroundColor(crossterm::style::Color::Blue),
            MoveTo(
                (((swidth - image.len() + 1) / 2) + i) as u16,
                (height / 2) as u16
            ),
            Print("="),
            MoveTo(
                (((swidth - image.len() + 1) / 2) + i) as u16,
                (height / 2) as u16 + 2
            ),
            Print("="),
        )
        .unwrap();
        if i == 0 || i == image.len() + 1 {
            execute!(
                stdout(),
                SetBackgroundColor(crossterm::style::Color::Blue),
                MoveTo(
                    (((swidth - image.len() + 1) / 2) + i) as u16,
                    (height / 2) as u16 + 1
                ),
                Print("|"),
            )
            .unwrap();
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
    execute!(
        stdout(),
        SetBackgroundColor(crossterm::style::Color::Reset),
        MoveTo(0, 0),
        Clear(ClearType::All),
    )
    .unwrap();
}
