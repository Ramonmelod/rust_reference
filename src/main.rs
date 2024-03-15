struct Vector {
    x:i32,
    y:i32,
    z:i32,
}

fn main() {
    let v1: Vector =  Vector {x:1, y:2,z:3};
    println!("{{{},{},{}}}",v1.x, v1.y,v1.z);
}
