mod refopt {
    
    use ref_split::*;

    #[ref_split(ref(MyStructRef), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct(i32, #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] u64);

}

mod mutopt {
    
    use ref_split::*;

    #[ref_split(ref(MyStructRef), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct(i32, #[rs_ignore(ref, mut, mutopt(MyStructMutopt, MyStructMutoptEx))] u64);

}

mod refmutopt {
    
    use ref_split::*;

    #[ref_split(ref(MyStructRef), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct(i32, #[rs_ignore(ref, mut, mutopt(MyStructMutopt, MyStructMutoptEx), mutopt(MyStructMutopt, MyStructMutoptEx))] u64);

}

fn main() {}