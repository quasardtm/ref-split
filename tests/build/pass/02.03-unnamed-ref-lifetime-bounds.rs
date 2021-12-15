use ref_destruct::*;

#[ref_destruct(MyStructRef)]
struct MyStruct<'a, 'b: 'a>(&'a i32, &'b u32);

fn main() {}