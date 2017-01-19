extern crate zmq;

fn main() {
    println!("Starting Broker..");

    let context = zmq::Context::new();
    let front_end = context.socket(zmq::ROUTER).unwrap();
    let back_end = context.socket(zmq::DEALER).unwrap();
    assert!(front_end.bind("tcp://*:5559").is_ok());
    assert!(back_end.bind("tcp://*:5560").is_ok());

    // Initialize Poll set
    let mut items = [
        front_end.as_poll_item(zmq::POLLIN),
        back_end.as_poll_item(zmq::POLLIN),
    ];

    loop {
        zmq::poll(&mut items, -1).unwrap();
        if items[0].is_readable() {
            loop {
                let recv = front_end.recv_msg(0);
                if recv.is_ok() {
                    let more = front_end.get_rcvmore().unwrap();
                    back_end.send_msg(recv.unwrap(), if more {zmq::SNDMORE} else {0});
                    if !more {
                        break;
                    }
                }
            }
        }
        if items[1].is_readable() {
            loop {
                let recv = back_end.recv_msg(0);
                if recv.is_ok() {
                    let more = back_end.get_rcvmore().unwrap();
                    front_end.send_msg(recv.unwrap(), if more {zmq::SNDMORE} else {0});
                    if !more {
                        break;
                    }
                }
            }
        }
    }
}
