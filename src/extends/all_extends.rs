use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::fmt::format;
use std::fs::OpenOptions;
use std::str::FromStr;
use std::{fs, env, vec};
use std::io::Read;
use std::path;
use std::path::PathBuf;
use substring::Substring;
use syn::ext::IdentExt;
use syn::{parse_macro_input, token::Token, Attribute, DeriveInput, Ident, Item, ItemFn, Stmt};
use pest::Parser;
use std::collections::HashMap;


#[derive(Parser)]
#[grammar = "./pestcf/struct_centent.pest"]
pub struct ExtendsParser;

    ///
    /// #[extends_struct(derive="Debug,Clone"&&extends="rsdata::dbs::arbatis::base_struct::BaseDO@struct")]
    /// 拆解属性路径
    /// ```
    /// fn path_split();
    /// ```
#[allow(warnings)]
pub fn path_split(extends_attr:String)->(String,String,String){
    let mut extends_path = String::new();
    let extends_split = extends_attr.split("::");
    let mut config_path = String::new();
    let config_path_rs = env::current_dir();

  
    match  config_path_rs{
        Ok(r)=>{
            if let Some(s) = r.to_str(){
                config_path = s.to_string();
            }
        }
        Err(e)=>{
            panic!("error:{:?}",e);
        }
    }
    
    extends_path += &config_path;

    let mut split_vec = vec![];
    extends_split.for_each(|f| {
        split_vec.push(f);
    });
    
    if !extends_attr.starts_with("crate") {
        let mut salsh_idx = extends_path.rfind("\\");
        if salsh_idx == None {
            salsh_idx = extends_path.rfind("/");
        }
        let (f_str, _) = extends_path.split_at(salsh_idx.unwrap());
        extends_path = f_str.to_string() + "/";
        //向上寻找一级文件夹
        if (!path::Path::new(format!("{}{}", extends_path, split_vec[0]).as_str()).exists()) {
            extends_path = extends_path
                .split_at(extends_path.rfind("\\").unwrap())
                .0
                .to_string()
                + "/";
        }
       
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
                //最后一位为对象+对象类型名
                mod_name = f.to_string();
                break;
            }
            extends_path += "/";
            extends_path += f;
        }
    }
    println!("extends_patha:{:?}",extends_path);
    println!("extends_pathb:{:?}",extends_path);
    let (block_name, type_name) = mod_name.split_once("@").unwrap();
    //let file_path = path::Path::new(&extends_path);
    return (extends_path,block_name.to_string(),type_name.to_string());
}



fn setVecAttr(clone_struct:&syn::DeriveInput)->Vec<String>{
    let mut attr_str_vec = vec!();
    match &clone_struct.data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Named(ff) => {
                ff.named.iter().for_each(|f| {
                    let identtrri = &f.ident;
                    let ty = &f.ty;
                    let mut attr_name_str = String::from("");
                    let mut ty_str = String::new();
                    match identtrri {
                        Some(r) => {
                            attr_name_str = r.to_string();
                        }
                        _ => {}
                    }
                    match ty {
                        syn::Type::Path(p) => {
                            let attr_str = p.path.segments.to_token_stream().to_string();
                            if (attr_str.contains("Option")) {
                                ty_str = attr_str
                                    .to_string()
                                    .substring(
                                        attr_str.find("<").unwrap() + 1,
                                        attr_str.rfind(">").unwrap(),
                                    )
                                    .trim()
                                    .to_string();
                            } else {
                                ty_str = attr_str.trim().to_string();
                            }
                        }
                        _ => {}
                    }
                    let attr_str = attr_name_str + ":" + &ty_str;
                    attr_str_vec.push(attr_str);
                });
            }
            _ => (),
        },
        _ => (),
    }
    attr_str_vec
}



    ///
    /// 读取父类子类合并的属性
    /// ```
    /// fn read_struct_attr();
    /// ```
fn read_struct_attr(black_name:String,parent_struct_content_str:String,current_struct_content_str:String)-> Vec<String>{
    let struct_name = Ident::new(black_name.as_str(),Span::call_site());
    let current_struct:proc_macro2::TokenStream = proc_macro2::TokenStream::from_str(&current_struct_content_str.as_str()).unwrap();
    let parent_struct:proc_macro2::TokenStream = proc_macro2::TokenStream::from_str(&parent_struct_content_str.as_str()).unwrap();
    let clone_struct_steam = TokenStream::from(quote! {
        pub struct #struct_name {
            #current_struct
            #parent_struct
        }
    });
    let clone_struct = syn::parse(clone_struct_steam).unwrap();
    setVecAttr(&clone_struct)
}


    ///
    /// 创建一个 内部值都为 None 的struct
    /// # Arguments
    ///
    /// * `Joinz` - xxxxxxxx
    ///
    /// # Example
    ///
    /// ```
    /// struct.get_new_none();
    /// ```
fn get_new_none(black_name:String,parent_struct_content_str:String,current_struct_content_str:String)->proc_macro2::TokenStream{
    let current_struct:proc_macro2::TokenStream = proc_macro2::TokenStream::from_str(&current_struct_content_str.as_str()).unwrap();
    let parent_struct:proc_macro2::TokenStream = proc_macro2::TokenStream::from_str(&parent_struct_content_str.as_str()).unwrap();
    let struct_name = black_name.parse::<proc_macro2::TokenStream>().unwrap();
    let clone_struct_new_model_quote = quote! {
        pub struct #struct_name {
            #current_struct
            #parent_struct
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
    let new_none_ts = proc_macro2::TokenStream::from_str(new_none_str.as_str()).unwrap();
    let clone_struct_new_none_fn_quote = quote! {
         pub fn new_none()->#struct_name{
             #struct_name {
                 #new_none_ts
              }
         }
    };
    clone_struct_new_none_fn_quote
}



