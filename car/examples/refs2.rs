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

fn main() {}
