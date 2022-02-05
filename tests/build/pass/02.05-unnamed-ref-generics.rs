use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct<T, U>(T, U);

fn main() {}