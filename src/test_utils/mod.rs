mod output_stream;

pub use self::output_stream::*;

use std::sync::Mutex;

lazy_static! {
    pub static ref LOCK: Mutex<()> = Mutex::new(());
}
