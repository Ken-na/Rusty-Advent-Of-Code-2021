//not very happy with the speed this performs at, haven't gone through to benchmark / figure out what's slowing it down, but it works for now. 

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn count_overlaps(map: Vec<u16>) -> u16{
    let mut count = 0;

    for i in map {
        if i > 1 {
            count += 1;
        }
        
    }

    return count;
}

fn add_map_point(xs: &mut Vec<u16>, ys: &mut Vec<u16>, counts: &mut Vec<u16>, new_x: u16, new_y: u16){
    for i in 0..xs.len() {
        if xs[i] == new_x && ys[i] == new_y{
            counts[i] += 1;
            return;
        }
    }

    xs.push(new_x);
    ys.push(new_y);
    counts.push(1);
}

fn main() -> Result<(), Error> {
    let input = read(File::open("./input.txt")?)?;

    let mut xs: Vec<u16> = Vec::new();
    let mut ys: Vec<u16> = Vec::new();
    let mut counts: Vec<u16> = Vec::new();

    for i in 0..input.len(){
        let read_outs: Vec<&str> = input[i].split(" -> ").collect();
        let controls_one: Vec<&str> = read_outs[0].split(',').collect();
        let controls_two: Vec<&str> = read_outs[1].split(',').collect();

        let x1 = controls_one[0].parse::<u16>().unwrap();
        let y1 = controls_one[1].parse::<u16>().unwrap();

        let x2 = controls_two[0].parse::<u16>().unwrap();
        let y2 = controls_two[1].parse::<u16>().unwrap();
        
        //moving y axis
        if x1 == x2 {
            let mut cursor = y1;
            while cursor != y2 {
                add_map_point(&mut xs, &mut ys, &mut counts, cursor, x1);
                if cursor > y2 {
                    cursor -= 1;
                }else{
                    cursor += 1;
                }
            }
            add_map_point(&mut xs, &mut ys, &mut counts, cursor, x1);
        }else if y1 == y2 {
            let mut cursor = x1;
            while cursor != x2 {
                add_map_point(&mut xs, &mut ys, &mut counts, y1, cursor);
                if cursor > x2 {
                    cursor -= 1;
                }else{
                    cursor += 1;
                }
            }
            add_map_point(&mut xs, &mut ys, &mut counts, y1, cursor);
        }
    }

    println!("SCORE: {}", count_overlaps(counts));
    Ok(())
}