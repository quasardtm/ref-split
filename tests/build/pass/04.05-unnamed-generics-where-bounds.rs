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
    struct MyStruct<'a, 'b, 'c, X, Y, Z>(
        X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] &'a mut Option<&'b Y>,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] &'c Z,
    )
    where
        'b: 'a,
        'c: 'a + 'b,
        X: Clone + 'static,
        Y: AsRef<X>,
        Z: std::ops::Add<X, Output = Y> + ?Sized;
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
    struct MyStruct<'a, 'b, 'c, X, Y, Z>(
        X,
        &'a mut Option<&'b Y>,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] &'c Z,
    )
    where
        'b: 'a,
        'c: 'a + 'b,
        X: Clone + 'static,
        Y: AsRef<X>,
        Z: std::ops::Add<X, Output = Y> + ?Sized;
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
    struct MyStruct<'a, 'b, 'c, X, Y, Z>(
        X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] &'a mut Option<&'b Y>,
        &'c Z,
    )
    where
        'b: 'a,
        'c: 'a + 'b,
        X: Clone + 'static,
        Y: AsRef<X>,
        Z: std::ops::Add<X, Output = Y> + ?Sized;
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
    struct MyStruct<'a, 'b, 'c, X, Y, Z>(
        X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] &'a mut Option<&'b Y>,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'c std::collections::hash_map::HashMap<Z, X>,
    )
    where
        'b: 'a,
        'c: 'a + 'b,
        X: Clone + 'static,
        Option<&'b Y>: AsRef<X>,
        std::collections::hash_map::HashMap<Z, X>: std::ops::Add<X, Output = Y>;
}

fn main() {}
