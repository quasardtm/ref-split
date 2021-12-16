use ref_destruct::*;

#[ref_destruct(ref(MyStructRef), ref(MyStructRefDup))]
struct MyStruct {
    x: i32,
}

fn main() {}