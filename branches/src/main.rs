
enum Data {
    Number(i32),
    String(String),
    Float(f32),
    Bool(bool),
}
fn main() {
    let _num_var = Data::Number(2);
    let _string = Data::String(String::from("hello"));
    let _float = Data::Float(3.14);
    let _bool = Data::Bool(true);
    let number: i32 = -3;

    match _float {
        Data::Number(x) => {
            if number.abs() < x {
                println!("The number is {}", x);
            } else {
                println!("The number is invalid for this experiment");
            }
        }
        Data::String(x) => println!("The string is {}", x),
        Data::Float(x) => {
            if (number as f32).abs() < x {
                println!("The float is {}", x)
            } else {
                println!("The float is small");
            }
        }
        Data::Bool(x) => println!("The boolean is {}", x),
    }

    let x = if_let();
    println!("if let {:?}", x);

    counter();
}

fn if_let() -> i32 {
    let condition = true;
    let statement = if condition { 5 } else { 6 };
    return statement;
}

fn counter() {
    let mut counter: u32 = 0;

    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    assert_eq!(result, 22)
}
