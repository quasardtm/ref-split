use ref_destruct::*;

#[ref_destruct(MyStructRef)]
struct MyStruct<T, U>(T, U);

fn main() {}