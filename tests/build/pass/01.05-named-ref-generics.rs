use ref_destruct::*;

#[ref_destruct(ref(MyStructRef))]
struct MyStruct<T, U> {
    x: T,
    y: U,
}

fn main() {}