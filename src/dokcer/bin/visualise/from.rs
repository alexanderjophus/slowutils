use std::{io::stdout, thread};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use itertools::Itertools;
use noise::{utils::*, Abs, Perlin};
use unicode_segmentation::UnicodeSegmentation;

pub fn from(seed: u32, swidth: usize, sheight: usize, image: String) {
    let perlin = Perlin::new(seed);

    let (width, height) = (swidth * 2, sheight - 1);

    let mut map = PlaneMapBuilder::new(Abs::new(perlin))
        .set_size(width, height)
        .build()
        .iter()
        .map(|ns| match ns {
            0.0..=0.2 => ' ',
            0.2..=0.4 => '░',
            0.4..=0.6 => '▒',
            0.6..=0.8 => '▓',
            0.8..=1.0 => '█',
            _ => unreachable!(),
        })
        .chunks(width)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect::<Vec<String>>();

    let row = &mut map[height / 2];

    // Convert the row into graphemes for correct Unicode handling
    let graphemes: Vec<&str> = row.graphemes(true).collect();

    // Calculate the start and end indices using graphemes
    let start_index = width - ((swidth + image.graphemes(true).count()) / 2);
    let end_index = width - ((swidth - image.graphemes(true).count()) / 2);

    // Perform the replacement by joining the graphemes back into a string
    let mut new_row = String::new();
    new_row.push_str(&graphemes[..start_index].concat());
    new_row.push_str(image.as_str());
    new_row.push_str(&graphemes[end_index..].concat());

    // insert the image as a string in the middle of the rightmost view of the map
    map[height / 2] = new_row.clone();

    execute!(stdout(), MoveTo(0, 0), Print(format!("FROM: {}", image))).expect("Unable to print");

    // for width to scroll left to right
    for x in 0..width - swidth {
        for (y, row) in map.iter().enumerate() {
            let graphemes: Vec<&str> = row.graphemes(true).collect();
            let slice = &graphemes[x..x + swidth].concat();

            execute!(stdout(), MoveTo(0, y as u16 + 1), Print(slice),).unwrap();
        }
        thread::sleep(std::time::Duration::from_millis(50));
    }

    animate_bounding_box(image.clone(), swidth, height, 100, 0);

    execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All),).unwrap();

    for _ in 0..4 {
        for (y, row) in map.iter_mut().enumerate() {
            let graphemes: Vec<&str> = row.graphemes(true).collect();
            let graphemes = graphemes
                .iter()
                .map(|g| match g {
                    &"█" => "▓",
                    &"▓" => "▒",
                    &"▒" => "░",
                    &"░" => " ",
                    _ => g,
                })
                .collect::<Vec<&str>>();
            let slice = &graphemes[width - swidth..width].concat();
            let new_row = graphemes.concat();
            *row = new_row.clone();

            execute!(stdout(), MoveTo(0, y as u16 + 1), Print(slice),).unwrap();
        }
        animate_bounding_box(image.clone(), swidth, height, 0, 0);
        thread::sleep(std::time::Duration::from_millis(500));
    }

    for i in 1..5 {
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All),).unwrap();
        // write image name in the middle of the screen
        let middle_row: Vec<&str> = map[height / 2].graphemes(true).collect();
        execute!(
            stdout(),
            MoveTo(0, (height / 2) as u16 + 1),
            Print(middle_row[width - swidth..width].concat()),
        )
        .unwrap();
        animate_bounding_box(image.clone(), swidth, height, 0, i);
        thread::sleep(std::time::Duration::from_millis(500));
    }

    execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All)).expect("Unable to print");
}

fn animate_bounding_box(image: String, swidth: usize, height: usize, delay: u64, spacing: usize) {
    // animate a bounding box around the image name
    for i in 0..image.len() + 2 + (spacing * 2) {
        // draw top and bottom lines
        execute!(
            stdout(),
            MoveTo(
                (((swidth - image.len() + 1) / 2) + i - spacing) as u16,
                ((height - spacing) / 2) as u16
            ),
            Print("█"),
            MoveTo(
                (((swidth - image.len() + 1) / 2) + i - spacing) as u16,
                ((height + spacing) / 2) as u16 + 2
            ),
            Print("█"),
        )
        .unwrap();

        // draw left and right lines
        if i == 0 {
            for j in 0..spacing + 1 {
                execute!(
                    stdout(),
                    MoveTo(
                        (((swidth - image.len() + 1) / 2) - spacing) as u16,
                        (((height - spacing) / 2) + j) as u16 + 1
                    ),
                    Print("█"),
                )
                .unwrap();
            }
        }
        if i == image.len() + 1 + (spacing * 2) {
            for j in 0..spacing + 1 {
                execute!(
                    stdout(),
                    MoveTo(
                        (((swidth - image.len() + 1) / 2) + i - spacing) as u16,
                        (((height - spacing) / 2) + j) as u16 + 1
                    ),
                    Print("█"),
                )
                .unwrap();
            }
        }

        thread::sleep(std::time::Duration::from_millis(delay));
    }
}
