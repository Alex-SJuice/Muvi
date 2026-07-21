pub mod loopback {
    use cpal::traits::{HostTrait, DeviceTrait};
    use std::sync::mpsc;

    pub fn create_loopback_stream(data_out: mpsc::Sender<Vec<f32>>) -> cpal::Stream {
        let host = cpal::default_host();

        let device = host.default_output_device()
            .unwrap();
        let supported_config = device.default_output_config()
            .unwrap();
        let config = supported_config.config();

        println!("{config:?}");

        device.build_input_stream(
            config,
            move |data: &[f32], _| {
                data_out.send(data.to_vec()).unwrap();
                /*unwrap is ok here since it should not be possible
                  for the main program to die before the stream*/
            },
            move |err| {panic!("{err:?}");},
            None
        ).unwrap()
    }
}