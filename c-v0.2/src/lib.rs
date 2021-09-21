pub trait ErrorHandler {}

pub fn cnb_runtime(_error_handler: impl ErrorHandler) {}
