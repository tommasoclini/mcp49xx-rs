use crate::{marker, private, Error};

#[doc(hidden)]
pub trait ResolutionSupport<CommE>: private::Sealed {
    fn check_value_is_appropriate(value: u16) -> Result<(), Error<CommE>>;
    fn get_value_for_spi(value: u16) -> [u8; 2];
}

impl<CommE> ResolutionSupport<CommE> for marker::Resolution12Bit {
    fn check_value_is_appropriate(value: u16) -> Result<(), Error<CommE>> {
        if value >= 1 << 12 {
            Err(Error::InvalidValue)
        } else {
            Ok(())
        }
    }
    fn get_value_for_spi(value: u16) -> [u8; 2] {
        [(value >> 8) as u8, (value & 0xff) as u8]
    }
}

impl<CommE> ResolutionSupport<CommE> for marker::Resolution10Bit {
    fn check_value_is_appropriate(value: u16) -> Result<(), Error<CommE>> {
        if value >= 1 << 10 {
            Err(Error::InvalidValue)
        } else {
            Ok(())
        }
    }
    fn get_value_for_spi(value: u16) -> [u8; 2] {
        [(value >> 6) as u8, ((value << 2) & 0xff) as u8]
    }
}

impl<CommE> ResolutionSupport<CommE> for marker::Resolution8Bit {
    fn check_value_is_appropriate(value: u16) -> Result<(), Error<CommE>> {
        if value >= 1 << 8 {
            Err(Error::InvalidValue)
        } else {
            Ok(())
        }
    }
    fn get_value_for_spi(value: u16) -> [u8; 2] {
        [(value >> 4) as u8, ((value << 4) & 0xff) as u8]
    }
}
