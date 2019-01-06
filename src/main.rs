#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
use actix;
use actix_web::http::Method;
use actix_web::{middleware, pred, server, App, HttpRequest, HttpResponse};
use clap;
use clap::Arg;
use failure::Error;
use futures::future::{result, FutureResult};
use log::{debug, error};
use std::env;
use std::process::Command;

mod dhcp_pool;

#[derive(Debug, Fail)]
pub enum DHCPDPoolExecuteError {
    #[fail(display = "no output from process error")]
    NoOutputError,
    #[fail(display = "dhcpd_pool error: {}", msg)]
    DHCPDPoolError { msg: String },
}

fn execute() -> Result<String, Error> {
    let output = Command::new("dhcpd-pools").args(&["--format=j"]).output()?;
    debug!("dhcpd-pools output: {:?}", output);

    if !output.stderr.is_empty() {
        let error = String::from_utf8(output.stderr)?;
        return Err(DHCPDPoolExecuteError::DHCPDPoolError { msg: error }.into());
    }

    if output.stdout.is_empty() {
        return Err(DHCPDPoolExecuteError::NoOutputError.into());
    }

    let output_string = String::from_utf8(output.stdout)?;
    let pool: dhcp_pool::DHCPDPool = serde_json::from_str(&output_string)?;

    let mut s = String::with_capacity(1024);
    for subnet in &pool.subnets {
        debug!("subnet {:?}", subnet);
        s.push_str(&format!(
            "dhcp_pool_used{{ip_version=\"4\",network=\"{}\",range=\"{}\"}} {}\n",
            subnet.location, subnet.range, subnet.used
        ));
        s.push_str(&format!(
            "dhcp_pool_free{{ip_version=\"4\",network=\"{}\",range=\"{}\"}} {}\n",
            subnet.location, subnet.range, subnet.free
        ));
        s.push_str(&format!(
            "dhcp_pool_touched{{ip_version=\"4\",network=\"{}\",range=\"{}\"}} {}\n",
            subnet.location, subnet.range, subnet.touched
        ));
    }

    Ok(s)
}

fn metrics_handler(_req: &HttpRequest) -> FutureResult<HttpResponse, actix_web::Error> {
    match execute() {
        Ok(s) => result(Ok(HttpResponse::Ok().content_type("text/plain").body(s))),
        Err(err) => {
            error!("{:?}", err);

            result(Ok(HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(format!("** ERR: {}", err))))
        }
    }
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
        env::set_var("RUST_LOG", "actix_web=warn,prometheus_dhcp_exporter=warn");
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
    .shutdown_timeout(0)
    .start();

    println!("starting exporter on {}", bind);

    sys.run();
}
