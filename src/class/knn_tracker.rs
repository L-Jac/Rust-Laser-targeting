use opencv::{core, imgproc as cv2, prelude::*, video};

struct KNN {
    gray: core::Mat,
    blur: core::Mat,
    fg_mask: core::Mat,
    thresh: core::Mat,
    erode_kernel: core::Mat,
    dilate_kernel: core::Mat,
    k_size: core::Size_,
    cents: types::VectorOfVectorOfPoint,
    knn: types::PtrOfBackgroundSubtractorKNN,
}

impl KNN {
    fn new() -> KNN {
        let k_size = core::Size_::new(7, 7);
        let mut cents = types::VectorOfVectorOfPoint::new();
        let mut knn = video::create_background_subtractor_knn(500, 200.0, false)?;
        let erode_kernel = cv2::get_structuring_element(cv2::MORPH_ELLIPSE, (7, 5))?;
        let dilate_kernel = cv2::get_structuring_element(cv2::MORPH_ELLIPSE, (17, 11))?;

        KNN {
            gray: core::Mat::default()?,
            blur: core::Mat::default()?,
            fg_mask: core::Mat::default()?,
            thresh: core::Mat::default()?,
            erode_kernel,
            dilate_kernel,
            k_size,
            cents,
            knn,
        }
    }

    fn apply(&mut self, frame: &core::Mat) -> types::VectorOfVectorOfPoint {
        self.gray = cv2::cvt_color(&frame, &mut self.gray, cv2::COLOR_BGR2GRAY, 0)?;
        self.blur = cv2::gaussian_blur(
            &self.gray,
            &mut self.blur,
            self.k_size,
            0.0,
            0.0,
            core::BOBORDER_DEFAULT,
        )?;
        self.fg_mask = self.knn.apply(&self.blur, &mut self.fg_mask, -1.0)?;
        self.thresh = cv2::threshold(
            &self.fg_mask,
            &mut self.thresh,
            244.0,
            255.0,
            cv2::THRESH_BINARY,
        )?;
        self.thresh = cv2::erode(&self.thresh, &mut self.thresh, &self.erode_kernel)?;
        self.thresh = cv2::dilate(&self.thresh, &mut self.thresh, &self.erode_kernel)?;
        self.cents = cv2::find_contours(
            &self.thresh,
            &mut self.cents,
            cv2::RETR_EXTERNAL,
            cv2::CHAIN_APPROX_SIMPLE,
        )?;
        self.cents
    }
}
