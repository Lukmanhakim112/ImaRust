use std::io;

const PI: f64 = 3.1415926535897932384626433832795;

fn calculate_cone_area(radius: i32, height: i32) -> f64 {
    let area: f64 = f64::trunc(PI * radius as f64 *
        (radius as f64 + (height.pow(2) as f64 + radius.pow(2) as f64).sqrt()) * 1000.0) / 1000.0;

    area
}

fn main() {
    let mut input = String::new();
    let mut values = Vec::new();

    io::stdin().read_line(&mut input).unwrap();

    for i in input.split(" ") {
        values.push(i.trim_end().parse::<i32>().unwrap());
    }

    println!("{}", calculate_cone_area(values[0], values[1]));
}
