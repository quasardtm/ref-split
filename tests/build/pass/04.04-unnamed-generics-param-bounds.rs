mod gen1 {
    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<'a, 'b: 'a, 'c: 'a + 'b, X: Clone + 'static, Y: AsRef<X>, Z: std::ops::Add<X, Output = Y> + ?Sized> (
        X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'a mut Option<&'b Y>,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'c Z,
    );
}
mod gen2 {
    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<'a, 'b: 'a, 'c: 'a + 'b, X: Clone + 'static, Y: AsRef<X>, Z: std::ops::Add<X, Output = Y> + ?Sized> (
        X,
        &'a mut Option<&'b Y>,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'c Z,
    );
}
mod gen3 {
    use ref_destruct::*;

    #[ref_destruct(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt), mutopt(MyStructMutopt), refopt(MyStructRefoptEx), mutopt(MyStructMutoptEx))]
    struct MyStruct<'a, 'b: 'a, 'c: 'a + 'b, X: Clone + 'static, Y: AsRef<X>, Z: std::ops::Add<X, Output = Y> + ?Sized> (
        X,
        #[rd_ignore(ref, mut, refopt(MyStructRefopt, MyStructRefoptEx))]
        &'a mut Option<&'b Y>,
        &'c Z,
    );
}

fn main() {}