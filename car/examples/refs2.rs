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

static VAR: B = B::X;
const XXX: X = X;

type Integer = i32;

type Vector = (f64, f64, u8);

struct Pair<T, U>(T, U);

type MyType = Pair<i32, B>;

trait Hashable {}

type Complex<T: Hashable> = Option<T>;

fn main() {}
