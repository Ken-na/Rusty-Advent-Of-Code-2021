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
    let mut score = 0;

    for i in 0..input.len(){
        let chars: Vec<char> = input[i].chars().collect();
        let mut open_chars: Vec<char> = Vec::new();
        let mut new_score = 0;
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
                            if chars[j] == ')'{
                                new_score += 3;
                            }else if chars[j] == ']'{
                                new_score += 57;
                            } else if chars[j] == '}'{
                                new_score += 1197;
                            }else if chars[j] == '>'{
                                new_score += 25137;
                            }
                            corrupt = true;
                        }else{
                            open_chars.pop();
                        }
                    }
                }
            }
            if corrupt {
                score += new_score;
                println!("(bad) {:?} | {}", input[i], new_score);
            }else{
                println!("(good) {:?} | {}", input[i], new_score);
            }
        }
    }
    println!("SCORE: {}", score);
    Ok(())
}