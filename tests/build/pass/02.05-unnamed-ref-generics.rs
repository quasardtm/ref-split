use ref_destruct::*;

#[ref_destruct(ref(MyStructRef))]
struct MyStruct<T, U>(T, U);

fn main() {}