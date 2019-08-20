fn main() {
    println!("Hello, world!");

    let t: (i32, f64, char) = (42, 6.12, 'j');
    let (_, _, _x) = t;

    println!("{}", _x);
}
