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

    while i < report.len(){
        let val: Vec<&str> = report[i].split(' ').collect();

        //u8 was too small lol. Sure there's a better size to use but we'll go with i32 for now. 
        match val[0] {
            "forward" => horizontal += val[1].parse::<i32>().unwrap(),
            "down" => depth += val[1].parse::<i32>().unwrap(),
            "up" => depth -= val[1].parse::<i32>().unwrap(),
            _ => println!("Direction incorrect."),
        }
        //println!("{} -> {}", val[0], val[1]);
        i += 1;
    }

    println!("RESULTS: Horizontal = {} | Depth = {} | Multiplied they equal {}", horizontal, depth, (horizontal * depth));

    Ok(())
}