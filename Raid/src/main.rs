use std::io;
mod hamming;


fn main() {
    let mut input_line_size_disks = String::new();
    let mut input_line_size_disk = String::new();
    println!("Введите колличество дисков: ");
    let _ = io::stdin().read_line(&mut input_line_size_disks);
    println!("Введите размер дисков: ");
    let _ = io::stdin().read_line(&mut input_line_size_disk);
    let input_size_disks = input_line_size_disks.trim().parse().expect("Input not an integer");
    let input_size_disk = input_line_size_disk.trim().parse().expect("Input not an integer");
    
    let mut matrix_disks = vec![vec![0; input_size_disk]; input_size_disks];
    
    let mut input_line_int = String::new();

    for pointer_for_array in 0..input_size_disks {
        println!("Введите данные, числа: ");
        for pointer_for_vector in 0..input_size_disk {
            let _ = io::stdin().read_line(&mut input_line_int);
            matrix_disks[pointer_for_array][pointer_for_vector] = input_line_int.trim().parse().expect("Input not an integer");
            input_line_int = String::new();
        }
    }
    let mut matrix_ecc = hamming::add_ecc_disks(input_size_disks, input_size_disk, &mut matrix_disks);
}