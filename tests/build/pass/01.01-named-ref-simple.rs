use ref_destruct::*;

#[ref_destruct(ref(MyStructRef))]
struct MyStruct {
    x: i32,
}

fn main() {}