#![feature(proc_macro_hygiene, decl_macro)] //feature gate which is using unstable features

#[macro_use]
extern crate rocket; //using rocket

#[get("/")] //route attribute
fn index() -> &'static str //referece with lifetime variable 'static
{
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch(); //creating new instace
}
