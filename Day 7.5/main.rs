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
    let mut input: Vec<u32> = Vec::new();
    let dirty_vec: Vec<&str> = dirty_input[0].split(',').collect();
    let mut fuel_count = 0;
    let mut pos_count = 0;

    let mut costs: Vec<u32> = Vec::new();
    let mut min = u32::MAX;
    let mut max = u32::MIN;

    let mut best_path = u32::MIN;
    let mut best_path_cost = i32::MAX;

    for i in 0..dirty_vec.len() {
        let val = dirty_vec[i].parse::<u32>().unwrap();
        input.push(val);

        //unsure if iter().min() etc would be faster. 
        if val > max {
            max = val;
        } else if val < min{
            min = val;
        }
    }

    for i in min..max{
        let mut val = 0;
        for j in 0..input.len(){
            //val += input[j] * i;
            let dist = (input[j] as i32 - i as i32).abs();
            val += (dist * (dist+1))/2;
        }
        println!("val: {}", val);
        if val < best_path_cost{
            best_path = i;
            best_path_cost = val;
        }
    }

    println!("MAX IS: {} | MIN: {} ", max, min);
    println!("MIDDLE IS: {} | FUEL COST: {} ", best_path, best_path_cost);
    
    Ok(())
}