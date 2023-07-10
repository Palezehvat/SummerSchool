use std::io;
mod hamming;


fn main() {
    let mut inputLineSizeDisks = String::new();
    println!("Введите колличество дисков: ");
    io::stdin().read_line(&mut inputLinesizeDisks);
    let mut inputSizeDisks = inputLinesizeDisks.trim().parse().expect("Input not an integer");
    let vectorDisks: Vec<Vec> = Vec::new();
    let mut inputLineSizeDisk = String::new();
    
    for pointer in 0..inputSizeDisks {
        println!("Введите колличество данных ");
        io::stdin().read_line(&mut inputLinesizeDisk);
        let mut inputSizeDisk = inputLineSizeDisk.trim().parse().expect("Input not an integer");

    }
}
