#[cfg(test)]
mod tests {
    use crate::bmff::boxes::moov::MovieHeaderBox;
    use crate::bmff::boxes::trak::TrackHeaderBox;
    use crate::bmff::boxes::mdat::MediaDataBox;
    use crate::bmff::boxes::free::FreeSpaceBox;
    use crate::bmff::boxes::idat::ItemDataBox;
    use crate::error::Result;
    use crate::io::WriteStream;

    #[test]
    fn test_moov_box() -> Result<()> {
        // Create a simple moov box with a mvhd box
        let mut writer = WriteStream::new();

        // Write mvhd box
        let mut mvhd_writer = WriteStream::new();

        // Write mvhd version and flags
        mvhd_writer.write_u8(1)?; // Version 1
        mvhd_writer.write_u8(0)?; // Flags
        mvhd_writer.write_u8(0)?;
        mvhd_writer.write_u8(0)?;

        // Write creation and modification times
        mvhd_writer.write_u64(0)?; // Creation time
        mvhd_writer.write_u64(0)?; // Modification time

        // Write timescale and duration
        mvhd_writer.write_u32(1000)?; // Timescale
        mvhd_writer.write_u64(5000)?; // Duration

        // Write rate and volume
        mvhd_writer.write_u32(0x00010000)?; // Rate (1.0)
        mvhd_writer.write_u16(0x0100)?; // Volume (1.0)

        // Write reserved
        mvhd_writer.write_u16(0)?;
        mvhd_writer.write_u32(0)?;
        mvhd_writer.write_u32(0)?;

        // Write matrix
        for _ in 0..9 {
            mvhd_writer.write_u32(0)?;
        }

        // Write pre-defined
        for _ in 0..6 {
            mvhd_writer.write_u32(0)?;
        }

        // Write next track ID
        mvhd_writer.write_u32(1)?;

        let mvhd_data = mvhd_writer.into_data();

        // Write mvhd box
        writer.write_u32(8 + mvhd_data.len() as u32)?; // Size (mvhd header + mvhd data)
        writer.write(b"mvhd")?;
        writer.write(&mvhd_data)?;

        // Parse the moov box
        let data = writer.into_data();
        println!("Data length: {}", data.len());

        // Parse the mvhd box directly
        let mvhd = MovieHeaderBox::parse(&data[8..])?;

        // Check the parsed data
        assert_eq!(mvhd.version, 1);
        assert_eq!(mvhd.timescale, 1000);
        assert_eq!(mvhd.duration, 5000);
        assert_eq!(mvhd.next_track_id, 1);

        Ok(())
    }

    #[test]
    fn test_trak_box() -> Result<()> {
        // Create a simple trak box with a tkhd box
        let mut writer = WriteStream::new();

        // Write tkhd box
        let mut tkhd_writer = WriteStream::new();

        // Write tkhd version and flags
        tkhd_writer.write_u8(1)?; // Version 1
        tkhd_writer.write_u8(0)?; // Flags
        tkhd_writer.write_u8(0)?;
        tkhd_writer.write_u8(1)?; // Flags = 1 (track enabled)

        // Write creation and modification times
        tkhd_writer.write_u64(0)?; // Creation time
        tkhd_writer.write_u64(0)?; // Modification time

        // Write track ID
        tkhd_writer.write_u32(1)?;

        // Write reserved
        tkhd_writer.write_u32(0)?;

        // Write duration
        tkhd_writer.write_u64(5000)?;

        // Write reserved
        tkhd_writer.write_u32(0)?;
        tkhd_writer.write_u32(0)?;

        // Write layer and alternate group
        tkhd_writer.write_u16(0)?; // Layer
        tkhd_writer.write_u16(0)?; // Alternate group

        // Write volume
        tkhd_writer.write_u16(0)?; // Volume

        // Write reserved
        tkhd_writer.write_u16(0)?;

        // Write matrix
        for _ in 0..9 {
            tkhd_writer.write_u32(0)?;
        }

        // Write width and height (fixed point 16.16)
        tkhd_writer.write_u32(640 << 16)?; // Width
        tkhd_writer.write_u32(480 << 16)?; // Height

        let tkhd_data = tkhd_writer.into_data();

        // Write tkhd box
        writer.write_u32(8 + tkhd_data.len() as u32)?; // Size (tkhd header + tkhd data)
        writer.write(b"tkhd")?;
        writer.write(&tkhd_data)?;

        // Parse the trak box
        let data = writer.into_data();
        println!("Data length: {}", data.len());

        // Parse the tkhd box directly
        let tkhd = TrackHeaderBox::parse(&data[8..])?;

        // Check the parsed data
        assert_eq!(tkhd.version, 1);
        assert_eq!(tkhd.track_id, 1);
        assert_eq!(tkhd.duration, 5000);
        assert_eq!(tkhd.width, 640);
        assert_eq!(tkhd.height, 480);

        Ok(())
    }

    #[test]
    fn test_mdat_box() -> Result<()> {
        // Create a simple mdat box with some data
        let mut writer = WriteStream::new();

        // Create some test data
        let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Write mdat box
        writer.write_u32(8 + test_data.len() as u32)?; // Size (mdat header + data)
        writer.write(b"mdat")?;
        writer.write(&test_data)?;

        // Parse the mdat box
        let data = writer.into_data();
        println!("Data length: {}", data.len());

        // Parse the mdat box directly
        let mdat = MediaDataBox::parse(&data[8..])?;

        // Check the parsed data
        assert_eq!(mdat.data.len(), test_data.len());
        assert_eq!(mdat.data, test_data);

        Ok(())
    }

    #[test]
    fn test_free_box() -> Result<()> {
        // Create a simple free box with some data
        let mut writer = WriteStream::new();

        // Create some test data
        let test_data = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        // Write free box
        writer.write_u32(8 + test_data.len() as u32)?; // Size (free header + data)
        writer.write(b"free")?;
        writer.write(&test_data)?;

        // Parse the free box
        let data = writer.into_data();
        println!("Data length: {}", data.len());

        // Parse the free box directly
        let free = FreeSpaceBox::parse(*b"free", &data[8..])?;

        // Check the parsed data
        assert_eq!(free.box_type, *b"free");
        assert_eq!(free.data.len(), test_data.len());
        assert_eq!(free.data, test_data);

        // Test with skip box type
        let mut writer = WriteStream::new();
        writer.write_u32(8 + test_data.len() as u32)?; // Size (skip header + data)
        writer.write(b"skip")?;
        writer.write(&test_data)?;

        let data = writer.into_data();
        let skip = FreeSpaceBox::parse(*b"skip", &data[8..])?;

        // Check the parsed data
        assert_eq!(skip.box_type, *b"skip");
        assert_eq!(skip.data.len(), test_data.len());
        assert_eq!(skip.data, test_data);

        Ok(())
    }

    #[test]
    fn test_idat_box() -> Result<()> {
        // Create a simple idat box with some data
        let mut writer = WriteStream::new();

        // Create some test data
        let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Write idat box
        writer.write_u32(8 + test_data.len() as u32)?; // Size (idat header + data)
        writer.write(b"idat")?;
        writer.write(&test_data)?;

        // Parse the idat box
        let data = writer.into_data();
        println!("Data length: {}", data.len());

        // Parse the idat box directly
        let idat = ItemDataBox::parse(&data[8..])?;

        // Check the parsed data
        assert_eq!(idat.data.len(), test_data.len());
        assert_eq!(idat.data, test_data);

        Ok(())
    }
}
