//! Tests for the BMFF parser.

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::error::Result;

    // Helper function to create a simple AVIF file for testing
    fn create_test_avif() -> Vec<u8> {
        let mut data = Vec::new();

        // ftyp box
        data.extend_from_slice(&[
            0x00, 0x00, 0x00, 0x18, // size (24 bytes)
            b'f', b't', b'y', b'p', // type (ftyp)
            b'a', b'v', b'i', b'f', // major_brand (avif)
            0x00, 0x00, 0x00, 0x00, // minor_version
            b'a', b'v', b'i', b'f', // compatible_brand (avif)
            b'm', b'i', b'f', b'1', // compatible_brand (mif1)
        ]);

        // meta box (full box)
        data.extend_from_slice(&[
            0x00, 0x00, 0x00, 0x10, // size (16 bytes)
            b'm', b'e', b't', b'a', // type (meta)
            0x00, 0x00, 0x00, 0x00, // version (0) and flags (0)
            // Add some dummy content to make the size correct
            0x00, 0x00, 0x00, 0x00
        ]);

        data
    }

    #[test]
    fn test_parse_avif() -> Result<()> {
        let data = create_test_avif();
        let metadata = parse_avif(&data)?;

        assert_eq!(std::str::from_utf8(&metadata.ftyp.major_brand).unwrap(), "avif");
        assert_eq!(metadata.ftyp.compatible_brands.len(), 2);
        assert_eq!(std::str::from_utf8(&metadata.ftyp.compatible_brands[0]).unwrap(), "avif");
        assert_eq!(std::str::from_utf8(&metadata.ftyp.compatible_brands[1]).unwrap(), "mif1");
        assert!(metadata.ftyp.is_avif());

        Ok(())
    }

    #[test]
    fn test_parser_next_box() -> Result<()> {
        let data = create_test_avif();
        let mut parser = Parser::new(&data);

        // Parse ftyp box
        let ftyp_box = parser.next_box()?.unwrap();
        assert_eq!(ftyp_box.header.box_type.as_str(), "ftyp");
        assert_eq!(ftyp_box.header.size, 24);
        assert_eq!(ftyp_box.data.len(), 16); // 24 - 8 (header size)

        // Parse meta box
        let meta_box = parser.next_box()?.unwrap();
        assert_eq!(meta_box.header.box_type.as_str(), "meta");
        assert_eq!(meta_box.header.size, 16);
        assert_eq!(meta_box.header.version, Some(0));
        assert_eq!(meta_box.header.flags, Some(0));
        assert_eq!(meta_box.data.len(), 4); // 16 - 8 (header size) - 4 (full box fields)

        // No more boxes
        assert!(parser.next_box()?.is_none());

        Ok(())
    }

    #[test]
    fn test_parse_ftyp() -> Result<()> {
        let data = create_test_avif();
        let mut parser = Parser::new(&data);

        let ftyp = parser.parse_ftyp()?;
        assert_eq!(std::str::from_utf8(&ftyp.major_brand).unwrap(), "avif");
        assert_eq!(ftyp.compatible_brands.len(), 2);

        Ok(())
    }

    #[test]
    fn test_malformed_box() {
        // Test with truncated data
        let data = vec![
            0x00, 0x00, 0x00, 0x10, // size (16 bytes)
            b'f', b't', b'y', b'p', // type (ftyp)
            // Only 4 bytes of data, but size indicates 8 bytes of data
        ];

        let mut parser = Parser::new(&data);
        let result = parser.next_box();
        assert!(result.is_err());

        // Test with invalid size
        let data = vec![
            0x00, 0x00, 0x00, 0x04, // size (4 bytes, too small)
            b'f', b't', b'y', b'p', // type (ftyp)
            0x00, 0x00, 0x00, 0x00, // data
        ];

        let mut parser = Parser::new(&data);
        let result = parser.next_box();
        assert!(result.is_err());
    }

    #[test]
    fn test_extended_size() -> Result<()> {
        // Create a box with extended size
        let mut data = Vec::new();
        data.extend_from_slice(&[
            0x00, 0x00, 0x00, 0x01, // size = 1 (indicates extended size)
            b'f', b't', b'y', b'p', // type (ftyp)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, // extended size (32 bytes)
            // 16 bytes of data
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        ]);

        let mut parser = Parser::new(&data);
        let box_ = parser.next_box()?.unwrap();

        assert_eq!(box_.header.box_type.as_str(), "ftyp");
        assert_eq!(box_.header.size, 32);
        assert_eq!(box_.data.len(), 16);

        Ok(())
    }

    #[test]
    fn test_uuid_box() -> Result<()> {
        // Create a uuid box
        let mut data = Vec::new();
        data.extend_from_slice(&[
            0x00, 0x00, 0x00, 0x20, // size (32 bytes)
            b'u', b'u', b'i', b'd', // type (uuid)
            // 16 bytes of UUID
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
            // 8 bytes of data
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
        ]);

        let mut parser = Parser::new(&data);
        let box_ = parser.next_box()?.unwrap();

        assert_eq!(box_.header.box_type.as_str(), "uuid");
        assert_eq!(box_.header.size, 32);
        assert!(box_.header.extended_type.is_some());
        assert_eq!(box_.header.extended_type.unwrap(), [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        ]);
        assert_eq!(box_.data.len(), 8);

        Ok(())
    }
}
