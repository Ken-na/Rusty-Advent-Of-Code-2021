//A report so we dont need to account for user input. 
    //probably nice since i've only done Hello World for rust so far ahah.

fn main(){
    let report: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let mut curr = 0;
    let mut i = 0;
    let mut strapp = "(N/A - no previous measurement)";
    let mut increased = 0;
    let mut decreased = 0;
    
    //let mut smaller = false;

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
    /* needs import of slice library, will try with a pythony while loop
    for i in range(0, report.len()){
        println!(report[i]);
    }*/
}