use crate::{marker, private, Channel, Error};

#[doc(hidden)]
pub trait ChannelSupport<CommE>: private::Sealed {
    fn check_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE>>;
}

impl<CommE> ChannelSupport<CommE> for marker::SingleChannel {
    fn check_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE>> {
        if channel != Channel::Ch0 {
            Err(Error::InvalidChannel)
        } else {
            Ok(())
        }
    }
}

impl<CommE> ChannelSupport<CommE> for marker::DualChannel {
    fn check_channel_is_appropriate(_channel: Channel) -> Result<(), Error<CommE>> {
        Ok(())
    }
}
