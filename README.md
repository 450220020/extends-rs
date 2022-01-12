# extends-rs
rust extends  struct


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