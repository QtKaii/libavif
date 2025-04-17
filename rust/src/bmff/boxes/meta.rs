//! Meta Box implementation.
//!
//! The Meta Box (`meta`) is a container box for metadata.

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// Meta Box.
#[derive(Debug)]
pub struct MetaBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Handler Reference Box.
    pub hdlr: Option<super::hdlr::HandlerBox>,
    /// Primary Item Box.
    pub pitm: Option<super::pitm::PrimaryItemBox>,
    /// Item Location Box.
    pub iloc: Option<super::iloc::ItemLocationBox>,
    /// Item Information Box.
    pub iinf: Option<super::iinf::ItemInfoBox>,
    /// Item Properties Box.
    pub iprp: Option<ItemPropertiesBox>,
}

/// Item Properties Box.
#[derive(Debug)]
pub struct ItemPropertiesBox {
    /// Item Property Container.
    pub ipco: ItemPropertyContainerBox,
    /// Item Property Association.
    pub ipma: ItemPropertyAssociationBox,
}

/// Item Property Container Box.
#[derive(Debug)]
pub struct ItemPropertyContainerBox {
    /// Properties.
    pub properties: Vec<ItemProperty>,
}

/// Item Property.
#[derive(Debug)]
pub enum ItemProperty {
    /// Image Spatial Extents Property.
    Ispe(super::ispe::ImageSpatialExtentsBox),
    /// Pixel Information Property.
    Pixi(super::pixi::PixelInformationBox),
    /// Colour Information Property.
    Colr(super::colr::ColourInformationBox),
    /// AV1 Configuration Property.
    Av1C(super::av1c::AV1ConfigurationBox),
    /// Unknown Property.
    Unknown {
        /// Property type.
        property_type: [u8; 4],
        /// Property data.
        data: Vec<u8>,
    },
}

/// Item Property Association Box.
#[derive(Debug)]
pub struct ItemPropertyAssociationBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Entries.
    pub entries: Vec<ItemPropertyAssociationEntry>,
}

/// Item Property Association Entry.
#[derive(Debug)]
pub struct ItemPropertyAssociationEntry {
    /// Item ID.
    pub item_id: u32,
    /// Associations.
    pub associations: Vec<ItemPropertyAssociation>,
}

/// Item Property Association.
#[derive(Debug)]
pub struct ItemPropertyAssociation {
    /// Essential.
    pub essential: bool,
    /// Property index.
    pub property_index: u16,
}

impl MetaBox {
    /// Parse a meta box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read version and flags
        let version = stream.read_u8()?;
        let flags = stream.read_u24()?;
        
        let mut hdlr = None;
        let mut pitm = None;
        let mut iloc = None;
        let mut iinf = None;
        let mut iprp = None;
        
