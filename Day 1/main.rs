use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

//https://users.rust-lang.org/t/reading-integers-from-a-file-into-vector/17517
fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    //test, got correct answer.
    //let report: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let mut curr = 0;
    let mut i = 0;
    let mut strapp = "(N/A - no previous measurement)";
    let mut increased = 0;
    let mut decreased = 0;
    
    let report = read(File::open("./input.txt")?)?;

    while i < report.len(){
        
        if i == 0{
            curr = report[i];
        }else{
            if curr < report[i]{
                strapp = "(increased)";
                increased += 1;
            }else{
                strapp = "(decreased)";
                decreased += 1;
            }
        }
        
        print!("Num: {} {}\n", report[i], strapp);
        curr = report[i];
        i += 1;
    }

    print!("Increased: {} | Decreased: {}\n", increased, decreased);

    Ok(())
}