use Muvi::loopback;
use std::sync::mpsc;
use cpal::traits::StreamTrait;
use raylib::prelude::*;
use dft;

//use std::thread::sleep;
use std::time::Instant;

fn main() {
    // let (data_send, data_receive) = mpsc::channel();

    // println!("Opening Audio Stream:");
    // let stream = loopback::create_loopback_stream(data_send);
    
    // println!("Starting Stream...");
    // stream.play().unwrap();
    // println!("Stream Started!");

    // let mut data = data_receive.try_recv().unwrap_or(vec![0.0;960]);
    // data.truncate(512);

    // let plan = dft::Plan::new(dft::Operation::Forward, 512);
    // let (mut rl, thread) = raylib::init()
    //     .size(800,600)
    //     .title("Test")
    //     .vsync()
    //     .build();

    // rl.set_target_fps(60);

    // while !rl.window_should_close() {
    //     let start = Instant::now();

    //     data = data_receive.try_recv().unwrap_or(data);
    //     data.truncate(512);
        
    //     let mut max_amp: f32 = 0.0;
    //     for val in &data {
    //         max_amp = max_amp.max(val.abs());
    //     }
    //     dft::transform(&mut data, &plan);


    //     let mut d = rl.begin_drawing(&thread);
    //     d.clear_background(Color::WHITE);
    //     d.draw_text(format!("{max_amp}").as_str(), 700, 400, 20, Color::BLACK);
    //     for i in 0..data.len() {
    //         d.draw_line(i as i32, 0, i as i32, data[i] as i32, Color::BLACK);
    //     }
    //     println!("{:?}", start.elapsed());
    // }
}
