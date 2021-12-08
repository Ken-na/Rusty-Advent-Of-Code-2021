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

    //let mut count = 0;
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
        //let mut curr_string = curr.to_string();
        //println!("{} bigger than/equal to {}", ones, zeros);
        curr.push_str("1");

        //curr = &*curr_string;
        //curr = curr.to_string().push_str("1");
    }else{
        //println!("{} less than to {}", ones, zeros);
        curr.push_str("0");
    }

    i = 0;

    while i < vec.len() {
        while i < vec.len() && !vec[i].starts_with(&curr) {
            //println!("{} Does not start with {}", vec[i], curr);
            vec.remove(i);
        }

        //println!("{}", vec[i]);

        /*if vec[i].starts_with(&curr){
            println!("{}", vec[i]);
        }else{
            println!("{} Does not start with {}", vec[i], curr);
        }*/
        
        i += 1;
    }
    
    //just for debugging.
    i = 0;
    //println!("CURR: {} | COUNT = {}", curr, ones);
    while i < vec.len() {
        if vec[i].starts_with(&curr){
            //println!("{}", vec[i]);
        }
        
        i += 1;
    }


    //println!("--------------");

    if curr.len() < vec[0].len(){
        vec = get_oxygen_rating(vec, curr);
    }
    
    
    return vec;
}

fn main() -> Result<(), Error> {
    let report = read(File::open("./input.txt")?)?;
    let mut i = 0;
    let bit_length = report[i].len();

    let mut counts = vec![0; bit_length];


    let oxygen = get_oxygen_rating(report, "".to_string());

    println!("Oxygen: {}", oxygen[0]);

    Ok(())
}