use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Reference Control"]
    pub refctrl: REFCTRL,
    #[doc = "0x02 - Average Control"]
    pub avgctrl: AVGCTRL,
    #[doc = "0x03 - Sampling Time Control"]
    pub sampctrl: SAMPCTRL,
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    _reserved0: [u8; 2usize],
    #[doc = "0x08 - Window Monitor Control"]
    pub winctrl: WINCTRL,
    _reserved1: [u8; 3usize],
    #[doc = "0x0c - Software Trigger"]
    pub swtrig: SWTRIG,
    _reserved2: [u8; 3usize],
    #[doc = "0x10 - Input Control"]
    pub inputctrl: INPUTCTRL,
    #[doc = "0x14 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved3: [u8; 1usize],
    #[doc = "0x16 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x17 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x18 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x19 - Status"]
    pub status: STATUS,
    #[doc = "0x1a - Result"]
    pub result: RESULT,
    #[doc = "0x1c - Window Monitor Lower Threshold"]
    pub winlt: WINLT,
    _reserved4: [u8; 2usize],
    #[doc = "0x20 - Window Monitor Upper Threshold"]
    pub winut: WINUT,
    _reserved5: [u8; 2usize],
    #[doc = "0x24 - Gain Correction"]
    pub gaincorr: GAINCORR,
    #[doc = "0x26 - Offset Correction"]
    pub offsetcorr: OFFSETCORR,
    #[doc = "0x28 - Calibration"]
    pub calib: CALIB,
    #[doc = "0x2a - Debug Control"]
    pub dbgctrl: DBGCTRL,
}
#[doc = "Average Control"]
pub struct AVGCTRL {
    register: VolatileCell<u8>,
}
#[doc = "Average Control"]
pub mod avgctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::AVGCTRL {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = "Possible values of the field `SAMPLENUM`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SAMPLENUMR {
        #[doc = "1 sample"]
        _1,
        #[doc = "2 samples"]
        _2,
        #[doc = "4 samples"]
        _4,
        #[doc = "8 samples"]
        _8,
        #[doc = "16 samples"]
        _16,
        #[doc = "32 samples"]
        _32,
        #[doc = "64 samples"]
        _64,
        #[doc = "128 samples"]
        _128,
        #[doc = "256 samples"]
        _256,
        #[doc = "512 samples"]
        _512,
        #[doc = "1024 samples"]
        _1024,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SAMPLENUMR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SAMPLENUMR::_1 => 0,
                SAMPLENUMR::_2 => 1,
                SAMPLENUMR::_4 => 2,
                SAMPLENUMR::_8 => 3,
                SAMPLENUMR::_16 => 4,
                SAMPLENUMR::_32 => 5,
                SAMPLENUMR::_64 => 6,
                SAMPLENUMR::_128 => 7,
                SAMPLENUMR::_256 => 8,
                SAMPLENUMR::_512 => 9,
                SAMPLENUMR::_1024 => 10,
                SAMPLENUMR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SAMPLENUMR {
            match value {
                0 => SAMPLENUMR::_1,
                1 => SAMPLENUMR::_2,
                2 => SAMPLENUMR::_4,
                3 => SAMPLENUMR::_8,
                4 => SAMPLENUMR::_16,
                5 => SAMPLENUMR::_32,
                6 => SAMPLENUMR::_64,
                7 => SAMPLENUMR::_128,
                8 => SAMPLENUMR::_256,
                9 => SAMPLENUMR::_512,
                10 => SAMPLENUMR::_1024,
                i => SAMPLENUMR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `_1`"]
        #[inline(always)]
        pub fn is_1(&self) -> bool {
            *self == SAMPLENUMR::_1
        }
        #[doc = "Checks if the value of the field is `_2`"]
        #[inline(always)]
        pub fn is_2(&self) -> bool {
            *self == SAMPLENUMR::_2
        }
        #[doc = "Checks if the value of the field is `_4`"]
        #[inline(always)]
        pub fn is_4(&self) -> bool {
            *self == SAMPLENUMR::_4
        }
        #[doc = "Checks if the value of the field is `_8`"]
        #[inline(always)]
        pub fn is_8(&self) -> bool {
            *self == SAMPLENUMR::_8
        }
        #[doc = "Checks if the value of the field is `_16`"]
        #[inline(always)]
        pub fn is_16(&self) -> bool {
            *self == SAMPLENUMR::_16
        }
        #[doc = "Checks if the value of the field is `_32`"]
        #[inline(always)]
        pub fn is_32(&self) -> bool {
            *self == SAMPLENUMR::_32
        }
        #[doc = "Checks if the value of the field is `_64`"]
        #[inline(always)]
        pub fn is_64(&self) -> bool {
            *self == SAMPLENUMR::_64
        }
        #[doc = "Checks if the value of the field is `_128`"]
        #[inline(always)]
        pub fn is_128(&self) -> bool {
            *self == SAMPLENUMR::_128
        }
        #[doc = "Checks if the value of the field is `_256`"]
        #[inline(always)]
        pub fn is_256(&self) -> bool {
            *self == SAMPLENUMR::_256
        }
        #[doc = "Checks if the value of the field is `_512`"]
        #[inline(always)]
        pub fn is_512(&self) -> bool {
            *self == SAMPLENUMR::_512
        }
        #[doc = "Checks if the value of the field is `_1024`"]
        #[inline(always)]
        pub fn is_1024(&self) -> bool {
            *self == SAMPLENUMR::_1024
        }
    }
    #[doc = r" Value of the field"]
    pub struct ADJRESR {
        bits: u8,
    }
    impl ADJRESR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = "Values that can be written to the field `SAMPLENUM`"]
    pub enum SAMPLENUMW {
        #[doc = "1 sample"]
        _1,
        #[doc = "2 samples"]
        _2,
        #[doc = "4 samples"]
        _4,
        #[doc = "8 samples"]
        _8,
        #[doc = "16 samples"]
        _16,
        #[doc = "32 samples"]
        _32,
        #[doc = "64 samples"]
        _64,
        #[doc = "128 samples"]
        _128,
        #[doc = "256 samples"]
        _256,
        #[doc = "512 samples"]
        _512,
        #[doc = "1024 samples"]
        _1024,
    }
    impl SAMPLENUMW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SAMPLENUMW::_1 => 0,
                SAMPLENUMW::_2 => 1,
                SAMPLENUMW::_4 => 2,
                SAMPLENUMW::_8 => 3,
                SAMPLENUMW::_16 => 4,
                SAMPLENUMW::_32 => 5,
                SAMPLENUMW::_64 => 6,
                SAMPLENUMW::_128 => 7,
                SAMPLENUMW::_256 => 8,
                SAMPLENUMW::_512 => 9,
                SAMPLENUMW::_1024 => 10,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SAMPLENUMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SAMPLENUMW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SAMPLENUMW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "1 sample"]
        #[inline(always)]
        pub fn _1(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_1)
        }
        #[doc = "2 samples"]
        #[inline(always)]
        pub fn _2(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_2)
        }
        #[doc = "4 samples"]
        #[inline(always)]
        pub fn _4(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_4)
        }
        #[doc = "8 samples"]
        #[inline(always)]
        pub fn _8(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_8)
        }
        #[doc = "16 samples"]
        #[inline(always)]
        pub fn _16(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_16)
        }
        #[doc = "32 samples"]
        #[inline(always)]
        pub fn _32(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_32)
        }
        #[doc = "64 samples"]
        #[inline(always)]
        pub fn _64(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_64)
        }
        #[doc = "128 samples"]
        #[inline(always)]
        pub fn _128(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_128)
        }
        #[doc = "256 samples"]
        #[inline(always)]
        pub fn _256(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_256)
        }
        #[doc = "512 samples"]
        #[inline(always)]
        pub fn _512(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_512)
        }
        #[doc = "1024 samples"]
        #[inline(always)]
        pub fn _1024(self) -> &'a mut W {
            self.variant(SAMPLENUMW::_1024)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ADJRESW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADJRESW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bits 0:3 - Number of Samples to be Collected"]
        #[inline(always)]
        pub fn samplenum(&self) -> SAMPLENUMR {
            SAMPLENUMR::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            })
        }
        #[doc = "Bits 4:6 - Adjusting Result / Division Coefficient"]
        #[inline(always)]
        pub fn adjres(&self) -> ADJRESR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            };
            ADJRESR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:3 - Number of Samples to be Collected"]
        #[inline(always)]
        pub fn samplenum(&mut self) -> _SAMPLENUMW {
            _SAMPLENUMW { w: self }
        }
        #[doc = "Bits 4:6 - Adjusting Result / Division Coefficient"]
        #[inline(always)]
        pub fn adjres(&mut self) -> _ADJRESW {
            _ADJRESW { w: self }
        }
    }
}
#[doc = "Calibration"]
pub struct CALIB {
    register: VolatileCell<u16>,
}
#[doc = "Calibration"]
pub mod calib {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::CALIB {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct LINEARITY_CALR {
        bits: u8,
    }
    impl LINEARITY_CALR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct BIAS_CALR {
        bits: u8,
    }
    impl BIAS_CALR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _LINEARITY_CALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LINEARITY_CALW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _BIAS_CALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BIAS_CALW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bits 0:7 - Linearity Calibration Value"]
        #[inline(always)]
        pub fn linearity_cal(&self) -> LINEARITY_CALR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            };
            LINEARITY_CALR { bits }
        }
        #[doc = "Bits 8:10 - Bias Calibration Value"]
        #[inline(always)]
        pub fn bias_cal(&self) -> BIAS_CALR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            };
            BIAS_CALR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:7 - Linearity Calibration Value"]
        #[inline(always)]
        pub fn linearity_cal(&mut self) -> _LINEARITY_CALW {
            _LINEARITY_CALW { w: self }
        }
        #[doc = "Bits 8:10 - Bias Calibration Value"]
        #[inline(always)]
        pub fn bias_cal(&mut self) -> _BIAS_CALW {
            _BIAS_CALW { w: self }
        }
    }
}
#[doc = "Control A"]
pub struct CTRLA {
    register: VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CTRLA {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWRSTR {
        bits: bool,
    }
    impl SWRSTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct ENABLER {
        bits: bool,
    }
    impl ENABLER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct RUNSTDBYR {
        bits: bool,
    }
    impl RUNSTDBYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Proxy"]
    pub struct _SWRSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWRSTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ENABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ENABLEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _RUNSTDBYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RUNSTDBYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bit 0 - Software Reset"]
        #[inline(always)]
        pub fn swrst(&self) -> SWRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SWRSTR { bits }
        }
        #[doc = "Bit 1 - Enable"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 2 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&self) -> RUNSTDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            RUNSTDBYR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Software Reset"]
        #[inline(always)]
        pub fn swrst(&mut self) -> _SWRSTW {
            _SWRSTW { w: self }
        }
        #[doc = "Bit 1 - Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
        #[doc = "Bit 2 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&mut self) -> _RUNSTDBYW {
            _RUNSTDBYW { w: self }
        }
    }
}
#[doc = "Control B"]
pub struct CTRLB {
    register: VolatileCell<u16>,
}
#[doc = "Control B"]
pub mod ctrlb {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::CTRLB {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct DIFFMODER {
        bits: bool,
    }
    impl DIFFMODER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LEFTADJR {
        bits: bool,
    }
    impl LEFTADJR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct FREERUNR {
        bits: bool,
    }
    impl FREERUNR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CORRENR {
        bits: bool,
    }
    impl CORRENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `RESSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum RESSELR {
        #[doc = "12-bit result"]
        _12BIT,
        #[doc = "For averaging mode output"]
        _16BIT,
        #[doc = "10-bit result"]
        _10BIT,
        #[doc = "8-bit result"]
        _8BIT,
    }
    impl RESSELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                RESSELR::_12BIT => 0,
                RESSELR::_16BIT => 1,
                RESSELR::_10BIT => 2,
                RESSELR::_8BIT => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> RESSELR {
            match value {
                0 => RESSELR::_12BIT,
                1 => RESSELR::_16BIT,
                2 => RESSELR::_10BIT,
                3 => RESSELR::_8BIT,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `_12BIT`"]
        #[inline(always)]
        pub fn is_12bit(&self) -> bool {
            *self == RESSELR::_12BIT
        }
        #[doc = "Checks if the value of the field is `_16BIT`"]
        #[inline(always)]
        pub fn is_16bit(&self) -> bool {
            *self == RESSELR::_16BIT
        }
        #[doc = "Checks if the value of the field is `_10BIT`"]
        #[inline(always)]
        pub fn is_10bit(&self) -> bool {
            *self == RESSELR::_10BIT
        }
        #[doc = "Checks if the value of the field is `_8BIT`"]
        #[inline(always)]
        pub fn is_8bit(&self) -> bool {
            *self == RESSELR::_8BIT
        }
    }
    #[doc = "Possible values of the field `PRESCALER`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PRESCALERR {
        #[doc = "Peripheral clock divided by 4"]
        DIV4,
        #[doc = "Peripheral clock divided by 8"]
        DIV8,
        #[doc = "Peripheral clock divided by 16"]
        DIV16,
        #[doc = "Peripheral clock divided by 32"]
        DIV32,
        #[doc = "Peripheral clock divided by 64"]
        DIV64,
        #[doc = "Peripheral clock divided by 128"]
        DIV128,
        #[doc = "Peripheral clock divided by 256"]
        DIV256,
        #[doc = "Peripheral clock divided by 512"]
        DIV512,
    }
    impl PRESCALERR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                PRESCALERR::DIV4 => 0,
                PRESCALERR::DIV8 => 1,
                PRESCALERR::DIV16 => 2,
                PRESCALERR::DIV32 => 3,
                PRESCALERR::DIV64 => 4,
                PRESCALERR::DIV128 => 5,
                PRESCALERR::DIV256 => 6,
                PRESCALERR::DIV512 => 7,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> PRESCALERR {
            match value {
                0 => PRESCALERR::DIV4,
                1 => PRESCALERR::DIV8,
                2 => PRESCALERR::DIV16,
                3 => PRESCALERR::DIV32,
                4 => PRESCALERR::DIV64,
                5 => PRESCALERR::DIV128,
                6 => PRESCALERR::DIV256,
                7 => PRESCALERR::DIV512,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DIV4`"]
        #[inline(always)]
        pub fn is_div4(&self) -> bool {
            *self == PRESCALERR::DIV4
        }
        #[doc = "Checks if the value of the field is `DIV8`"]
        #[inline(always)]
        pub fn is_div8(&self) -> bool {
            *self == PRESCALERR::DIV8
        }
        #[doc = "Checks if the value of the field is `DIV16`"]
        #[inline(always)]
        pub fn is_div16(&self) -> bool {
            *self == PRESCALERR::DIV16
        }
        #[doc = "Checks if the value of the field is `DIV32`"]
        #[inline(always)]
        pub fn is_div32(&self) -> bool {
            *self == PRESCALERR::DIV32
        }
        #[doc = "Checks if the value of the field is `DIV64`"]
        #[inline(always)]
        pub fn is_div64(&self) -> bool {
            *self == PRESCALERR::DIV64
        }
        #[doc = "Checks if the value of the field is `DIV128`"]
        #[inline(always)]
        pub fn is_div128(&self) -> bool {
            *self == PRESCALERR::DIV128
        }
        #[doc = "Checks if the value of the field is `DIV256`"]
        #[inline(always)]
        pub fn is_div256(&self) -> bool {
            *self == PRESCALERR::DIV256
        }
        #[doc = "Checks if the value of the field is `DIV512`"]
        #[inline(always)]
        pub fn is_div512(&self) -> bool {
            *self == PRESCALERR::DIV512
        }
    }
    #[doc = r" Proxy"]
    pub struct _DIFFMODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIFFMODEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _LEFTADJW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LEFTADJW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FREERUNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FREERUNW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CORRENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CORRENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `RESSEL`"]
    pub enum RESSELW {
        #[doc = "12-bit result"]
        _12BIT,
        #[doc = "For averaging mode output"]
        _16BIT,
        #[doc = "10-bit result"]
        _10BIT,
        #[doc = "8-bit result"]
        _8BIT,
    }
    impl RESSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                RESSELW::_12BIT => 0,
                RESSELW::_16BIT => 1,
                RESSELW::_10BIT => 2,
                RESSELW::_8BIT => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _RESSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: RESSELW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "12-bit result"]
        #[inline(always)]
        pub fn _12bit(self) -> &'a mut W {
            self.variant(RESSELW::_12BIT)
        }
        #[doc = "For averaging mode output"]
        #[inline(always)]
        pub fn _16bit(self) -> &'a mut W {
            self.variant(RESSELW::_16BIT)
        }
        #[doc = "10-bit result"]
        #[inline(always)]
        pub fn _10bit(self) -> &'a mut W {
            self.variant(RESSELW::_10BIT)
        }
        #[doc = "8-bit result"]
        #[inline(always)]
        pub fn _8bit(self) -> &'a mut W {
            self.variant(RESSELW::_8BIT)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `PRESCALER`"]
    pub enum PRESCALERW {
        #[doc = "Peripheral clock divided by 4"]
        DIV4,
        #[doc = "Peripheral clock divided by 8"]
        DIV8,
        #[doc = "Peripheral clock divided by 16"]
        DIV16,
        #[doc = "Peripheral clock divided by 32"]
        DIV32,
        #[doc = "Peripheral clock divided by 64"]
        DIV64,
        #[doc = "Peripheral clock divided by 128"]
        DIV128,
        #[doc = "Peripheral clock divided by 256"]
        DIV256,
        #[doc = "Peripheral clock divided by 512"]
        DIV512,
    }
    impl PRESCALERW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                PRESCALERW::DIV4 => 0,
                PRESCALERW::DIV8 => 1,
                PRESCALERW::DIV16 => 2,
                PRESCALERW::DIV32 => 3,
                PRESCALERW::DIV64 => 4,
                PRESCALERW::DIV128 => 5,
                PRESCALERW::DIV256 => 6,
                PRESCALERW::DIV512 => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _PRESCALERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PRESCALERW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: PRESCALERW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "Peripheral clock divided by 4"]
        #[inline(always)]
        pub fn div4(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV4)
        }
        #[doc = "Peripheral clock divided by 8"]
        #[inline(always)]
        pub fn div8(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV8)
        }
        #[doc = "Peripheral clock divided by 16"]
        #[inline(always)]
        pub fn div16(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV16)
        }
        #[doc = "Peripheral clock divided by 32"]
        #[inline(always)]
        pub fn div32(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV32)
        }
        #[doc = "Peripheral clock divided by 64"]
        #[inline(always)]
        pub fn div64(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV64)
        }
        #[doc = "Peripheral clock divided by 128"]
        #[inline(always)]
        pub fn div128(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV128)
        }
        #[doc = "Peripheral clock divided by 256"]
        #[inline(always)]
        pub fn div256(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV256)
        }
        #[doc = "Peripheral clock divided by 512"]
        #[inline(always)]
        pub fn div512(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV512)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bit 0 - Differential Mode"]
        #[inline(always)]
        pub fn diffmode(&self) -> DIFFMODER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            DIFFMODER { bits }
        }
        #[doc = "Bit 1 - Left-Adjusted Result"]
        #[inline(always)]
        pub fn leftadj(&self) -> LEFTADJR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            LEFTADJR { bits }
        }
        #[doc = "Bit 2 - Free Running Mode"]
        #[inline(always)]
        pub fn freerun(&self) -> FREERUNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            FREERUNR { bits }
        }
        #[doc = "Bit 3 - Digital Correction Logic Enabled"]
        #[inline(always)]
        pub fn corren(&self) -> CORRENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            CORRENR { bits }
        }
        #[doc = "Bits 4:5 - Conversion Result Resolution"]
        #[inline(always)]
        pub fn ressel(&self) -> RESSELR {
            RESSELR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            })
        }
        #[doc = "Bits 8:10 - Prescaler Configuration"]
        #[inline(always)]
        pub fn prescaler(&self) -> PRESCALERR {
            PRESCALERR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            })
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Differential Mode"]
        #[inline(always)]
        pub fn diffmode(&mut self) -> _DIFFMODEW {
            _DIFFMODEW { w: self }
        }
        #[doc = "Bit 1 - Left-Adjusted Result"]
        #[inline(always)]
        pub fn leftadj(&mut self) -> _LEFTADJW {
            _LEFTADJW { w: self }
        }
        #[doc = "Bit 2 - Free Running Mode"]
        #[inline(always)]
        pub fn freerun(&mut self) -> _FREERUNW {
            _FREERUNW { w: self }
        }
        #[doc = "Bit 3 - Digital Correction Logic Enabled"]
        #[inline(always)]
        pub fn corren(&mut self) -> _CORRENW {
            _CORRENW { w: self }
        }
        #[doc = "Bits 4:5 - Conversion Result Resolution"]
        #[inline(always)]
        pub fn ressel(&mut self) -> _RESSELW {
            _RESSELW { w: self }
        }
        #[doc = "Bits 8:10 - Prescaler Configuration"]
        #[inline(always)]
        pub fn prescaler(&mut self) -> _PRESCALERW {
            _PRESCALERW { w: self }
        }
    }
}
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::DBGCTRL {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct DBGRUNR {
        bits: bool,
    }
    impl DBGRUNR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Proxy"]
    pub struct _DBGRUNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DBGRUNW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bit 0 - Debug Run"]
        #[inline(always)]
        pub fn dbgrun(&self) -> DBGRUNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            DBGRUNR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Debug Run"]
        #[inline(always)]
        pub fn dbgrun(&mut self) -> _DBGRUNW {
            _DBGRUNW { w: self }
        }
    }
}
#[doc = "Event Control"]
pub struct EVCTRL {
    register: VolatileCell<u8>,
}
#[doc = "Event Control"]
pub mod evctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::EVCTRL {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct STARTEIR {
        bits: bool,
    }
    impl STARTEIR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SYNCEIR {
        bits: bool,
    }
    impl SYNCEIR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct RESRDYEOR {
        bits: bool,
    }
    impl RESRDYEOR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WINMONEOR {
        bits: bool,
    }
    impl WINMONEOR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Proxy"]
    pub struct _STARTEIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STARTEIW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SYNCEIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SYNCEIW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _RESRDYEOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESRDYEOW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _WINMONEOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WINMONEOW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bit 0 - Start Conversion Event In"]
        #[inline(always)]
        pub fn startei(&self) -> STARTEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            STARTEIR { bits }
        }
        #[doc = "Bit 1 - Synchronization Event In"]
        #[inline(always)]
        pub fn syncei(&self) -> SYNCEIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SYNCEIR { bits }
        }
        #[doc = "Bit 4 - Result Ready Event Out"]
        #[inline(always)]
        pub fn resrdyeo(&self) -> RESRDYEOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            RESRDYEOR { bits }
        }
        #[doc = "Bit 5 - Window Monitor Event Out"]
        #[inline(always)]
        pub fn winmoneo(&self) -> WINMONEOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            WINMONEOR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Start Conversion Event In"]
        #[inline(always)]
        pub fn startei(&mut self) -> _STARTEIW {
            _STARTEIW { w: self }
        }
        #[doc = "Bit 1 - Synchronization Event In"]
        #[inline(always)]
        pub fn syncei(&mut self) -> _SYNCEIW {
            _SYNCEIW { w: self }
        }
        #[doc = "Bit 4 - Result Ready Event Out"]
        #[inline(always)]
        pub fn resrdyeo(&mut self) -> _RESRDYEOW {
            _RESRDYEOW { w: self }
        }
        #[doc = "Bit 5 - Window Monitor Event Out"]
        #[inline(always)]
        pub fn winmoneo(&mut self) -> _WINMONEOW {
            _WINMONEOW { w: self }
        }
    }
}
#[doc = "Gain Correction"]
pub struct GAINCORR {
    register: VolatileCell<u16>,
}
#[doc = "Gain Correction"]
pub mod gaincorr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::GAINCORR {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct GAINCORRR {
        bits: u16,
    }
    impl GAINCORRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _GAINCORRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _GAINCORRW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bits 0:11 - Gain Correction Value"]
        #[inline(always)]
        pub fn gaincorr(&self) -> GAINCORRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u16
            };
            GAINCORRR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:11 - Gain Correction Value"]
        #[inline(always)]
        pub fn gaincorr(&mut self) -> _GAINCORRW {
            _GAINCORRW { w: self }
        }
    }
}
#[doc = "Input Control"]
pub struct INPUTCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Input Control"]
pub mod inputctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::INPUTCTRL {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = "Possible values of the field `MUXPOS`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum MUXPOSR {
        #[doc = "ADC AIN0 Pin"]
        PIN0,
        #[doc = "ADC AIN1 Pin"]
        PIN1,
        #[doc = "ADC AIN2 Pin"]
        PIN2,
        #[doc = "ADC AIN3 Pin"]
        PIN3,
        #[doc = "ADC AIN4 Pin"]
        PIN4,
        #[doc = "ADC AIN5 Pin"]
        PIN5,
        #[doc = "ADC AIN6 Pin"]
        PIN6,
        #[doc = "ADC AIN7 Pin"]
        PIN7,
        #[doc = "ADC AIN8 Pin"]
        PIN8,
        #[doc = "ADC AIN9 Pin"]
        PIN9,
        #[doc = "ADC AIN10 Pin"]
        PIN10,
        #[doc = "ADC AIN11 Pin"]
        PIN11,
        #[doc = "ADC AIN12 Pin"]
        PIN12,
        #[doc = "ADC AIN13 Pin"]
        PIN13,
        #[doc = "ADC AIN14 Pin"]
        PIN14,
        #[doc = "ADC AIN15 Pin"]
        PIN15,
        #[doc = "ADC AIN16 Pin"]
        PIN16,
        #[doc = "ADC AIN17 Pin"]
        PIN17,
        #[doc = "ADC AIN18 Pin"]
        PIN18,
        #[doc = "ADC AIN19 Pin"]
        PIN19,
        #[doc = "Temperature Reference"]
        TEMP,
        #[doc = "Bandgap Voltage"]
        BANDGAP,
        #[doc = "1/4  Scaled Core Supply"]
        SCALEDCOREVCC,
        #[doc = "1/4  Scaled I/O Supply"]
        SCALEDIOVCC,
        #[doc = "DAC Output"]
        DAC,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl MUXPOSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                MUXPOSR::PIN0 => 0,
                MUXPOSR::PIN1 => 1,
                MUXPOSR::PIN2 => 2,
                MUXPOSR::PIN3 => 3,
                MUXPOSR::PIN4 => 4,
                MUXPOSR::PIN5 => 5,
                MUXPOSR::PIN6 => 6,
                MUXPOSR::PIN7 => 7,
                MUXPOSR::PIN8 => 8,
                MUXPOSR::PIN9 => 9,
                MUXPOSR::PIN10 => 10,
                MUXPOSR::PIN11 => 11,
                MUXPOSR::PIN12 => 12,
                MUXPOSR::PIN13 => 13,
                MUXPOSR::PIN14 => 14,
                MUXPOSR::PIN15 => 15,
                MUXPOSR::PIN16 => 16,
                MUXPOSR::PIN17 => 17,
                MUXPOSR::PIN18 => 18,
                MUXPOSR::PIN19 => 19,
                MUXPOSR::TEMP => 24,
                MUXPOSR::BANDGAP => 25,
                MUXPOSR::SCALEDCOREVCC => 26,
                MUXPOSR::SCALEDIOVCC => 27,
                MUXPOSR::DAC => 28,
                MUXPOSR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> MUXPOSR {
            match value {
                0 => MUXPOSR::PIN0,
                1 => MUXPOSR::PIN1,
                2 => MUXPOSR::PIN2,
                3 => MUXPOSR::PIN3,
                4 => MUXPOSR::PIN4,
                5 => MUXPOSR::PIN5,
                6 => MUXPOSR::PIN6,
                7 => MUXPOSR::PIN7,
                8 => MUXPOSR::PIN8,
                9 => MUXPOSR::PIN9,
                10 => MUXPOSR::PIN10,
                11 => MUXPOSR::PIN11,
                12 => MUXPOSR::PIN12,
                13 => MUXPOSR::PIN13,
                14 => MUXPOSR::PIN14,
                15 => MUXPOSR::PIN15,
                16 => MUXPOSR::PIN16,
                17 => MUXPOSR::PIN17,
                18 => MUXPOSR::PIN18,
                19 => MUXPOSR::PIN19,
                24 => MUXPOSR::TEMP,
                25 => MUXPOSR::BANDGAP,
                26 => MUXPOSR::SCALEDCOREVCC,
                27 => MUXPOSR::SCALEDIOVCC,
                28 => MUXPOSR::DAC,
                i => MUXPOSR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `PIN0`"]
        #[inline(always)]
        pub fn is_pin0(&self) -> bool {
            *self == MUXPOSR::PIN0
        }
        #[doc = "Checks if the value of the field is `PIN1`"]
        #[inline(always)]
        pub fn is_pin1(&self) -> bool {
            *self == MUXPOSR::PIN1
        }
        #[doc = "Checks if the value of the field is `PIN2`"]
        #[inline(always)]
        pub fn is_pin2(&self) -> bool {
            *self == MUXPOSR::PIN2
        }
        #[doc = "Checks if the value of the field is `PIN3`"]
        #[inline(always)]
        pub fn is_pin3(&self) -> bool {
            *self == MUXPOSR::PIN3
        }
        #[doc = "Checks if the value of the field is `PIN4`"]
        #[inline(always)]
        pub fn is_pin4(&self) -> bool {
            *self == MUXPOSR::PIN4
        }
        #[doc = "Checks if the value of the field is `PIN5`"]
        #[inline(always)]
        pub fn is_pin5(&self) -> bool {
            *self == MUXPOSR::PIN5
        }
        #[doc = "Checks if the value of the field is `PIN6`"]
        #[inline(always)]
        pub fn is_pin6(&self) -> bool {
            *self == MUXPOSR::PIN6
        }
        #[doc = "Checks if the value of the field is `PIN7`"]
        #[inline(always)]
        pub fn is_pin7(&self) -> bool {
            *self == MUXPOSR::PIN7
        }
        #[doc = "Checks if the value of the field is `PIN8`"]
        #[inline(always)]
        pub fn is_pin8(&self) -> bool {
            *self == MUXPOSR::PIN8
        }
        #[doc = "Checks if the value of the field is `PIN9`"]
        #[inline(always)]
        pub fn is_pin9(&self) -> bool {
            *self == MUXPOSR::PIN9
        }
        #[doc = "Checks if the value of the field is `PIN10`"]
        #[inline(always)]
        pub fn is_pin10(&self) -> bool {
            *self == MUXPOSR::PIN10
        }
        #[doc = "Checks if the value of the field is `PIN11`"]
        #[inline(always)]
        pub fn is_pin11(&self) -> bool {
            *self == MUXPOSR::PIN11
        }
        #[doc = "Checks if the value of the field is `PIN12`"]
        #[inline(always)]
        pub fn is_pin12(&self) -> bool {
            *self == MUXPOSR::PIN12
        }
        #[doc = "Checks if the value of the field is `PIN13`"]
        #[inline(always)]
        pub fn is_pin13(&self) -> bool {
            *self == MUXPOSR::PIN13
        }
        #[doc = "Checks if the value of the field is `PIN14`"]
        #[inline(always)]
        pub fn is_pin14(&self) -> bool {
            *self == MUXPOSR::PIN14
        }
        #[doc = "Checks if the value of the field is `PIN15`"]
        #[inline(always)]
        pub fn is_pin15(&self) -> bool {
            *self == MUXPOSR::PIN15
        }
        #[doc = "Checks if the value of the field is `PIN16`"]
        #[inline(always)]
        pub fn is_pin16(&self) -> bool {
            *self == MUXPOSR::PIN16
        }
        #[doc = "Checks if the value of the field is `PIN17`"]
        #[inline(always)]
        pub fn is_pin17(&self) -> bool {
            *self == MUXPOSR::PIN17
        }
        #[doc = "Checks if the value of the field is `PIN18`"]
        #[inline(always)]
        pub fn is_pin18(&self) -> bool {
            *self == MUXPOSR::PIN18
        }
        #[doc = "Checks if the value of the field is `PIN19`"]
        #[inline(always)]
        pub fn is_pin19(&self) -> bool {
            *self == MUXPOSR::PIN19
        }
        #[doc = "Checks if the value of the field is `TEMP`"]
        #[inline(always)]
        pub fn is_temp(&self) -> bool {
            *self == MUXPOSR::TEMP
        }
        #[doc = "Checks if the value of the field is `BANDGAP`"]
        #[inline(always)]
        pub fn is_bandgap(&self) -> bool {
            *self == MUXPOSR::BANDGAP
        }
        #[doc = "Checks if the value of the field is `SCALEDCOREVCC`"]
        #[inline(always)]
        pub fn is_scaledcorevcc(&self) -> bool {
            *self == MUXPOSR::SCALEDCOREVCC
        }
        #[doc = "Checks if the value of the field is `SCALEDIOVCC`"]
        #[inline(always)]
        pub fn is_scalediovcc(&self) -> bool {
            *self == MUXPOSR::SCALEDIOVCC
        }
        #[doc = "Checks if the value of the field is `DAC`"]
        #[inline(always)]
        pub fn is_dac(&self) -> bool {
            *self == MUXPOSR::DAC
        }
    }
    #[doc = "Possible values of the field `MUXNEG`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum MUXNEGR {
        #[doc = "ADC AIN0 Pin"]
        PIN0,
        #[doc = "ADC AIN1 Pin"]
        PIN1,
        #[doc = "ADC AIN2 Pin"]
        PIN2,
        #[doc = "ADC AIN3 Pin"]
        PIN3,
        #[doc = "ADC AIN4 Pin"]
        PIN4,
        #[doc = "ADC AIN5 Pin"]
        PIN5,
        #[doc = "ADC AIN6 Pin"]
        PIN6,
        #[doc = "ADC AIN7 Pin"]
        PIN7,
        #[doc = "Internal Ground"]
        GND,
        #[doc = "I/O Ground"]
        IOGND,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl MUXNEGR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                MUXNEGR::PIN0 => 0,
                MUXNEGR::PIN1 => 1,
                MUXNEGR::PIN2 => 2,
                MUXNEGR::PIN3 => 3,
                MUXNEGR::PIN4 => 4,
                MUXNEGR::PIN5 => 5,
                MUXNEGR::PIN6 => 6,
                MUXNEGR::PIN7 => 7,
                MUXNEGR::GND => 24,
                MUXNEGR::IOGND => 25,
                MUXNEGR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> MUXNEGR {
            match value {
                0 => MUXNEGR::PIN0,
                1 => MUXNEGR::PIN1,
                2 => MUXNEGR::PIN2,
                3 => MUXNEGR::PIN3,
                4 => MUXNEGR::PIN4,
                5 => MUXNEGR::PIN5,
                6 => MUXNEGR::PIN6,
                7 => MUXNEGR::PIN7,
                24 => MUXNEGR::GND,
                25 => MUXNEGR::IOGND,
                i => MUXNEGR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `PIN0`"]
        #[inline(always)]
        pub fn is_pin0(&self) -> bool {
            *self == MUXNEGR::PIN0
        }
        #[doc = "Checks if the value of the field is `PIN1`"]
        #[inline(always)]
        pub fn is_pin1(&self) -> bool {
            *self == MUXNEGR::PIN1
        }
        #[doc = "Checks if the value of the field is `PIN2`"]
        #[inline(always)]
        pub fn is_pin2(&self) -> bool {
            *self == MUXNEGR::PIN2
        }
        #[doc = "Checks if the value of the field is `PIN3`"]
        #[inline(always)]
        pub fn is_pin3(&self) -> bool {
            *self == MUXNEGR::PIN3
        }
        #[doc = "Checks if the value of the field is `PIN4`"]
        #[inline(always)]
        pub fn is_pin4(&self) -> bool {
            *self == MUXNEGR::PIN4
        }
        #[doc = "Checks if the value of the field is `PIN5`"]
        #[inline(always)]
        pub fn is_pin5(&self) -> bool {
            *self == MUXNEGR::PIN5
        }
        #[doc = "Checks if the value of the field is `PIN6`"]
        #[inline(always)]
        pub fn is_pin6(&self) -> bool {
            *self == MUXNEGR::PIN6
        }
        #[doc = "Checks if the value of the field is `PIN7`"]
        #[inline(always)]
        pub fn is_pin7(&self) -> bool {
            *self == MUXNEGR::PIN7
        }
        #[doc = "Checks if the value of the field is `GND`"]
        #[inline(always)]
        pub fn is_gnd(&self) -> bool {
            *self == MUXNEGR::GND
        }
        #[doc = "Checks if the value of the field is `IOGND`"]
        #[inline(always)]
        pub fn is_iognd(&self) -> bool {
            *self == MUXNEGR::IOGND
        }
    }
    #[doc = r" Value of the field"]
    pub struct INPUTSCANR {
        bits: u8,
    }
    impl INPUTSCANR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct INPUTOFFSETR {
        bits: u8,
    }
    impl INPUTOFFSETR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = "Possible values of the field `GAIN`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum GAINR {
        #[doc = "1x"]
        _1X,
        #[doc = "2x"]
        _2X,
        #[doc = "4x"]
        _4X,
        #[doc = "8x"]
        _8X,
        #[doc = "16x"]
        _16X,
        #[doc = "1/2x"]
        DIV2,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl GAINR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                GAINR::_1X => 0,
                GAINR::_2X => 1,
                GAINR::_4X => 2,
                GAINR::_8X => 3,
                GAINR::_16X => 4,
                GAINR::DIV2 => 15,
                GAINR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> GAINR {
            match value {
                0 => GAINR::_1X,
                1 => GAINR::_2X,
                2 => GAINR::_4X,
                3 => GAINR::_8X,
                4 => GAINR::_16X,
                15 => GAINR::DIV2,
                i => GAINR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `_1X`"]
        #[inline(always)]
        pub fn is_1x(&self) -> bool {
            *self == GAINR::_1X
        }
        #[doc = "Checks if the value of the field is `_2X`"]
        #[inline(always)]
        pub fn is_2x(&self) -> bool {
            *self == GAINR::_2X
        }
        #[doc = "Checks if the value of the field is `_4X`"]
        #[inline(always)]
        pub fn is_4x(&self) -> bool {
            *self == GAINR::_4X
        }
        #[doc = "Checks if the value of the field is `_8X`"]
        #[inline(always)]
        pub fn is_8x(&self) -> bool {
            *self == GAINR::_8X
        }
        #[doc = "Checks if the value of the field is `_16X`"]
        #[inline(always)]
        pub fn is_16x(&self) -> bool {
            *self == GAINR::_16X
        }
        #[doc = "Checks if the value of the field is `DIV2`"]
        #[inline(always)]
        pub fn is_div2(&self) -> bool {
            *self == GAINR::DIV2
        }
    }
    #[doc = "Values that can be written to the field `MUXPOS`"]
    pub enum MUXPOSW {
        #[doc = "ADC AIN0 Pin"]
        PIN0,
        #[doc = "ADC AIN1 Pin"]
        PIN1,
        #[doc = "ADC AIN2 Pin"]
        PIN2,
        #[doc = "ADC AIN3 Pin"]
        PIN3,
        #[doc = "ADC AIN4 Pin"]
        PIN4,
        #[doc = "ADC AIN5 Pin"]
        PIN5,
        #[doc = "ADC AIN6 Pin"]
        PIN6,
        #[doc = "ADC AIN7 Pin"]
        PIN7,
        #[doc = "ADC AIN8 Pin"]
        PIN8,
        #[doc = "ADC AIN9 Pin"]
        PIN9,
        #[doc = "ADC AIN10 Pin"]
        PIN10,
        #[doc = "ADC AIN11 Pin"]
        PIN11,
        #[doc = "ADC AIN12 Pin"]
        PIN12,
        #[doc = "ADC AIN13 Pin"]
        PIN13,
        #[doc = "ADC AIN14 Pin"]
        PIN14,
        #[doc = "ADC AIN15 Pin"]
        PIN15,
        #[doc = "ADC AIN16 Pin"]
        PIN16,
        #[doc = "ADC AIN17 Pin"]
        PIN17,
        #[doc = "ADC AIN18 Pin"]
        PIN18,
        #[doc = "ADC AIN19 Pin"]
        PIN19,
        #[doc = "Temperature Reference"]
        TEMP,
        #[doc = "Bandgap Voltage"]
        BANDGAP,
        #[doc = "1/4  Scaled Core Supply"]
        SCALEDCOREVCC,
        #[doc = "1/4  Scaled I/O Supply"]
        SCALEDIOVCC,
        #[doc = "DAC Output"]
        DAC,
    }
    impl MUXPOSW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                MUXPOSW::PIN0 => 0,
                MUXPOSW::PIN1 => 1,
                MUXPOSW::PIN2 => 2,
                MUXPOSW::PIN3 => 3,
                MUXPOSW::PIN4 => 4,
                MUXPOSW::PIN5 => 5,
                MUXPOSW::PIN6 => 6,
                MUXPOSW::PIN7 => 7,
                MUXPOSW::PIN8 => 8,
                MUXPOSW::PIN9 => 9,
                MUXPOSW::PIN10 => 10,
                MUXPOSW::PIN11 => 11,
                MUXPOSW::PIN12 => 12,
                MUXPOSW::PIN13 => 13,
                MUXPOSW::PIN14 => 14,
                MUXPOSW::PIN15 => 15,
                MUXPOSW::PIN16 => 16,
                MUXPOSW::PIN17 => 17,
                MUXPOSW::PIN18 => 18,
                MUXPOSW::PIN19 => 19,
                MUXPOSW::TEMP => 24,
                MUXPOSW::BANDGAP => 25,
                MUXPOSW::SCALEDCOREVCC => 26,
                MUXPOSW::SCALEDIOVCC => 27,
                MUXPOSW::DAC => 28,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _MUXPOSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MUXPOSW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: MUXPOSW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "ADC AIN0 Pin"]
        #[inline(always)]
        pub fn pin0(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN0)
        }
        #[doc = "ADC AIN1 Pin"]
        #[inline(always)]
        pub fn pin1(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN1)
        }
        #[doc = "ADC AIN2 Pin"]
        #[inline(always)]
        pub fn pin2(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN2)
        }
        #[doc = "ADC AIN3 Pin"]
        #[inline(always)]
        pub fn pin3(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN3)
        }
        #[doc = "ADC AIN4 Pin"]
        #[inline(always)]
        pub fn pin4(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN4)
        }
        #[doc = "ADC AIN5 Pin"]
        #[inline(always)]
        pub fn pin5(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN5)
        }
        #[doc = "ADC AIN6 Pin"]
        #[inline(always)]
        pub fn pin6(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN6)
        }
        #[doc = "ADC AIN7 Pin"]
        #[inline(always)]
        pub fn pin7(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN7)
        }
        #[doc = "ADC AIN8 Pin"]
        #[inline(always)]
        pub fn pin8(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN8)
        }
        #[doc = "ADC AIN9 Pin"]
        #[inline(always)]
        pub fn pin9(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN9)
        }
        #[doc = "ADC AIN10 Pin"]
        #[inline(always)]
        pub fn pin10(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN10)
        }
        #[doc = "ADC AIN11 Pin"]
        #[inline(always)]
        pub fn pin11(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN11)
        }
        #[doc = "ADC AIN12 Pin"]
        #[inline(always)]
        pub fn pin12(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN12)
        }
        #[doc = "ADC AIN13 Pin"]
        #[inline(always)]
        pub fn pin13(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN13)
        }
        #[doc = "ADC AIN14 Pin"]
        #[inline(always)]
        pub fn pin14(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN14)
        }
        #[doc = "ADC AIN15 Pin"]
        #[inline(always)]
        pub fn pin15(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN15)
        }
        #[doc = "ADC AIN16 Pin"]
        #[inline(always)]
        pub fn pin16(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN16)
        }
        #[doc = "ADC AIN17 Pin"]
        #[inline(always)]
        pub fn pin17(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN17)
        }
        #[doc = "ADC AIN18 Pin"]
        #[inline(always)]
        pub fn pin18(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN18)
        }
        #[doc = "ADC AIN19 Pin"]
        #[inline(always)]
        pub fn pin19(self) -> &'a mut W {
            self.variant(MUXPOSW::PIN19)
        }
        #[doc = "Temperature Reference"]
        #[inline(always)]
        pub fn temp(self) -> &'a mut W {
            self.variant(MUXPOSW::TEMP)
        }
        #[doc = "Bandgap Voltage"]
        #[inline(always)]
        pub fn bandgap(self) -> &'a mut W {
            self.variant(MUXPOSW::BANDGAP)
        }
        #[doc = "1/4 Scaled Core Supply"]
        #[inline(always)]
        pub fn scaledcorevcc(self) -> &'a mut W {
            self.variant(MUXPOSW::SCALEDCOREVCC)
        }
        #[doc = "1/4 Scaled I/O Supply"]
        #[inline(always)]
        pub fn scalediovcc(self) -> &'a mut W {
            self.variant(MUXPOSW::SCALEDIOVCC)
        }
        #[doc = "DAC Output"]
        #[inline(always)]
        pub fn dac(self) -> &'a mut W {
            self.variant(MUXPOSW::DAC)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `MUXNEG`"]
    pub enum MUXNEGW {
        #[doc = "ADC AIN0 Pin"]
        PIN0,
        #[doc = "ADC AIN1 Pin"]
        PIN1,
        #[doc = "ADC AIN2 Pin"]
        PIN2,
        #[doc = "ADC AIN3 Pin"]
        PIN3,
        #[doc = "ADC AIN4 Pin"]
        PIN4,
        #[doc = "ADC AIN5 Pin"]
        PIN5,
        #[doc = "ADC AIN6 Pin"]
        PIN6,
        #[doc = "ADC AIN7 Pin"]
        PIN7,
        #[doc = "Internal Ground"]
        GND,
        #[doc = "I/O Ground"]
        IOGND,
    }
    impl MUXNEGW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                MUXNEGW::PIN0 => 0,
                MUXNEGW::PIN1 => 1,
                MUXNEGW::PIN2 => 2,
                MUXNEGW::PIN3 => 3,
                MUXNEGW::PIN4 => 4,
                MUXNEGW::PIN5 => 5,
                MUXNEGW::PIN6 => 6,
                MUXNEGW::PIN7 => 7,
                MUXNEGW::GND => 24,
                MUXNEGW::IOGND => 25,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _MUXNEGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MUXNEGW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: MUXNEGW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "ADC AIN0 Pin"]
        #[inline(always)]
        pub fn pin0(self) -> &'a mut W {
            self.variant(MUXNEGW::PIN0)
        }
        #[doc = "ADC AIN1 Pin"]
        #[inline(always)]
        pub fn pin1(self) -> &'a mut W {
            self.variant(MUXNEGW::PIN1)
        }
        #[doc = "ADC AIN2 Pin"]
        #[inline(always)]
        pub fn pin2(self) -> &'a mut W {
            self.variant(MUXNEGW::PIN2)
        }
        #[doc = "ADC AIN3 Pin"]
        #[inline(always)]
        pub fn pin3(self) -> &'a mut W {
            self.variant(MUXNEGW::PIN3)
        }
        #[doc = "ADC AIN4 Pin"]
        #[inline(always)]
        pub fn pin4(self) -> &'a mut W {
            self.variant(MUXNEGW::PIN4)
        }
        #[doc = "ADC AIN5 Pin"]
        #[inline(always)]
        pub fn pin5(self) -> &'a mut W {
            self.variant(MUXNEGW::PIN5)
        }
        #[doc = "ADC AIN6 Pin"]
        #[inline(always)]
        pub fn pin6(self) -> &'a mut W {
            self.variant(MUXNEGW::PIN6)
        }
        #[doc = "ADC AIN7 Pin"]
        #[inline(always)]
        pub fn pin7(self) -> &'a mut W {
            self.variant(MUXNEGW::PIN7)
        }
        #[doc = "Internal Ground"]
        #[inline(always)]
        pub fn gnd(self) -> &'a mut W {
            self.variant(MUXNEGW::GND)
        }
        #[doc = "I/O Ground"]
        #[inline(always)]
        pub fn iognd(self) -> &'a mut W {
            self.variant(MUXNEGW::IOGND)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _INPUTSCANW<'a> {
        w: &'a mut W,
    }
    impl<'a> _INPUTSCANW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _INPUTOFFSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _INPUTOFFSETW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `GAIN`"]
    pub enum GAINW {
        #[doc = "1x"]
        _1X,
        #[doc = "2x"]
        _2X,
        #[doc = "4x"]
        _4X,
        #[doc = "8x"]
        _8X,
        #[doc = "16x"]
        _16X,
        #[doc = "1/2x"]
        DIV2,
    }
    impl GAINW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                GAINW::_1X => 0,
                GAINW::_2X => 1,
                GAINW::_4X => 2,
                GAINW::_8X => 3,
                GAINW::_16X => 4,
                GAINW::DIV2 => 15,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _GAINW<'a> {
        w: &'a mut W,
    }
    impl<'a> _GAINW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: GAINW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "1x"]
        #[inline(always)]
        pub fn _1x(self) -> &'a mut W {
            self.variant(GAINW::_1X)
        }
        #[doc = "2x"]
        #[inline(always)]
        pub fn _2x(self) -> &'a mut W {
            self.variant(GAINW::_2X)
        }
        #[doc = "4x"]
        #[inline(always)]
        pub fn _4x(self) -> &'a mut W {
            self.variant(GAINW::_4X)
        }
        #[doc = "8x"]
        #[inline(always)]
        pub fn _8x(self) -> &'a mut W {
            self.variant(GAINW::_8X)
        }
        #[doc = "16x"]
        #[inline(always)]
        pub fn _16x(self) -> &'a mut W {
            self.variant(GAINW::_16X)
        }
        #[doc = "1/2x"]
        #[inline(always)]
        pub fn div2(self) -> &'a mut W {
            self.variant(GAINW::DIV2)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:4 - Positive Mux Input Selection"]
        #[inline(always)]
        pub fn muxpos(&self) -> MUXPOSR {
            MUXPOSR::_from({
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 8:12 - Negative Mux Input Selection"]
        #[inline(always)]
        pub fn muxneg(&self) -> MUXNEGR {
            MUXNEGR::_from({
                const MASK: u8 = 31;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 16:19 - Number of Input Channels Included in Scan"]
        #[inline(always)]
        pub fn inputscan(&self) -> INPUTSCANR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            INPUTSCANR { bits }
        }
        #[doc = "Bits 20:23 - Positive Mux Setting Offset"]
        #[inline(always)]
        pub fn inputoffset(&self) -> INPUTOFFSETR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            INPUTOFFSETR { bits }
        }
        #[doc = "Bits 24:27 - Gain Factor Selection"]
        #[inline(always)]
        pub fn gain(&self) -> GAINR {
            GAINR::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:4 - Positive Mux Input Selection"]
        #[inline(always)]
        pub fn muxpos(&mut self) -> _MUXPOSW {
            _MUXPOSW { w: self }
        }
        #[doc = "Bits 8:12 - Negative Mux Input Selection"]
        #[inline(always)]
        pub fn muxneg(&mut self) -> _MUXNEGW {
            _MUXNEGW { w: self }
        }
        #[doc = "Bits 16:19 - Number of Input Channels Included in Scan"]
        #[inline(always)]
        pub fn inputscan(&mut self) -> _INPUTSCANW {
            _INPUTSCANW { w: self }
        }
        #[doc = "Bits 20:23 - Positive Mux Setting Offset"]
        #[inline(always)]
        pub fn inputoffset(&mut self) -> _INPUTOFFSETW {
            _INPUTOFFSETW { w: self }
        }
        #[doc = "Bits 24:27 - Gain Factor Selection"]
        #[inline(always)]
        pub fn gain(&mut self) -> _GAINW {
            _GAINW { w: self }
        }
    }
}
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: VolatileCell<u8>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::INTENCLR {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct RESRDYR {
        bits: bool,
    }
    impl RESRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct OVERRUNR {
        bits: bool,
    }
    impl OVERRUNR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WINMONR {
        bits: bool,
    }
    impl WINMONR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SYNCRDYR {
        bits: bool,
    }
    impl SYNCRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Proxy"]
    pub struct _RESRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESRDYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _OVERRUNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVERRUNW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _WINMONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WINMONW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SYNCRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SYNCRDYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bit 0 - Result Ready Interrupt Enable"]
        #[inline(always)]
        pub fn resrdy(&self) -> RESRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            RESRDYR { bits }
        }
        #[doc = "Bit 1 - Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn overrun(&self) -> OVERRUNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            OVERRUNR { bits }
        }
        #[doc = "Bit 2 - Window Monitor Interrupt Enable"]
        #[inline(always)]
        pub fn winmon(&self) -> WINMONR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            WINMONR { bits }
        }
        #[doc = "Bit 3 - Synchronization Ready Interrupt Enable"]
        #[inline(always)]
        pub fn syncrdy(&self) -> SYNCRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SYNCRDYR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Result Ready Interrupt Enable"]
        #[inline(always)]
        pub fn resrdy(&mut self) -> _RESRDYW {
            _RESRDYW { w: self }
        }
        #[doc = "Bit 1 - Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn overrun(&mut self) -> _OVERRUNW {
            _OVERRUNW { w: self }
        }
        #[doc = "Bit 2 - Window Monitor Interrupt Enable"]
        #[inline(always)]
        pub fn winmon(&mut self) -> _WINMONW {
            _WINMONW { w: self }
        }
        #[doc = "Bit 3 - Synchronization Ready Interrupt Enable"]
        #[inline(always)]
        pub fn syncrdy(&mut self) -> _SYNCRDYW {
            _SYNCRDYW { w: self }
        }
    }
}
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: VolatileCell<u8>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::INTENSET {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct RESRDYR {
        bits: bool,
    }
    impl RESRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct OVERRUNR {
        bits: bool,
    }
    impl OVERRUNR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WINMONR {
        bits: bool,
    }
    impl WINMONR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SYNCRDYR {
        bits: bool,
    }
    impl SYNCRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Proxy"]
    pub struct _RESRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESRDYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _OVERRUNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVERRUNW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _WINMONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WINMONW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SYNCRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SYNCRDYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bit 0 - Result Ready Interrupt Enable"]
        #[inline(always)]
        pub fn resrdy(&self) -> RESRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            RESRDYR { bits }
        }
        #[doc = "Bit 1 - Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn overrun(&self) -> OVERRUNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            OVERRUNR { bits }
        }
        #[doc = "Bit 2 - Window Monitor Interrupt Enable"]
        #[inline(always)]
        pub fn winmon(&self) -> WINMONR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            WINMONR { bits }
        }
        #[doc = "Bit 3 - Synchronization Ready Interrupt Enable"]
        #[inline(always)]
        pub fn syncrdy(&self) -> SYNCRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SYNCRDYR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Result Ready Interrupt Enable"]
        #[inline(always)]
        pub fn resrdy(&mut self) -> _RESRDYW {
            _RESRDYW { w: self }
        }
        #[doc = "Bit 1 - Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn overrun(&mut self) -> _OVERRUNW {
            _OVERRUNW { w: self }
        }
        #[doc = "Bit 2 - Window Monitor Interrupt Enable"]
        #[inline(always)]
        pub fn winmon(&mut self) -> _WINMONW {
            _WINMONW { w: self }
        }
        #[doc = "Bit 3 - Synchronization Ready Interrupt Enable"]
        #[inline(always)]
        pub fn syncrdy(&mut self) -> _SYNCRDYW {
            _SYNCRDYW { w: self }
        }
    }
}
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: VolatileCell<u8>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::INTFLAG {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct RESRDYR {
        bits: bool,
    }
    impl RESRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct OVERRUNR {
        bits: bool,
    }
    impl OVERRUNR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WINMONR {
        bits: bool,
    }
    impl WINMONR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SYNCRDYR {
        bits: bool,
    }
    impl SYNCRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Proxy"]
    pub struct _RESRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESRDYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _OVERRUNW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVERRUNW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _WINMONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WINMONW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SYNCRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SYNCRDYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bit 0 - Result Ready"]
        #[inline(always)]
        pub fn resrdy(&self) -> RESRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            RESRDYR { bits }
        }
        #[doc = "Bit 1 - Overrun"]
        #[inline(always)]
        pub fn overrun(&self) -> OVERRUNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            OVERRUNR { bits }
        }
        #[doc = "Bit 2 - Window Monitor"]
        #[inline(always)]
        pub fn winmon(&self) -> WINMONR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            WINMONR { bits }
        }
        #[doc = "Bit 3 - Synchronization Ready"]
        #[inline(always)]
        pub fn syncrdy(&self) -> SYNCRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SYNCRDYR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - Result Ready"]
        #[inline(always)]
        pub fn resrdy(&mut self) -> _RESRDYW {
            _RESRDYW { w: self }
        }
        #[doc = "Bit 1 - Overrun"]
        #[inline(always)]
        pub fn overrun(&mut self) -> _OVERRUNW {
            _OVERRUNW { w: self }
        }
        #[doc = "Bit 2 - Window Monitor"]
        #[inline(always)]
        pub fn winmon(&mut self) -> _WINMONW {
            _WINMONW { w: self }
        }
        #[doc = "Bit 3 - Synchronization Ready"]
        #[inline(always)]
        pub fn syncrdy(&mut self) -> _SYNCRDYW {
            _SYNCRDYW { w: self }
        }
    }
}
#[doc = "Offset Correction"]
pub struct OFFSETCORR {
    register: VolatileCell<u16>,
}
#[doc = "Offset Correction"]
pub mod offsetcorr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::OFFSETCORR {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct OFFSETCORRR {
        bits: u16,
    }
    impl OFFSETCORRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _OFFSETCORRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OFFSETCORRW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bits 0:11 - Offset Correction Value"]
        #[inline(always)]
        pub fn offsetcorr(&self) -> OFFSETCORRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u16
            };
            OFFSETCORRR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:11 - Offset Correction Value"]
        #[inline(always)]
        pub fn offsetcorr(&mut self) -> _OFFSETCORRW {
            _OFFSETCORRW { w: self }
        }
    }
}
#[doc = "Reference Control"]
pub struct REFCTRL {
    register: VolatileCell<u8>,
}
#[doc = "Reference Control"]
pub mod refctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::REFCTRL {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = "Possible values of the field `REFSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum REFSELR {
        #[doc = "1.0V voltage reference"]
        INT1V,
        #[doc = "1/1.48 VDDANA"]
        INTVCC0,
        #[doc = "1/2 VDDANA (only for VDDANA > 2.0V)"]
        INTVCC1,
        #[doc = "External reference"]
        AREFA,
        #[doc = "External reference"]
        AREFB,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl REFSELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                REFSELR::INT1V => 0,
                REFSELR::INTVCC0 => 1,
                REFSELR::INTVCC1 => 2,
                REFSELR::AREFA => 3,
                REFSELR::AREFB => 4,
                REFSELR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> REFSELR {
            match value {
                0 => REFSELR::INT1V,
                1 => REFSELR::INTVCC0,
                2 => REFSELR::INTVCC1,
                3 => REFSELR::AREFA,
                4 => REFSELR::AREFB,
                i => REFSELR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `INT1V`"]
        #[inline(always)]
        pub fn is_int1v(&self) -> bool {
            *self == REFSELR::INT1V
        }
        #[doc = "Checks if the value of the field is `INTVCC0`"]
        #[inline(always)]
        pub fn is_intvcc0(&self) -> bool {
            *self == REFSELR::INTVCC0
        }
        #[doc = "Checks if the value of the field is `INTVCC1`"]
        #[inline(always)]
        pub fn is_intvcc1(&self) -> bool {
            *self == REFSELR::INTVCC1
        }
        #[doc = "Checks if the value of the field is `AREFA`"]
        #[inline(always)]
        pub fn is_arefa(&self) -> bool {
            *self == REFSELR::AREFA
        }
        #[doc = "Checks if the value of the field is `AREFB`"]
        #[inline(always)]
        pub fn is_arefb(&self) -> bool {
            *self == REFSELR::AREFB
        }
    }
    #[doc = r" Value of the field"]
    pub struct REFCOMPR {
        bits: bool,
    }
    impl REFCOMPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Values that can be written to the field `REFSEL`"]
    pub enum REFSELW {
        #[doc = "1.0V voltage reference"]
        INT1V,
        #[doc = "1/1.48 VDDANA"]
        INTVCC0,
        #[doc = "1/2 VDDANA (only for VDDANA > 2.0V)"]
        INTVCC1,
        #[doc = "External reference"]
        AREFA,
        #[doc = "External reference"]
        AREFB,
    }
    impl REFSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                REFSELW::INT1V => 0,
                REFSELW::INTVCC0 => 1,
                REFSELW::INTVCC1 => 2,
                REFSELW::AREFA => 3,
                REFSELW::AREFB => 4,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _REFSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _REFSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: REFSELW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "1.0V voltage reference"]
        #[inline(always)]
        pub fn int1v(self) -> &'a mut W {
            self.variant(REFSELW::INT1V)
        }
        #[doc = "1/1.48 VDDANA"]
        #[inline(always)]
        pub fn intvcc0(self) -> &'a mut W {
            self.variant(REFSELW::INTVCC0)
        }
        #[doc = "1/2 VDDANA (only for VDDANA > 2.0V)"]
        #[inline(always)]
        pub fn intvcc1(self) -> &'a mut W {
            self.variant(REFSELW::INTVCC1)
        }
        #[doc = "External reference"]
        #[inline(always)]
        pub fn arefa(self) -> &'a mut W {
            self.variant(REFSELW::AREFA)
        }
        #[doc = "External reference"]
        #[inline(always)]
        pub fn arefb(self) -> &'a mut W {
            self.variant(REFSELW::AREFB)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _REFCOMPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _REFCOMPW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bits 0:3 - Reference Selection"]
        #[inline(always)]
        pub fn refsel(&self) -> REFSELR {
            REFSELR::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            })
        }
        #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
        #[inline(always)]
        pub fn refcomp(&self) -> REFCOMPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            REFCOMPR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:3 - Reference Selection"]
        #[inline(always)]
        pub fn refsel(&mut self) -> _REFSELW {
            _REFSELW { w: self }
        }
        #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
        #[inline(always)]
        pub fn refcomp(&mut self) -> _REFCOMPW {
            _REFCOMPW { w: self }
        }
    }
}
#[doc = "Result"]
pub struct RESULT {
    register: VolatileCell<u16>,
}
#[doc = "Result"]
pub mod result {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    impl super::RESULT {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct RESULTR {
        bits: u16,
    }
    impl RESULTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bits 0:15 - Result Conversion Value"]
        #[inline(always)]
        pub fn result(&self) -> RESULTR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u16
            };
            RESULTR { bits }
        }
    }
}
#[doc = "Sampling Time Control"]
pub struct SAMPCTRL {
    register: VolatileCell<u8>,
}
#[doc = "Sampling Time Control"]
pub mod sampctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::SAMPCTRL {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct SAMPLENR {
        bits: u8,
    }
    impl SAMPLENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _SAMPLENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SAMPLENW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bits 0:5 - Sampling Time Length"]
        #[inline(always)]
        pub fn samplen(&self) -> SAMPLENR {
            let bits = {
                const MASK: u8 = 63;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            };
            SAMPLENR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:5 - Sampling Time Length"]
        #[inline(always)]
        pub fn samplen(&mut self) -> _SAMPLENW {
            _SAMPLENW { w: self }
        }
    }
}
#[doc = "Status"]
pub struct STATUS {
    register: VolatileCell<u8>,
}
#[doc = "Status"]
pub mod status {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    impl super::STATUS {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct SYNCBUSYR {
        bits: bool,
    }
    impl SYNCBUSYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bit 7 - Synchronization Busy"]
        #[inline(always)]
        pub fn syncbusy(&self) -> SYNCBUSYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SYNCBUSYR { bits }
        }
    }
}
#[doc = "Software Trigger"]
pub struct SWTRIG {
    register: VolatileCell<u8>,
}
#[doc = "Software Trigger"]
pub mod swtrig {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::SWTRIG {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct FLUSHR {
        bits: bool,
    }
    impl FLUSHR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct STARTR {
        bits: bool,
    }
    impl STARTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Proxy"]
    pub struct _FLUSHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FLUSHW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _STARTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STARTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bit 0 - ADC Conversion Flush"]
        #[inline(always)]
        pub fn flush(&self) -> FLUSHR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            FLUSHR { bits }
        }
        #[doc = "Bit 1 - ADC Start Conversion"]
        #[inline(always)]
        pub fn start(&self) -> STARTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            STARTR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - ADC Conversion Flush"]
        #[inline(always)]
        pub fn flush(&mut self) -> _FLUSHW {
            _FLUSHW { w: self }
        }
        #[doc = "Bit 1 - ADC Start Conversion"]
        #[inline(always)]
        pub fn start(&mut self) -> _STARTW {
            _STARTW { w: self }
        }
    }
}
#[doc = "Window Monitor Control"]
pub struct WINCTRL {
    register: VolatileCell<u8>,
}
#[doc = "Window Monitor Control"]
pub mod winctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::WINCTRL {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = "Possible values of the field `WINMODE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum WINMODER {
        #[doc = "No window mode (default)"]
        DISABLE,
        #[doc = "Mode 1: RESULT > WINLT"]
        MODE1,
        #[doc = "Mode 2: RESULT < WINUT"]
        MODE2,
        #[doc = "Mode 3: WINLT < RESULT < WINUT"]
        MODE3,
        #[doc = "Mode 4: !(WINLT < RESULT < WINUT)"]
        MODE4,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl WINMODER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                WINMODER::DISABLE => 0,
                WINMODER::MODE1 => 1,
                WINMODER::MODE2 => 2,
                WINMODER::MODE3 => 3,
                WINMODER::MODE4 => 4,
                WINMODER::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> WINMODER {
            match value {
                0 => WINMODER::DISABLE,
                1 => WINMODER::MODE1,
                2 => WINMODER::MODE2,
                3 => WINMODER::MODE3,
                4 => WINMODER::MODE4,
                i => WINMODER::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == WINMODER::DISABLE
        }
        #[doc = "Checks if the value of the field is `MODE1`"]
        #[inline(always)]
        pub fn is_mode1(&self) -> bool {
            *self == WINMODER::MODE1
        }
        #[doc = "Checks if the value of the field is `MODE2`"]
        #[inline(always)]
        pub fn is_mode2(&self) -> bool {
            *self == WINMODER::MODE2
        }
        #[doc = "Checks if the value of the field is `MODE3`"]
        #[inline(always)]
        pub fn is_mode3(&self) -> bool {
            *self == WINMODER::MODE3
        }
        #[doc = "Checks if the value of the field is `MODE4`"]
        #[inline(always)]
        pub fn is_mode4(&self) -> bool {
            *self == WINMODER::MODE4
        }
    }
    #[doc = "Values that can be written to the field `WINMODE`"]
    pub enum WINMODEW {
        #[doc = "No window mode (default)"]
        DISABLE,
        #[doc = "Mode 1: RESULT > WINLT"]
        MODE1,
        #[doc = "Mode 2: RESULT < WINUT"]
        MODE2,
        #[doc = "Mode 3: WINLT < RESULT < WINUT"]
        MODE3,
        #[doc = "Mode 4: !(WINLT < RESULT < WINUT)"]
        MODE4,
    }
    impl WINMODEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                WINMODEW::DISABLE => 0,
                WINMODEW::MODE1 => 1,
                WINMODEW::MODE2 => 2,
                WINMODEW::MODE3 => 3,
                WINMODEW::MODE4 => 4,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _WINMODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WINMODEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: WINMODEW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No window mode (default)"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(WINMODEW::DISABLE)
        }
        #[doc = "Mode 1: RESULT > WINLT"]
        #[inline(always)]
        pub fn mode1(self) -> &'a mut W {
            self.variant(WINMODEW::MODE1)
        }
        #[doc = "Mode 2: RESULT < WINUT"]
        #[inline(always)]
        pub fn mode2(self) -> &'a mut W {
            self.variant(WINMODEW::MODE2)
        }
        #[doc = "Mode 3: WINLT < RESULT < WINUT"]
        #[inline(always)]
        pub fn mode3(self) -> &'a mut W {
            self.variant(WINMODEW::MODE3)
        }
        #[doc = "Mode 4: !(WINLT < RESULT < WINUT)"]
        #[inline(always)]
        pub fn mode4(self) -> &'a mut W {
            self.variant(WINMODEW::MODE4)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bits 0:2 - Window Monitor Mode"]
        #[inline(always)]
        pub fn winmode(&self) -> WINMODER {
            WINMODER::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            })
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:2 - Window Monitor Mode"]
        #[inline(always)]
        pub fn winmode(&mut self) -> _WINMODEW {
            _WINMODEW { w: self }
        }
    }
}
#[doc = "Window Monitor Lower Threshold"]
pub struct WINLT {
    register: VolatileCell<u16>,
}
#[doc = "Window Monitor Lower Threshold"]
pub mod winlt {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::WINLT {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct WINLTR {
        bits: u16,
    }
    impl WINLTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _WINLTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WINLTW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bits 0:15 - Window Lower Threshold"]
        #[inline(always)]
        pub fn winlt(&self) -> WINLTR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u16
            };
            WINLTR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:15 - Window Lower Threshold"]
        #[inline(always)]
        pub fn winlt(&mut self) -> _WINLTW {
            _WINLTW { w: self }
        }
    }
}
#[doc = "Window Monitor Upper Threshold"]
pub struct WINUT {
    register: VolatileCell<u16>,
}
#[doc = "Window Monitor Upper Threshold"]
pub mod winut {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::WINUT {
        #[doc = r" Modifies the contents of the register"]
        #[inline(always)]
        pub fn modify<F>(&self, f: F)
        where
            for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        #[doc = r" Writes to the register"]
        #[inline(always)]
        pub fn write<F>(&self, f: F)
        where
            F: FnOnce(&mut W) -> &mut W,
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        #[doc = r" Writes the reset value to the register"]
        #[inline(always)]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    #[doc = r" Value of the field"]
    pub struct WINUTR {
        bits: u16,
    }
    impl WINUTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _WINUTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WINUTW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bits 0:15 - Window Upper Threshold"]
        #[inline(always)]
        pub fn winut(&self) -> WINUTR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u16
            };
            WINUTR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:15 - Window Upper Threshold"]
        #[inline(always)]
        pub fn winut(&mut self) -> _WINUTW {
            _WINUTW { w: self }
        }
    }
}
