use ref_split::*;

#[ref_split(mut(MyStructMut), mut(MyStructMutDup))]
struct MyStruct {
    x: i32,
}

fn main() {}