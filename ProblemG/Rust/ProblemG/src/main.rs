use std::convert::TryFrom;


fn main() {

    let first_num_raw = get_input();
    let first_num = first_num_raw.parse::<u32>().unwrap();

    let mut square : Vec<Vec<u32>> = vec![];
    for _ in 0..first_num {
        let current_line = get_input();
        let vec_of_nums_raw : Vec<String> = current_line
            .split(' ')
            .map(|s|
                s.to_owned().to_owned()
            )
            .collect::<Vec<String>>();
        let vec_of_nums = vec_of_nums_raw.iter().map(|num| num.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        if vec_of_nums.len() > first_num as usize {
            panic!("More than original amount in square!");
        }
        square.push(vec_of_nums);
    }

    let mut paces = first_num - 1;

    let mut x : usize = 0;
    let mut y : usize = 0;
    let mut direction : u16 = 0;
    let mut turner = 2;
    let even : bool = first_num % 2 == 0;

    loop {
        for _ in 0..paces {
            println!("X is : {}, Y is : {}, current num : {} ", y, x, square[y][x]);
            match direction {
                0 => x += 1,
                1 => y += 1,
                2 => x -= 1,
                3 => y -= 1,
                _ => unreachable!(),
            };
        }
        if direction == turner {
            if paces <= 1 {
                println!("Breaking!");
                break;
            }
            paces -= 1;
            println!("Paces is now : {}", paces);
        }
        direction += 1;
        if direction > 3 {
            direction = 0;
            if even {
                turner -= 1;
                if turner < 0 {
                    turner = 3;
                }
                println!("Turner is {}", turner);
            }
        }
        println!("Turning to {} direction", direction);
    }


}


fn get_input() -> String {
    
    let mut line : String = String::new();
    let handle = std::io::stdin().read_line(&mut line);
    match handle {
        Ok(_) => line.trim().to_owned(),
        Err(e) => {
            println!("There was an error {}", e);
            String::from("0")
        },
    }
}


/*
    public static String get_input() {
        Scanner in = new Scanner(System.in);
        return in.nextLine();
    }
*/