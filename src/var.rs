pub fn var_learn() {
    let a = 1;
    let b: i64 = 1;
    println!("a = {a}, b = {b}");
    // a = 2
    // b = 2

    let mut c = 2;
    c = 3;
    println!("c = {c}");

    let x = 1000;
    {
        let x = 100;
        println!("inner x = {x}");
    }
    println!("outer x = {x}");

    let x = "hello";
    println!("new x = {x}");

    let mut x = "this";
    x = "that";
    println!("new mut x = {x}");
}