use std::io;

/**
 * Program ini berguna untuk
 * menghitung hari yang jatuh pada hari ke-n
 * note: hari pertama adalah jumat
 *
 * TP 05 - No. 2 [IMA]
 */


fn main() {
    let mut input = String::new();

    let day_names = vec![
        "vendredi", "samedi", "dimanche",
        "lundi", "mardi", "mercredi", "jeudi"
    ];

    io::stdin().read_line(&mut input).unwrap();

    let index: i32 = input.trim_end().parse::<i32>().unwrap() % 7;

    println!("{}", day_names[index as usize]);
}

