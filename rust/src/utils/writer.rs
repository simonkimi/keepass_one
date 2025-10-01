use std::io::{Cursor, SeekFrom};

use byteorder::{WriteBytesExt, LE};

pub trait Writable {
    fn write<W: std::io::Write + std::io::Seek + Sized>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error>;
}

pub trait FixedSize {
    fn fix_size(&self) -> usize;
}

pub trait FixedSizeExt: std::io::Write + std::io::Seek + Sized {
    fn write_fixed_size_data<T: FixedSize + Writable>(
        &mut self,
        data: &T,
    ) -> Result<(), std::io::Error> {
        self.write_u32::<LE>(data.fix_size() as u32)?;
        data.write(self)?;
        Ok(())
    }
}

pub trait WSExt: std::io::Write + std::io::Seek + Sized {
    fn write_with_calculated_length<T: Writable>(
        &mut self,
        data: &T,
    ) -> Result<(), std::io::Error> {
        let len_field_pos = self.stream_position()?;
        self.seek(SeekFrom::Current(4))?;
        let data_start = self.stream_position()?;
        data.write(self)?;
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

    fn write_bytes_with_length(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.write_u32::<LE>(data.len() as u32)?;
        self.write_all(data)?;
        Ok(())
    }
}

pub trait WritableExt: Writable {
    fn write_to_buffer(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        let mut writer = Cursor::new(&mut buffer);
        self.write(&mut writer)?;
        Ok(buffer)
    }
}

impl<T: std::io::Write + std::io::Seek> WSExt for T {}
impl<T: std::io::Write + std::io::Seek + Sized> FixedSizeExt for T {}
impl<T: Writable> WritableExt for T {}
