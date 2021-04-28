fn main() {
    
    let top = get_input().parse::<i32>().unwrap();

    for num in 1..top {
        match num {
            x if x % 3 == 0 && x % 5 == 0 => print!("Fizz Buzz"),
            x if x % 3 == 0 => print!("Fizz"),
            x if x % 5 == 0 => print!("Buzz"),
            _ => print!("{}", num)
        }
        match num {
            x if x % 10 == 0 => println!(","),
            _ => print!(", ")
        }
    }
    match top {
        x if x % 3 == 0 && x % 5 == 0 => print!("Fizz Buzz."),
        x if x % 3 == 0 => print!("Fizz."),
        x if x % 5 == 0 => print!("Buzz."),
        _ => print!("{}.", top)
    }
    println!()

}

fn get_input() -> String {
    loop {
        let mut input = String::default();
        match std::io::stdin().read_line(&mut input) {
            Ok(s) => return input.trim().to_owned(),
            Err(e) => {
                println!("Unable to read input. Please try again (error {})", e);
                println!("Please try again!");
            }
        }
    }
}