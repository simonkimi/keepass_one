use std::io::{Cursor, Seek, SeekFrom};

pub trait CursorExt {
    fn read_slice(&mut self, len: usize) -> anyhow::Result<&[u8]>;
    fn remaining(&self) -> usize;
}

impl CursorExt for Cursor<&[u8]> {
    fn read_slice(&mut self, len: usize) -> anyhow::Result<&[u8]> {
        if len == 0 {
            return Ok(&[]);
        }

        let current_pos = self.position() as usize;
        let end_pos = current_pos.checked_add(len).ok_or(anyhow::anyhow!(
            "Attempt to read beyond the end of the buffer"
        ))?;
        if end_pos > self.get_ref().len() {
            return Err(anyhow::anyhow!(
                "Attempt to read beyond the end of the buffer"
            ));
        }

        let slice = &self.get_ref()[current_pos..end_pos];
        self.seek(SeekFrom::Current(len as i64))?;
        Ok(slice)
    }
    
    fn remaining(&self) -> usize {
        self.get_ref().len() - self.position() as usize
    }
}
