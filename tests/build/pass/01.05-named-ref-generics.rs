use ref_destruct::*;

#[ref_destruct(MyStructRef)]
struct MyStruct<T, U> {
    x: T,
    y: U,
}

fn main() {}