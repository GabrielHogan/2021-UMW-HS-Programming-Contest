fn main() {
    let cases = get_line().parse::<usize>().unwrap();
    let mut inputs : Vec<Vec<i32>> = vec![];

    for _ in 0..cases {
        let n = get_line().parse::<usize>().unwrap();
        inputs.push(get_line().split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    }

    for ledger in inputs {
        let mut scoops = 0;

        for x in 0..ledger.len() - 2 {
            for y in x + 1..ledger.len() - 1 {
                for z in y + 1..ledger.len() {
                    if ledger[x] < ledger[y] && ledger[y] < ledger[z] {
                        scoops += 1;
                    }
                }
            }
        }
        println!("{}", scoops);
    }
}

fn get_line() -> String {
    
    let mut line : String = String::new();
    let handle = std::io::stdin().read_line(&mut line);
    match handle {
        Ok(_) => line.trim().to_owned(),
        Err(e) => panic!("There was an error {}", e),
    }
}
