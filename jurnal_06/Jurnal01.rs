use std::io;

/**
 * Program ini, akan menyapa 2 nama
 * sesuai dengan waktu yang diberikan
 *
 * Jurnal 06 - No. 1 [IMA]
 */

fn main() {
    let mut name_01 = String::new();
    let mut name_02 = String::new();
    let mut greeting = String::new();
    

    io::stdin().read_line(&mut name_01).unwrap();
    io::stdin().read_line(&mut name_02).unwrap();
    io::stdin().read_line(&mut greeting).unwrap();

    let time = greeting.trim_end().parse::<i32>().unwrap();

    if time <= 11 {
        greeting = String::from("pagi");
    } else if time <= 14 {
        greeting = String::from("siang");
    } else if time <= 17 {
        greeting = String::from("sore");
    } else {
        greeting = String::from("malam");
    }

    println!("Halo, {}. Selamat {}.", name_01.trim_end(), greeting);
    println!("Halo, {}. Selamat {}.", name_02.trim_end(), greeting);

}
