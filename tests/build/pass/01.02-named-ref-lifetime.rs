use ref_destruct::*;

#[ref_destruct(MyStructRef)]
struct MyStruct<'a> {
    x: &'a i32,
}

fn main() {}