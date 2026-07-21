use Muvi::loopback;
use std::sync::mpsc;
use cpal::traits::StreamTrait;

//use std::thread::sleep;
//use std::time::Duration;

fn main() {
    let (data_send, data_receive) = mpsc::channel();

    println!("Opening Audio Stream:");
    let stream = loopback::create_loopback_stream(data_send);
    
    println!("Starting Stream...");
    stream.play().unwrap();
    println!("Stream Started!");

    loop {
        let data = data_receive.try_recv().unwrap_or(vec![0.0;960]);
        println!("Data: {data:?}"); 
    }
}
