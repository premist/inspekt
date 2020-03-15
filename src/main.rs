#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::env;
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_prometheus::PrometheusMetrics;

fn get_env_var(env_var: &str) -> String {
    return match env::var(env_var) {
        Ok(val) => val,
        Err(_e) => String::from("")
    };
}

#[get("/")]
fn index() -> Template {
    
    let mut context = HashMap::<String, String>::new();
    let downward_values = ["NODE_NAME","HOST_IP","POD_NAME","POD_NAMESPACE"];

    for val in downward_values.iter() {
        context.insert(val.to_string(), get_env_var(val));
    }
    
    return Template::render("index", &context);
}

fn main() {
    let prometheus = PrometheusMetrics::new();

    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .attach(Template::fairing())
        .mount("/", routes![index])
        .launch();
}
