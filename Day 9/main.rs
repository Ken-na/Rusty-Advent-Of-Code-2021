use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

//depth first search, does not work, pretty sure breadth would though.
fn check_surrounding(input: &mut Vec<u16>, checked: &mut Vec<bool>, pos: usize, width: usize) -> u16{
    let mut count = 0;
    checked[pos] = true;
    
    //counts.push(check_surrounding(input, checked, i, width));
    if pos < width { //top
        if pos % width == 0{ //left
            if (input[pos] < input[pos + width] || checked[pos + width])
            && (input[pos] < input[pos + 1]  || checked[pos + 1]) {
                //risk += input[pos] + 1;
                count += 1;
                if !checked[pos + 1]{
                    count += check_surrounding(input, checked, pos + 1, width);
                }
                if !checked[pos + width]{
                    count += check_surrounding(input, checked, pos + width, width);
                }
            }
        }else if (pos + 1) % width == 0{ //right
            if (input[pos] < input[pos + width]  || checked[pos + width])
            && (input[pos] < input[pos - 1]  || checked[pos - 1]) {
                //risk += input[pos] + 1;
                count += 1;
                if !checked[pos + width]{
                    count += check_surrounding(input, checked, pos + width, width);
                }
                if !checked[pos - 1]{
                    count += check_surrounding(input, checked, pos - 1, width);
                }
            }
        }else{//middle
            if (input[pos] < input[pos + width]  || checked[pos + width])
            && (input[pos] < input[pos + 1]  || checked[pos + 1]) 
            && (input[pos] < input[pos - 1]  || checked[pos - 1]) {
                //risk += input[pos] + 1;
                count += 1;
                if !checked[pos + width]{
                    count += check_surrounding(input, checked, pos + width, width);
                }
                if !checked[pos + 1]{
                    count += check_surrounding(input, checked, pos + 1, width);
                }
                if !checked[pos - 1]{
                    count += check_surrounding(input, checked, pos - 1, width);
                }
            }
        }
    }else if pos >= input.len() - width { //bot
        if pos % width == 0{ //left
            if (input[pos] < input[pos - width]  || checked[pos - width])
            && (input[pos] < input[pos + 1]  || checked[pos + 1]) {
                //risk += input[pos] + 1;
                count += 1;
                if !checked[pos - width]{
                    count += check_surrounding(input, checked, pos - width, width);
                }
                if !checked[pos + 1]{
                    count += check_surrounding(input, checked, pos + 1, width);
                }
            }
        }else if (pos + 1) % width == 0{ //right
            if (input[pos] < input[pos - width] || checked[pos - width]) 
            && (input[pos] < input[pos - 1] || checked[pos - 1])  {
                //risk += input[pos] + 1;
                count += 1;
                if !checked[pos - width]{
                    count += check_surrounding(input, checked, pos - width, width);
                }
                if !checked[pos - 1]{
                    count += check_surrounding(input, checked, pos - 1, width);
                }
            }
        }else{//middle
            if (input[pos] < input[pos - width] || checked[pos - width]) 
            && (input[pos] < input[pos + 1] || checked[pos + 1]) 
            && (input[pos] < input[pos - 1] || checked[pos - 1])  {
                //risk += input[pos] + 1;
                count += 1;
                if !checked[pos - width]{
                    count += check_surrounding(input, checked, pos - width, width);
                }
                if !checked[pos + 1]{
                    count += check_surrounding(input, checked, pos + 1, width);
                }
                if !checked[pos - 1]{
                    count += check_surrounding(input, checked, pos - 1, width);
                }
            }
        }
    }else if pos % width == 0{ //left
        if (input[pos] < input[pos + width] || checked[pos + width]) 
        && (input[pos] < input[pos - width] || checked[pos - width]) 
        && (input[pos] < input[pos + 1] || checked[pos + 1])  {
            //risk += input[pos] + 1;
            count += 1;
            if !checked[pos + width]{
                count += check_surrounding(input, checked, pos + width, width);
            }
            if !checked[pos - width]{
                count += check_surrounding(input, checked, pos - width, width);
            }
            if !checked[pos + 1]{
                count += check_surrounding(input, checked, pos + 1, width);
            }
        }
    }else if (pos + 1) % width == 0{ //right
        if (input[pos] < input[pos + width] || checked[pos + width]) 
        && (input[pos] < input[pos - width] || checked[pos - width]) 
        && (input[pos] < input[pos - 1] || checked[pos - 1])  {
            //risk += input[pos] + 1;
            count += 1;
            if !checked[pos + width]{
                count += check_surrounding(input, checked, pos + width, width);
            }
            if !checked[pos - width]{
                count += check_surrounding(input, checked, pos - width, width);
            }
            if !checked[pos - 1]{
                count += check_surrounding(input, checked, pos - 1, width);
            }
        }
    }else{ //middle
        if (input[pos] < input[pos + width] || checked[pos + width]) 
        && (input[pos] < input[pos - width] || checked[pos - width]) 
        && (input[pos] < input[pos + 1] || checked[pos + 1])  
        && (input[pos] < input[pos - 1] || checked[pos - 1]) {
            //risk += input[pos] + 1;
            count += 1;
            if !checked[pos + width]{
                count += check_surrounding(input, checked, pos + width, width);
            }
            if !checked[pos - width]{
                count += check_surrounding(input, checked, pos - width, width);
            }
            if !checked[pos + 1]{
                count += check_surrounding(input, checked, pos + 1, width);
            }
            if !checked[pos - 1]{
                count += check_surrounding(input, checked, pos - 1, width);
            }
        }
    }

    return count;
}


