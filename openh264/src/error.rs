use openh264_sys2::{dsErrorFree, DECODING_STATE};
use std::fmt::{Debug, Display, Formatter};

/// Error struct if something goes wrong.
#[derive(Debug)]
pub struct Error {
    native: i64,
    decoding_state: DECODING_STATE,
    misc: Option<String>,
    #[cfg(feature = "backtrace")]
    backtrace: Option<std::backtrace::Backtrace>,
}

impl Error {
    pub(crate) fn from_native(native: i64) -> Self {
        Error {
            native,
            decoding_state: dsErrorFree,
            misc: None,
            #[cfg(feature = "backtrace")]
            backtrace: Some(std::backtrace::Backtrace::capture()),
        }
    }

    #[allow(unused)]
    pub(crate) fn from_decoding_state(decoding_state: DECODING_STATE) -> Self {
        Error {
            native: 0,
            decoding_state,
            misc: None,
            #[cfg(feature = "backtrace")]
            backtrace: Some(std::backtrace::Backtrace::capture()),
        }
    }

    pub(crate) fn msg(msg: &str) -> Self {
        Error {
            native: 0,
            decoding_state: dsErrorFree,
            misc: Some(msg.to_string()),
            #[cfg(feature = "backtrace")]
            backtrace: Some(std::backtrace::Backtrace::capture()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("OpenH264 encountered an error. Native:")?;
        <i64 as std::fmt::Display>::fmt(&self.native, f)?;
        f.write_str(". Decoding State:")?;
        <std::os::raw::c_int as std::fmt::Display>::fmt(&self.decoding_state, f)?;
        f.write_str(". User Message:")?;
        self.misc.fmt(f)?;

        #[cfg(feature = "backtrace")]
        {
            f.write_str(". Backtraces enabled.")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    #[cfg(feature = "backtrace")]
    fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        self.backtrace.as_ref()
    }
}

/// Helper trait to check the various error values produced by OpenH264.
pub(crate) trait NativeErrorExt {
    fn ok(self) -> Result<(), Error>;
}

impl NativeErrorExt for u64 {
    fn ok(self) -> Result<(), Error> {
        if self == 0 {
            Ok(())
        } else {
            Err(Error::from_native(self as i64))
        }
    }
}

impl NativeErrorExt for i64 {
    fn ok(self) -> Result<(), Error> {
        if self == 0 {
            Ok(())
        } else {
            Err(Error::from_native(self as i64))
        }
    }
}

impl NativeErrorExt for i32 {
    fn ok(self) -> Result<(), Error> {
        if self == 0 {
            Ok(())
        } else {
            Err(Error::from_native(self as i64))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Error;
    use openh264_sys2::dsRefListNullPtrs;

    #[test]
    fn errors_wont_panic() {
        dbg!(Error::from_native(1));
        dbg!(Error::from_decoding_state(dsRefListNullPtrs));
        dbg!(Error::msg("hello world"));

        println!("{}", Error::from_native(1));
        println!("{}", Error::from_decoding_state(dsRefListNullPtrs));
        println!("{}", Error::msg("hello world"));

        println!("{:?}", Error::from_native(1));
        println!("{:?}", Error::from_decoding_state(dsRefListNullPtrs));
        println!("{:?}", Error::msg("hello world"));

        println!("{:#?}", Error::from_native(1));
        println!("{:#?}", Error::from_decoding_state(dsRefListNullPtrs));
        println!("{:#?}", Error::msg("hello world"));
    }
}
