use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let report = read(File::open("./input.txt")?)?;
    let mut i = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    let mut aim = 0;

    while i < report.len(){
        let val: Vec<&str> = report[i].split(' ').collect();

        let  mut new_horizontal = 0;

        match val[0] {
            "forward" => new_horizontal = val[1].parse::<i32>().unwrap(),
            "down" => aim += val[1].parse::<i32>().unwrap(),
            "up" => aim -= val[1].parse::<i32>().unwrap(),
            _ => println!("Direction incorrect."),
        }
        horizontal += new_horizontal;

        depth += new_horizontal * aim;

        i += 1;
    }

    println!("RESULTS: Horizontal = {} | Depth = {} | Multiplied they equal {}", horizontal, depth, (horizontal * depth));

    Ok(())
}