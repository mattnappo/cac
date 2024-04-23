enum B {
    X,
    Y,
}

struct X;

enum A {
    V(B),
    U { x: B },
    T(X),
}

struct M {
    a: A,
    b: B,
    x: X,
}

union MyU {
    int: i32,
    float: f64,
}

static VAR: B = B::X;
const XXX: X = X;

type Integer = i32;

type Vector = (f64, f64, u8);

struct VSingle(f32, f64);

//struct Pair<T, U>(T, U);

//type MyType = Pair<i32, B>;

trait Ty {}

mod mm {
    pub struct Smm;
}

fn modfn(t: mm::Smm) {}

trait Hashable: Ty {}

fn main() {}
