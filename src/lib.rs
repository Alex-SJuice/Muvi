pub mod loopback {
    use cpal::traits::{HostTrait, DeviceTrait};
    use std::sync::mpsc;

    pub fn create_loopback_stream(tx: mpsc::Sender<Vec<f32>>) -> cpal::Stream {
        let host = cpal::default_host();

        let device = host.default_output_device()
            .unwrap();
        let supported_config = device.default_output_config()
            .unwrap();
        let config = supported_config.config();

        println!("{config:?}");

        device.build_input_stream(
            config,
            move |data: &[f32], _| {let _ = tx.send(data.to_vec()).unwrap();},
            move |err| {panic!("{err:?}");},
            None
        ).unwrap()
    }
}