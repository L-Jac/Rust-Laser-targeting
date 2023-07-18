use opencv::{
    core,
    prelude::*,
    videoio::{
        VideoCapture, 
        VideoCaptureTrait, 
        CAP_PROP_FPS, 
        CAP_PROP_FRAME_HEIGHT, 
        CAP_PROP_FRAME_WIDTH,
    },
};

struct WebcamStream {
    stream_id: i32,
    vcap: VideoCapture,
    grabbed: bool,
    frame: Mat,
}

impl WebcamStream {
    fn new(stream_id: i32) -> WebcamStream {
        let mut vcap = VideoCapture::new(stream_id, 0).unwrap();
        vcap.set(CAP_PROP_FRAME_WIDTH, 640.0).unwrap();
        vcap.set(CAP_PROP_FRAME_HEIGHT, 480.0).unwrap();
        vcap.set(CAP_PROP_FPS, 30.0).unwrap();
        if !vcap.is_opened().unwrap() {
            println!("[Exiting]: Error accessing webcam stream.");
            std::process::exit(0);
        }
        let fps_input_stream = vcap.get(CAP_PROP_FPS).unwrap() as i32;
        println!("FPS of webcam hardware/input stream: {}", fps_input_stream);

        let frame = Mat::default().unwrap();
        let grabbed = vcap.read(&mut frame).unwrap();

        WebcamStream {
            stream_id,
            vcap,
            grabbed,
            frame,
        }
    }

    fn read(&mut self) -> Mat {
        if !self.vcap.read(&mut self.frame).unwrap() {
            println!("[Exiting] No more frames to read");
            self.vcap.release().unwrap();
            std::process::exit(0);
        }
        self.frame.clone().unwrap()
    }
}
