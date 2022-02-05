mod gen1 {
    use ref_split::*;

    #[ref_split(
        ref(MyStructRef),
        mut(MyStructMut),
        refopt(MyStructRefopt),
        mutopt(MyStructMutopt),
        refopt(MyStructRefoptEx),
        mutopt(MyStructMutoptEx)
    )]
    struct MyStruct<'a, X, Y, Z>(
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] X,
        &'a mut Option<Y>,
        for<'c> fn(&'c Z) -> Option<&'c X>,
    )
    where
        for<'b> X: Fn(&'b Y) -> Option<&'b Y>;
}

mod gen2 {
    use ref_split::*;

    #[ref_split(
        ref(MyStructRef),
        mut(MyStructMut),
        refopt(MyStructRefopt),
        mutopt(MyStructMutopt),
        refopt(MyStructRefoptEx),
        mutopt(MyStructMutoptEx)
    )]
    struct MyStruct<'a, X, Y, Z>(
        X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] &'a mut Option<Y>,
        for<'c> fn(&'c Z) -> Option<&'c X>,
    )
    where
        for<'b> X: Fn(&'b Y) -> Option<&'b Y>;
}

mod gen3 {
    use ref_split::*;

    #[ref_split(
        ref(MyStructRef),
        mut(MyStructMut),
        refopt(MyStructRefopt),
        mutopt(MyStructMutopt),
        refopt(MyStructRefoptEx),
        mutopt(MyStructMutoptEx)
    )]
    struct MyStruct<'a, X, Y, Z>(
        X,
        &'a mut Option<Y>,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        for<'c> fn(&'c Z) -> Option<&'c X>,
    )
    where
        for<'b> X: Fn(&'b Y) -> Option<&'b Y>;
}

mod gen4 {
    use ref_split::*;

    #[ref_split(
        ref(MyStructRef),
        mut(MyStructMut),
        refopt(MyStructRefopt),
        mutopt(MyStructMutopt),
        refopt(MyStructRefoptEx),
        mutopt(MyStructMutoptEx)
    )]
    struct MyStruct<'a, X, Y, Z>(
        X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] &'a mut Option<Y>,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        for<'c> fn(&'c Z) -> Option<&'c X>,
    )
    where
        for<'b> X: Fn(&'b Y) -> Option<&'b Y>;
}
fn main() {}
