use ref_destruct::*;

#[ref_destruct(mut(MyStructMut), mut(MyStructMutDup))]
struct MyStruct {
    x: i32,
}

fn main() {}