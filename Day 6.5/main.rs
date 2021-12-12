use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn print_fish(fish: &mut [u64]){
    let mut count = 0;
    for i in 0..fish.len(){
        count += fish[i]
    }

    print!("{},", count);
}

fn print_fish_by_time(fish: &mut [u64]){
    for i in 0..fish.len(){
        print!("{}: {}, ", i, fish[i]);
    }
}

fn expand_fish(fish: &mut [u64]){
    let mut fish_to_add = 0;
    for i in 0..(fish.len() - 1){
        if i == 0 {
            fish_to_add = fish[i];
        }
        fish[i] = fish[i + 1];
    }
    fish[6] += fish_to_add;
    fish[8] = fish_to_add;
}

fn main() -> Result<(), Error> {
    let dirty_input = read(File::open("./input.txt")?)?;
    let mut input: [u64; 9] = [0; 9];
    let dirty_vec: Vec<&str> = dirty_input[0].split(',').collect();
    let days = 256;

    for i in 0..dirty_vec.len() {
        input[dirty_vec[i].parse::<usize>().unwrap()] += 1;
    }

    print!("Initial State:");
    print_fish_by_time(&mut input);
    println!("\n");

    for i in 0..days {
        expand_fish(&mut input);
        print!("(there are ");
        print_fish(&mut input);
        println!(" lanternfish at day {})", i + 1);
        //print_fish_by_time(&mut input);
        println!("\n");
    }
    Ok(())
}