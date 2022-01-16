mod gen1 {
    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<'a, 'b, X, Y, Z> (
        X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'a Y,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'b Z,
    );
}
mod gen2 {
    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<'a, 'b, X, Y, Z> (
        X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'a Y,
        &'b Z,
    );
}
fn main() {}