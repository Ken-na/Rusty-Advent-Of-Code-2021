use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

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

    let mut count = 0;

    for i in 0..dirty_input.len() {
        let val: Vec<&str> = dirty_input[i].split('|').collect();
        test_input.push(val[0]);
        results_input.push(val[1]);
        let results_split: Vec<&str> = val[1].split_whitespace().collect();

        for j in 0..results_split.len(){
            let c = results_split[j].chars().count();
            println!("{}: {}", results_split[j], c);

            if c == 2 || c == 4 || c == 3 || c == 7 {
                count += 1;
            }
        }
    }

    println!("{} occurances", count);
    Ok(())
}