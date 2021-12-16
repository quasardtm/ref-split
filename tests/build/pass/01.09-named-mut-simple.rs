use ref_destruct::*;

#[ref_destruct(mut(MyStructMut))]
struct MyStruct {
    x: i32,
}

fn main() {}