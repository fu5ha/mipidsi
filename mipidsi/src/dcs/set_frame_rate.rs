use crate::Error;
use crate::FrameRate;
use crate::Inversion;

use super::DcsCommand;

/// Set Tearing Effect
#[derive(Debug, Clone, Copy)]
pub struct SetFrameRate(pub FrameRate, pub Inversion);

impl DcsCommand for SetFrameRate {
    fn instruction(&self) -> u8 {
        0xC6
    }

    fn fill_params_buf(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        let frame_rate_control = self.0 as u8;
        let inversion = (self.1 as u8) << 5;
        let data = frame_rate_control | inversion;
        buffer[0] = data;
        Ok(1)
    }
}
