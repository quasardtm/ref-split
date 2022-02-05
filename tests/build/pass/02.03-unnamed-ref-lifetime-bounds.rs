use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct<'a, 'b: 'a>(&'a i32, &'b u32);

fn main() {}