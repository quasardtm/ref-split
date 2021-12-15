use ref_destruct::*;

#[ref_destruct(MyStructRef)]
struct MyStruct<'a>(&'a mut i32);

fn main() {}