fn main() -> Result<(), Error> {
    let dirty_input = read(File::open("./input.txt")?)?;
    let mut input: Vec<u16> = Vec::new();
    let mut counts: Vec<u16> = Vec::new();
    let mut checked: Vec<bool> = Vec::new();
    let width = dirty_input[0].len();
    let mut risk = 0;

    for i in 0..dirty_input.len() {
        for passed_char in dirty_input[i].chars(){
            input.push(passed_char.to_digit(10).unwrap() as u16);
            checked.push(false);
        }
    }

    for i in 0..input.len(){
        if !checked[i] {
            if i < width { //top
                if i % width == 0{ //left
                    if input[i] < input[i + width]
                    && input[i] < input[i + 1] {
                        //risk += input[i] + 1;
                        //checked[i] = true;
                        counts.push(check_surrounding(&mut input, &mut checked, i, width));
                    }
                }else if (i + 1) % width == 0{ //right
                    if input[i] < input[i + width]
                    && input[i] < input[i - 1] {
                        //risk += input[i] + 1;
                        //checked[i] = true;
                        counts.push(check_surrounding(&mut input, &mut checked, i, width));
                    }
                }else{//middle
                    if input[i] < input[i + width]
                    && input[i] < input[i + 1] 
                    && input[i] < input[i - 1] {
                        //risk += input[i] + 1;
                        //checked[i] = true;
                        counts.push(check_surrounding(&mut input, &mut checked, i, width));
                    }
                }
            }else if i >= input.len() - width { //bot
                if i % width == 0{ //left
                    if input[i] < input[i - width]
                    && input[i] < input[i + 1] {
                        //risk += input[i] + 1;
                        //checked[i] = true;
                        counts.push(check_surrounding(&mut input, &mut checked, i, width));
                    }
                }else if (i + 1) % width == 0{ //right
                    if input[i] < input[i - width]
                    && input[i] < input[i - 1] {
                        //risk += input[i] + 1;
                        //checked[i] = true;
                        counts.push(check_surrounding(&mut input, &mut checked, i, width));
                    }
                }else{//middle
                    if input[i] < input[i - width]
                    && input[i] < input[i + 1] 
                    && input[i] < input[i - 1] {
                        //risk += input[i] + 1;
                        //checked[i] = true;
                        counts.push(check_surrounding(&mut input, &mut checked, i, width));
                    }
                }
            }else if i % width == 0{ //left
                if input[i] < input[i + width]
                && input[i] < input[i - width]
                && input[i] < input[i + 1] {
                    //risk += input[i] + 1;
                    //checked[i] = true;
                    counts.push(check_surrounding(&mut input, &mut checked, i, width));
                }
            }else if (i + 1) % width == 0{ //right
                if input[i] < input[i + width]
                && input[i] < input[i - width]
                && input[i] < input[i - 1] {
                    //risk += input[i] + 1;
                    //checked[i] = true;
                    counts.push(check_surrounding(&mut input, &mut checked, i, width));
                }
            }else{ //middle
                if input[i] < input[i + width]
                && input[i] < input[i - width]
                && input[i] < input[i + 1] 
                && input[i] < input[i - 1] {
                    //risk += input[i] + 1;
                    //checked[i] = true;
                    counts.push(check_surrounding(&mut input, &mut checked, i, width));
                }
            }
        }
    }

    for i in 0..input.len(){
        if checked[i]{
            print!("[{}] ", input[i]);
        }else{
            print!(" {}  ", input[i]);
        }

        if (i + 1) % width == 0 {
            print!("\n");
        }
    }

    counts.sort();
    print!("\n");print!("\n");
    for i in 0..counts.len() {
        println!("{} \n", counts[i]);
    }

    println!("COUNT: {}", counts[counts.len() - 1] * counts[counts.len() - 2] * counts[counts.len() - 3]);
    Ok(())
}