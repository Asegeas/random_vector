use rand::prelude::SliceRandom;

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
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
    println!("the final list is {:?}", final_list);
    println!("the original list is {:?}", selection);
}
