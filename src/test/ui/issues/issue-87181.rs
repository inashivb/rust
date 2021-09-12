trait Foo {
    fn get(&self);
}

struct Bar<T>(T);

struct Inner();

impl Foo for Inner {
    fn get(&self) { }
}

fn main() {
    let thing = Bar(Inner);
    thing.0.get();
    //~^ ERROR no method named `get` found for fn item `fn() -> Inner {Inner}` in the current scope
}
