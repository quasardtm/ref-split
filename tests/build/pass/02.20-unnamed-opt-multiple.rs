mod single_arg {
    use ref_destruct::*;

    #[ref_destruct(refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct(i32);
}

mod multi_arg {
    use ref_destruct::*;

    #[ref_destruct(refopt(MyStructRefopt, MyStructRefoptEx), mutopt(MyStructMutopt, MyStructMutoptEx))]
    struct MyStruct(i32);
}

fn main() {}