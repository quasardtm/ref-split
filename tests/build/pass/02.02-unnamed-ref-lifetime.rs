use ref_destruct::*;

#[ref_destruct(ref(MyStructRef))]
struct MyStruct<'a>(&'a i32);

fn main() {}