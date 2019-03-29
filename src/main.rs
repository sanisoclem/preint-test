#![feature(proc_macro_hygiene, decl_macro)]

extern crate rustaroo_api;

fn main() {
    rustaroo_api::rocket().launch();
}