        while stream.has_more() {
            // Read box header
            let box_size = stream.read_u32()? as usize;
            if box_size < 8 {
                return Err(Error::BmffParse("Invalid box size".to_string()));
            }
            
            let mut box_type = [0u8; 4];
            stream.read_exact(&mut box_type)?;
            
            // Calculate box data size
            let data_size = box_size - 8;
            
            // Parse box based on type
            match &box_type {
                b"hdlr" => {
                    if hdlr.is_some() {
                        return Err(Error::BmffParse("Duplicate hdlr box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    hdlr = Some(super::hdlr::HandlerBox::parse(&box_data)?);
                },
                b"pitm" => {
                    if pitm.is_some() {
                        return Err(Error::BmffParse("Duplicate pitm box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    pitm = Some(super::pitm::PrimaryItemBox::parse(&box_data)?);
                },
                b"iloc" => {
                    if iloc.is_some() {
                        return Err(Error::BmffParse("Duplicate iloc box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    iloc = Some(super::iloc::ItemLocationBox::parse(&box_data)?);
                },
                b"iinf" => {
                    if iinf.is_some() {
                        return Err(Error::BmffParse("Duplicate iinf box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    iinf = Some(super::iinf::ItemInfoBox::parse(&box_data)?);
                },
                b"iprp" => {
                    if iprp.is_some() {
                        return Err(Error::BmffParse("Duplicate iprp box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    iprp = Some(ItemPropertiesBox::parse(&box_data)?);
                },
                _ => {
                    // Skip unknown box
                    stream.skip(data_size)?;
                }
            }
        }
        
        Ok(Self {
            version,
            flags,
            hdlr,
            pitm,
            iloc,
            iinf,
            iprp,
        })
    }
}

impl ItemPropertiesBox {
    /// Parse an item properties box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        let mut ipco = None;
        let mut ipma = None;
        
        while stream.has_more() {
            // Read box header
            let box_size = stream.read_u32()? as usize;
            if box_size < 8 {
                return Err(Error::BmffParse("Invalid box size".to_string()));
            }
            
            let mut box_type = [0u8; 4];
            stream.read_exact(&mut box_type)?;
            
            // Calculate box data size
            let data_size = box_size - 8;
            
            // Parse box based on type
            match &box_type {
                b"ipco" => {
                    if ipco.is_some() {
                        return Err(Error::BmffParse("Duplicate ipco box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    ipco = Some(ItemPropertyContainerBox::parse(&box_data)?);
                },
                b"ipma" => {
                    if ipma.is_some() {
                        return Err(Error::BmffParse("Duplicate ipma box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    ipma = Some(ItemPropertyAssociationBox::parse(&box_data)?);
                },
                _ => {
                    // Skip unknown box
                    stream.skip(data_size)?;
                }
            }
        }
        
        // Ensure we have both ipco and ipma boxes
        let ipco = ipco.ok_or_else(|| Error::BmffParse("Missing ipco box".to_string()))?;
        let ipma = ipma.ok_or_else(|| Error::BmffParse("Missing ipma box".to_string()))?;
        
        Ok(Self {
            ipco,
            ipma,
        })
    }
}

impl ItemPropertyContainerBox {
    /// Parse an item property container box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        let mut properties = Vec::new();
        
        while stream.has_more() {
            // Read box header
            let box_size = stream.read_u32()? as usize;
            if box_size < 8 {
                return Err(Error::BmffParse("Invalid box size".to_string()));
            }
            
            let mut box_type = [0u8; 4];
            stream.read_exact(&mut box_type)?;
            
            // Calculate box data size
            let data_size = box_size - 8;
            
            // Parse box based on type
            let property = match &box_type {
                b"ispe" => {
                    let box_data = stream.read_exact_buffer(data_size)?;
                    ItemProperty::Ispe(super::ispe::ImageSpatialExtentsBox::parse(&box_data)?)
                },
                b"pixi" => {
                    let box_data = stream.read_exact_buffer(data_size)?;
                    ItemProperty::Pixi(super::pixi::PixelInformationBox::parse(&box_data)?)
                },
                b"colr" => {
                    let box_data = stream.read_exact_buffer(data_size)?;
                    ItemProperty::Colr(super::colr::ColourInformationBox::parse(&box_data)?)
                },
                b"av1C" => {
                    let box_data = stream.read_exact_buffer(data_size)?;
                    ItemProperty::Av1C(super::av1c::AV1ConfigurationBox::parse(&box_data)?)
                },
                _ => {
                    // Unknown property
                    let data = stream.read_exact_buffer(data_size)?;
                    ItemProperty::Unknown {
                        property_type: box_type,
                        data: data.to_vec(),
                    }
                }
            };
            
            properties.push(property);
        }
        
        Ok(Self {
            properties,
        })
    }
}

impl ItemPropertyAssociationBox {
    /// Parse an item property association box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read version and flags
        let version = stream.read_u8()?;
        let flags = stream.read_u24()?;
        
        // Read entry count
        let entry_count = stream.read_u32()? as usize;
        
        // Read entries
        let mut entries = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            // Read item ID
            let item_id = if (flags & 1) == 1 {
                stream.read_u32()?
            } else {
                stream.read_u16()? as u32
            };
            
            // Read association count
            let association_count = stream.read_u8()? as usize;
            
            // Read associations
            let mut associations = Vec::with_capacity(association_count);
            for _ in 0..association_count {
                // Read essential and property index
                let essential_and_property_index = if (version == 0) || (version == 1) {
                    stream.read_u8()? as u16
                } else {
                    stream.read_u16()?
                };
                
                let essential = (essential_and_property_index & 0x80) != 0;
                let property_index = if (version == 0) || (version == 1) {
                    essential_and_property_index & 0x7F
                } else {
                    essential_and_property_index & 0x7FFF
                };
                
                associations.push(ItemPropertyAssociation {
                    essential,
                    property_index,
                });
            }
            
            entries.push(ItemPropertyAssociationEntry {
                item_id,
                associations,
            });
        }
        
        Ok(Self {
            version,
            flags,
            entries,
        })
    }
}
