use ref_split::*;

#[ref_split(refopt(MyStructRefopt, MyStructRefoptEx), mutopt(MyStructMutopt, MyStructMutoptEx()))]
struct MyStruct {
    x: i32,
}

fn main() {}