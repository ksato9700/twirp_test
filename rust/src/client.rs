use apis::service;
use futures::future::Future;
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let hyper_client = Client::new(&core.handle());
    let service_client = service::Greeter::new_client(hyper_client, "http://localhost:5000");

    let work = service_client
        .say_hello(
            service::HelloRequest {
                name: "Ken".to_string(),
                ver: 123,
                blood_type: 1, // service::BloodType::B,
            }
            .into(),
        )
        .and_then(|res| Ok(println!("{:?}", res.output.message)));
    core.run(work).unwrap();
}
