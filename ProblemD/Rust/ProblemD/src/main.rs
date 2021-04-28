fn main() {

    let el = get_input().parse::<i32>().unwrap();
    let sc = get_input().parse::<i32>().unwrap();

    if el <= sc {
        println!("{}", el * 10);
    }

    if sc < el {
        println!("{}", sc * 10);
    }

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