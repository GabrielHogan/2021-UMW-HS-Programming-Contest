fn main() {
    let mut input : String;
    let mut year : i32;
    loop {
        input = String::default();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                year = match input.trim().parse::<i32>(){
                    Ok(n) => n,
                    Err(e) => {
                        println!("There was an error parsing '{}'! (error: {})", input.trim(), e);
                        println!("Please try again!");
                        continue;
                    }
                };
                break;
            },
            Err(e) => {
                println!("Unable to read input. Please try again (error {})", e);
                println!("Please try again!");
            }
        }
    }
    year = year - 2019;

    println!("{}", 9900 + (year * 50));
}
