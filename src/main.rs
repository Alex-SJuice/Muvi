use Muvi::loopback;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    println!("Opening Audio Stream:");

    let stream = loopback::create_loopback_stream(tx.clone());
    
}

