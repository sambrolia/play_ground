use std::mem;

fn main() {
    //_tuples();
    //_arrays();
    //_strings();
}

fn _tuples() {
    let t: (i32, f64, char) = (42, 6.12, 'j');
    let (_, _, _x) = t;
    println!("{}", _x);

    let t = (1, 'a', false);
    let f = (2, t);
    println!("{:#?}", f);
}

fn _arrays() {
    let xs: [i32; 5] = [4,5,6,7,8];
    println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs));

    let ys = &xs[2 .. 4]; //Range up to but not including 4
    println!("{} {}", ys[0], ys[1]);

    println!("{:?} {:?}", ys, xs);
}

fn _strings() {
    let t = "test String";
    let _s = "String".to_string();
    let ss = String::from(t);

    let slice = &ss[0..4];
    println!("{}", slice);

    let h = String::from("Hello, ");
    let w = String::from("Concatinated World!");
    let s = h + &w; // Concatination takes reference

    println!("{}", s);
}