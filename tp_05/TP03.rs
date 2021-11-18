use std::io;

/**
 * Program ini adalah sebuah 
 * kalkulator sederhana
 *
 * TP 05 - No. 3 [IMA]
 */

fn main() {
    let mut input = String::new();
    let mut numbers = Vec::new();
    let mut operator = String::new();

    io::stdin().read_line(&mut input).unwrap();

    for i in input.split_whitespace() {
        let data = i.trim_end().parse::<i32>(); 

        match data {
            Ok(data) => {
                numbers.push(data);
            },
            Err(..) => {
                operator = i.to_string();
            } 
        };
    }

    match operator.as_str() {
        "+" => println!("{}", numbers[0] + numbers[1]),
        "-" => println!("{}", numbers[0] - numbers[1]),
        "*" => println!("{}", numbers[0] * numbers[1]),
        "/" => println!("{}", numbers[0] / numbers[1]),
        _ => println!("Out of bound")
    }


}
