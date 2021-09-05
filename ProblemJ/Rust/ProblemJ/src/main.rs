fn main() {
    
    let cases = get_input_u();
    let mut testnumbers : Vec<i32> = vec![];

    for _ in 0..cases {
        testnumbers.push(get_input_i());
    }

    let mut total = 0;

    for x in 0..cases {
        total = 0;
        let current = testnumbers[x];
        for y in 1..current {
            if current % y == 0 {
                total += y;
                print!("{} ", y);
            }
        }
        if total == current {
            println!("YES");
        } else {
            println!("NO");
        }
    }
    

}

fn get_input_u() -> usize {
    
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

fn get_input_i() -> i32 {
    
    let mut line : String = String::new();
    let handle = std::io::stdin().read_line(&mut line);
    match handle {
        Ok(_) => line.trim().to_owned().parse::<i32>().unwrap(),
        Err(e) => {
            println!("There was an error {}", e);
            0
        },
    }
}