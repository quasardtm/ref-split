use ref_split::*;

#[ref_split(ref(MyStructRef), ref(MyStructRefDup))]
struct MyStruct {
    x: i32,
}

fn main() {}