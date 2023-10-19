use std::io;

const COINS: [u32; 6] = [500, 100, 25, 10, 5, 1];

struct Coin {
    value: u32,
    count: u32,
}

impl Coin {
    fn new(value: u32) -> Coin {
        Coin { value, count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

fn read_money() -> u32 {
    let mut input = String::new();

    println!("How much money to change?");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Input not an integer.");
            std::process::exit(1);
        }
    }
}

fn main() {
    let money = read_money();
    let mut coins_info: Vec<Coin> = COINS.iter().map(|&value| Coin::new(value)).collect();

    let mut remaining_money = money;
    let mut counter = 0;

    for coin_info in coins_info.iter_mut() {
        while remaining_money >= coin_info.value {
            remaining_money -= coin_info.value;
            counter += 1;
            coin_info.increment();
        }
    }

    println!("Number of coins: {}", counter);
    for coin_info in coins_info.iter() {
        println!("{}: {}", coin_info.value, coin_info.count);
    }
}
