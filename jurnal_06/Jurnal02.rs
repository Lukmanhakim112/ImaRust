use std::io;

/**
 * Program ini, akan mengganti huruf
 * berdasar kan sisa bagi dari panjang lirik dibagi 5
 * sesuai dengan ketentukan berikut:
 *
 *  -------------------------------
 *  | Sisa Bagi | Huruf pengganti |
 *  -------------------------------
 *  |    0      |        A        |
 *  |    1      |        E        |
 *  |    2      |        I        |
 *  |    3      |        O        |
 *  |    4      |        U        |
 *  -------------------------------
 *
 * Jurnal 06 - No. 2 [IMA]
 */

fn main() {
    let mut lyrics = String::new();
    io::stdin().read_line(&mut lyrics).unwrap();

    let char_replace = vec!["a", "e", "i", "o", "u"];
    let lyrics_len = lyrics.trim_end().len() % 5;

    lyrics = lyrics.replace(&['a', 'i', 'u', 'e', 'o'][..], char_replace[lyrics_len]);

    println!("{}", lyrics);

}
