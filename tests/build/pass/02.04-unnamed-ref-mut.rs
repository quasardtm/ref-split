use ref_destruct::*;

#[ref_destruct(ref(MyStructRef))]
struct MyStruct<'a>(&'a mut i32);

fn main() {}