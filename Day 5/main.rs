use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn print_boards(map: &[u16], width: usize, height: usize){
    for x in 0..width{
        for y in 0..height{
            print!("{} ", map[(y * width) + x]);
        }
        print!("\n");

    }
}

fn count_overlaps(map: &[u16], width: usize, height: usize) -> i32{
    let mut count = 0;

    for x in 0..width{
        for y in 0..height{
            if map[(y * width) + x] > 1 {
                count += 1;
            }
        }
    }

    return count;
}

fn main() -> Result<(), Error> {
    let input = read(File::open("./input.txt")?)?;
    let width = 1000;
    let height = 1000;
    println!("hello world");

    /*let mut x1s: Vec<i32> = Vec::new();
    let mut y1s: Vec<i32> = Vec::new();
    let mut x2s: Vec<i32> = Vec::new();
    let mut y2s: Vec<i32> = Vec::new();*/

    //let mut status: Vec<i32> = Vec::new();
    let mut map: [u16; 1000000] = [0; 1000000];
    //let mut map: [u16; 1000000] = [0; 1000000];

    //println!("hello world");
    //print_boards(&map, width, height);

    for i in 0..input.len(){
        let read_outs: Vec<&str> = input[i].split(" -> ").collect();
        let controls_one: Vec<&str> = read_outs[0].split(',').collect();
        let controls_two: Vec<&str> = read_outs[1].split(',').collect();

        let x1 = controls_one[0].parse::<usize>().unwrap();
        let y1 = controls_one[1].parse::<usize>().unwrap();

        let x2 = controls_two[0].parse::<usize>().unwrap();
        let y2 = controls_two[1].parse::<usize>().unwrap();


        //println!("--------------[ Step {} ]--------------", i);
        //println!("-------------- {} {} -> {} {} --------------", x1, y1, x2, y2);
        //println!("-------------- {} {} -> {} {} --------------", controls_one[0], controls_one[1], controls_two[0], controls_two[1]);

        //println!("");

        
        //moving y axis
        if x1 == x2 {
            let mut cursor = y1;
            while cursor != y2 {
                map[(cursor * width) + x1] += 1;

                if cursor > y2 {
                    cursor -= 1;
                }else{
                    cursor += 1;
                }
            }
            map[(cursor * width) + x1] += 1;
            
        }else if y1 == y2 {
            let mut cursor = x1;
            while cursor != x2 {
                map[(y1 * width) + cursor] += 1;
                
                if cursor > x2 {
                    cursor -= 1;
                }else{
                    cursor += 1;
                }
            }
            map[(y1 * width) + cursor] += 1;

        }

        /*for x in controls_one[0].parse::<usize>().unwrap()..controls_two[0].parse::<usize>().unwrap(){
            for y in controls_one[1].parse::<usize>().unwrap()..controls_two[1].parse::<usize>().unwrap(){
                map[(y * width) + x] += 1;
                println!("hit: {}", (y * width) + x);
            }
        }*/

        //println!("");
        

        //print_boards(&map, width, height);
    }

    println!("SCORE: {}", count_overlaps(&map, width, height));
    Ok(())
}