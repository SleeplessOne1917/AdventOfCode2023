use std::{
    cell::OnceCell,
    collections::{HashMap, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    ops::ControlFlow::{Break, Continue},
    path::Path,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum NumType {
    Digit,
    DigitOrWord,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FromDirection {
    Left,
    Right,
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

const WORD_NUM_MAP: OnceCell<HashMap<&'static str, u8>> = OnceCell::new();

fn init_num_map() -> HashMap<&'static str, u8> {
    HashMap::from([
        ("one", b'1'),
        ("two", b'2'),
        ("three", b'3'),
        ("four", b'4'),
        ("five", b'5'),
        ("six", b'6'),
        ("seven", b'7'),
        ("eight", b'8'),
        ("nine", b'9'),
    ])
}

const MAX_WORD_LEN: usize = 5;

fn get_digit<I>(mut bytes: I, num_type: NumType, from_direction: FromDirection) -> Option<u8>
where
    I: Iterator<Item = u8>,
{
    let cf = bytes.try_fold(VecDeque::new(), |mut byte_queue, byte| {
        if byte.is_ascii_digit() {
            Break(byte)
        } else if num_type == NumType::DigitOrWord {
            if from_direction == FromDirection::Left {
                byte_queue.push_back(byte);
            } else {
                byte_queue.push_front(byte);
            }

            let word = byte_queue
                .iter()
                .map(|&byte| byte as char)
                .collect::<String>();

            for &key in WORD_NUM_MAP
                .get_or_init(init_num_map)
                .keys()
                .filter(|k| k.len() <= byte_queue.len())
            {
                if word.contains(key) {
                    return Break(*WORD_NUM_MAP.get_or_init(init_num_map).get(key).unwrap());
                }
            }

            if byte_queue.len() == MAX_WORD_LEN {
                if from_direction == FromDirection::Left {
                    byte_queue.pop_front();
                } else {
                    byte_queue.pop_back();
                }
            }

            Continue(byte_queue)
        } else {
            Continue(byte_queue)
        }
    });

    if let Break(byte) = cf {
        Some(byte)
    } else {
        None
    }
}

fn process_digits(x: u8, y: u8) -> u16 {
    ((10 * (x - b'0')) + (y - b'0')).into()
}

fn solution(num_type: NumType) {
    if let Ok(lines) = read_lines("src/day_1/input.txt") {
        let sum = lines.fold(0_u16, |acc, line| {
            let line = line.unwrap_or_else(|_| String::new());
            let bytes = line.bytes();
            let left = get_digit(bytes.clone(), num_type, FromDirection::Left).unwrap_or(b'0');
            let right = get_digit(bytes.rev(), num_type, FromDirection::Right).unwrap_or(left);

            acc + process_digits(left, right)
        });

        println!("{sum}");
    }
}

pub fn solution_1() {
    solution(NumType::Digit);
}

pub fn solution_2() {
    solution(NumType::DigitOrWord);
}
