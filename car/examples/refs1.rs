fn a(p: (i128, i128)) {
    // b();
}

fn b(i: i32) {
    a((2, 2));
    let x: f32 = 3.1;
}

struct Unit;
fn c(t: Unit) {}

struct Point {
    x: Num,
    y: Num,
}

struct Num(f32);

fn main() {}
