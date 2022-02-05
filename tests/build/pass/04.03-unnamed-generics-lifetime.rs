mod gen1 {
    use ref_split::*;

    #[ref_split(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<'a, 'b, X, Y, Z> (
        X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'a Y,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'b Z,
    );
}
mod gen2 {
    use ref_split::*;

    #[ref_split(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<'a, 'b, X, Y, Z> (
        X,
        #[rs_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'a Y,
        &'b Z,
    );
}
fn main() {}