use std::collections::{HashSet, VecDeque};

use crate::utils::read_lines;

fn read_schematic() -> Vec<Vec<u8>> {
    read_lines("src/day_3/input.txt")
        .expect("Could not read schematic!")
        .fold(Vec::new(), |mut acc, line| {
            acc.push(line.expect("Could not read schematic!").into_bytes());
            acc
        })
}

pub fn solution_1() {
    let schematic = read_schematic();
    let symbol_coords_list = schematic.iter().enumerate().flat_map(|(row, line)| {
        line.iter().enumerate().filter_map(move |(col, c)| {
            if !(c.is_ascii_digit() || *c == b'.') {
                Some((row, col))
            } else {
                None
            }
        })
    });

    let mut checked_coords_set = HashSet::new();
    let mut get_num = |coords: (usize, usize)| {
        let (row, mut col) = coords;
        let mut bytes = VecDeque::new();

        while let Some(c) = schematic.get(row).and_then(|row| row.get(col)) {
            if c.is_ascii_digit() && !checked_coords_set.contains(&(row, col)) {
                bytes.push_front(*c);
                checked_coords_set.insert((row, col));
                col += 1;
            } else {
                break;
            }
        }

        col = coords.1.checked_sub(1).unwrap_or(coords.1);

        while let Some(c) = schematic.get(row).and_then(|row| row.get(col)) {
            if c.is_ascii_digit() && !checked_coords_set.contains(&(row, col)) {
                bytes.push_back(*c);
                checked_coords_set.insert((row, col));
                col = col.saturating_sub(1);
            } else {
                break;
            }
        }

        // Digits were put in reverse order so powers of 10 multiplies them correctly
        bytes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                10_usize.pow(i.try_into().expect("Power is too big!")) * ((b - b'0') as usize)
            })
            .sum::<usize>()
    };

    let mut num_str = String::new();
    let sum = symbol_coords_list
        .fold(HashSet::new(), |mut nums, (row, col)| {
            for row in (row - 1)..=(row + 1) {
                for col in (col - 1)..=(col + 1) {
                    nums.insert(get_num((row, col)));
                }
            }

            nums
        })
        .iter()
        .enumerate()
        .map(|(i, num)| {
            num_str.push_str(format!("{}{num}", if i == 0 { "" } else { "\n" }).as_str());
            num
        })
        .sum::<usize>();
    std::fs::write("src/day_3/nums.txt", num_str).unwrap();

    // Create file to see which numbers got replaced
    let mut out_str = String::new();
    for (row, line) in schematic.iter().enumerate() {
        let line = line
            .iter()
            .enumerate()
            .map(|(col, c)| {
                (if checked_coords_set.contains(&(row, col)) {
                    b'x'
                } else {
                    *c
                }) as char
            })
            .collect::<String>();
        out_str.push_str(format!("{}{line}", if row == 0 { "" } else { "\n" }).as_str());
    }
    std::fs::write("src/day_3/out.txt", out_str).unwrap();

    println!("{sum}");
}
