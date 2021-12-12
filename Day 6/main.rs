use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn print_fish(fish: &mut Vec<u16>){
    for i in 0..fish.len(){
        print!("{},", fish[i]);
    }
    print!("\n");
}

fn expand_fish(fish: &mut Vec<u16>){
    let mut fish_to_add = 0;
    for i in 0..fish.len(){
        if fish[i] == 0 {
            fish_to_add += 1;
            fish[i] = 6;
        }else{
            fish[i] -= 1;
        }
    }

    for j in 0..fish_to_add{
        fish.push(8);
    }
}

fn main() -> Result<(), Error> {
    let dirty_input = read(File::open("./input.txt")?)?;
    let mut input: Vec<u16> = Vec::new();
    let dirty_vec: Vec<&str> = dirty_input[0].split(',').collect();
    let days = 256;

    for i in 0..dirty_vec.len() {
        input.push(dirty_vec[i].parse::<u16>().unwrap());
    }

    print!("Initial State:");
    print_fish(&mut input);

    for i in 0..days {
        expand_fish(&mut input);
        println!("(there are {} lanternfish at day {})", input.len(), i);
    }

    println!("Hell world");
    
    Ok(())
}