fn main() {
    
    let gold_total = get_input().parse::<i32>().unwrap();
    
    let mut cost : Vec<i32> = get_input()
    .split(' ')
    .map(|s| 
        s
        .to_owned()
        .to_owned()
        .trim()
        .to_owned()
        .parse::<i32>()
        .unwrap()
    )
    .collect();

    let mut onboard : Vec<i32> = get_input()
                                    .split(' ')
                                    .map(|s| 
                                        s
                                        .to_owned()
                                        .to_owned()
                                        .trim()
                                        .to_owned()
                                        .parse::<i32>()
                                        .unwrap()
                                    )
                                    .collect();

    let mut needed : Vec<i32> =  get_input()
                                    .split(' ')
                                    .map(|s| 
                                        s
                                        .to_owned()
                                        .to_owned()
                                        .trim()
                                        .to_owned()
                                        .parse::<i32>()
                                        .unwrap()
                                    )
                                    .collect();

    let mut total_needed : Vec<i32> = vec![0; 5];
    for x in 0..5 {
        total_needed[x] = needed[x] - onboard[x];
    }

    let mut individual_cost : Vec<i32> = vec![0; 5];
    for x in 0..5 {
        individual_cost[x] = total_needed[x] * cost[x];
    }

    let mut sum = 0;
    for x in 0..5 {
        sum += individual_cost[x];
    }

    if gold_total >= sum {
        println!("{}", gold_total - sum);
    } else {
        println!("Captain, I need more gold pieces.")
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