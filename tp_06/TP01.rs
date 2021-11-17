use std::io;


/**
 * Program ini, berguna untuk mengkonversi
 * suhu fahrenheit ke celcius.
 *
 * TP06 - Soal No. 1 [IMA]
 *
 */

fn fahr_to_celcius(celcius: &mut Vec<f32>) {
    for index in 0..celcius.len() {
        celcius[index] = (celcius[index] - 32.0) / 1.8;
    }
}

fn main(){
    let mut temps_holder = String::new();

    io::stdin().read_line(&mut temps_holder).unwrap();

    let mut fahrenhein_temp = Vec::new();
    for temp in temps_holder.split(" ") {
        fahrenhein_temp.push(temp.trim_end().parse::<f32>().unwrap())

    }

    fahr_to_celcius(&mut fahrenhein_temp);
    for temp in fahrenhein_temp {
        print!("{} ", temp);
    }

}
