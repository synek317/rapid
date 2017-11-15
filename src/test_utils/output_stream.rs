use std::sync::{Arc, Mutex};
use std::io::{self, Write};

#[derive(Clone)]
pub struct OutputStream(Arc<Mutex<Vec<u8>>>);

impl OutputStream {
    pub fn new() -> Self {
        OutputStream (Arc::new(Mutex::new(Vec::new())))
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&*self.0.lock().unwrap()).to_string()
    }

    pub fn capture<F: FnOnce(Self)>(f: F) -> String {
        let output_stream = Self::new();

        f(output_stream.clone());

        output_stream.to_string()
    }
}

impl Write for OutputStream  {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (*self.0.lock().unwrap()).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (*self.0.lock().unwrap()).flush()
    }
}
