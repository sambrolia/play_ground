use std::mem;

fn main() {
    //_tuples();
    //_arrays();
    //_strings();
    _ownership();
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

fn take(v: Vec<i32>) {
    println!("We took v: {}", v[10] + v[100]);
}

fn cop(a: i32, b: i32) {
    println!("{}", a + b);
}

fn _ownership() {
    let x = 1;
    {
        let a = 10;
        // x + a; // Would not error, a in scope
    }
    // x + a; //would error, a not in scope

    let s = String::from("String");
    
    //let y = s;
    //println!("{}", s); //would error - value used here after move
    
    let _y = &s;
    println!("{}", s);



    // Take ownership
    let mut v = Vec::new();

    for i in 1 .. 1000 {
        v.push(i);
    }

    take(v);
    //println!("{}", v[5]); //Would error - v was moved to the take function
    println!("Finished!");

    
    // Copy
    let a = 31;
    let b = 45;

    cop(a,b);

    println!("we have a: {} and b: {}", a, b);
}