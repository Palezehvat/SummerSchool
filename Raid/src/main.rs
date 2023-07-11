use std::io;
mod hamming;


fn main() {
    let mut input_line_size_disks = String::new();
    let mut input_line_size_disk = String::new();
    println!("Введите колличество дисков: ");
    io::stdin().read_line(&mut input_line_size_disks);
    println!("Введите размер дисков: ");
    io::stdin().read_line(&mut input_line_size_disk);
    let mut input_size_disks = input_line_size_disks.trim().parse().expect("Input not an integer");
    let mut input_size_disk = input_line_size_disk.trim().parse().expect("Input not an integer");
    
    let mut matrix_disks = vec![vec!['0'; input_size_disk]; input_size_disks];
    
    let mut input_line_char = String::new();

    for pointer_for_array in 0..input_size_disks {
        println!("Введите данные, числа: ");
        for pointer_for_vector in 0..input_size_disk {
            io::stdin().read_line(&mut input_line_char);
            let char_vec: Vec<char> = input_line_char.chars().collect();
            if char_vec.len() > 1 {
                std::panic!();
            }
            matrix_disks[pointer_for_array][pointer_for_vector] = char_vec[0];
        }
    }
    let mut matrix_ecc = hamming.addEccDisks(matrix_disks);
}
