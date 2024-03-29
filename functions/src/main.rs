// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    println!("Area is {}", area_of(width, height));
    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {
    return x * y;
}

fn volume(width: i32, height: i32, depth: i32)-> i32 {
    return width * height * depth;
}