//! Tests for the I/O module.

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::io::{ReadStream, WriteStream};

    #[test]
    fn test_read_write_basic() -> Result<()> {
        let mut writer = WriteStream::new();
        writer.write_u8(42)?;
        writer.write_u16(0x1234)?;
        writer.write_u32(0x12345678)?;
        writer.write_u64(0x1234567890ABCDEF)?;
        writer.write_string("Hello, world!")?;

        let data = writer.into_data();
        let mut reader = ReadStream::new(&data);

        assert_eq!(reader.read_u8()?, 42);
        assert_eq!(reader.read_u16()?, 0x1234);
        assert_eq!(reader.read_u32()?, 0x12345678);
        assert_eq!(reader.read_u64()?, 0x1234567890ABCDEF);
        assert_eq!(reader.remaining(), b"Hello, world!");

        Ok(())
    }

    #[test]
    fn test_box_writing() -> Result<()> {
        let mut writer = WriteStream::new();
        
        // Write a simple box
        let marker = writer.write_box(b"test")?;
        writer.write_u32(0x12345678)?;
        writer.finish_box(marker)?;

        // Write a full box
        let marker = writer.write_full_box(b"full", 1, 0x123456)?;
        writer.write_u32(0x87654321)?;
        writer.finish_box(marker)?;

        let data = writer.into_data();
        let mut reader = ReadStream::new(&data);

        // Verify the simple box
        assert_eq!(reader.read_u32()?, 12); // Box size (8 + 4)
        assert_eq!(reader.read_fixed_string(4)?, "test");
        assert_eq!(reader.read_u32()?, 0x12345678);

        // Verify the full box
        assert_eq!(reader.read_u32()?, 16); // Box size (8 + 4 + 4)
        assert_eq!(reader.read_fixed_string(4)?, "full");
        assert_eq!(reader.read_u8()?, 1); // Version
        let flags_high = reader.read_u8()? as u32;
        let flags_mid = reader.read_u8()? as u32;
        let flags_low = reader.read_u8()? as u32;
        let flags = (flags_high << 16) | (flags_mid << 8) | flags_low;
        assert_eq!(flags, 0x123456);
        assert_eq!(reader.read_u32()?, 0x87654321);

        Ok(())
    }

    #[test]
    fn test_fixed_string_writing() -> Result<()> {
        let mut writer = WriteStream::new();
        
        // Write a string shorter than the fixed length
        writer.write_fixed_string("Hello", 10)?;
        
        // Write a string longer than the fixed length
        writer.write_fixed_string("Hello, world!", 5)?;

        let data = writer.into_data();
        let mut reader = ReadStream::new(&data);

        // Verify the first string (with padding)
        let mut buf = [0u8; 10];
        reader.read_exact(&mut buf)?;
        assert_eq!(&buf[0..5], b"Hello");
        assert_eq!(&buf[5..], &[0, 0, 0, 0, 0]);

        // Verify the second string (truncated)
        let mut buf = [0u8; 5];
        reader.read_exact(&mut buf)?;
        assert_eq!(&buf, b"Hello");

        Ok(())
    }
}
