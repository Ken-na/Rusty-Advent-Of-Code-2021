use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

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

//4348570 is too high
//822163 is too low.. 

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}
fn main() -> Result<(), Error> {
    let mut dirty_input = read(File::open("./input.txt")?)?;
    let mut test_input: Vec<&str> = Vec::new();
    let mut results_input: Vec<&str> = Vec::new();

    /*let mut a = '0';
    let mut b = '0';
    let mut c = '0';
    let mut d = '0';
    let mut e = '0';
    let mut f = '0';
    let mut g = '0';*/

    let mut b = '0';
    let mut d = '0';

    let mut c = '0';
    let mut f = '0';

    let mut one_chars: Vec<char> = Vec::new();
    let mut four_chars: Vec<char> = Vec::new();
    let mut seven_chars: Vec<char> = Vec::new();
    let mut eight_chars: Vec<char> = Vec::new();

    let mut zero_chars: Vec<char> = Vec::new();
    let mut two_chars: Vec<char> = Vec::new();
    let mut three_chars: Vec<char> = Vec::new();
    let mut nine_chars: Vec<char> = Vec::new();

    let mut count = 0;
    let mut total = 0;

    for i in 0..dirty_input.len() {
        let val: Vec<&str> = dirty_input[i].split('|').collect();
        test_input.push(val[0]);
        results_input.push(val[1]);
        let input_split: Vec<&str> = val[0].split_whitespace().collect();
        let results_split: Vec<&str> = val[1].split_whitespace().collect();

        for j in 0..input_split.len(){
            let c = input_split[j].chars().count();
            let split_symbols: Vec<char> = input_split[j].chars().collect();
                
            //println!("{}: {}", input_split[j], c);

            if c == 2 { //one
                //count += 1;
                for j in 0..c {
                    one_chars.push(split_symbols[j]);
                    //println!("one_char push: {}", split_symbols[j]);
                }
                //println!("{:?}", one_chars);
            }else if c == 4 { //four
                for j in 0..c {
                    four_chars.push(split_symbols[j]);
                    //println!("four_chars push: {}", split_symbols[j]);
                }
            }else if c == 3 { //seven
                for j in 0..c {
                    seven_chars.push(split_symbols[j]);
                    //println!("seven_chars push: {}", split_symbols[j]);
                }
            }else if c == 7 { //eight
                for j in 0..c {
                    eight_chars.push(split_symbols[j]);
                    //println!("eight_chars push: {}", split_symbols[j]);
                }
            }
        }

        let mut not_in_common = four_chars.clone();

        for mut i in 0..one_chars.len(){
            let mut j = 0;

            while j < not_in_common.len() {
                if one_chars[i] == not_in_common[j]{
                    //pot = four_chars[i];
                    not_in_common.remove(j);
                    i = 0;
                    j = 0;
                }else{
                    j += 1;
                }
            }
        }

        //actually identity of b and D isnt important, but we can use it to figure out other symbols. 
        c = not_in_common[0];
        f = not_in_common[1];

        b = one_chars[0];
        d = one_chars[1];

        //println!("\nb: {} d: {}", b, d);
        //println!("c: {} f: {}", c, f);
        //println!("4: {:?} 1: {:?}", four_chars, one_chars);
        println!("\nspitout:\n");
        for i in 0..results_split.len(){
            let split_symbols: Vec<char> = results_split[i].chars().collect();

            //println!("test: {} - {} - 2", results_split.len(), i);
            //println!("test: {}", (results_split.len() - i - 2));

            match split_symbols.len() {
                2 => print!("{} + ", 1 * 10_u32.pow(((results_split.len() - i - 1) as u32).into())),
                //2 => total += 1 * 10_u32.pow(((results_split.len() - i - 1) as u32).into()),
                4 => print!("{} + ", 4 * 10_u32.pow(((results_split.len() - i - 1) as u32).into())),
                //4 => total += 4 * 10_u32.pow(((results_split.len() - i - 1) as u32).into()),
                3 => print!("{} + ", 7 * 10_u32.pow(((results_split.len() - i - 1) as u32).into())),
                //3 => total += 7 * 10_u32.pow(((results_split.len() - i - 1) as u32).into()),
                7 => print!("{} + ", 8 * 10_u32.pow(((results_split.len() - i - 1) as u32).into())),
                //7 => total += 8 * 10_u32.pow(((results_split.len() - i - 1) as u32).into()),


                //should be either five or six. 
                _ => {
                    //let mut position_modifier = 1.p;
                    let mut val: u32 = 0;
                    let mut contains_four_a = false; //four
                    let mut contains_four_b = false; //four

                    let mut contains_one_a = false; //one
                    let mut contains_one_b = false; //one 
                    for i in 0..split_symbols.len(){
                        //println!("i: {} char: {} len: {}", i, split_symbols[i], split_symbols.len());
                        //unsure why my match statement did not work here. 
                        /*match split_symbols[i]{
                            b => contains_four_a = true,
                            d => contains_four_b = true,

                            c => contains_one_a = true,
                            f => contains_one_b = true,
                            _ => println!("err in inner match"),
                        }*/

                        if split_symbols[i] == b {
                            //print!("(b){} == {}, ", split_symbols[i], b);
                            contains_four_a = true;
                        }
                        else if split_symbols[i] == d {
                            //print!("(d){} == {}, ", split_symbols[i], d);
                            contains_four_b = true;
                        }
                        else if split_symbols[i] == c {
                            //print!("(c){} == {}, ", split_symbols[i], c);
                            contains_one_a = true;
                        }
                        else if split_symbols[i] == f {
                            //print!("(f){} == {}, ", split_symbols[i], f);
                            contains_one_b = true;
                        }
                    }
                   

                    //println!("{} {} {} {}", contains_four_a, contains_four_b, contains_one_a, contains_one_b);
                    //println!("{} {} {} {}", b, d, c, f);

                    if split_symbols.len() == 5 {
                        if (contains_four_a ^ contains_four_b) && contains_one_a && contains_one_b {
                        //if ((contains_four_a && !contains_four_b) || (!contains_four_a && contains_four_b)) && contains_one_a && contains_one_b {

                            val = 3; //swapped with 3 as they were not being applied correctly.
                        }else if (contains_four_a ^ contains_four_b) && (contains_one_a ^ contains_one_b){
                            val = 2;
                        }else{
                            val = 5;
                        }
                    }else{ //6
                        if (contains_four_a ^ contains_four_b) && contains_one_a && contains_one_b {
                            val = 0;
                        }else if (contains_one_a ^ contains_one_b) && contains_four_a && contains_four_b {
                            val = 9;
                        }else{

                            val = 6;
                        }
                    }
                    
                    //print!("{} ", val);

                    //print!("{}({} {} {} {}) ", val, contains_one_a, contains_one_b, contains_four_a, contains_four_b);
                    //print!(" :{:?}: ",split_symbols);
                    //print!(" ({} {} {} {}) ", b, d, c, f);
                    //print!(" [{} {}] ", (contains_one_a ^ contains_one_b), (contains_four_a ^ contains_four_b));

                    total = val * 10_u32.pow(((results_split.len() - i - 1) as u32).into());
                    print!("{} + ", total);
                },
            }
        }
    }

    println!("TOTAL: {}", total);
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
    

    /*

    for i in 0..one_chars.len(){
        let mut not_in_common = seven_chars.clone();

        let mut j = 0;
        while j < not_in_common.len() {
            if one_chars[i] == not_in_common[j]{
                not_in_common.remove(j);
            }else{
                j += 1;
            }
        }
        a = not_in_common[0];
    }

    for i in 0..one_chars.len(){
        //let mut pot = '0';
        let mut not_in_common = four_chars.clone();

        let mut j = 0;
        while j < not_in_common.len() {
            if one_chars[i] == not_in_common[j]{
                //pot = four_chars[i];
                not_in_common.remove(j);
            }else{
                j += 1;
            }
        }
        b = not_in_common[0];
    }*/

    //println!("A: {}", a);
    //println!("B: {}", b);

    //println!("{} occurances", count);
    Ok(())
}