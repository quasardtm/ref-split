use ref_destruct::*;

#[ref_destruct(MyStructRef)]
struct MyStruct<'a>(&'a i32);

fn main() {}