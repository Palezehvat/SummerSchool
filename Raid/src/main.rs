use std::io;
//mod hamming;


fn main() {
    let mut input_line_size_disks = String::new();
    let mut input_line_size_disk = String::new();
    println!("Введите колличество дисков: ");
    io::stdin().read_line(&mut input_line_size_disks);
    println!("Введите размер дисков: ");
    io::stdin().read_line(&mut input_line_size_disk);
    let mut input_size_disks = input_line_size_disks.trim().parse().expect("Input not an integer");
    let mut input_size_disk = input_line_size_disk.trim().parse().expect("Input not an integer");
    
    let mut matrix_disks = vec![vec![0; input_size_disk]; input_size_disks];
    
    let mut input_line_number = String::new();

    for pointer_for_array in 0..input_size_disks {
        println!("Введите данные, числа: ");
        for pointer_for_vector in 0..input_size_disk {
            io::stdin().read_line(&mut input_line_number);
            let mut input_number = input_line_number.trim().parse().expect("Input not an integer");
            matrix_disks[pointer_for_array][pointer_for_vector] = input_number;
        }
    }
    //hamming.addEccDisks(matrixDisks);
}
