use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};
mod attribute;

/// 構造体の参照をデストラクトする構造体を定義する属性。
/// ```rust
/// use ref_split::ref_split;
/// #[ref_split(ref(MyStructRef), mut(MyStructMut))]
/// struct MyStruct {
///     x: usize,
///     y: usize,
/// }
/// ```
/// 上記の様に定義された場合は、以下のように展開される。
/// ```rust
/// use ref_split::ref_split;
/// struct MyStruct {
///     x: usize,
///     y: usize,
/// }
/// 
/// struct MyStructRef<'a> {
///     pub x: &'a usize,
///     pub y: &'a usize,
/// }
/// 
/// impl<'a> ::core::convert::From<&'a MyStruct> for MyStructRef<'a>
/// {
///     fn from(v: &'a MyStruct) -> Self {
///        MyStructRef { x: &v.x, y: &v.y }
///     }
/// }
/// 
/// impl<'a> ::ref_split::RefSplit for &'a MyStruct {
///     type Struct = MyStructRef<'a>;
/// }
/// 
/// struct MyStructMut<'a> {
///     pub x: &'a mut usize,
///     pub y: &'a mut usize,
/// }
/// 
/// impl<'a> ::core::convert::From<&'a mut MyStruct> for MyStructMut<'a>
/// {
///     fn from(v: &'a mut MyStruct) -> Self {
///         MyStructMut {
///             x: &mut v.x,
///             y: &mut v.y,
///         }
///     }
/// }
/// 
/// impl<'a> ::ref_split::RefSplit for &'a mut MyStruct {
///     type Struct = MyStructMut<'a>;
/// }
/// ```
/// 引数`ref(StructIdent)`で、不変な参照からのFromが定義され、引数`mut(StructIdent)`で、可変な参照からのFromが定義される。
/// 
/// `RefSplit`が1つしか実装できないため、`ref`,`mut`は、型一つしか指定できない。
/// 引数`refopt(StructIdent, StructIdent2, ...)`, `refmut(StructIdent, StructIdent2, ...)`を指定すると、`RefSplit`の実装を省略する。この場合は複数定義が可能となる。
/// ```rust
/// use ref_split::ref_split;
/// #[ref_split(ref(MyStructRef), mut(MyStructMut), refopt(MyStructRefopt, MyStructRefopt2), mutopt(MyStructMutopt, MyStructMutopt2))]
/// struct MyStruct {
///     x: usize,
///     y: usize,
/// }
/// ```
/// 
/// `#[ref_split]`への引数は省略できず、必ず`ref`か`mut`のどちらか1つが必要となる。
/// `ref`や`mut`の引数も省略できない。
/// 
/// # 対象アイテム
/// 1つ以上のフィールドがある構造体に属性を付加できる。名前付き、名前無し（タプル型）どちらでも可。
/// 構造体以外には付加できない。
/// 
/// また、下記の無視属性によって、すべてのフィールドを無視することはできない。
/// 
/// # 無視属性
/// 
/// 通常、すべてのフィールドがデストラクト用構造体に展開される。
/// 展開したくないフィールドに対しては、`#[rs_ignore]`属性を付加する。
/// ```rust
/// use ref_split::ref_split;
/// #[ref_split(ref(MyStructRef), mut(MyStructMut))]
/// struct MyStruct {
///     x: usize,
///     #[rs_ignore]
///     y: usize,
/// }
/// ```
/// 上記のようにすると、変換時に`y`は展開されない。
/// 
/// `ref`と`mut`で`#[rs_ignore]`を分けて設定したい場合は、`#[rs_ignore]`の引数に`ref`または`mut`を与える。
/// ```rust
/// use ref_split::ref_split;
/// #[ref_split(ref(MyStructRef), mut(MyStructMut))]
/// struct MyStruct {
///     #[rs_ignore(ref)]
///     x: usize,
///     #[rs_ignore(mut)]
///     y: usize,
/// }
/// ```
/// # 可視性
/// 
/// 参照用構造体の可視性は、元の構造体の可視性を引き継ぐ。
/// 以下はコンパイルエラーとなる。
/// ```compile_fail
/// mod inner {
///     use ref_split::ref_split;
///     #[ref_split(ref(MyStructRef), mut(MyStructMut))]
///     struct MyStruct {
///         #[rs_ignore(ref)]
///         x: usize,
///         #[rs_ignore(mut)]
///         y: usize,
///     }
/// }
/// 
/// use ref_split::ref_split;
/// impl inner::MyStructRef<'_> {
///     fn print(&self) { println!("{}", self.y) }
/// }
/// ```
/// ```text
/// 
/// impl inner::MyStructRef<'_> {
///             ^^^^^^^^^^^ private struct
/// 
/// ```
/// 以下は問題なくコンパイルは通る。
/// ```rust
/// mod inner {
///     use ref_split::ref_split;
///     #[ref_split(ref(MyStructRef), mut(MyStructMut))]
///     pub struct MyStruct {
///         #[rs_ignore(ref)]
///         x: usize,
///         #[rs_ignore(mut)]
///         y: usize,
///     }
/// }
/// 
/// use ref_split::ref_split;
/// impl inner::MyStructRef<'_> {
///     fn print(&self) { println!("{}", self.y) }
/// }
/// ```
/// 
/// # TODO
/// - 現在、Self型の使用が出来ないので、対応する。
#[proc_macro_attribute]
pub fn ref_split(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    attribute::proc(args, input.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
