fn main() {
    second_function(5);
    //convert second parameter from &str to String
    labelled_measurement(10, "inches".to_string());
}

fn second_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn labelled_measurement(value: i32, unit_label: String) {
    println!("The value of x is: {}{}", value, unit_label);
}