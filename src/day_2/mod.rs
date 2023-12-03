use crate::utils::read_lines;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cube {
    Red(u8),
    Green(u8),
    Blue(u8),
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Bag {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn parse_num<'a, I>(mut group: I) -> u8
where
    I: Iterator<Item = &'a str>,
{
    group
        .next()
        .expect("Should be number")
        .parse::<u8>()
        .expect("Should be number")
}

pub fn solution1() {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    if let Ok(lines) = read_lines("src/day_2/input.txt") {
        let sum: usize = lines
            .enumerate()
            .map(|(i, line)| {
                let line = line.expect("Should be line");
                let line = &line[(line.find(':').unwrap() + 1)..];
                let is_possible = line
                    .split(';')
                    .flat_map(|section| section.split(","))
                    .map(|group| {
                        let mut group = group.trim().split(' ');

                        match group.next_back().expect("Should be color") {
                            "red" => Cube::Red(parse_num(group)),
                            "green" => Cube::Green(parse_num(group)),
                            "blue" => Cube::Blue(parse_num(group)),
                            c @ _ => panic!("Color {c} not recognized"),
                        }
                    })
                    .all(|cube| match cube {
                        Cube::Red(c) => c <= bag.red,
                        Cube::Green(c) => c <= bag.green,
                        Cube::Blue(c) => c <= bag.blue,
                    });

                if is_possible {
                    i + 1
                } else {
                    0
                }
            })
            .sum();

        println!("{sum}");
    }
}
