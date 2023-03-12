fn main() {

    floating_point_types();
    numeric_operations();
    boolean_type();
    char_type();
}

fn floating_point_types() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!(" x is {x}");
    println!(" y is {y}");
}

fn numeric_operations() {
     // addition
     let sum = 5 + 10;
     println!("addition is : {sum}");

     // subtraction
     let difference = 95.5 - 4.3;
     println!("subtraction is : {difference}");
    
     // multiplication
     let product = 4 * 30;
     println!("multiplication is : {product}");

     // division
     let quotient = 56.7 / 32.2;
     let truncated = -5 / 3; // Results in -1
     println!("quotient : {quotient}");
     println!("truncated : {truncated}");

     // remainder
     let remainder = 43 % 5;
     println!("remainder is : {remainder}");
}

fn boolean_type() {
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t is : {t}");
    println!("f is : {f}");
}

fn char_type() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c is : {c}, z is : {z}, heart_eyed_cat is : {heart_eyed_cat}");
}