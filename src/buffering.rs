use crate::{marker, private, Error};

#[doc(hidden)]
pub trait BufferingSupport<CommE>: private::Sealed {
    fn check_buffering_is_appropriate(buffered: bool) -> Result<(), Error<CommE>>;
}

impl<CommE> BufferingSupport<CommE> for marker::Buffered {
    fn check_buffering_is_appropriate(_buffered: bool) -> Result<(), Error<CommE>> {
        Ok(())
    }
}

impl<CommE> BufferingSupport<CommE> for marker::Unbuffered {
    fn check_buffering_is_appropriate(buffered: bool) -> Result<(), Error<CommE>> {
        if buffered {
            Err(Error::BufferingNotSupported)
        } else {
            Ok(())
        }
    }
}
