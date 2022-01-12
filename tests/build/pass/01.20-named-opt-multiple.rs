use ref_destruct::*;

#[ref_destruct(refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
struct MyStruct {
    x: i32,
}

fn main() {}