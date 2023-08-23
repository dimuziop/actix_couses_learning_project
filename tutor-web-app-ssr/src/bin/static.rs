use std::env;
use std::fmt::Error;
use actix_web::{App, HttpServer, HttpResponse, Result, web, error};
use actix_files as fs;
use actix_web::web::Data;
use actix_web::web::Form;
use dotenv::dotenv;
use log::debug;
use serde::{Deserialize, Serialize};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    debug!("Listening on: {}, check out your browser", addr);
    HttpServer::new(|| {
        let tera = Tera::new(concat!(
        env!("CARGO_MANIFEST_DIR"), "/static/iter1/**/*"
        )).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .service(fs::Files::new(
                "/static", "./static",
            ).show_files_listing())
            .configure(app_config)
    })
        .bind(addr)?.run().await
}

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/users").route(web::post().to(handle_post)))
            .service(web::resource("/tutors").route(web::get().to(handle_get_tutors)))
    );
}

async fn index(tmpl: web::Data<tera::Tera>) -> std::result::Result<HttpResponse, actix_web::Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "Bob");
    let s = tmpl.render("form.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template Error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

async fn handle_post(tmpl: web::Data<tera::Tera>, params: Form<Tutor>) -> std::result::Result<HttpResponse, actix_web::Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name",  &params.name);
    ctx.insert("text", "Welcome");

    debug!("The user name is: {}", &params.name);

    let s = tmpl.render("user.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template Error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>) -> std::result::Result<HttpResponse, actix_web::Error> {
    let tutors: Vec<Tutor> = vec![
        Tutor {name: "Micho".to_string()},
        Tutor {name: "Tito".to_string()},
        Tutor {name: "Negro".to_string()},
        Tutor {name: "Cabezón".to_string()},
    ];

    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutors);

    let rendered_html = tmpl.render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template Error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered_html))
}

#[cfg(test)]
mod tests {
    use actix_web::dev::ServiceResponse;
    use actix_web::http::{header, StatusCode};
    use actix_web::http::header::HeaderValue;
    use actix_web::dev::Service;
    use actix_web::test;
    use actix_web::test::TestRequest;
    use super::*;

    #[actix_rt::test]
    async fn post_unit_test() {
        let params = Form(Tutor {
            name: "Aristoteles".to_string(),
        });

        let tera = Tera::new(concat!(
        env!("CARGO_MANIFEST_DIR"), "/static/iter1/**/*"
        )).unwrap();

        let webdata_tera = Data::new(tera);

        let resp = handle_post(webdata_tera, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/html")
        );
    }

    #[actix_rt::test]
    async fn post_integration_test() {
        let params = Form(Tutor {
            name: "Aristoteles".to_string(),
        });

        let tera = Tera::new(concat!(
        env!("CARGO_MANIFEST_DIR"), "/static/iter1/**/*"
        )).unwrap();
        let webdata_tera = Data::new(tera);

        let mut app = test::init_service(
            App::new().app_data(webdata_tera).configure(app_config)
        ).await;

        let req = TestRequest::post()
            .uri("/users")
            .set_form(&Tutor{name: "Aristoteles".to_string()})
            .to_request();

        let resp: ServiceResponse = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/html")
        );
    }
}