use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecureDataError {
    #[cfg(windows)]
    #[error("CryptProtectMemory failed: {0}")]
    CryptProtectMemoryFailed(windows::core::Error),

    #[cfg(windows)]
    #[error("CryptUnprotectMemory failed: {0}")]
    CryptUnprotectMemoryFailed(windows::core::Error),

    #[cfg(windows)]
    #[error("VirtualLock failed: {0}")]
    VirtualLockFailed(windows::core::Error),

    #[cfg(unix)]
    #[error("Memory lock failed: {0}")]
    MemoryLockFailed(String),

    #[cfg(unix)]
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[cfg(unix)]
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Invalid data length: expected {expected}, got {actual}")]
    InvalidDataLength { expected: usize, actual: usize },
}

#[cfg(windows)]
pub mod memory_crypt {
    use windows::Win32::Security::Cryptography::{
        CryptProtectMemory, CryptUnprotectMemory, CRYPTPROTECTMEMORY_BLOCK_SIZE,
        CRYPTPROTECTMEMORY_SAME_PROCESS,
    };
    use windows::Win32::System::Memory::{VirtualLock, VirtualUnlock};

    use crate::crypto::memory_crypt::SecureDataError;

    const BLOCK_SIZE: usize = CRYPTPROTECTMEMORY_BLOCK_SIZE as usize;

    pub fn crypt_memory(data: &[u8]) -> Result<Vec<u8>, SecureDataError> {
        let size = data.len();
        let mut buffer = data.to_vec();

        let padding = (BLOCK_SIZE - (size % BLOCK_SIZE)) % BLOCK_SIZE;
        if padding > 0 {
            buffer.resize(size + padding, 0);
        }

        unsafe {
            CryptProtectMemory(
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as u32,
                CRYPTPROTECTMEMORY_SAME_PROCESS,
            )
            .map_err(SecureDataError::CryptProtectMemoryFailed)?;
            Ok(buffer)
        }
    }

    pub fn uncrypt_memory(data: &[u8], original_len: usize) -> Result<Vec<u8>, SecureDataError> {
        let mut temp_buffer = data.to_vec();
        unsafe {
            CryptUnprotectMemory(
                temp_buffer.as_mut_ptr() as *mut _,
                temp_buffer.len() as u32,
                CRYPTPROTECTMEMORY_SAME_PROCESS,
            )
            .map_err(SecureDataError::CryptUnprotectMemoryFailed)?;

            if temp_buffer.len() < original_len {
                return Err(SecureDataError::InvalidDataLength {
                    expected: original_len,
                    actual: temp_buffer.len(),
                });
            }

            if temp_buffer.len() != original_len {
                temp_buffer.truncate(original_len);
            }

            Ok(temp_buffer)
        }
    }

    pub fn mlock(data: &mut Vec<u8>) -> Result<(), SecureDataError> {
        if data.is_empty() {
            return Ok(());
        }

        unsafe {
            let ptr = data.as_mut_ptr() as *const std::ffi::c_void;
            let len = data.len();

            VirtualLock(ptr, len).map_err(SecureDataError::VirtualLockFailed)?;
        }
        Ok(())
    }

    pub fn munlock(data: &[u8]) {
        if data.is_empty() {
            return;
        }

        unsafe {
            let ptr = data.as_ptr() as *const std::ffi::c_void;
            let len = data.len();

            if let Err(err) = VirtualUnlock(ptr, len) {
                eprintln!("Warning: VirtualUnlock failed: {}", err);
            }
        }
    }
}

#[cfg(unix)]
pub mod memory_crypt {
    use crate::crypto::memory_crypt::SecureDataError;

    pub fn crypt_memory(data: &[u8]) -> Result<Vec<u8>, SecureDataError> {
        Ok(data.to_vec())
    }

    pub fn uncrypt_memory(data: &[u8], original_len: usize) -> Result<Vec<u8>, SecureDataError> {
        Ok(data.to_vec())
    }

    pub fn mlock(data: &mut Vec<u8>) -> Result<(), SecureDataError> {
        if data.is_empty() {
            return Ok(());
        }

        unsafe {
            let ptr = data.as_mut_ptr() as *const std::ffi::c_void;
            let len = data.len();

            if libc::mlock(ptr, len) != 0 {
                let errno = std::io::Error::last_os_error();
                return Err(SecureDataError::MemoryLockFailed(format!(
                    "mlock failed for {} bytes: {}",
                    len, errno
                )));
            }

            #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
            {
                if libc::madvise(ptr, len, libc::MADV_NOCORE) != 0 {
                    eprintln!(
                        "Warning: madvise MADV_NOCORE failed: {}",
                        std::io::Error::last_os_error()
                    );
                }
            }

            #[cfg(target_os = "linux")]
            {
                if libc::madvise(ptr, len, libc::MADV_DONTDUMP) != 0 {
                    eprintln!(
                        "Warning: madvise MADV_DONTDUMP failed: {}",
                        std::io::Error::last_os_error()
                    );
                }
            }
        }
        Ok(())
    }

    pub fn munlock(data: &[u8]) {
        if data.is_empty() {
            return;
        }

        unsafe {
            let ptr = data.as_ptr() as *const std::ffi::c_void;
            let len = data.len();
            if libc::munlock(ptr, len) != 0 {
                eprintln!(
                    "Warning: munlock failed: {}",
                    std::io::Error::last_os_error()
                );
            }
        }
    }
}
