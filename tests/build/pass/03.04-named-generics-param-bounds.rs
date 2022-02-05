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
    struct MyStruct<
        'a,
        'b: 'a,
        'c: 'a + 'b,
        X: Clone + 'static,
        Y: AsRef<X>,
        Z: std::ops::Add<X, Output = Y> + ?Sized,
    > {
        x: X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: &'a mut Option<&'b Y>,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        z: &'c Z,
    }
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
    struct MyStruct<
        'a,
        'b: 'a,
        'c: 'a + 'b,
        X: Clone + 'static,
        Y: AsRef<X>,
        Z: std::ops::Add<X, Output = Y> + ?Sized,
    > {
        x: X,
        y: &'a mut Option<&'b Y>,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        z: &'c Z,
    }
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
    struct MyStruct<
        'a,
        'b: 'a,
        'c: 'a + 'b,
        X: Clone + 'static,
        Y: AsRef<X>,
        Z: std::ops::Add<X, Output = Y> + ?Sized,
    > {
        x: X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        y: &'a mut Option<&'b Y>,
        z: &'c Z,
    }
}

fn main() {}
