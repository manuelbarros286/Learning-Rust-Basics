fn main() {
    let mut x = 6;
    println!("x is valued at {}", x);
    x += 5;
    println!("x is valued at {}", x);
    const SPACES: &str = "text";
    if !SPACES.is_empty() {
        let _spaces = SPACES.len();
    }

    let list: (i32, char) = (1231, 'a');
    println!("list is {:?}", list);

    array_index();
}


use std::io;
fn array_index() {
    const ARRAY_CHECK: [u32; 8] = [2; 8];
    println!("Check array index:");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index = index.trim().parse::<u32>().expect("Failed to parse index");

    let element = ARRAY_CHECK[index as usize];

    println!("The element is {}", element);
}