fn merge_derive(parent_derive_str:String,derive_current:String)->String{
    let mut derive_rs = String::new();
    let mut dervie_vec = vec!();
   
    if (parent_derive_str.contains("#[derive(")) {
        let str_s1 = parent_derive_str
            .split_at(parent_derive_str.find("(").unwrap() + 1)
            .1;
        let str_rs = str_s1.split_at(str_s1.find(")").unwrap()).0;
        str_rs.split(",").for_each(|f| {
            dervie_vec.push(f.trim().to_string());
        });
    }
    if(!derive_current.trim().eq("")){
        derive_current.split(",").for_each(|f|{
            let der_str = f.to_string();
            if(!dervie_vec.contains(&der_str)){
                dervie_vec.push(der_str);
            }
        })
    }
 
    for der in dervie_vec {
        derive_rs+=&der;
        derive_rs+=",";
    }
    derive_rs = derive_rs.split_at(derive_rs.rfind(",").unwrap()).0.to_string();
    "#[derive(".to_string()+&derive_rs+")]"
}

fn  extends_str(extends_path:String,black_name:String,attr_str: String, _input: String)->Option<TokenStream>{

    let mut unparsed_file = String::new();
    if let Ok(r)=fs::read_to_string(&extends_path){
        unparsed_file = r.clone();
    }
    let mut parent_derive_str = String::new();
    let mut parent_struct_content_str = String::new();
    let mut parent_impl_content_str = String::new();
    let mut current_struct_content_str = String::new();
    let mut current_struct_name_str = String::new();
    let file = ExtendsParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") 
        .next().unwrap();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::strcutstr => {
                let mut  inner_rules = line.into_inner(); 
                let derive_str = inner_rules.next().unwrap().as_str().to_string();
                let name =  inner_rules.next().unwrap().as_str().to_string();
                if name.eq(&black_name){
                    parent_derive_str = derive_str;
                    parent_struct_content_str =  inner_rules.next().unwrap().as_str().trim().to_string();
                }
            }
            Rule::structimplstr => {
                let mut inner_rules = line.into_inner(); 
                let name =  inner_rules.next().unwrap().as_str();
                if name.eq(&black_name){
                    parent_impl_content_str =  inner_rules.next().unwrap().as_str().trim().to_string();
                }
            }
            Rule::EOI => (),
            _ => (),
        }
    }

    
    let file = ExtendsParser::parse(Rule::file, &_input)
        .expect("unsuccessful parse") 
        .next().unwrap();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::strcutstr => {
                let mut  inner_rules = line.into_inner(); 
                 inner_rules.next().unwrap().as_str().to_string();
                current_struct_name_str =  inner_rules.next().unwrap().as_str().trim().to_string();
                current_struct_content_str =  inner_rules.next().unwrap().as_str().trim().to_string();
            }
            Rule::EOI => (),
            _ => (),
        }
    }
   
    let mut derive_current = String::new();
    let attr_split = attr_str.split("&&");
    attr_split.for_each(|_atrra| {
        let (key, val) = _atrra.split_once("=").unwrap();
        if let "derive" = key {
            derive_current = val.to_string();
        }
    });
    
    let  derive_rs = merge_derive(parent_derive_str,derive_current);
    
    let attr_str_vec = read_struct_attr(current_struct_name_str.clone(), parent_struct_content_str.clone(), current_struct_content_str.clone());
     
    let new_none_impl = get_new_none(current_struct_name_str.clone(), parent_struct_content_str.clone(), current_struct_content_str.clone());

    let current_struct:proc_macro2::TokenStream = proc_macro2::TokenStream::from_str(&current_struct_content_str.as_str()).unwrap();
    let parent_struct:proc_macro2::TokenStream = proc_macro2::TokenStream::from_str(&parent_struct_content_str.as_str()).unwrap();
    let struct_name = current_struct_name_str.parse::<proc_macro2::TokenStream>().unwrap();
    let parent_impl = parent_impl_content_str.parse::<proc_macro2::TokenStream>().unwrap();
    let struct_derive = derive_rs.parse::<proc_macro2::TokenStream>().unwrap();
    let attr_impl = quote! {
        pub fn  get_struct_name()->String{
            String::from(#current_struct_name_str)
        }
        pub fn  get_struct_attr_str()->Vec<String>{
            vec!(#(String::from(#attr_str_vec)),*)
        }
        #new_none_impl
        #parent_impl
    };

    let merge_token_strem = TokenStream::from(quote! {
        #struct_derive
        pub struct #struct_name{
            #current_struct
            #parent_struct
        }

        impl #struct_name{
            #attr_impl
        }
    });
    return Some(merge_token_strem.into());
}

#[allow(warnings)]
pub fn impl_extends(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    let input_str = _input.to_string();
    let the_block_str =  input_str.clone();
    //解析入参
    let mut derive = String::new();
    let mut extends_attr = String::new();
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
        if let "extends" = key {
            extends_attr = val.to_string();
        }
    });
    
    //最终文件路径
    let (extends_path,black_name,type_name) =  path_split(extends_attr);
   
    let extents_rs = extends_str(extends_path,black_name,attr_str,the_block_str);

    if let Some(ts) =extents_rs{
        return ts;
    }
    
    TokenStream::from(quote!(input_str))
}

