use std::io;

struct Vector {
    x:i32,
    y:i32,
    z:i32,
}

fn main() {


    println!("what is your name?");

    let mut name = String::new(); // instanciação do de uma String
    io::stdin().read_line(&mut name)
    .expect("Failed to read line");

    println!("Hello!{}",name);

    show_struct(); // chamada da função show_struct


}

fn show_struct() {
    let v1: Vector =  Vector {x:1, y:2,z:3};
    println!("{{{},{},{}}}",v1.x, v1.y,v1.z);
}