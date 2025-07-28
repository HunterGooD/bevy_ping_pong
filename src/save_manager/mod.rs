pub mod game;
pub mod settings;

#[cfg(target_arch = "wasm32")]
use crate::prelude::*;

#[cfg(target_arch = "wasm32")]
pub struct LocalStorageWriter {
    pub key: String,
    pub buffer: Vec<u8>,
}

#[cfg(target_arch = "wasm32")]
impl std::io::Write for LocalStorageWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        info!("write func call and extend buffer {}", buf.len());
        self.flush();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let s = String::from_utf8(self.buffer.clone()).unwrap();
        info!("start save to {} with data {}", self.key, s.clone());
        web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item(&self.key, &s)
            .unwrap();
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
pub struct LocalStorageReader {
    pub data: Vec<u8>,
    pub position: usize,
}

#[cfg(target_arch = "wasm32")]
impl LocalStorageReader {
    pub fn new(key: String) -> Self {
        let data = match web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item(&key)
            .unwrap()
        {
            None => "".to_string(),
            Some(str) => str,
        };
        Self {
            data: data.into_bytes(),
            position: 0,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl std::io::Read for LocalStorageReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining = self.data.len() - self.position;
        if remaining == 0 {
            return Ok(0);
        }

        let len = buf.len().min(remaining);
        buf[..len].copy_from_slice(&self.data[self.position..self.position + len]);
        self.position += len;
        Ok(len)
    }
}
