use std::any::Any;
use crate::Data::Number;

enum Data {
    Number(i32),
    String(String),
    Float(f32),
    Bool(bool),
}
fn main() {
    let _numVar = Data::Number(2);
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
        _ => println!("Variant cannot be matched"),
    }
}
