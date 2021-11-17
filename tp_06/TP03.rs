use std::io;

/**
 * Program ini, berguna untuk mengkakulasi
 *
 * luas: persegi, persegi panjang,
 * segitiga siku-siku, lingkaran.
 *
 * TP06 - Soal No. 3 [IMA]
 *
 */

const PI: f32 = 3.14;

fn area_around_square(side: i32) {
    println!("{} {}", side*side, side*4);
}

fn area_around_rectangle(widht: i32, height: i32) {
    println!("{} {}", widht * height, 2 * (widht + height));
}

fn area_around_circle(diameter: i32) {
    // Truncate decimal places to 2
    let area: f32 = f32::trunc((diameter / 2).pow(2) as f32 * PI * 100.0) / 100.0;
    let around: f32 = f32::trunc(diameter as f32 * PI * 100.0) / 100.0;

    println!("{} {}", area, around);
}

fn area_around_triangel(base: i32, height: i32) {
    // Truncate decimal places to 2
    let around = base as f32 + height as f32 + ((base as f32).powf(2.0) + (height as f32).powf(2.0)).sqrt(); 

    println!("{} {}", (base * height) / 2, around);
}

fn input() -> Vec<i32> {
    let mut holder = String::new();
    let mut values = Vec::new();

    io::stdin().read_line(&mut holder).unwrap();

    for temp in holder.split(" ") {
        values.push(temp.trim_end().parse::<i32>().unwrap())

    }
    values
} 

fn main() {
    let mut choices = String::new();

    io::stdin().read_line(&mut choices).unwrap();
    let number: Vec<i32> = input();

    match choices.to_lowercase().trim_end() {
        "persegi" => {
            area_around_square(number[0]);
        },
        "persegi panjang" => {
            area_around_rectangle(number[0], number[1]);
        }
        "segitiga" => {
            area_around_triangel(number[0], number[1]);
        },
        "lingkaran" => {
            area_around_circle(number[0]);
        },
        _ => println!("Out of bound")
    }

}
