use embedded_hal::timer::{CountDown, Periodic};

use void::Void;
pub struct InstantTimer {}

impl CountDown for InstantTimer {
    type Time = Void;

    fn start<T>(&mut self, _timeout: T)
    where
        T: Into<Void>,
    {
    }
    fn wait(&mut self) -> nb::Result<(), Void> {
        Ok(())
    }
}

impl Periodic for InstantTimer {}
