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

#[cfg(unix)]
mod memlock {
    extern crate libc;

    pub fn mlock<T: Sized>(cont: *mut T, count: usize) {
        let byte_num = count * std::mem::size_of::<T>();
        unsafe {
            let ptr = cont as *mut libc::c_void;
            libc::mlock(ptr, byte_num);
            #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
            libc::madvise(ptr, byte_num, libc::MADV_NOCORE);
            #[cfg(target_os = "linux")]
            libc::madvise(ptr, byte_num, libc::MADV_DONTDUMP);
        }
    }

    pub fn munlock<T: Sized>(cont: *mut T, count: usize) {
        let byte_num = count * std::mem::size_of::<T>();
        unsafe {
            let ptr = cont as *mut libc::c_void;
            libc::munlock(ptr, byte_num);
            #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
            libc::madvise(ptr, byte_num, libc::MADV_CORE);
            #[cfg(target_os = "linux")]
            libc::madvise(ptr, byte_num, libc::MADV_DODUMP);
        }
    }
}

#[cfg(not(unix))]
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
