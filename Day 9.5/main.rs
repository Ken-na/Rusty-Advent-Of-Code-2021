use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}
fn main() -> Result<(), Error> {
    let dirty_input = read(File::open("./input.txt")?)?;
    let mut input: Vec<u16> = Vec::new();
    let width = dirty_input[0].len();
    let height = dirty_input.len();

    let mut risk = 0;

    for i in 0..dirty_input.len() {
        //let bytes = dirty_input[i].as_bytes();
        //let split_symbols: Vec<char> = dirty_input[i].chars().collect();

        for passed_char in dirty_input[i].chars(){
            input.push(passed_char.to_digit(10).unwrap() as u16);
            //print!("{} ", input[0]);
        }
        //print!("\n");
        //println!("{}", split_symbols[0]);

    }

    for i in 0..input.len(){
        if i < width { //top
            if i % width == 0{ //left
                if input[i] < input[i + width]
                && input[i] < input[i + 1] {
                    risk += input[i] + 1;
                    //print!("{}\n", input[i]);
                }
            }
            else if (i + 1) % width == 0{ //right
                if input[i] < input[i + width]
                && input[i] < input[i - 1] {
                    risk += input[i] + 1;
                    //print!("{}\n", input[i]);
                }
            }else{//middle
                if input[i] < input[i + width]
                && input[i] < input[i + 1] 
                && input[i] < input[i - 1] {
                    risk += input[i] + 1;
                    //print!("{}\n", input[i]);
                }
            }
            
        }
        else if i >= input.len() - width { //bot
            if i % width == 0{ //left
                if input[i] < input[i - width]
                && input[i] < input[i + 1] {
                    risk += input[i] + 1;
                    //print!("{}\n", input[i]);
                }
            }
            else if (i + 1) % width == 0{ //right
                if input[i] < input[i - width]
                && input[i] < input[i - 1] {
                    risk += input[i] + 1;
                    //print!("{}\n", input[i]);
                }
            }else{//middle
                if input[i] < input[i - width]
                && input[i] < input[i + 1] 
                && input[i] < input[i - 1] {
                    risk += input[i] + 1;
                    //print!("{}\n", input[i]);
                }
            }
        }else if i % width == 0{ //left
            //print!("{}\n", input[i]);
            if input[i] < input[i + width]
                && input[i] < input[i - width]
                && input[i] < input[i + 1] {
                risk += input[i] + 1;
                //print!("{}\n", input[i]);
            }
        }
        else if (i + 1) % width == 0{ //right
            //print!("{}\n", input[i]);
            if input[i] < input[i + width]
                && input[i] < input[i - width]
                && input[i] < input[i - 1] {
                risk += input[i] + 1;
                //print!("{}\n", input[i]);
            }
        }else{ //middle
            if input[i] < input[i + width]
            && input[i] < input[i - width]
            && input[i] < input[i + 1] 
            && input[i] < input[i - 1] {
            risk += input[i] + 1;
            //print!("{}\n", input[i]);
        }
        }
    }

    for i in 0..input.len() {
        //if i <
    }

    println!("COUNT: {}", risk);
    Ok(())
}