/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Default)]
pub struct Outer {
    pub i: u8,
}
#[repr(C)]
pub struct AutoIdVector {
    pub ar: Outer,
}
#[test]
fn bindgen_test_layout_AutoIdVector() {
    assert_eq!(::std::mem::size_of::<AutoIdVector>() , 1usize , concat ! (
               "Size of: " , stringify ! ( AutoIdVector ) ));
    assert_eq! (::std::mem::align_of::<AutoIdVector>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( AutoIdVector ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const AutoIdVector ) ) . ar as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( AutoIdVector ) , "::" ,
                stringify ! ( ar ) ));
}
impl Default for AutoIdVector {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[test]
fn __bindgen_test_layout_Outer_instantiation() {
    assert_eq!(::std::mem::size_of::<Outer>() , 1usize , concat ! (
               "Size of template specialization: " , stringify ! ( Outer ) ));
    assert_eq!(::std::mem::align_of::<Outer>() , 1usize , concat ! (
               "Alignment of template specialization: " , stringify ! ( Outer
               ) ));
}
