pub mod loopback {
    use std::error;

    fn nearest_p2_below (num: u32) -> u32 {
        if num == 0 {return 0;}
        2u32.pow(num.ilog2())
    }

    pub trait Capture {
        fn new() -> Result<Self, Box<dyn error::Error>>;
        fn query_data(&self) -> Vec<f32>;
        fn query_err(&self) -> Option<Box<dyn error::Error>>;
        
        fn get_callback_rate(&self) -> u32;
        fn get_buffer_size(&self) -> usize;
    }

    #[cfg(target_os = "linux")]
    mod linux {
        use cpal::traits::{HostTrait, DeviceTrait};
        use cpal::{StreamConfig, SupportedBufferSize, BufferSize};
        use std::sync::mpsc;
        use std::error;

        pub struct LinuxCapture {
            stream: cpal::Stream,
            receiver: mpsc::Receiver<Vec<f32>>,
            err: mpsc::Receiver<Box<dyn error::Error>>,
            callback_rate: u32,
            buffer_size: usize,
        }

        impl Capture for LinuxCapture {
            pub fn new() -> Result<Self, Box<dyn error::Error>> {
                let (data_send, data_receive) = mpsc::channel();
                let (err_send, err_receive) = mpsc::channel();

                let host = cpal::default_host();
                let device = 
                    host.devices()?
                        .find(|d| {
                            d.description()
                            .is_ok_and(|desc| desc.name().contains(".monitor"))
                        })
                        .unwrap_or(host.default_output_device()?);
                for i in host.devices().unwrap() {
                    println!("{i:?}");
                } 
                println!("{device:?}");

                let config_range = device.default_output_config()?;
                let buffer_size = match config_range.buffer_size() {
                    SupportedBufferSize::Range { _, max } => BufferSize::Fixed(nearest_p2_below(max)),
                    SupportedBufferSize::Unknown => BufferSize::Fixed(1024u32),
                }
                let config = StreamConfig {
                    channels: 1,
                    sample_rate: config_range.sample_rate(),
                    buffer_size,
                }
                println!("{config:?}");

                let stream = device.build_input_stream(
                    config,
                    move |data: &[f32], _| {
                        data_send.send(data.to_vec()).unwrap();
                        /*unwrap is ok here since it should not be possible
                          for the main program to die before the stream*/
                    },
                    move |err| {
                        err_send.send(err).unwrap();
                    },
                    None
                )?;

                stream.play()?;

                LinuxCapture {
                    stream,
                    receiver: data_receive,
                    err: err_receive,
                    callback_rate: config_range.sample_rate()/buffer_size,
                    buffer_size,
                }
            }
            pub fn query_data(&self) -> Vec<f32> {
                let data = self.receiver.try_recv().unwrap_or(vec![0.0; self.buffer_size]);
                data.truncate(self.buffer_size);
                data
            }
            pub fn query_err(&self) -> Option<Box<dyn error::Error>> {
                self.err.try_recv().ok()
            }

            pub fn get_callback_rate(&self) -> u32 {
                self.callback_rate
            }
            pub fn get_buffer_size(&self) -> usize {
                self.buffer_size
            }
        }
    }

    #[cfg(target_os = "windows")]
    mod window {
        use wasapi::*;
        use std::thread;

        pub struct WindowsCapture {
            client: AudioClient,
            session_control: 
            receiver: mpsc::Receiver<Vec<f32>>,
            err: mpsc::Receiver<Box<dyn error::Error>>,
            callback_rate: u32,
            buffer_size: usize,
        }

        impl Capture for WindowsCapture {
            pub fn new() -> Result<Self, Box<dyn error::Error>>{
                let (data_send, data_receive) = mpsc::channel();
                let (err_send, err_receive) = mpsc::channel();

                let enumerator = DeviceEnumerator::new()?;
                let device = enumerator.get_default_device(&Direction::Render)?;

                let mut audio_client = device.iaudioclient()?;
                println!("{device:?}");
                
                let desired_format = WaveFormat::new(32, 32, &SampleType::Float, , 1, None);
                let blockalign = desired_format.get_blockalign();

                let (def_time, min_time) = audio_client.get_device_period()?;

                let mode = StreamMode::EventsShared {
                    autoconvert: true,
                    buffer_duration_hns: min_time,
                }
                audio_client.initialize_client(&desired_format, &Direction::Render, &mode)?;

                let h_event = audio_client.set_get_eventhandle()?;
                let buffer_frame_count = audio_client.get_buffer_size()?;
            
                let render_client = audio_client.get_audiocaptureclient()?;
                let mut sample_queue: VecDeque<f32> = VecDeque::with_capacity(
                    100 * blockalign as usize * (1024 + 2 * buffer_frame_count as usize),
                );
                let session_control = audio_client.get_audiosessioncontrol()?;

                println!("state before start: {:?}", session_control.get_state());
                audio_client.start_stream()?;
                println!("state after start: {:?}", session_control.get_state());
                
                WindowsCapture {
                    client: audio_client.clone(),

                }
                
                let join_handle = thread::spawn(move || {
                    loop {
                        while sample_queue.len() > (blockalign as usize * chunksize) {
                            let mut chunk = vec![0.0f32; blockalign as usize * chunksize];
                            for element in chunk.iter_mut() {
                                *element = sample_queue.pop_front().unwrap();
                            }
                            data_send.send(chunk).unwrap();
                        }
                        render_client.read_from_device_to_deque(&mut sample_queue).unwrap();
                        if h_event.wait_for_event(3000).is_err() {
                            err_send.send("timeout error, stopping capture");
                            audio_client.stop_stream().unwrap();
                            break;
                        }
                    }
                });
            }
            pub fn query_data(&self) -> Vec<f32> {}
            pub fn query_err(&self) -> Option<Box<dyn error::Error>> {}
        }
    }

    #[cfg(target_os = "linux")]
    pub type DataStream = linux::LinuxCapture;
    #[cfg(target_os = "windows")]
    pub type DataStream = window::WindowsCapture;
}

pub mod visualizer {

}