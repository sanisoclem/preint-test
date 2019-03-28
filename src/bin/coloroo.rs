#![feature(proc_macro_hygiene, decl_macro)]

extern crate coloroo_api;

fn main() {
    coloroo_api::rocket().launch();
}
