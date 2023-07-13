fn new_number_for_matrix_ecc(matrix_disks: &mut Vec<Vec<i32>>, pointer_of_all_disks: usize, pointer_of_disk: usize) -> i32 {
    let mut size_of_controling_bits = matrix_disks[pointer_of_all_disks][pointer_of_disk].ilog2();
    let mut new_number = 0;

    for pointer_of_controling_bits in 0..size_of_controling_bits {
                    
        let mut result_after_degree = i32::pow(2, pointer_of_controling_bits);
        let mut position_for_count_in_matrix = result_after_degree.clone() - 1;
        let mut summary_of_bits = 0;
        let mut counter = 0;

        while position_for_count_in_matrix < 32 {
            if position_for_count_in_matrix != result_after_degree - 1 {
                summary_of_bits += (matrix_disks[pointer_of_all_disks][pointer_of_disk] >> position_for_count_in_matrix) & 1;
            }
            if counter + 1 < result_after_degree {
                position_for_count_in_matrix += 1;
                counter += 1;
            }
            else {
                position_for_count_in_matrix += result_after_degree + 1;
                counter = 0;
                }
            }
            summary_of_bits = summary_of_bits % 2;
            new_number += summary_of_bits;
            new_number = new_number << 1;
    }
    new_number
}



// Creates additional ecc disks to protect information
pub fn add_ecc_disks(input_size_disks: usize, input_size_disk: usize, matrix_disks: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut size_of_controling_bits: u32 = 0;
    let mut matrix_ecc = vec![vec![0; input_size_disk]; input_size_disks];

    for pointer_of_all_disks in 0..input_size_disks {
        for pointer_of_disk in 0..input_size_disk {
            matrix_ecc[pointer_of_all_disks][pointer_of_disk] = new_number_for_matrix_ecc(matrix_disks, pointer_of_all_disks, pointer_of_disk);
        }
    }
    matrix_ecc
}

// Return fixed ecc matrix
pub fn fix_errors(input_size_disks: usize, input_size_disk: usize, matrix_disks: &mut Vec<Vec<i32>>, matrix_ecc: &mut Vec<Vec<i32>>) {
    let the_newly_calculated_matrix = add_ecc_disks(input_size_disks, input_size_disk, matrix_disks);

    for pointer_of_all_disks in 0..input_size_disks {
        for pointer_of_disk in 0..input_size_disk {
            let mut error_place = 0;
            for pointer in 0..32 {
                if the_newly_calculated_matrix[pointer_of_all_disks][pointer_of_disk] >> pointer & 1 
                != matrix_ecc[pointer_of_all_disks][pointer_of_disk] >> pointer & 1 {
                    error_place += i32::pow(2, pointer);
                }
            }
            if error_place == 0 {
                if the_newly_calculated_matrix[pointer_of_all_disks][pointer_of_disk] & 1 
                != matrix_ecc[pointer_of_all_disks][pointer_of_disk] & 1 {
                    if matrix_ecc[pointer_of_all_disks][pointer_of_disk] & 1 == 0 {
                        matrix_ecc[pointer_of_all_disks][pointer_of_disk] -= 1;
                    }
                    else {
                        matrix_ecc[pointer_of_all_disks][pointer_of_disk] += 1;
                    }              
                }
            }
            else {
                if matrix_ecc[pointer_of_all_disks][pointer_of_disk] >> error_place & 1 == 0 {
                    matrix_ecc[pointer_of_all_disks][pointer_of_disk] += i32::pow(2, error_place as u32);
                }
                else {
                    matrix_ecc[pointer_of_all_disks][pointer_of_disk] -= i32::pow(2, error_place as u32);
                }
            }
        }
    }
}

// Change number in choosen disk and choosen position
pub fn change_in_disk(choosen_disk: usize, choosen_position: usize, matrix_disks: &mut Vec<Vec<i32>>, matrix_ecc: &mut Vec<Vec<i32>>, new_number: i32) {
    matrix_disks[choosen_disk][choosen_position] = new_number;
    matrix_ecc[choosen_disk][choosen_position] = new_number_for_matrix_ecc(matrix_disks, choosen_disk, choosen_position);
}