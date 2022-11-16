use crate::{dcs::Madctl, error::InitError, Error, ModelOptions};
use display_interface::WriteOnlyDataCommand;
use embedded_graphics_core::prelude::RgbColor;
use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin};

// existing model implementations
// mod ili9342c;
// mod ili9486;
// mod st7735s;
mod st7789;

// pub use ili9342c::*;
// pub use ili9486::*;
// pub use st7735s::*;
pub use st7789::*;

pub trait Model {
    type ColorFormat: RgbColor;

    /// Initializes the display for this model with MADCTL from [crate::Display]
    /// and returns the value of MADCTL set by init
    fn init<RST, DELAY, DI>(
        &mut self,
        dcs: &mut DI,
        delay: &mut DELAY,
        options: &ModelOptions,
        rst: &mut Option<RST>,
    ) -> Result<Madctl, InitError<RST::Error>>
    where
        RST: OutputPin,
        DELAY: DelayUs<u32>,
        DI: WriteOnlyDataCommand;

    fn hard_reset<RST, DELAY>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), InitError<RST::Error>>
    where
        RST: OutputPin,
        DELAY: DelayUs<u32>,
    {
        rst.set_low().map_err(InitError::Pin)?;
        delay.delay_us(10);
        rst.set_high().map_err(InitError::Pin)?;

        Ok(())
    }

    /// Writes pixels to the display IC via the given DisplayInterface
    /// Any pixel color format conversion is done here
    fn write_pixels<DI, I>(&mut self, di: &mut DI, colors: I) -> Result<(), Error>
    where
        DI: WriteOnlyDataCommand,
        I: IntoIterator<Item = Self::ColorFormat>;

    ///
    /// Creates default [ModelOptions] for this particular [Model]. This serves
    /// as a "sane default". There can be additional variants which will be provided via
    /// helper constructors.
    ///
    fn default_options() -> ModelOptions;
}
