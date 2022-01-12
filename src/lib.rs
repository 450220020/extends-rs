mod extends;
extern crate proc_macro;
extern crate proc_macro2;
use proc_macro::TokenStream;




#[proc_macro_attribute]
pub fn extends_struct(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    extends::struct_extends::impl_extends_struct(_attr, _input)
}



// -test code
// use extends::struct_extends;

// #[derive(Debug)]
// struct matoparent {
//     pub idr: i8,
//     pub idri: u8,
// }

// #[extends_struct(derive="Clone"&&extends="rsdata::dbs::arbatis::base_struct::BaseDO*struct")]
// struct bbc {
//     pub a: Option<i8>,
// }

// #[test]
// fn showa() {
//     println!("{:#?}", bbc::new_none());
//     let mut a = bbc::new_none();
//     a.result = Some(false);
//     println!("{:#?}", a);
//     println!("aaffff:{:?}", bbc::get_struct_attr_str());
// }
