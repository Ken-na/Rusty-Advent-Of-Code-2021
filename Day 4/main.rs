use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

use std::convert::TryInto;

#[derive(Clone)]
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

fn print_boards(boards: Vec<Board>){
    //yucky printer of boards. 
    for i in 0..boards.len(){
        print!("\n");

        let mut pos = 0;
        while pos < boards[i].numbers.len(){

            print!("{} {} {} {} {}", boards[i].numbers[pos], boards[i].numbers[pos + 1], boards[i].numbers[pos + 2], boards[i].numbers[pos + 3], boards[i].numbers[pos + 4]);

            print!("\t");

            print!("{} {} {} {} {}", boards[i].activated[pos], boards[i].activated[pos + 1], boards[i].activated[pos + 2], boards[i].activated[pos + 3], boards[i].activated[pos + 4]);
            
            print!("\n");
            pos += 5;

        }
    }
}

fn check_num_against_boards(mut boards: Vec<Board>, num: i32) -> Vec<Board>{
    let boards_count = boards.len();
    let boards_numbers_count = boards[0].numbers.len();

    for i in 0..boards_count{
        let mut pos = 0;
        while pos < boards_numbers_count{
            if boards[i].numbers[pos] == num {
                boards[i].activated[pos] = true;
            }
            pos += 1;
        }
    }

    return boards;
}
fn check_boards_for_winners(boards: Vec<Board>) -> i32{
    let boards_count = boards.len();
    let boards_numbers_count = boards[0].activated.len();

    for i in 0..boards_count{
        let mut pos = 0;
        while pos < boards_numbers_count{

            if pos % 5 == 0
            && boards[i].activated[pos] 
            && boards[i].activated[pos + 1] 
            && boards[i].activated[pos + 2]
            && boards[i].activated[pos + 3]
            && boards[i].activated[pos + 4]{
                return i.try_into().unwrap();
            }else if pos < 5
            && boards[i].activated[pos] 
            && boards[i].activated[pos + 5] 
            && boards[i].activated[pos + 10]
            && boards[i].activated[pos + 15]
            && boards[i].activated[pos + 20]{
                return i.try_into().unwrap();
            }


            /*if boards[i].numbers[pos] == num {
                boards[i].activated[pos] = true;
            }*/
            pos += 1;
        }
    }

    return -1;
}

fn count_score(board: Board, winning_num: i32) -> i32{
    let boards_numbers_count = board.activated.len();
    let mut score = 0;

    for i in 0..boards_numbers_count{
        if !board.activated[i] {
            score += board.numbers[i];
        }
    }

    return score * winning_num;
}



fn main() -> Result<(), Error> {
    let input = read(File::open("./input.txt")?)?;
    let read_outs: Vec<&str> = input[0].split(',').collect();


    //create the boards. 
    let boards_count = (input.len() - 1) / 6;
    let mut boards: Vec<Board> = Vec::new();
    println!("BOARDS COUNT: {}", boards_count);
    for i in 0..boards_count{
        //println!("{} = {}", i, input[i]);

        let mut j = 2 + (6 * i);
        let mut num_array: [i32; 25] = [0; 25];
        let mut empty_bool: [bool; 25] = [false; 25];
        let mut pos = 0;

        while j < 2 + (6 * (i + 1)) && j < input.len(){
            //let bytes = input[j].as_bytes();
            let val: Vec<&str> = input[j].split_whitespace().collect();
            for k in 0..val.len(){
                num_array[pos] = val[k].parse::<i32>().unwrap();
                //print!("{} ", val[k]);
                pos += 1;
            }
            //print!("\n");
            j += 1;
        }

        boards.push(Board {
            numbers: num_array,
            activated: empty_bool
        });
    }
    
    //print_boards(boards);
    

        /*
    */
    
    let entries = boards[0].numbers.len();
    let mut finished = -1;
    println!("check: {}", read_outs.len());
    let mut i = 0;
    let mut score = 0;
    //let boards_count
    //run through the read outs. 
    while i < read_outs.len() && finished == -1{
        //println!("{} = {}", i, read_outs[i]);
        let num = read_outs[i].parse::<i32>().unwrap();
        println!("------------ [{}] ------------", i);
        
        boards = check_num_against_boards(boards, read_outs[i].parse::<i32>().unwrap());

        finished = check_boards_for_winners((& mut *boards).to_vec());

        print_boards((& mut *boards).to_vec());

        if finished != -1 {
            score = count_score(boards[finished as usize].clone(), num)
        }
        i += 1;
    }

    if finished != -1{
        println!("------------ [WINNER IS: {}, SCORE OF: {}] ------------", finished + 1, score);
    }else{
        println!("------------ [NO WINNER] ------------");
    }
    

/*
    for i in 1..input.len() {
        println!("{}    {}", i, input[i])
    }*/

    Ok(())
}