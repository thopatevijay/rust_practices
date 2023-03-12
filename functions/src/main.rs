fn main() {
    parameters(5);
    print_labeled_measurement(5, 'h');
    statements_expression();
    functions_with_return_values();
}

fn parameters(x: i32) {
    println!("The value of x is : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// Functions with Return Values

fn functions_with_return_values() {

    // Example:
    // fn five() -> i32 {
    //     5
    // }
    
    // fn main() {
    //     let x = five();
    
    //     println!("The value of x is: {x}");
    // }

    let x = plus_one(5);
    
    println!("The value of x is: {x}");
    
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
}

