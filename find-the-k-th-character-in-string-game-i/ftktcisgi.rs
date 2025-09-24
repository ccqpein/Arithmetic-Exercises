extern crate temp_macro;
use temp_macro::make_answer;

// extern crate proc_macro;

// use proc_macro::TokenStream;
// use syn::{Expr, ExprLit, ExprTuple, Lit, parse_macro_input};

// #[proc_macro]
// pub fn make_answer(item: TokenStream) -> TokenStream {
//     let expr: Expr = parse_macro_input!(item as Expr);

//     let mut string_value: Option<String> = None;
//     let mut int_value: Option<i64> = None;

//     let Expr::Tuple(ExprTuple { elems, .. }) = expr else {
//         panic!()
//     };

//     for elem in elems {
//         match elem {
//             Expr::Lit(ExprLit { lit, .. }) => {
//                 match lit {
//                     Lit::Str(lit_str) => {
//                         if string_value.is_none() {
//                             string_value = Some(lit_str.value());
//                         } else {
//                         }
//                     }
//                     Lit::Int(lit_int) => {
//                         if int_value.is_none() {
//                             match lit_int.base10_parse::<i64>() {
//                                 Ok(val) => int_value = Some(val),
//                                 Err(e) => {
//                                     // Handle parsing error for the integer
//                                     return syn::Error::new_spanned(
//                                         lit_int,
//                                         format!("failed to parse integer literal: {}", e),
//                                     )
//                                     .to_compile_error()
//                                     .into();
//                                 }
//                             }
//                         } else {
//                         }
//                     }
//                     _ => {}
//                 }
//             }
//             _ => {}
//         }
//     }

//     //println!("{:?}, {:?}", string_value, int_value);
//     let mut ss = string_value.unwrap();
//     let intt = int_value.unwrap() as usize;
//     for _ in 0..intt {
//         ss = one_round(ss);
//         if ss.len() >= intt {
//             return format!("const all: &str = \"{}\";", ss).parse().unwrap();
//         }
//     }

//     r#"const all: &str = "";"#.parse().unwrap()
// }

// fn one_round(s: String) -> String {
//     let mut res = vec![];
//     for c in s.chars() {
//         res.push(c);
//         res.push(match c {
//             'a'..='y' => (c as u8 + 1) as char,
//             'z' => 'a',
//             _ => unreachable!(),
//         })
//     }
//     String::from_iter(res)
// }

make_answer!(("a", 500));

fn main() {
    //make_answer!(("a", 0));
    //println!("{}", answer());
    println!("{}", all.get(499..500).unwrap());
}
