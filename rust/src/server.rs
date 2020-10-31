use futures::future;
use hyper::server::Http;
use std::thread;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate prost_twirp;

mod service {
    include!(concat!(env!("OUT_DIR"), "/helloworld.rs"));
}
use service::{Greeter, HelloReply, HelloRequest, PTReq, PTRes};


#[derive(Clone)]
struct GreeterService;
impl Greeter for GreeterService {
    fn say_hello(&self, req: PTReq<HelloRequest>) -> PTRes<HelloReply> {
        println!("say_hello: request={:?}", req);
        Box::new(future::ok(
            HelloReply {
                message: format!("Hello, {}", req.input.name),
            }
            .into(),
        ))
    }
}

fn main() {
    let thread_res = thread::spawn(|| {
        let addr = "0.0.0.0:5000".parse().unwrap();
        println!("Starting server: {}", addr);
        let server = Http::new()
            .bind(&addr, move || Ok(Greeter::new_server(GreeterService)))
            .unwrap();
        let res = server.run().or_else(|err| {
            println!("{:?}", err);
            Err(err)
        });
        res
    });
    if let Err(err) = thread_res.join() {
        println!("Server panicked: {:?}", err);
    }
}
