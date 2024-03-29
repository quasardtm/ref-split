mod refopt {
    use ref_split::*;

    #[ref_split(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct {
        x: i32,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: u32,
    }
}

mod mutopt {
    use ref_split::*;

    #[ref_split(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct {
        x: i32,
        #[rs_ignore(ref, mut, mutopt(MyStructMutopt, MyStructMutoptEx))]
        y: u32,
    }
}

mod refmutopt {
    use ref_split::*;
    
    #[ref_split(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct {
        x: i32,
        #[rs_ignore(ref, mut, mutopt(MyStructMutopt, MyStructMutoptEx), mutopt(MyStructMutopt, MyStructMutoptEx))]
        y: u32,
    }
}

fn main() {}