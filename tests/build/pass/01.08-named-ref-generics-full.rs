use ref_destruct::*;
use core::convert::*;

#[ref_destruct(MyStructRef)]
pub struct MyStruct<'a, 'b, T, U, V>
where
    'a: 'b,
    T: AsRef<U> + AsRef<V>,
    V: 'static,
{
    x: &'a T,
    y: &'b mut U,
    z: &'static V,
}

fn main() {}