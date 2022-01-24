mod extends;
extern crate proc_macro;
extern crate proc_macro2;
use proc_macro::TokenStream;
#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;


    ///
    /// 实现继承struct 内部的属性与impl 函数，不建议用在复杂的结构体作为父级
    /// 使用:
    /// #[extends_struct(derive="Debug,Clone"&&extends="rsdata::dbs::arbatis::base_struct::BaseDO@struct")]
    /// struct 子结构体{
    ///     子结构体必须要有内容
    /// }
    /// 会持续更新,不过基于实际应用开发变更，之后会被实现  IOC  AOP  Single  thread基于属性宏的实现 底部 基于 once,dashMap,parking_lot,pest
    /// 版本0.1.6 将是这个继承方案的稳定版本,后续不会在这个内容上改动，但会有新的内容出现
    /// ```
    ///  #[extends_struct()];
    /// ```
#[allow(warnings)]
#[proc_macro_attribute]
pub fn extends_struct(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    extends::all_extends::impl_extends(_attr, _input)
}



#[test]
fn name() {
    extends::all_extends::path_split("rsdata::dbs::arbatis::base_struct::BaseDO@struct".to_string());
}


    //rsdata::dbs::arbatis::base_struct::BaseDO*struct
    //同级crate 中 rsdata/src/dbs/arbatis/base_struct.rs   struct BaseDO
    //如果不存在向上一级寻找  仅1次 
    // #[extends_struct(derive="Debug,Clone"&&extends="rsdata::dbs::arbatis::base_struct::BaseDO*struct")]
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