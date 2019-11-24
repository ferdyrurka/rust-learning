extern crate actix_web;
#[macro_use]
extern crate tera;

use actix_web::{HttpServer, App, HttpResponse, Responder, get};
use tera::Context;

#[get("/")]
fn home_page() -> impl Responder {
    let tera = compile_templates!("templates/**/*");

    match tera.render("home.html.tera", &Context::new()) {
        Ok(render) => HttpResponse::Ok().content_type("text/html").body(render),
        Err(_error) => HttpResponse::Ok().content_type("text/html").body("Internal server error"),
    }
}

#[get("/unwrap")]
fn unwrap() -> impl Responder {
    let tera = compile_templates!("templates/**/*");

    let mut context: Context = Context::new();
    context.insert("username", &"Ferdyrurka");

    let render = tera.render("unwrap.html.tera", &context).unwrap();

    HttpResponse::Ok().content_type("text/html").body(render)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .service(home_page)
            .service(unwrap)
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .unwrap()
    ;
}

