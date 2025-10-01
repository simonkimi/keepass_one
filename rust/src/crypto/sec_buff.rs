pub struct SecBuffer {
    content: Vec<u8>,
}

impl SecBuffer {
    pub fn new(data: &[u8]) -> Self {
        let mut cont = system_crypto::encrypt(data).unwrap();
        memlock::mlock(cont.as_mut_ptr(), cont.capacity());
        Self { content: cont }
    }
}

mod memlock {
    pub fn mlock<T: Sized>(cont: *mut T, count: usize) {}

    pub fn munlock<T: Sized>(cont: *mut T, count: usize) {}
}

mod system_crypto {
    pub fn encrypt(data: &[u8]) -> anyhow::Result<Vec<u8>> {
        Ok(data.to_vec())
    }
    pub fn decrypt(data: &[u8]) -> anyhow::Result<Vec<u8>> {
        Ok(data.to_vec())
    }
}
