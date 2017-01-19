extern crate zmq;
extern crate iron;
extern crate router;

use std::thread;
use std::time::Duration;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::{Router};

fn main() {
    println!("Starting client..");
    let mut router = Router::new();
    router.get("/light", light_handler, "light_handler");

    Iron::new(router).http("localhost:3000").unwrap();

    fn light_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("light").unwrap_or("/");

        let context = zmq::Context::new();
        let requester = context.socket(zmq::REQ).unwrap();

        requester.connect("tcp://localhost:5559");
        let mut msg = zmq::Message::new();
        requester.send_msg(msg.unwrap(), 0);
        println!("Client sent a message");
        requester.recv_msg(0);
        println!("Client Received a response");
        thread::sleep(Duration::from_millis(1000));

        Ok(Response::with((status::Ok, "received response")))
    }
}
