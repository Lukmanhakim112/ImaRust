use std::io;

/**
 * Program ini, berguna untuk menghapus semua huruf vokal
 * pada sebuah teks.
 *
 * TP06 - Soal No. 2 [IMA]
 *
 */

fn trim_vocal(mut text: String) -> String {

    // Need refactor of this 2 lines of code
    text = text.replace(&['a', 'i', 'u', 'e', 'o'][..], "");
    text.replace(&['A', 'I', 'U', 'E', 'O'][..], "")

}

fn main(){
    let mut text = String::new();

    io::stdin().read_line(&mut text).unwrap();

    print!("{}", trim_vocal(text));

}
