use b::CustomErrorHandler;
use c::cnb_runtime;

fn main() {
    cnb_runtime(CustomErrorHandler {});
}
