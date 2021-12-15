use ref_destruct::*;
use core::convert::*;

#[ref_destruct(MyStructRef)]
pub struct MyStruct<'a, 'b, T, U, V>(&'a T, &'b mut U, &'static V)
where
    'a: 'b,
    T: AsRef<U> + AsRef<V>,
    V: 'static
;

fn main() {}