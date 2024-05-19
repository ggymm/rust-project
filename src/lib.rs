pub fn var_learn() {
    let a = 1;
    let b: i64 = 1;
    println!("a = {a}, b = {b}");
    // a = 2
    // b = 2

    let mut c = 2;
    println!("old mut c = {c}");
    c = 3; // 只有 mut 修饰的变量才能重新赋值
    println!("new mut c = {c}");

    let x = 1000;
    {
        let x = 100; // 重新定义了一个新的 x
        println!("inner x = {x}"); // 100
    }
    println!("outer x = {x}"); // 1000

    let x = "hello";
    println!("new x = {x}");

    let mut x = "this"; // 重新定义了一个可变的 x
    println!("old mut x = {x}");
    x = "that";
    println!("new mut x = {x}");
}

static LANGUAGE: &str = "Rust";
static mut LANGUAGE_MUT: &str = "C++";

pub fn const_learn() {
    const MAX_POINTS: u32 = 100_000; // 常量必须指定类型
    println!("MAX_POINTS = {MAX_POINTS}");

    unsafe {
        LANGUAGE_MUT = "C++";
        println!("LANGUAGE_MUT = {LANGUAGE_MUT}"); // C++
    }

    println!("LANGUAGE = {LANGUAGE}"); // Rust
}

pub fn datatype_learn() {
    let a: i8 = 1;
    let b: i16 = 2;
    let c: i32 = 3;
    let d: i64 = 4;
    let e: i128 = 5;
    let f: isize = 6;
    println!("a = {a}, b = {b}, c = {c}, d = {d}, e = {e}, f = {f}");

    let a: u8 = 1;
    let b: u16 = 2;
    let c: u32 = 3;
    let d: u64 = 4;
    let e: u128 = 5;
    let f: usize = 6;
    println!("a = {a}, b = {b}, c = {c}, d = {d}, e = {e}, f = {f}");

    let a: f32 = 1.0;
    let b: f64 = 2.0;
    println!("a = {a}, b = {b}");

    let a: bool = true;
    let b: char = 'a';
    println!("a = {a}, b = {b}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        var_learn();
        const_learn();
        datatype_learn();
    }
}
