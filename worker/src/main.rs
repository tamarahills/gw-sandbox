extern crate zmq;

fn main() {
    println!("Starting worker..");

    let context = zmq::Context::new();

    let responder = context.socket(zmq::REP).unwrap();
    responder.connect("tcp://localhost:5560");
    loop {
        let msg = responder.recv_msg(0);
        println!("Worker received a message");
        responder.send_msg(msg.unwrap(), 0);
    }
}
