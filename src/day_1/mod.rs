use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn solution_1() {
    if let Ok(lines) = read_lines("input.txt") {
        let sum = lines.fold(0_u16, |acc, line| {
            let line = line.unwrap_or_else(|_| String::new());
            let (half_1, half_2) = line.split_at(line.len() / 2);

            (match (
                half_1.bytes().find(u8::is_ascii_digit),
                half_2.bytes().rfind(u8::is_ascii_digit),
            ) {
                (Some(x), Some(y)) => process_digits(x, y),
                (Some(x), None) => process_digits(x, x),
                (None, Some(y)) => process_digits(y, y),
                _ => 0,
            }) + acc
        });

        println!("{sum}");
    }
}

fn process_digits(x: u8, y: u8) -> u16 {
    ((10 * (x - b'0')) + (y - b'0')).into()
}
