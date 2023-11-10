use rand::prelude::SliceRandom;
use std::io;


fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    let mut count:i32 = 0;
    let mut input:String = String::new();
    println!("Please enter the length of your set of random vectors: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let length: i32 = input
        .trim()
        .parse()
        .expect("Invalid input");
    let mut final_list:Vec<Vec<i32>> = vec![];
    while count < length {
        final_list.push(get_random_vector());
        count += 1;
    }
    println!("the final list is {:?}", final_list);
    println!("the original list is", );
}

fn get_random_vector() -> Vec<i32> {
    let mut selection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut final_list = vec![];
    let mut count: i32 = 0;
    let length = selection.len().try_into().unwrap();
    while count < length {
        let index = selection.choose(&mut rand::thread_rng()).unwrap();
        let elem = selection.iter().position(|&r| r == *index).unwrap();
        final_list.push(selection[elem]);
        selection.remove(elem);
        count += 1;
    }
    final_list
}
//fn sudoku_scramble() {
//    let mut selection = vec![
//        [1, 2, 3, 4, 5, 6, 7, 8, 9],
//        [2, 1, 3, 4, 5, 6, 7, 8, 9],
//        [3, 2, 1, 4, 5, 6, 7, 8, 9],
//        [4, 2, 3, 1, 5, 6, 7, 8, 9],
//        [5, 2, 3, 4, 1, 6, 7, 8, 9],
//        [6, 2, 3, 4, 5, 1, 7, 8, 9],
//        [7, 2, 3, 4, 5, 6, 1, 8, 9],
//        [8, 2, 3, 4, 5, 6, 7, 1, 9],
//        [9, 2, 3, 4, 5, 6, 7, 8, 1]];
//    let mut final_list = vec![];
//    let mut count: i32 = 0;
//    let length = selection.len().try_into().unwrap();
//    while count < length {
//        let index = selection.choose(&mut rand::thread_rng()).unwrap();
//        let elem = selection.iter().position(|&r| r == *index).unwrap();
//        final_list.push(selection[elem]);
//      selection.remove(elem);
//        count += 1;
//  }
//    for ele in selection.iter() {
//        let mut final_list = vec![];
//        let mut count: i32 = 0;
//        let length = ele.len().try_into().unwrap();
//        while count < length {
//            let index = ele.choose(&mut rand::thread_rng()).unwrap();
//            let elem = ele.iter().position(|&r| r == *index).unwrap();
//            final_list.push(ele[elem]);
//            ele.remove(elem);
//            count += 1;
//        }
//        for n in final_list.iter() {
//            ele.push(final_list[*n]);
//        }
//
//    }
//    println!("the final list is {:?}", final_list);
//    println!("the original list is {:?}", selection);
//
//}
