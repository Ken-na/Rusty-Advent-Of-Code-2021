use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

struct Board {
    numbers: [i32; 25],
    activated: [bool; 25]
}

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}


fn main() -> Result<(), Error> {
    let input = read(File::open("./input.txt")?)?;
    let read_outs: Vec<&str> = input[0].split(',').collect();

    let boards_count = (input.len() - 1) / 6;

    let mut boards = Vec::new();

    println!("BOARDS COUNT: {}", boards_count);

    for i in 0..read_outs.len(){
        println!("{} = {}", i, read_outs[i]);
        let mut j = 3 + (6 * i);
        let mut num_array: [i32; 25];
        let mut pos = 0;
        while j < 3 + (6 * (i + 1)){


        }

        boards.push(Board {
            numbers: num_array,
            activated: [bool; 25]
        });
    }


/*
    for i in 1..input.len() {
        println!("{}    {}", i, input[i])
    }*/

    Ok(())
}