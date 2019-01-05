use actix;
use actix_web::http::Method;
use actix_web::{
    http, middleware, pred, server, App, Error, HttpRequest, HttpResponse, Path, Responder,
};
use futures::future::{result, FutureResult};
use std::env;

// async handler
fn index_async(req: &HttpRequest) -> FutureResult<HttpResponse, Error> {
    println!("{:?}", req);

    result(Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", 100))))
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    let sys = actix::System::new("prometheus_dhcp_exporter");

    let addr = server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/metrics", |r| r.method(Method::GET).a(index_async))
            .default_resource(|r| {
                // all requests that are not `GET`
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|req| HttpResponse::MethodNotAllowed());
            })
    })
    .bind("127.0.0.1:9979")
    .unwrap()
    .start();
    sys.run();
}
