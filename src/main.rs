#[macro_use]
extern crate serde_derive;
use actix;
use actix_web::http::Method;
use actix_web::{middleware, pred, server, App, Error, HttpRequest, HttpResponse};
use clap;
use clap::Arg;
use futures::future::{result, FutureResult};
use log::{debug, error, info};
use std::env;
use std::process::Command;

mod dhcp_pool;

fn metrics_handler(_req: &HttpRequest) -> FutureResult<HttpResponse, Error> {
    //    println!("{:?}", req);
    let output = match Command::new("dhcpd-pools").args(&["--format=j"]).output() {
        Ok(output) => output,
        Err(error) => {
            error!("invoke process error: {:?}", error);
            return result(Ok(HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(format!("** ERR: {}", error))));
        }
    };

    debug!("{:?}", output);

    if output.stderr.len() > 0 {
        let error = String::from_utf8(output.stderr).unwrap();
        error!("dhcpd_pools error: {}", error);
        return result(Ok(HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body(error)));
    }

    let output_string = String::from_utf8(output.stdout).unwrap();
    let pool: dhcp_pool::DHCPDPool = match serde_json::from_str(&output_string) {
        Ok(pool) => pool,
        Err(error) => {
            error!("json parse error: {:?}", error);
            return result(Ok(HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(format!("json parse error: {}", error))));
        }
    };

    result(Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("{:?}", pool))))
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
        env::set_var("RUST_LOG", "actix_web=debug,prometheus_dhcp_exporter=debug");
    } else {
        env::set_var("RUST_LOG", "actix_web=info,prometheus_dhcp_exporter=info");
    }
    env_logger::init();

    let bind = format!("0.0.0.0:{}", matches.value_of("port").unwrap());

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
