use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use std::fs;
use std::io::Read;
use std::path;
use std::path::PathBuf;
use syn::ext::IdentExt;
use syn::{parse_macro_input, token::Token, Attribute, DeriveInput, Ident, Item, ItemFn, Stmt};

pub fn impl_extends_struct(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    //解析入参
    let mut derive = String::new();
    let mut extends = String::new();
    let attr_str = _attr
        .to_string()
        .replace("/", "")
        .replace('\n', "")
        .replace('\\', "")
        .replace('"', "")
        .replace(" ", "");
    let attr_split = attr_str.split("&&");
    attr_split.for_each(|_atrra| {
        let (key, val) = _atrra.split_once("=").unwrap();
        if let "derive" = key {
            derive = val.to_string();
        }
        if let "extends" = key {
            extends = val.to_string();
        }
    });
    let (content_code_str, derive_vec) = split_mod_str(extends);

    //合并父类的 宏
    if (derive.len() > 0) {
        derive += ",";
    }
    for der in derive_vec {
        if (!derive.contains(&der)) {
            derive += &der;
            derive += ",";
        }
    }
    //去除结尾的逗号
    derive = derive.split_at(derive.rfind(",").unwrap()).0.to_string();
    let derive_input = parse_macro_input!(_input as DeriveInput);
    let struct_name = derive_input.ident;
    let struct_name_str = struct_name.to_string();
    let mut attr_ald = vec![];
    let mut attr_str_vec: Vec<String> = vec![];
    // let  derive_aainput = parse_macro_input!(stream as Stmt);
    //eprintln!("c:{:#?}",_attr);
    //eprintln!("a:{:#?}",stream);

    match &derive_input.data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Named(ff) => {
                ff.named.iter().for_each(|f| {
                    let identtrri = &f.ident;
                    let ty = &f.ty;
                    let afs = quote! {pub #identtrri:#ty,};
                    attr_ald.push(afs);
                });
            }
            _ => (),
        },
        _ => (),
    }

    //来自父类的内容
    let extents_parent_attr = content_code_str
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let mut extents_parent_derive = String::new();
    if (derive.len() > 0) {
        extents_parent_derive = "#[derive(".to_string() + &derive + ")]";
    }

    //struct impl 内容
    //let impl_extents_parent_fn =   impl_content_code_str.parse::<proc_macro2::TokenStream>().unwrap();

    //组合内容
    let derive_code = extents_parent_derive
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let clone_struct_quote = quote! {
        #derive_code
        struct #struct_name {
            #(#attr_ald)*
            #extents_parent_attr
        }
    };

    //创建一个new  默认的 new  none
    let clone_struct_new_model_quote = quote! {
         #struct_name {
            #(#attr_ald)*
            #extents_parent_attr
        }
    };

    let struct_all_code_str = clone_struct_new_model_quote.to_string();

    let right_st_str = &struct_all_code_str
        .split_at(struct_all_code_str.find("{").unwrap() + 1)
        .1;
    let center_code_str = right_st_str.split_at(right_st_str.find("}").unwrap()).0;
    //let center_code_str = center_code_str.replace('\n', "").replace('\r', "");
    let attr_code_str_split = center_code_str.trim().split(",");

    let mut new_none_str = String::new();
    attr_code_str_split.for_each(|f| {
        if (f.contains(":")) {
            let (attr_name, _) = f.split_once(":").unwrap();
            new_none_str += &(attr_name.replace("pub", "").to_string() + ":None,");
        }
    });

    let new_none_ts = new_none_str.parse::<proc_macro2::TokenStream>().unwrap();
    let clone_struct_new_none_fn_quote = quote! {
         pub fn new_none()->#struct_name{
             #struct_name {
                 #new_none_ts
              }
         }
    };

    let clone_struct_stream = TokenStream::from(clone_struct_quote);
    let clone_struct = parse_macro_input!(clone_struct_stream as DeriveInput);

    match &clone_struct.data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Named(ff) => {
                ff.named.iter().for_each(|f| {
                    let identtrri = &f.ident;
                    let ty = &f.ty;
                    let mut attr_name_str = String::from("");
                    let mut ty_str = String::from("");
                    match identtrri {
                        Some(r) => {
                            attr_name_str = r.to_string();
                        }
                        _ => {}
                    }
                    match ty {
                        syn::Type::Path(p) => {
                            ty_str = format!("{}", p.path.segments[0].ident);
                        }
                        _ => {}
                    }
                    let attr_str = attr_name_str + &(":".to_string()) + &ty_str;
                    attr_str_vec.push(attr_str);
                });
            }
            _ => (),
        },
        _ => (),
    }

    //let ast =  "atr_".to_string()+&struct_name.to_string();
    //let aident = Ident::new(ast.as_str(),Span::call_site());

    // let attr_trait = quote! {
    //     trait #aident {
    //         fn get_struct_name()->String;
    //         fn get_struct_attr_str()->Vec<String>;
    //     }
    // };

    let attr_impl = quote! {
        impl #struct_name{
            fn  get_struct_name()->String{
                String::from(#struct_name_str)
            }
            fn  get_struct_attr_str()->Vec<String>{
                vec!(#(String::from(#attr_str_vec)),*)
            }

            #clone_struct_new_none_fn_quote
      }
    };

    let apai = TokenStream::from(quote! {
       #clone_struct
       #attr_impl
    });
    //eprintln!("{:#?}",apai.to_string());
    apai.into()
}

