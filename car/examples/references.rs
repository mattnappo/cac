fn a(p: (i128, i128)) {
    // b();
}

fn b(i: i32) {
    a((2, 2));
    let x: f32 = 3.1;
}

struct Unit;
fn c(t: Unit) {}

fn main() {}
