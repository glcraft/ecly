use lazy_static::lazy_static;

use std::{io::*, sync::{Arc, Mutex}};

lazy_static! {
    pub static ref LOG: Arc<Mutex<BufWriter<Stdout>>> = Arc::new(Mutex::new(BufWriter::new(stdout())));
}
#[macro_export]
macro_rules! log_writeln {
    ($($arg:tt)*) => {
        writeln!($crate::log::log.lock().expect("log lock error"), $($arg)*)
    };
}

pub fn flush() -> std::io::Result<()>{
    LOG.lock().expect("log lock error").flush()
}