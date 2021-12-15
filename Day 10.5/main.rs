use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let input = read(File::open("./input.txt")?)?;
    let mut scores: Vec<u64> = Vec::new();

    for i in 0..input.len(){
        let chars: Vec<char> = input[i].chars().collect();
        let mut open_chars: Vec<char> = Vec::new();
        let mut corrupt = false;
        if chars[0] == ')' || chars[0] == ']' || chars[0] == '}' || chars[0] == '>' {
            corrupt = true;
        }else{
            for j in 0..input[i].len(){
                if !corrupt {
                    if chars[j] == '(' || chars[j] == '[' || chars[j] == '{' || chars[j] == '<' {
                        open_chars.push(chars[j]);
                    }
                    else{
                        let mut found = false;
                        if (open_chars[open_chars.len() - 1] == '(' && chars[j] == ')') || (open_chars[open_chars.len() - 1] == '[' && chars[j] == ']') || (open_chars[open_chars.len() - 1] == '{' && chars[j] == '}') || (open_chars[open_chars.len() - 1] == '<' && chars[j] == '>') {
                            found = true;
                        }
                        if !found{
                            corrupt = true;
                        }else{
                            open_chars.pop();
                        }
                    }
                }
            }
            if !corrupt {
                let mut new_score: u64 = 0;
                while open_chars.len() > 0 {
                    new_score *= 5;
                    let new_char = open_chars.pop().unwrap();
                    if new_char == '(' {
                        new_score += 1;
                    }else if new_char == '[' {
                        new_score += 2;
                    }else if new_char == '{' {
                        new_score += 3;
                    }else if new_char == '<' {
                        new_score += 4;
                    }
                }
                scores.push(new_score);
            }
        }
    }

    scores.sort();

    println!("MIDDLE SCORE: {}", scores[scores.len() / 2]);
    Ok(())
}