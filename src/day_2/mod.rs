use std::rc::Rc;

use crate::utils::read_lines;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Bag {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

fn parse_num<'a, I>(mut group: I) -> usize
where
    I: Iterator<Item = &'a str>,
{
    group
        .next()
        .expect("Should be number")
        .parse::<usize>()
        .expect("Should be number")
}

fn process_lines() -> impl Iterator<Item = Rc<[Cube]>> {
    read_lines("src/day_2/input.txt")
        .expect("Could not read file")
        .map(|line| {
            let line = line.expect("Should be line");
            (&line[(line.find(':').unwrap() + 1)..])
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
                .collect::<Rc<[Cube]>>()
        })
        .into_iter()
}

pub fn solution_1() {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum: usize = process_lines()
        .enumerate()
        .map(|(i, line)| {
            let is_possible = line.iter().all(|cube| match cube {
                Cube::Red(c) => *c <= bag.red,
                Cube::Green(c) => *c <= bag.green,
                Cube::Blue(c) => *c <= bag.blue,
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

pub fn solution_2() {
    let powersum: usize = process_lines()
        .map(|line| {
            let bag = line.iter().fold(
                Bag {
                    red: 0,
                    blue: 0,
                    green: 0,
                },
                |mut bag, cube| {
                    match *cube {
                        Cube::Red(n) => {
                            if n > bag.red {
                                bag.red = n;
                            }
                        }
                        Cube::Green(n) => {
                            if n > bag.green {
                                bag.green = n;
                            }
                        }
                        Cube::Blue(n) => {
                            if n > bag.blue {
                                bag.blue = n;
                            }
                        }
                    }

                    bag
                },
            );

            bag.red * bag.blue * bag.green
        })
        .sum();

    println!("{powersum}");
}