#[allow(warnings)]
fn split_mod_str(extends: String) -> (std::string::String, std::vec::Vec<String>) {
    let mut extends_path = String::new();
    let extends_split = extends.split("::");
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    extends_path += config_path.to_str().unwrap();

    let mut split_vec = vec![];
    extends_split.for_each(|f| {
        split_vec.push(f);
    });

    if !extends.starts_with("crate") {
        let mut salsh_idx = extends_path.rfind("\\");
        if salsh_idx == None {
            salsh_idx = extends_path.rfind("/");
        }
        let (f_str, _) = extends_path.split_at(salsh_idx.unwrap());
        extends_path = f_str.to_string() + "/";
        extends_path += split_vec[0];
    }
    extends_path += "/src";

    let mut mod_name = String::new();
    let split_size = split_vec.len();
    if split_size < 1 {
        panic!("error");
    } else if split_vec.len() == 2 {
        extends_path += "/";
        mod_name = split_vec[1].to_string();
        let path_check_url_lib = extends_path.clone() + "/lib.rs";
        let paht_exist = path::Path::new(&path_check_url_lib);
        if (paht_exist.exists()) {
            extends_path = path_check_url_lib;
        } else {
            extends_path = extends_path.clone() + "/main.rs";
        }
    } else {
        let mut idx = 0_usize;
        for f in split_vec {
            idx += 1;
            if idx < 2 {
                continue;
            }
            if (idx >= split_size) {
                extends_path += ".rs";
                mod_name = f.to_string();
                break;
            }
            extends_path += "/";
            extends_path += f;
        }
    }
    let (block_name, type_name) = mod_name.split_once("*").unwrap();
    let file_path = path::Path::new(&extends_path);
    let mut file = fs::File::open(file_path).ok().unwrap();
    let mut read_str = String::new();
    file.read_to_string(&mut read_str);

    // if (type_name == "struct") {
    //     // read_str.find()
    // }
    let mut codesln = vec![];
    let rsp = read_str.split("\n");
    let mut row_idx = 0_usize;

    //结构体判断记录的内容
    let mut first_row = 0_usize;
    let mut start_read = false;
    let mut ln_idx = vec![];

    //结构体实现判断记录的内容
    // let  mut impl_start_read  = false;
    // let mut impl_ln_idx = vec!();

    rsp.for_each(|f| {
        codesln.push(f);

        //读取结构体内容
        if f.contains("struct") && f.contains(&block_name) && f.contains("{") {
            first_row = row_idx;
            start_read = true;
        }
        if (start_read) {
            ln_idx.push(row_idx);
            if f.contains("}") {
                start_read = false;
            }
        }
        //读取结构体默认的实现内容
        // if f.contains("impl")&&f.contains(&block_name)&&f.contains("{") {
        //     impl_start_read = true;
        // }
        // if(impl_start_read){
        //     println!("-------{:?}",f);
        //     impl_ln_idx.push(row_idx);
        //     if f.contains("}"){
        //         impl_start_read = false;
        //     }
        // }

        row_idx += 1;
    });
    //print!("---dive{:?}",codesln);
    let mut dervie_vec = vec![];

    let derive_dome_str = codesln[first_row - 1];
    if (derive_dome_str.contains("#[derive(")) {
        let str_s1 = derive_dome_str
            .split_at(derive_dome_str.find("(").unwrap() + 1)
            .1;
        let str_rs = str_s1.split_at(str_s1.find(")").unwrap()).0;
        str_rs.split(",").for_each(|f| {
            dervie_vec.push(f.to_string());
        });
    }

    let mut content_code_str = String::new();
    let mut content_idx = 0_usize;
    let blook_code_len = ln_idx.len();
    for number in ln_idx {
        if (content_idx >= 1 && content_idx < (blook_code_len - 1)) {
            //println!("-------{:?}", codesln[number]);
            content_code_str += codesln[number];
        }
        content_idx += 1;
    }
    // let mut impl_content_idx = 0_usize;
    // let mut impl_content_code_str = String::new();
    // let impl_blook_code_len = impl_ln_idx.len();
    // for number in impl_ln_idx {
    //     if(impl_content_idx>=1&&impl_content_idx<(impl_blook_code_len-1)){
    //         impl_content_code_str+= codesln[number];
    //     }
    //     impl_content_idx+=1;
    // }
    // println!("content_code_str:{:?},d-vec:{:?}",content_code_str,dervie_vec);

    (content_code_str, dervie_vec)
}
