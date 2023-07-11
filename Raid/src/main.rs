use std::io;
mod hamming;


fn main() {
    let mut inputLineSizeDisks = String::new();
    println!("Введите колличество дисков: ");
    println!("Введите размер дисков: ")
    io::stdin().read_line(&mut inputLinesizeDisks);
    let mut inputSizeDisks = inputLinesizeDisks.trim().parse().expect("Input not an integer");
    let arrayOfVectorsDisks: [Vec, inputSizeDisks];
    let mut inputLineSizeDisk = String::new();
    let mut inputLineNumber = String::new();

    for pointerForArray in 0..inputSizeDisks {
        println!("Введите колличество данных ");
        io::stdin().read_line(&mut inputLineSizeDisk);
        let mut inputSizeDisk = inputLineSizeDisk.trim().parse().expect("Input not an integer");

        for pointerForVector in 0..inputSizeDisk {
            println!("Введите данные, числа");
            io::stdin().read_line(&mut inputNumber);
            let mut inputNumber = inputLineNumber.trim().parse().expect("Input not an integer");
            arrayOfVectorsDisks[pointerForArray].push(inputNumber);
        }
    }
    hamming.addEccDisks(arrayOfVectorsDisks);
}
