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
    struct MyStruct<X, Y, Z>(
        X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] Y,
        Z,
    );
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
    struct MyStruct<X, Y, Z>(
        X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] Y,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))] Z,
    );
}
fn main() {}
