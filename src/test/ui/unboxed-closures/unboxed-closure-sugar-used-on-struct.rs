// Test that parentheses form doesn't work with struct types appearing in argument types.

struct Bar<A> {
    f: A
}

fn foo(b: Box<Bar()>) {
    //~^ ERROR parenthesized type parameters may only be used with a `Fn` trait
    //~| ERROR missing generics for struct `Bar`
}

fn main() { }
