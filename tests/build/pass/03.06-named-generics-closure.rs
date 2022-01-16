mod gen1 {
    use ref_destruct::*;

    #[ref_destruct(
        ref(MyStructRef),
        mut(MyStructMut),
        refopt(MyStructRefopt),
        mutopt(MyStructMutopt),
        refopt(MyStructRefoptEx),
        mutopt(MyStructMutoptEx)
    )]
    struct MyStruct<'a, X, Y, Z>
    where
        for<'b> X: Fn(&'b Y) -> Option<&'b Y>,
    {
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        x: X,
        y: &'a mut Option<Y>,
        z: for<'c> fn(&'c Z) -> Option<&'c X>,
    }
}

mod gen2 {
    use ref_destruct::*;

    #[ref_destruct(
        ref(MyStructRef),
        mut(MyStructMut),
        refopt(MyStructRefopt),
        mutopt(MyStructMutopt),
        refopt(MyStructRefoptEx),
        mutopt(MyStructMutoptEx)
    )]
    struct MyStruct<'a, X, Y, Z>
    where
        for<'b> X: Fn(&'b Y) -> Option<&'b Y>,
    {
        x: X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: &'a mut Option<Y>,
        z: for<'c> fn(&'c Z) -> Option<&'c X>,
    }
}

mod gen3 {
    use ref_destruct::*;

    #[ref_destruct(
        ref(MyStructRef),
        mut(MyStructMut),
        refopt(MyStructRefopt),
        mutopt(MyStructMutopt),
        refopt(MyStructRefoptEx),
        mutopt(MyStructMutoptEx)
    )]
    struct MyStruct<'a, X, Y, Z>
    where
        for<'b> X: Fn(&'b Y) -> Option<&'b Y>,
    {
        x: X,
        y: &'a mut Option<Y>,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        z: for<'c> fn(&'c Z) -> Option<&'c X>,
    }
}

mod gen4 {
    use ref_destruct::*;

    #[ref_destruct(
        ref(MyStructRef),
        mut(MyStructMut),
        refopt(MyStructRefopt),
        mutopt(MyStructMutopt),
        refopt(MyStructRefoptEx),
        mutopt(MyStructMutoptEx)
    )]
    struct MyStruct<'a, X, Y, Z>
    where
        for<'b> X: Fn(&'b Y) -> Option<&'b Y>,
    {
        x: X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: &'a mut Option<Y>,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        z: for<'c> fn(&'c Z) -> Option<&'c X>,
    }
}
fn main() {}
