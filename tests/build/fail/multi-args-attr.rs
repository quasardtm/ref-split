use ref_destruct::*;

#[ref_destruct(MyStructRef, MyStructMut)]
struct MyStruct {
    x: i32,
}

fn main() {}