use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

//my aim is to sort the list and choose the median, that appears to be what the example solution is. 
fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let dirty_input = read(File::open("./input.txt")?)?;
    let mut input: Vec<u16> = Vec::new();
    let dirty_vec: Vec<&str> = dirty_input[0].split(',').collect();
    let mut fuel_count = 0;

    for i in 0..dirty_vec.len() {
        input.push(dirty_vec[i].parse::<u16>().unwrap());
    }

    input.sort();

    let mut middle = input[input.len() / 2];

    println!("MIDDLE IS: {}", middle);

    for i in 0..input.len() {
        input.push(dirty_vec[i].parse::<u16>().unwrap());
        fuel_count += (input[i] as i32 - middle as i32).abs()
    }
    
    println!("Fuel Count: {}", fuel_count);

    Ok(())
}