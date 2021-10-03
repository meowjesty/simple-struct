use simple_struct::simple_struct;

#[simple_struct("SimpleFoo", a, b)]
#[derive(Debug)]
struct Foo {
    a: u32,
    b: u32,
    c: u32,
}

fn main() {
    let foo = Foo { a: 0, b: 1, c: 2 };
    let simple = SimpleFoo { a: 0, b: 1 };

    println!("{:#?}", foo);
    println!("{:#?}", simple);
}
