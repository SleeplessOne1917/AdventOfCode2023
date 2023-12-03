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
    let symbol_indices = schematic.iter().enumerate().flat_map(|(y, line)| {
        line.iter().enumerate().filter_map(move |(x, c)| {
            if !(c.is_ascii_digit() || *c == b'.') {
                Some((y, x))
            } else {
                None
            }
        })
    });

    let mut checked_indices = HashSet::new();
    let mut get_num = |coords: (usize, usize)| {
        let (y, mut x) = coords;
        let mut bytes = VecDeque::new();

        while let Some(c) = schematic.get(y).and_then(|row| row.get(x)) {
            if c.is_ascii_digit() && !checked_indices.contains(&(y, x)) {
                bytes.push_front(*c);
                checked_indices.insert((y, x));
                x += 1;
            } else {
                break;
            }
        }

        x = coords.1.checked_sub(1).unwrap_or(coords.1);

        while let Some(c) = schematic.get(y).and_then(|row| row.get(x)) {
            if c.is_ascii_digit() && !checked_indices.contains(&(y, x)) {
                bytes.push_back(*c);
                checked_indices.insert((y, x));
                x = x.saturating_sub(1);
            } else {
                break;
            }
        }

        bytes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                10_usize.pow(i.try_into().expect("Power is too big!")) * ((b - b'0') as usize)
            })
            .sum::<usize>()
    };

    let sum = symbol_indices
        .flat_map(|(y, x)| {
            let mut nums = Vec::new();
            for y1 in (y - 1)..=(y + 1) {
                for x1 in (x - 1)..=(x + 1) {
                    let num = get_num((y1, x1));
                    if num > 0 {
                        println!("{num}");
                    }
                    nums.push(num);
                }
            }

            nums
        })
        .sum::<usize>();

    let mut foo = String::new();
    for (y, line) in schematic.iter().enumerate() {
        let line = line
            .iter()
            .enumerate()
            .map(|(x, c)| {
                (if checked_indices.contains(&(y, x)) {
                    b'x'
                } else {
                    *c
                }) as char
            })
            .collect::<String>();
        foo.push_str(format!("{}{line}", if y == 0 { "" } else { "\n" }).as_str());
    }
    std::fs::write("src/day_3/out.txt", foo).unwrap();

    println!("{sum}");
}
