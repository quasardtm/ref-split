use ref_destruct::*;

#[ref_destruct(mutopt(MyStructMutopt))]
struct MyStruct {
    x: i32,
}

fn main() {}