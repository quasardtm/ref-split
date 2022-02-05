use ref_split::*;

#[ref_split(ref(MyStructRef))]
struct MyStruct<'a> {
    x: &'a i32,
}

fn main() {}