// mod errors;
// mod for_windows;
//
// pub use errors::{Error, Result};


use ie_runtime::IEWorkspace;


fn main() {
    let ie = IEWorkspace::default();
    ie.run()
}