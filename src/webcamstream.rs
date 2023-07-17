use opencv::{
    core,
    prelude::*,
    videoio::{CAP_PROP_FPS, 
        CAP_PROP_FRAME_HEIGHT, 
        CAP_PROP_FRAME_WIDTH, 
        VideoCapture, 
        VideoCaptureTrait},
};
use std::{
    sync::{Arc, Mutex},
    thread,
};

struct WebcamStream {
    stream_id: i32,
    vcap: Arc<Mutex<VideoCapture>>,
    grabbed: bool,
    frame: Arc<Mutex<Mat>>,
    stopped: Arc<Mutex<bool>>,
    t: thread::JoinHandle<()>,
}

impl WebcamStream {
    fn new(stream_id: i32) -> Self {
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

        let stopped = Arc::new(Mutex::new(true));
        let vcap = Arc::new(Mutex::new(vcap));
        let frame = Arc::new(Mutex::new(frame));

        let stopped_clone = Arc::clone(&stopped);
        let vcap_clone = Arc::clone(&vcap);
        let frame_clone = Arc::clone(&frame);
        let t = thread::spawn(move || Self::update(stopped_clone, vcap_clone, frame_clone));

        Self {
            stream_id,
            vcap,
            grabbed,
            frame,
            stopped,
            t,
        }
    }

    fn start(&mut self) {
        *self.stopped.lock().unwrap() = false;
    }

    fn update(stopped: Arc<Mutex<bool>>, vcap: Arc<Mutex<VideoCapture>>, frame: Arc<Mutex<Mat>>) {
        loop {
            if *stopped.lock().unwrap() {
                break;
            }
            if !vcap.lock().unwrap().read(&mut frame.lock().unwrap()).unwrap() {
                println!("[Exiting] No more frames to read");
                *stopped.lock().unwrap() = true;
                break;
            }
        }
        vcap.lock().unwrap().release().unwrap();
    }

    fn read(&self) -> Mat {
        self.frame.lock().unwrap().clone().unwrap()
    }

    fn stop(&mut self) {
        *self.stopped.lock().unwrap() = true;
    }
}
