use std::fmt;

use super::{ffi, PowerDevice, PowerIterator};
use crate::platform::traits::BatteryManager;
use crate::Result;

#[derive(Default)]
pub struct PowerManager;

impl BatteryManager for PowerManager {
    type Iterator = PowerIterator;

    fn new() -> Result<Self> {
        Ok(PowerManager {})
    }
}

impl fmt::Debug for PowerManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WindowsManager").finish()
    }
}
