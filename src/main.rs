#[macro_use]
extern crate serde_derive;
use actix;
use actix_web::http::Method;
use actix_web::{middleware, pred, server, App, Error, HttpRequest, HttpResponse};
use clap;
use clap::Arg;
use futures::future::{result, FutureResult};
use log::info;
use std::env;

mod dhcp_pool;

fn metrics_handler(_req: &HttpRequest) -> FutureResult<HttpResponse, Error> {
    //    println!("{:?}", req);

    result(Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", 100))))
}

fn main() {
    let matches = clap::App::new("prometheus_dhcp_exporter")
        .version("0.1")
        .author("Francesco Cogno <francesco.cogno@outlook.com>")
        .arg(
            Arg::with_name("port")
                .short("p")
                .help("exporter port (default 9979)")
                .default_value("9979")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("verbose logging")
                .takes_value(false),
        )
        .get_matches();

    if matches.is_present("verbose") {
        println!("verbose logging enabled");
        env::set_var("RUST_LOG", "actix_web=debug");
    }
    env_logger::init();

    let bind = format!("127.0.0.1:{}", matches.value_of("port").unwrap());

    let sys = actix::System::new("prometheus_dhcp_exporter");

    let _addr = server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/metrics", |r| r.method(Method::GET).a(metrics_handler))
            .default_resource(|r| {
                // all requests that are not `GET`
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_| HttpResponse::MethodNotAllowed());
            })
    })
    .bind(bind.clone())
    .unwrap()
    .start();

    println!("starting exporter on {}", bind);

    sys.run();
}
