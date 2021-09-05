fn main() {
    let points = get_input();
    let periods = get_input();

    for top in (points - 1..periods).step_by(2) {
        println!("{}", " ".repeat((periods - top - 1)  / 2) + ".".repeat(top + 1).as_str());
    }

    for bottom in (points..periods - 1).step_by(2).rev() {
        println!("{}", " ".repeat((periods - bottom) / 2) + ".".repeat(bottom).as_str());
    }

}

fn get_input() -> usize {
    
    let mut line : String = String::new();
    let handle = std::io::stdin().read_line(&mut line);
    match handle {
        Ok(_) => line.trim().to_owned().parse::<usize>().unwrap(),
        Err(e) => {
            println!("There was an error {}", e);
            0
        },
    }
}