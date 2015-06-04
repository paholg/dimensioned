#![feature(optin_builtin_traits)]
// #![feature(plugin_registrar, rustc_private)]
// extern crate syntax;
// extern crate rustc;
extern crate num;

// use rustc::plugin::Registry;

// use syntax::ptr::P;
// use syntax::ast::{Item, MetaItem};
// use syntax::ext::base::ExtCtxt;
// use syntax::codemap::Span;
// use syntax::ext::base::SyntaxExtension::Modifier;
// use syntax::parse::token::intern;

// #[plugin_registrar]
// pub fn registrar(reg: &mut Registry) {
//     reg.register_syntax_extension(intern("extension"), Modifier(Box::new(expand)));
// }

// fn expand(_: &mut ExtCtxt, _: Span, _: &MetaItem, item: P<Item>) -> P<Item> {
//     println!("Hello World!");
//     return item;
// }

pub use dimensioned::*;

mod peano;
#[macro_use]
mod dimensioned;
#[macro_use]
mod make_units;

pub mod si;
pub mod cgs;
