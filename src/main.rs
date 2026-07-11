//use cpal::Data;
use cpal::traits::{HostTrait, DeviceTrait//, StreamTrait
};

fn main() {
    println!("Opening Audio Stream:");
    let host = cpal::default_host();
    let device = dbg!(host.default_output_device())
        .expect("No device found");
    let supported_config = device.supported_output_configs()
        .expect("Error while querying configs")
        .next()
        .expect("No supported configs")
        .with_max_sample_rate();

    dbg!(&supported_config);
    // let stream = device.build_input_stream(
    //     supported_config,
    //     move |data: &mut [f32], _: &cpal::InputCallbackInfo| {

    //     },
    //     move |err| panic!("{}", err),
    //     None
    // ).unwrap();
}
