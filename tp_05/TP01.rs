use std::io;

/**
 * Program ini berguna untuk
 * mengkonversi bulan ke-n ke nama bulan
 * contoh: Bulan ke 4 == April
 *
 * TP 05 - No. 1 [IMA]
 */

fn main() {
    let mut input = String::new();

    let month_names = vec![
        "janvier", "fevrier", "mars", "avril", "mai", "juin", "jullet",
        "aout", "septembre", "octobre", "novembre", "decembre"
    ];

    io::stdin().read_line(&mut input).unwrap();

    let index: i32 = input.trim_end().parse::<i32>().unwrap() - 1;

    println!("{}", month_names[index as usize]);
}
