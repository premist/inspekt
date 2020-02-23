#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::env;
use std::collections::HashMap;
use rocket_contrib::templates::Template;

fn get_env_var(env_var: &str) -> String {
    return match env::var(env_var) {
        Ok(val) => val,
        Err(_e) => String::from("")
    };
}

#[get("/")]
fn index() -> Template {
    let a = get_env_var("YOLOSWAG");
    
    // let retval = format!("Yoloswag value: {}", a);
    // retval

    let mut context = HashMap::<String, String>::new();
    context.insert(
        "yoloswag".to_string(),
        "aaaaa".to_string()
    );
    
    return Template::render("index", &context);
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .launch();
}
