//this is not going to be the most efficient solution, im sure there's something funky i can do to assess the bytes directly / avoid conversions. But alas, I am not at that point just yet.

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

use std::convert::TryInto;

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let report = read(File::open("./input.txt")?)?;
    let mut i = 0;
    let bit_length = report[i].len();

    let mut counts = vec![0; bit_length];

    while i < report.len(){
        let bytes = report[i].as_bytes();

        for j in 0..bit_length {
            if bytes[j] == 0b110001 {
                counts[j] += 1;
            }
        }

        i += 1;
    }

    let mut gamma = 0;

    //try into and unwrap are compiler suggestions.
    let max_size = 2_i32.pow((bit_length  - 1).try_into().unwrap());

    for i in 0..bit_length {
        if counts[i] > (report.len() / 2){
            if i != 0 {
                gamma += 2_i32.pow((bit_length - i - 1).try_into().unwrap()) as usize; //incorrect
            }else{
                gamma += max_size as usize;
            }
        }
    }

    let epsilon = (max_size as usize * 2) - 1 - gamma;

    /*print!("Frequency (brute): ");
    for i in 0..bit_length {
        print!("{}-", counts[i]);
    }
    print!("\n");*/

    println!("Gamma: {} ({:b})", gamma, gamma);
    println!("Epsilon: {} ({:b})", epsilon, epsilon);
    println!("Multiplied: {}", epsilon * gamma);

    Ok(())
}