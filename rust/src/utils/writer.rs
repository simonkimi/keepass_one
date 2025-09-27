use std::io::SeekFrom;

pub trait Writable {
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error>;
}

impl<W: std::io::Write + std::io::Seek> WriteSeekExt for W {}

pub trait WriteSeekExt: std::io::Write + std::io::Seek {
    fn write_with_lenu32<F>(&mut self, write_fn: F) -> Result<(), std::io::Error>
    where
        F: FnOnce(&mut Self) -> Result<(), std::io::Error>,
    {
        let len_field_pos = self.stream_position()?;
        self.seek(SeekFrom::Current(4))?;
        let data_start = self.stream_position()?;
        write_fn(self)?;
        let data_end = self.stream_position()?;
        let data_len = data_end - data_start;
        if data_len > u32::MAX as u64 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "data length exceeds u32::MAX",
            ));
        }
        self.seek(SeekFrom::Start(len_field_pos))?;
        self.write_all(&(data_len as u32).to_le_bytes())?;
        self.seek(SeekFrom::Start(data_end))?;
        Ok(())
    }
}
