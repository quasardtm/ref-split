use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct<'a>(&'a i32);

fn main() {}