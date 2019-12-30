#![feature(new_uninit)]

#[macro_use]
extern crate actix_web;
extern crate sysfs_gpio;

use std::{env, io};
use actix_web::{middleware, App, error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_rt;
use sysfs_gpio::{Direction, Pin};

#[get("/gpio/{pin}")]
async fn get_pin(req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    match req.match_info().get("pin") {
        Some(s) => match s.parse::<u64>() {
            Ok(pin) => {
                let gpio_pin = Pin::new(pin);
                match gpio_pin.export() {
                    Ok(_) => match gpio_pin.set_direction(Direction::In) {
                        Ok(_) => match gpio_pin.get_value() {
                            Ok(value) => match gpio_pin.unexport() {
                                Ok(_) => Ok(HttpResponse::Ok().body(value.to_string())),
                                Err(err) => Err(error::ErrorInternalServerError(format!("failed to unexport pin {}: {}", pin, err)))
                            }
                            Err(err) => Err(error::ErrorInternalServerError(format!("failed to get value from pin {}: {}", pin, err)))
                        },
                        Err(err) => Err(error::ErrorInternalServerError(format!("failed to export pin {}: {}", pin, err)))
                    },
                    Err(err) => Err(error::ErrorInternalServerError(format!("failed to set pin {} direction In: {}", pin, err)))
                }
            },
            Err(_) => Err(error::ErrorBadRequest("invalid pin"))
        },
        None => Err(error::ErrorBadRequest("pin param missing"))
    }
}

#[post("/gpio/{pin}/on")]
async fn post_pin_on(req: HttpRequest) -> impl Responder {
    post_pin(req, 1).await
}

#[post("/gpio/{pin}/off")]
async fn post_pin_off(req: HttpRequest) -> impl Responder  {
    post_pin(req, 0).await
}

async fn post_pin(req: HttpRequest, value: u8) -> impl Responder {
    match req.match_info().get("pin") {
        Some(s) => match s.parse::<u64>() {
            Ok(pin) => {
                let gpio_pin = Pin::new(pin);
                match gpio_pin.export() {
                    Ok(_) => match gpio_pin.set_direction(Direction::Out) {
                        Ok(_) => match gpio_pin.set_value(value) {
                            Ok(_) => match gpio_pin.unexport() {
                                Ok(_) => Ok(HttpResponse::NoContent().finish()),
                                Err(err) => Err(error::ErrorInternalServerError(format!("failed to unexport pin {}: {}", pin, err)))
                            }
                            Err(err) => Err(error::ErrorInternalServerError(format!("failed to write pin {}: {}", pin, err)))
                        },
                        Err(err) => Err(error::ErrorInternalServerError(format!("failed to export pin {}: {}", pin, err)))
                    },
                    Err(err) => Err(error::ErrorInternalServerError(format!("failed set {} pin direction out: {}", pin, err)))
                }
            },
            Err(_) => Err(error::ErrorBadRequest("invalid pin"))
        },
        None => Err(error::ErrorBadRequest("pin missing"))
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_pin)
            .service(post_pin_on)
            .service(post_pin_off)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}