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
    struct MyStruct<X, Y, Z>(
        X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] Y,
        Z,
    );
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
    struct MyStruct<X, Y, Z>(
        X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] Y,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] Z,
    );
}
fn main() {}
