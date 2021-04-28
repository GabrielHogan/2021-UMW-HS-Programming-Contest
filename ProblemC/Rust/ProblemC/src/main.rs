fn main() {
    
    let i = get_input();
    let ib = get_input();

    let result = if i.len() > ib.len() {
        i.clone()
    } else if i.len() < ib.len() {
        ib.clone()
    } else {
        String::from("TIE")
    };
    println!("{}", result);

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