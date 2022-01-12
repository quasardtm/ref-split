use ref_destruct::*;

#[ref_destruct(refopt(MyStructRefopt, MyStructRefoptEx), mutopt(MyStructMutopt, MyStructMutoptEx()))]
struct MyStruct {
    x: i32,
}

fn main() {}