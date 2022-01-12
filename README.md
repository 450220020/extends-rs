# extends-rs
rust extends  struct


    //extends=rsdata::dbs::arbatis::base_struct::BaseDO@struct      注意BaseDO 与struct中间有个阿尔法符号 用来标记继承那种类型的内容
    //同级crate 中 rsdata/src/dbs/arbatis/base_struct.rs   struct BaseDO
    //如果不存在向上一级寻找  仅1次 
     #[extends_struct(derive="Debug,Clone"&&extends="rsdata::dbs::arbatis::base_struct::BaseDO@struct")]
     struct bbc {
         pub a: Option<i8>,
     }

     #[test]
     fn showa() {
         println!("{:#?}", bbc::new_none());
         let mut a = bbc::new_none();
         a.result = Some(false);
         println!("{:#?}", a);
         println!("aaffff:{:?}", bbc::get_struct_attr_str());

     }