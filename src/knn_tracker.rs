use opencv::{core, imgproc as cv2, prelude::*, video};

struct KNN {
    gray: core::Mat,
    blur: core::Mat,
    fg_mask: core::Mat,
    thresh: core::Mat,
    erode_kernel: core::Mat,
    dilate_kernel: core::Mat,
    k_size: core::Size,
    cents: types::VectorOfVectorOfPoint,
    knn: types::PtrOfBackgroundSubtractorKNN,
}

impl KNN {
    fn new() -> KNN {
        let k_size = core::Size::new(7, 7);
        let mut cents = types::VectorOfVectorOfPoint::new();
        let mut knn = video::create_background_subtractor_knn(500, 200.0, false).unwrp();
        let erode_kernel = cv2::get_structuring_element(cv2::MORPH_ELLIPSE, (7, 5));
        let dilate_kernel = cv2::get_structuring_element(cv2::MORPH_ELLIPSE, (17, 11));

        KNN {
            gray: core::Mat::default().unwrap(),
            blur: core::Mat::default().unwrap(),
            fg_mask: core::Mat::default().unwrap(),
            thresh: core::Mat::default().unwrap(),
            erode_kernel,
            dilate_kernel,
            k_size,
            cents,
            knn,
        }
    }

    fn apply(&mut self, frame: &core::Mat) -> types::VectorOfVectorOfPoint {
        cv2::cvt_color(&frame, &mut self.gray, cv2::COLOR_BGR2GRAY, 0)?;
        cv2::gaussian_blur(
            &self.gray,
            &mut self.blur,
            self.k_size,
            0.0,
            0.0,
            core::BOBORDER_DEFAULT,
        );
        self.knn.apply(&self.blur, &mut self.fg_mask, -1.0)?;
        cv2::threshold(
            &self.fg_mask,
            &mut self.thresh,
            244.0,
            255.0,
            cv2::THRESH_BINARY,
        )?;
        cv2::erode(&self.thresh, &mut self.thresh, &self.erode_kernel)?;
        cv2::dilate(&self.thresh, &mut self.thresh, &self.erode_kernel)?;
        cv2::find_contours(
            &self.thresh,
            &mut self.cents,
            cv2::RETR_EXTERNAL,
            cv2::CHAIN_APPROX_SIMPLE,
        )?;
        self.cents
    }
}
