use ref_destruct::*;

#[ref_destruct(refopt(MyStructRefopt))]
struct MyStruct {
    x: i32,
}

fn main() {}