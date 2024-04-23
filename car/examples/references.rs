fn a() {
    // b();
}

fn b() {
    a();
}

struct Unit;
fn c(t: Unit) {}

fn main() {}
