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

//not the most fancy solution, O(2n) speed.
fn get_oxygen_rating(mut vec: Vec<String>, mut curr: String) -> Vec<String>{
    let mut i = 0;
    let mut ones = 0;
    let mut zeros = 0;

    while i < vec.len() {
        let bytes = vec[i].as_bytes();
        if bytes[curr.len()] == 0b110001 {
            ones += 1;
        }else{
            zeros += 1;
        }
        
        i += 1;
    }

    if ones >= zeros {
        curr.push_str("1");
    }else{
        curr.push_str("0");
    }

    i = 0;

    while i < vec.len() {
        while i < vec.len() && !vec[i].starts_with(&curr) {
            vec.remove(i);
        }
        
        i += 1;
    }

    if vec.len() > 1 &&  curr.len() < vec[0].len(){
        vec = get_oxygen_rating(vec, curr);
    }
    
    return vec;
}


//not the most fancy solution, O(2n) speed.
fn get_co2_rating(mut vec: Vec<String>, mut curr: String) -> Vec<String>{
    let mut i = 0;
    let mut ones = 0;
    let mut zeros = 0;

    while i < vec.len() {
        let bytes = vec[i].as_bytes();
        if bytes[curr.len()] == 0b110001 {
            ones += 1;
        }else{
            zeros += 1;
        }
        
        i += 1;
    }

    if ones < zeros {
        curr.push_str("1");
    }else{
        curr.push_str("0");
    }

    i = 0;

    while i < vec.len() {
        while i < vec.len() && !vec[i].starts_with(&curr) {
            vec.remove(i);
        }
        
        i += 1;
    }
    /*
    //just for debugging.
    i = 0;
    println!("CURR: {} | COUNT = {}", curr, ones);
    while i < vec.len() {
        if vec[i].starts_with(&curr){
            println!("{}", vec[i]);
        }
        
        i += 1;
    }*/

    if vec.len() > 1 && curr.len() < vec[0].len(){
        vec = get_co2_rating(vec, curr);
    }
    
    return vec;
}

fn convert_stringbinary_to_int(vec: String) -> usize{

    let bit_length = vec.len();
    let bytes = vec.as_bytes();
    let mut val = 0;

    //try into and unwrap are compiler suggestions.
    let max_size = 2_i32.pow((bit_length  - 1).try_into().unwrap());

    //println!("VAL: {} | bit_length = {} | max_size = {}", vec, bit_length, max_size);

    for i in 0..bit_length {
        if bytes[i] == 0b110001{
            if i != 0 {
                val += 2_i32.pow((bit_length - i - 1).try_into().unwrap()) as usize; //incorrect
            }else{
                val += max_size as usize;
            }
        }
    }

    return val;
}

fn main() -> Result<(), Error> {
    let report = read(File::open("./input.txt")?)?;
    let second_report = report.clone();
    let mut i = 0;
    let bit_length = report[i].len();

    let mut counts = vec![0; bit_length];


    let oxygen = get_oxygen_rating(report, "".to_string());
    let co2 = get_co2_rating(second_report, "".to_string());

    println!("Oxygen: {}", oxygen[0]);
    println!("CO2: {}", co2[0]);
    let oxygen_value = convert_stringbinary_to_int(oxygen[0].to_string());
    let co2_value = convert_stringbinary_to_int(co2[0].to_string());

    println!("{} * {} = {}", oxygen_value, co2_value, co2_value * oxygen_value);

    Ok(())
}