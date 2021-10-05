use std::io;
use std::fmt; 

fn main() {

    let mut x1 = String::new();
    let mut y1 = String::new();
    let mut x2 = String::new();
    let mut y2 = String::new();

    println!("--------------------");
    println!("This program calculates the angle between 2 vectors \nPlease follow the prompt.");
    println!("--------------------");

    println!("Please input your first vectors x coordinate: ");
    io::stdin()
        .read_line(&mut x1)
        .expect("Failed to read line");

    let x1: f32 = x1.trim().parse().expect("Did you enter a number?");

    println!("Please input your first vectors y coordinate: ");
    io::stdin()
        .read_line(&mut y1)
        .expect("Failed to read line");

    let y1: f32 = y1.trim().parse().expect("Did you enter a number?");

    println!("Please input your second vectors x coordinate: ");
    io::stdin()         
        .read_line(&mut x2)
        .expect("Failed to read line");

        
    let x2: f32 = x2.trim().parse().expect("Did you enter a number?");

    println!("Please input your second vectors y coordinate: ");
    io::stdin()         
        .read_line(&mut y2)
        .expect("Failed to read line");

    let y2: f32 = y2.trim().parse().expect("Did you enter a number?");

    let first_point = vec![x1, y1];
    let second_point = vec![x2, y2];

    println!("\nFirst Point: [{}, {}]", first_point[0], first_point[1]);
    println!("Second Point: [{}, {}]", second_point[0], second_point[1]);

    let dot_prod: f32 = first_point.iter().zip(second_point.iter()).map(|(x, y)| x * y).sum();

    let magnitude_1: f32 = (first_point[0].powf(2.) + first_point[1].powf(2.)).sqrt();
    let magnitude_2: f32 = (second_point[0].powf(2.) + second_point[1].powf(2.)).sqrt();

    let angle: f32 = dot_prod / (magnitude_1 * magnitude_2);

    println!("\nThe angle between the two vectors is: {} degrees", angle);
}

