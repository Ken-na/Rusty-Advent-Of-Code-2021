use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);

    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn print_map(map: [u8; 100], width: usize){
    let mut pos = 0;
    while pos < 100{
        if map[pos] == 0{
            print!("({}) ", map[pos]);
        }else{
            print!("[{}] ", map[pos]);
        }
        pos += 1;

        if pos % width == 0 {
            print!("\n");
        }
    }
}

fn glow(map: &mut [u8; 100], width: usize, pos: usize){
    map[pos] += 1;
    if map[pos] > 9 && map[pos] < 100{
        map[pos] += 100;
        if pos % width != 0 {
            if pos as i8 - 1 >= 0{
                if map[pos - 1] <= 9{
                    glow(map, width, pos - 1);
                }
            }

            if (pos as i8 - 1) + (width as i8) < 100{
                if map[((pos as i8 - 1) + width as i8) as usize] <= 9{
                    glow(map, width, ((pos as i8 - 1) + width as i8) as usize);
                }
            }

            if pos as i8 - 1 - width as i8 >= 0{
                if map[(pos as i8 - 1 - width as i8) as usize] <= 9{
                    glow(map, width, pos - 1 - width);
                }
            }
        }
        if (pos % width) + 1 != 10 {
            if pos + 1 < 100{
                if map[pos + 1] <= 9{
                    glow(map, width, pos + 1);
                }
            }
            if pos + 1 + width < 100{
                if map[pos + 1 + width] <= 9{
                    glow(map, width, pos + 1 + width);
                }
            }
            if pos as i8 + 1 - width as i8 >= 0{
                if map[pos + 1 - width] <= 9{
                    glow(map, width, pos + 1 - width);
                }
            }
        }
        if pos as i8 - width as i8 >= 0{
            if map[pos - width] <= 9{
                glow(map, width, pos - width);
            }
        }
        if pos + width < 100{
            if map[pos + width] <= 9{
                glow(map, width, pos + width);
            }   
        }
    }
}

fn main() -> Result<(), Error> {
    let dirty_input = read(File::open("./input.txt")?)?;
    let mut input_arr: [u8; 100] = [0; 100];
    let steps = 1000;
    let width = 10;
    let mut glow_count = 0;

    let mut pos = 0;
    for i in 0..dirty_input.len(){
        for j in 0..dirty_input[i].len(){
            let dirty_char: Vec<char> = dirty_input[i].chars().collect();
            input_arr[pos]  = dirty_char[j].to_digit(10).unwrap() as u8;
            
            pos += 1;
        }
        
    }

    println!("Before any steps:");
    print_map(input_arr, width);

    let mut i = 0;
    while i < steps{
        let mut all_flash = true;

        for j in 0..100 {
            glow(&mut input_arr, width, j);
        }

        for j in 0..100 {
            if input_arr[j] > 9 {
                input_arr[j] = 0;
                glow_count += 1;
            }else {
                all_flash = false;
            }          
        }

        //part 2
        if all_flash{
            println!("\n\nAfter step {} ({} Glows):", i + 1, glow_count);
            print_map(input_arr, width);
            break;
        }

        i += 1;

        //Part 1.
        //println!("\n\nAfter step {} ({} Glows):", i, glow_count);
        //print_map(input_arr, width);
    }
    Ok(())
}