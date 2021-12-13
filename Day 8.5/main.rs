use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};


//VERY UGLY SOLUTION, HAD WRITTEN IT NICELY BUT DURING BUG HUNTING FORMATTING / MATCHES WENT IN THE LOO


/*
 aaaa
b    c
b    c
 dddd
e    f
e    f
 gggg
*/

//2 has 5 digits
//3 has 5 digits
//5 has 5 digits

//0 has 6 digits
//6 has 6 digits
//9 has 6 digits

//1 has 2 digits!
//4 has 4 digits!
//7 has 3 digits!
//8 has 7 digits!

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}
fn main() -> Result<(), Error> {
    let dirty_input = read(File::open("./input.txt")?)?;
    let mut test_input: Vec<&str> = Vec::new();
    let mut results_input: Vec<&str> = Vec::new();

    let mut total = 0;

    for i in 0..dirty_input.len() {
        let mut one_chars: Vec<char> = Vec::new();
        let mut four_chars: Vec<char> = Vec::new();


        let val: Vec<&str> = dirty_input[i].split('|').collect();
        test_input.push(val[0]);
        results_input.push(val[1]);
        let input_split: Vec<&str> = val[0].split_whitespace().collect();
        let results_split: Vec<&str> = val[1].split_whitespace().collect();

        for j in 0..input_split.len(){
            let c = input_split[j].chars().count();
            let split_symbols: Vec<char> = input_split[j].chars().collect();

            if c == 2 { //one
                for j in 0..c {
                    one_chars.push(split_symbols[j]);
                }
            }else if c == 4 { //four
                for j in 0..c {
                    four_chars.push(split_symbols[j]);
                }
            }
        }

        let mut not_in_common = four_chars.clone();
        for mut w in 0..one_chars.len(){
            let mut j = 0;

            while j < not_in_common.len() {
                if one_chars[w] == not_in_common[j]{
                    not_in_common.remove(j);
                    w = 0;
                    j = 0;
                }else{
                    j += 1;
                }
            }
        }

        //actually identity of c and f isnt important, but we can use it to figure out other symbols. 
        let c = one_chars[0];
        let f = one_chars[1];

        //println!("C/F {:?}", not_in_common);

        let b = not_in_common[0];
        let d = not_in_common[1];

        //println!("B/D {:?}", one_chars);

        for k in 0..results_split.len(){
            let split_symbols: Vec<char> = results_split[k].chars().collect();
            if split_symbols.len() == 2 {
                //print!("{} + ", 1 * 10_u32.pow(((results_split.len() - k - 1) as u32).into()));
                total += 1 * 10_u32.pow(((results_split.len() - k - 1) as u32).into());
            }else if split_symbols.len() == 4 {
                //print!("{} + ", 4 * 10_u32.pow(((results_split.len() - k - 1) as u32).into()));
                total += 4 * 10_u32.pow(((results_split.len() - k - 1) as u32).into());
            }else if split_symbols.len() == 3 {
                //print!("{} + ", 7 * 10_u32.pow(((results_split.len() - k - 1) as u32).into()));
                total += 7 * 10_u32.pow(((results_split.len() - k - 1) as u32).into());
            }else if split_symbols.len() == 7 {
                //print!("{} + ", 8 * 10_u32.pow(((results_split.len() - k - 1) as u32).into()));
                total += 8 * 10_u32.pow(((results_split.len() - k - 1) as u32).into());
            }else {
                let mut val: u32 = 0;
                let mut contains_four_a = false; //four
                let mut contains_four_b = false; //four

                let mut contains_one_a = false; //one
                let mut contains_one_b = false; //one 
                for j in 0..split_symbols.len(){
                    if split_symbols[j] == b {
                        //print!("(b / four_a {} == {}), ", split_symbols[j], b);
                        contains_four_a = true;
                    }
                    else if split_symbols[j] == d {
                        //print!("(d / four_b {} == {}), ", split_symbols[j], d);
                        contains_four_b = true;
                    }
                    else if split_symbols[j] == c {
                        //print!("(c / one_a {} == {}), ", split_symbols[j], c);
                        contains_one_a = true;
                    }
                    else if split_symbols[j] == f {
                        //print!("(f / one_b {} == {}), ", split_symbols[j], f);
                        contains_one_b = true;
                    }
                }
                
                if split_symbols.len() == 5 {
                    if (contains_four_a ^ contains_four_b) && contains_one_a && contains_one_b {
                        val = 3; 
                    }else if (contains_four_a ^ contains_four_b) && (contains_one_a ^ contains_one_b){
                        val = 2;
                    }else{
                        val = 5;
                    }
                }else{ //6
                    if (contains_four_a ^ contains_four_b) && contains_one_a && contains_one_b {
                        val = 0;
                    }else if (contains_one_a ^ contains_one_b) && contains_four_a && contains_four_b {
                        val = 6;
                    }else{

                        val = 9;
                    }
                }

                total += val * 10_u32.pow(((results_split.len() - k - 1) as u32).into());
                //print!("{} + ", new_total);
            }
            
        }
        //print!("\n\n");
    }

    println!("TOTAL: {}", total);

    Ok(())
}