use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct<T, U> {
    x: T,
    y: U,
}

fn main() {}