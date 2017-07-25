use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: CTRLBSET,
    _reserved0: [u8; 2usize],
    #[doc = "0x08 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - Recoverable FaultA Configuration"]
    pub fctrla: FCTRLA,
    #[doc = "0x10 - Recoverable FaultB Configuration"]
    pub fctrlb: FCTRLB,
    #[doc = "0x14 - Waveform Extension Configuration"]
    pub wexctrl: WEXCTRL,
    #[doc = "0x18 - Driver Configuration"]
    pub drvctrl: DRVCTRL,
    _reserved1: [u8; 2usize],
    #[doc = "0x1e - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved2: [u8; 1usize],
    #[doc = "0x20 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x24 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x28 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x2c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x30 - Status"]
    pub status: STATUS,
    #[doc = "0x34 - Count"]
    pub count: COUNT,
    #[doc = "0x38 - Pattern"]
    pub patt: PATT,
    _reserved3: [u8; 2usize],
    #[doc = "0x3c - Waveform Control"]
    pub wave: WAVE,
    #[doc = "0x40 - Period"]
    pub per: PER,
    #[doc = "0x44 - Compare and Capture"]
    pub cc0: CC,
    #[doc = "0x48 - Compare and Capture"]
    pub cc1: CC,
    #[doc = "0x4c - Compare and Capture"]
    pub cc2: CC,
    #[doc = "0x50 - Compare and Capture"]
    pub cc3: CC,
    _reserved4: [u8; 16usize],
    #[doc = "0x64 - Pattern Buffer"]
    pub pattb: PATTB,
    _reserved5: [u8; 2usize],
    #[doc = "0x68 - Waveform Control Buffer"]
    pub waveb: WAVEB,
    #[doc = "0x6c - Period Buffer"]
    pub perb: PERB,
    #[doc = "0x70 - Compare and Capture Buffer"]
    pub ccb0: CCB,
    #[doc = "0x74 - Compare and Capture Buffer"]
    pub ccb1: CCB,
    #[doc = "0x78 - Compare and Capture Buffer"]
    pub ccb2: CCB,
    #[doc = "0x7c - Compare and Capture Buffer"]
    pub ccb3: CCB,
}
#[doc = "Compare and Capture"]
pub struct CC {
    register: VolatileCell<u32>,
}
#[doc = "Compare and Capture"]
pub mod cc {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::CC {
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
    pub struct CCR {
        bits: u32,
    }
    impl CCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _CCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
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
        #[doc = "Bits 0:23 - Compare and Capture value"]
        #[inline(always)]
        pub fn cc(&self) -> CCR {
            let bits = {
                const MASK: u32 = 16777215;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            CCR { bits }
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
        #[doc = "Bits 0:23 - Compare and Capture value"]
        #[inline(always)]
        pub fn cc(&mut self) -> _CCW {
            _CCW { w: self }
        }
    }
}
#[doc = "Compare and Capture Buffer"]
pub struct CCB {
    register: VolatileCell<u32>,
}
#[doc = "Compare and Capture Buffer"]
pub mod ccb {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::CCB {
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
    pub struct CCBR {
        bits: u32,
    }
    impl CCBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _CCBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCBW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
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
        #[doc = "Bits 0:23 - Compare and Capture buffer value"]
        #[inline(always)]
        pub fn ccb(&self) -> CCBR {
            let bits = {
                const MASK: u32 = 16777215;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            CCBR { bits }
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
        #[doc = "Bits 0:23 - Compare and Capture buffer value"]
        #[inline(always)]
        pub fn ccb(&mut self) -> _CCBW {
            _CCBW { w: self }
        }
    }
}
#[doc = "Count"]
pub struct COUNT {
    register: VolatileCell<u32>,
}
#[doc = "Count"]
pub mod count {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::COUNT {
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
    pub struct COUNTR {
        bits: u32,
    }
    impl COUNTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _COUNTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _COUNTW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
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
        #[doc = "Bits 0:23 - Count Value"]
        #[inline(always)]
        pub fn count(&self) -> COUNTR {
            let bits = {
                const MASK: u32 = 16777215;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            COUNTR { bits }
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
        #[doc = "Bits 0:23 - Count Value"]
        #[inline(always)]
        pub fn count(&mut self) -> _COUNTW {
            _COUNTW { w: self }
        }
    }
}
#[doc = "Control A"]
pub struct CTRLA {
    register: VolatileCell<u32>,
}
#[doc = "Control A"]
pub mod ctrla {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
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
    #[doc = "Possible values of the field `RESOLUTION`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum RESOLUTIONR {
        #[doc = "undocumented"]
        NONE,
        #[doc = "undocumented"]
        DITH4,
        #[doc = "undocumented"]
        DITH5,
        #[doc = "undocumented"]
        DITH6,
    }
    impl RESOLUTIONR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                RESOLUTIONR::NONE => 0,
                RESOLUTIONR::DITH4 => 1,
                RESOLUTIONR::DITH5 => 2,
                RESOLUTIONR::DITH6 => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> RESOLUTIONR {
            match value {
                0 => RESOLUTIONR::NONE,
                1 => RESOLUTIONR::DITH4,
                2 => RESOLUTIONR::DITH5,
                3 => RESOLUTIONR::DITH6,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == RESOLUTIONR::NONE
        }
        #[doc = "Checks if the value of the field is `DITH4`"]
        #[inline(always)]
        pub fn is_dith4(&self) -> bool {
            *self == RESOLUTIONR::DITH4
        }
        #[doc = "Checks if the value of the field is `DITH5`"]
        #[inline(always)]
        pub fn is_dith5(&self) -> bool {
            *self == RESOLUTIONR::DITH5
        }
        #[doc = "Checks if the value of the field is `DITH6`"]
        #[inline(always)]
        pub fn is_dith6(&self) -> bool {
            *self == RESOLUTIONR::DITH6
        }
    }
    #[doc = "Possible values of the field `PRESCALER`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PRESCALERR {
        #[doc = "undocumented"]
        DIV1,
        #[doc = "undocumented"]
        DIV2,
        #[doc = "undocumented"]
        DIV4,
        #[doc = "undocumented"]
        DIV8,
        #[doc = "undocumented"]
        DIV16,
        #[doc = "undocumented"]
        DIV64,
        #[doc = "undocumented"]
        DIV256,
        #[doc = "undocumented"]
        DIV1024,
    }
    impl PRESCALERR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                PRESCALERR::DIV1 => 0,
                PRESCALERR::DIV2 => 1,
                PRESCALERR::DIV4 => 2,
                PRESCALERR::DIV8 => 3,
                PRESCALERR::DIV16 => 4,
                PRESCALERR::DIV64 => 5,
                PRESCALERR::DIV256 => 6,
                PRESCALERR::DIV1024 => 7,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> PRESCALERR {
            match value {
                0 => PRESCALERR::DIV1,
                1 => PRESCALERR::DIV2,
                2 => PRESCALERR::DIV4,
                3 => PRESCALERR::DIV8,
                4 => PRESCALERR::DIV16,
                5 => PRESCALERR::DIV64,
                6 => PRESCALERR::DIV256,
                7 => PRESCALERR::DIV1024,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DIV1`"]
        #[inline(always)]
        pub fn is_div1(&self) -> bool {
            *self == PRESCALERR::DIV1
        }
        #[doc = "Checks if the value of the field is `DIV2`"]
        #[inline(always)]
        pub fn is_div2(&self) -> bool {
            *self == PRESCALERR::DIV2
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
        #[doc = "Checks if the value of the field is `DIV64`"]
        #[inline(always)]
        pub fn is_div64(&self) -> bool {
            *self == PRESCALERR::DIV64
        }
        #[doc = "Checks if the value of the field is `DIV256`"]
        #[inline(always)]
        pub fn is_div256(&self) -> bool {
            *self == PRESCALERR::DIV256
        }
        #[doc = "Checks if the value of the field is `DIV1024`"]
        #[inline(always)]
        pub fn is_div1024(&self) -> bool {
            *self == PRESCALERR::DIV1024
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
    #[doc = "Possible values of the field `PRESCSYNC`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PRESCSYNCR {
        #[doc = "undocumented"]
        GCLK,
        #[doc = "undocumented"]
        PRESC,
        #[doc = "undocumented"]
        RESYNC,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl PRESCSYNCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                PRESCSYNCR::GCLK => 0,
                PRESCSYNCR::PRESC => 1,
                PRESCSYNCR::RESYNC => 2,
                PRESCSYNCR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> PRESCSYNCR {
            match value {
                0 => PRESCSYNCR::GCLK,
                1 => PRESCSYNCR::PRESC,
                2 => PRESCSYNCR::RESYNC,
                i => PRESCSYNCR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `GCLK`"]
        #[inline(always)]
        pub fn is_gclk(&self) -> bool {
            *self == PRESCSYNCR::GCLK
        }
        #[doc = "Checks if the value of the field is `PRESC`"]
        #[inline(always)]
        pub fn is_presc(&self) -> bool {
            *self == PRESCSYNCR::PRESC
        }
        #[doc = "Checks if the value of the field is `RESYNC`"]
        #[inline(always)]
        pub fn is_resync(&self) -> bool {
            *self == PRESCSYNCR::RESYNC
        }
    }
    #[doc = r" Value of the field"]
    pub struct ALOCKR {
        bits: bool,
    }
    impl ALOCKR {
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
    pub struct MSYNCR {
        bits: bool,
    }
    impl MSYNCR {
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
    pub struct CPTEN0R {
        bits: bool,
    }
    impl CPTEN0R {
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
    pub struct CPTEN1R {
        bits: bool,
    }
    impl CPTEN1R {
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
    pub struct CPTEN2R {
        bits: bool,
    }
    impl CPTEN2R {
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
    pub struct CPTEN3R {
        bits: bool,
    }
    impl CPTEN3R {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `RESOLUTION`"]
    pub enum RESOLUTIONW {
        #[doc = "`0`"]
        NONE,
        #[doc = "`1`"]
        DITH4,
        #[doc = "`10`"]
        DITH5,
        #[doc = "`11`"]
        DITH6,
    }
    impl RESOLUTIONW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                RESOLUTIONW::NONE => 0,
                RESOLUTIONW::DITH4 => 1,
                RESOLUTIONW::DITH5 => 2,
                RESOLUTIONW::DITH6 => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _RESOLUTIONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESOLUTIONW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: RESOLUTIONW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(RESOLUTIONW::NONE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn dith4(self) -> &'a mut W {
            self.variant(RESOLUTIONW::DITH4)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn dith5(self) -> &'a mut W {
            self.variant(RESOLUTIONW::DITH5)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn dith6(self) -> &'a mut W {
            self.variant(RESOLUTIONW::DITH6)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `PRESCALER`"]
    pub enum PRESCALERW {
        #[doc = "`0`"]
        DIV1,
        #[doc = "`1`"]
        DIV2,
        #[doc = "`10`"]
        DIV4,
        #[doc = "`11`"]
        DIV8,
        #[doc = "`100`"]
        DIV16,
        #[doc = "`101`"]
        DIV64,
        #[doc = "`110`"]
        DIV256,
        #[doc = "`111`"]
        DIV1024,
    }
    impl PRESCALERW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                PRESCALERW::DIV1 => 0,
                PRESCALERW::DIV2 => 1,
                PRESCALERW::DIV4 => 2,
                PRESCALERW::DIV8 => 3,
                PRESCALERW::DIV16 => 4,
                PRESCALERW::DIV64 => 5,
                PRESCALERW::DIV256 => 6,
                PRESCALERW::DIV1024 => 7,
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
        #[doc = "`0`"]
        #[inline(always)]
        pub fn div1(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV1)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn div2(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV2)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn div4(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV4)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn div8(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV8)
        }
        #[doc = "`100`"]
        #[inline(always)]
        pub fn div16(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV16)
        }
        #[doc = "`101`"]
        #[inline(always)]
        pub fn div64(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV64)
        }
        #[doc = "`110`"]
        #[inline(always)]
        pub fn div256(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV256)
        }
        #[doc = "`111`"]
        #[inline(always)]
        pub fn div1024(self) -> &'a mut W {
            self.variant(PRESCALERW::DIV1024)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `PRESCSYNC`"]
    pub enum PRESCSYNCW {
        #[doc = "`0`"]
        GCLK,
        #[doc = "`1`"]
        PRESC,
        #[doc = "`10`"]
        RESYNC,
    }
    impl PRESCSYNCW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                PRESCSYNCW::GCLK => 0,
                PRESCSYNCW::PRESC => 1,
                PRESCSYNCW::RESYNC => 2,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _PRESCSYNCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PRESCSYNCW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: PRESCSYNCW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn gclk(self) -> &'a mut W {
            self.variant(PRESCSYNCW::GCLK)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn presc(self) -> &'a mut W {
            self.variant(PRESCSYNCW::PRESC)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn resync(self) -> &'a mut W {
            self.variant(PRESCSYNCW::RESYNC)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ALOCKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ALOCKW<'a> {
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
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MSYNCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MSYNCW<'a> {
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
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CPTEN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CPTEN0W<'a> {
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
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CPTEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CPTEN1W<'a> {
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
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CPTEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CPTEN2W<'a> {
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
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CPTEN3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CPTEN3W<'a> {
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
            const OFFSET: u8 = 27;
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
        #[doc = "Bit 0 - Software Reset"]
        #[inline(always)]
        pub fn swrst(&self) -> SWRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWRSTR { bits }
        }
        #[doc = "Bit 1 - Enable"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bits 5:6 - Enhanced Resolution"]
        #[inline(always)]
        pub fn resolution(&self) -> RESOLUTIONR {
            RESOLUTIONR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 8:10 - Prescaler"]
        #[inline(always)]
        pub fn prescaler(&self) -> PRESCALERR {
            PRESCALERR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 11 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&self) -> RUNSTDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RUNSTDBYR { bits }
        }
        #[doc = "Bits 12:13 - Prescaler and Counter Synchronization Selection"]
        #[inline(always)]
        pub fn prescsync(&self) -> PRESCSYNCR {
            PRESCSYNCR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 14 - Auto Lock"]
        #[inline(always)]
        pub fn alock(&self) -> ALOCKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ALOCKR { bits }
        }
        #[doc = "Bit 15 - Master Synchronization"]
        #[inline(always)]
        pub fn msync(&self) -> MSYNCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MSYNCR { bits }
        }
        #[doc = "Bit 24 - Capture Channel 0 Enable"]
        #[inline(always)]
        pub fn cpten0(&self) -> CPTEN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CPTEN0R { bits }
        }
        #[doc = "Bit 25 - Capture Channel 1 Enable"]
        #[inline(always)]
        pub fn cpten1(&self) -> CPTEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CPTEN1R { bits }
        }
        #[doc = "Bit 26 - Capture Channel 2 Enable"]
        #[inline(always)]
        pub fn cpten2(&self) -> CPTEN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CPTEN2R { bits }
        }
        #[doc = "Bit 27 - Capture Channel 3 Enable"]
        #[inline(always)]
        pub fn cpten3(&self) -> CPTEN3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CPTEN3R { bits }
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
        #[doc = "Bits 5:6 - Enhanced Resolution"]
        #[inline(always)]
        pub fn resolution(&mut self) -> _RESOLUTIONW {
            _RESOLUTIONW { w: self }
        }
        #[doc = "Bits 8:10 - Prescaler"]
        #[inline(always)]
        pub fn prescaler(&mut self) -> _PRESCALERW {
            _PRESCALERW { w: self }
        }
        #[doc = "Bit 11 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&mut self) -> _RUNSTDBYW {
            _RUNSTDBYW { w: self }
        }
        #[doc = "Bits 12:13 - Prescaler and Counter Synchronization Selection"]
        #[inline(always)]
        pub fn prescsync(&mut self) -> _PRESCSYNCW {
            _PRESCSYNCW { w: self }
        }
        #[doc = "Bit 14 - Auto Lock"]
        #[inline(always)]
        pub fn alock(&mut self) -> _ALOCKW {
            _ALOCKW { w: self }
        }
        #[doc = "Bit 15 - Master Synchronization"]
        #[inline(always)]
        pub fn msync(&mut self) -> _MSYNCW {
            _MSYNCW { w: self }
        }
        #[doc = "Bit 24 - Capture Channel 0 Enable"]
        #[inline(always)]
        pub fn cpten0(&mut self) -> _CPTEN0W {
            _CPTEN0W { w: self }
        }
        #[doc = "Bit 25 - Capture Channel 1 Enable"]
        #[inline(always)]
        pub fn cpten1(&mut self) -> _CPTEN1W {
            _CPTEN1W { w: self }
        }
        #[doc = "Bit 26 - Capture Channel 2 Enable"]
        #[inline(always)]
        pub fn cpten2(&mut self) -> _CPTEN2W {
            _CPTEN2W { w: self }
        }
        #[doc = "Bit 27 - Capture Channel 3 Enable"]
        #[inline(always)]
        pub fn cpten3(&mut self) -> _CPTEN3W {
            _CPTEN3W { w: self }
        }
    }
}
#[doc = "Control B Clear"]
pub struct CTRLBCLR {
    register: VolatileCell<u8>,
}
#[doc = "Control B Clear"]
pub mod ctrlbclr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CTRLBCLR {
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
    pub struct DIRR {
        bits: bool,
    }
    impl DIRR {
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
    pub struct LUPDR {
        bits: bool,
    }
    impl LUPDR {
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
    pub struct ONESHOTR {
        bits: bool,
    }
    impl ONESHOTR {
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
    #[doc = "Possible values of the field `IDXCMD`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum IDXCMDR {
        #[doc = "undocumented"]
        DISABLE,
        #[doc = "undocumented"]
        SET,
        #[doc = "undocumented"]
        CLEAR,
        #[doc = "undocumented"]
        HOLD,
    }
    impl IDXCMDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                IDXCMDR::DISABLE => 0,
                IDXCMDR::SET => 1,
                IDXCMDR::CLEAR => 2,
                IDXCMDR::HOLD => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> IDXCMDR {
            match value {
                0 => IDXCMDR::DISABLE,
                1 => IDXCMDR::SET,
                2 => IDXCMDR::CLEAR,
                3 => IDXCMDR::HOLD,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == IDXCMDR::DISABLE
        }
        #[doc = "Checks if the value of the field is `SET`"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            *self == IDXCMDR::SET
        }
        #[doc = "Checks if the value of the field is `CLEAR`"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            *self == IDXCMDR::CLEAR
        }
        #[doc = "Checks if the value of the field is `HOLD`"]
        #[inline(always)]
        pub fn is_hold(&self) -> bool {
            *self == IDXCMDR::HOLD
        }
    }
    #[doc = "Possible values of the field `CMD`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CMDR {
        #[doc = "undocumented"]
        NONE,
        #[doc = "undocumented"]
        RETRIGGER,
        #[doc = "undocumented"]
        STOP,
        #[doc = "undocumented"]
        UPDATE,
        #[doc = "undocumented"]
        READSYNC,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CMDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CMDR::NONE => 0,
                CMDR::RETRIGGER => 1,
                CMDR::STOP => 2,
                CMDR::UPDATE => 3,
                CMDR::READSYNC => 4,
                CMDR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CMDR {
            match value {
                0 => CMDR::NONE,
                1 => CMDR::RETRIGGER,
                2 => CMDR::STOP,
                3 => CMDR::UPDATE,
                4 => CMDR::READSYNC,
                i => CMDR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == CMDR::NONE
        }
        #[doc = "Checks if the value of the field is `RETRIGGER`"]
        #[inline(always)]
        pub fn is_retrigger(&self) -> bool {
            *self == CMDR::RETRIGGER
        }
        #[doc = "Checks if the value of the field is `STOP`"]
        #[inline(always)]
        pub fn is_stop(&self) -> bool {
            *self == CMDR::STOP
        }
        #[doc = "Checks if the value of the field is `UPDATE`"]
        #[inline(always)]
        pub fn is_update(&self) -> bool {
            *self == CMDR::UPDATE
        }
        #[doc = "Checks if the value of the field is `READSYNC`"]
        #[inline(always)]
        pub fn is_readsync(&self) -> bool {
            *self == CMDR::READSYNC
        }
    }
    #[doc = r" Proxy"]
    pub struct _DIRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIRW<'a> {
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
    pub struct _LUPDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LUPDW<'a> {
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
    pub struct _ONESHOTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ONESHOTW<'a> {
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
    #[doc = "Values that can be written to the field `IDXCMD`"]
    pub enum IDXCMDW {
        #[doc = "`0`"]
        DISABLE,
        #[doc = "`1`"]
        SET,
        #[doc = "`10`"]
        CLEAR,
        #[doc = "`11`"]
        HOLD,
    }
    impl IDXCMDW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                IDXCMDW::DISABLE => 0,
                IDXCMDW::SET => 1,
                IDXCMDW::CLEAR => 2,
                IDXCMDW::HOLD => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _IDXCMDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDXCMDW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: IDXCMDW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(IDXCMDW::DISABLE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn set(self) -> &'a mut W {
            self.variant(IDXCMDW::SET)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn clear(self) -> &'a mut W {
            self.variant(IDXCMDW::CLEAR)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn hold(self) -> &'a mut W {
            self.variant(IDXCMDW::HOLD)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CMD`"]
    pub enum CMDW {
        #[doc = "`0`"]
        NONE,
        #[doc = "`1`"]
        RETRIGGER,
        #[doc = "`10`"]
        STOP,
        #[doc = "`11`"]
        UPDATE,
        #[doc = "`100`"]
        READSYNC,
    }
    impl CMDW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CMDW::NONE => 0,
                CMDW::RETRIGGER => 1,
                CMDW::STOP => 2,
                CMDW::UPDATE => 3,
                CMDW::READSYNC => 4,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CMDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CMDW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CMDW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(CMDW::NONE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn retrigger(self) -> &'a mut W {
            self.variant(CMDW::RETRIGGER)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn stop(self) -> &'a mut W {
            self.variant(CMDW::STOP)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn update(self) -> &'a mut W {
            self.variant(CMDW::UPDATE)
        }
        #[doc = "`100`"]
        #[inline(always)]
        pub fn readsync(self) -> &'a mut W {
            self.variant(CMDW::READSYNC)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
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
        #[doc = "Bit 0 - Counter Direction"]
        #[inline(always)]
        pub fn dir(&self) -> DIRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            DIRR { bits }
        }
        #[doc = "Bit 1 - Lock Update"]
        #[inline(always)]
        pub fn lupd(&self) -> LUPDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            LUPDR { bits }
        }
        #[doc = "Bit 2 - One-Shot"]
        #[inline(always)]
        pub fn oneshot(&self) -> ONESHOTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            ONESHOTR { bits }
        }
        #[doc = "Bits 3:4 - Ramp Index Command"]
        #[inline(always)]
        pub fn idxcmd(&self) -> IDXCMDR {
            IDXCMDR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            })
        }
        #[doc = "Bits 5:7 - TCC Command"]
        #[inline(always)]
        pub fn cmd(&self) -> CMDR {
            CMDR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 5;
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
        #[doc = "Bit 0 - Counter Direction"]
        #[inline(always)]
        pub fn dir(&mut self) -> _DIRW {
            _DIRW { w: self }
        }
        #[doc = "Bit 1 - Lock Update"]
        #[inline(always)]
        pub fn lupd(&mut self) -> _LUPDW {
            _LUPDW { w: self }
        }
        #[doc = "Bit 2 - One-Shot"]
        #[inline(always)]
        pub fn oneshot(&mut self) -> _ONESHOTW {
            _ONESHOTW { w: self }
        }
        #[doc = "Bits 3:4 - Ramp Index Command"]
        #[inline(always)]
        pub fn idxcmd(&mut self) -> _IDXCMDW {
            _IDXCMDW { w: self }
        }
        #[doc = "Bits 5:7 - TCC Command"]
        #[inline(always)]
        pub fn cmd(&mut self) -> _CMDW {
            _CMDW { w: self }
        }
    }
}
#[doc = "Control B Set"]
pub struct CTRLBSET {
    register: VolatileCell<u8>,
}
#[doc = "Control B Set"]
pub mod ctrlbset {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CTRLBSET {
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
    pub struct DIRR {
        bits: bool,
    }
    impl DIRR {
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
    pub struct LUPDR {
        bits: bool,
    }
    impl LUPDR {
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
    pub struct ONESHOTR {
        bits: bool,
    }
    impl ONESHOTR {
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
    #[doc = "Possible values of the field `IDXCMD`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum IDXCMDR {
        #[doc = "undocumented"]
        DISABLE,
        #[doc = "undocumented"]
        SET,
        #[doc = "undocumented"]
        CLEAR,
        #[doc = "undocumented"]
        HOLD,
    }
    impl IDXCMDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                IDXCMDR::DISABLE => 0,
                IDXCMDR::SET => 1,
                IDXCMDR::CLEAR => 2,
                IDXCMDR::HOLD => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> IDXCMDR {
            match value {
                0 => IDXCMDR::DISABLE,
                1 => IDXCMDR::SET,
                2 => IDXCMDR::CLEAR,
                3 => IDXCMDR::HOLD,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == IDXCMDR::DISABLE
        }
        #[doc = "Checks if the value of the field is `SET`"]
        #[inline(always)]
        pub fn is_set(&self) -> bool {
            *self == IDXCMDR::SET
        }
        #[doc = "Checks if the value of the field is `CLEAR`"]
        #[inline(always)]
        pub fn is_clear(&self) -> bool {
            *self == IDXCMDR::CLEAR
        }
        #[doc = "Checks if the value of the field is `HOLD`"]
        #[inline(always)]
        pub fn is_hold(&self) -> bool {
            *self == IDXCMDR::HOLD
        }
    }
    #[doc = "Possible values of the field `CMD`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CMDR {
        #[doc = "undocumented"]
        NONE,
        #[doc = "undocumented"]
        RETRIGGER,
        #[doc = "undocumented"]
        STOP,
        #[doc = "undocumented"]
        UPDATE,
        #[doc = "undocumented"]
        READSYNC,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CMDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CMDR::NONE => 0,
                CMDR::RETRIGGER => 1,
                CMDR::STOP => 2,
                CMDR::UPDATE => 3,
                CMDR::READSYNC => 4,
                CMDR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CMDR {
            match value {
                0 => CMDR::NONE,
                1 => CMDR::RETRIGGER,
                2 => CMDR::STOP,
                3 => CMDR::UPDATE,
                4 => CMDR::READSYNC,
                i => CMDR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == CMDR::NONE
        }
        #[doc = "Checks if the value of the field is `RETRIGGER`"]
        #[inline(always)]
        pub fn is_retrigger(&self) -> bool {
            *self == CMDR::RETRIGGER
        }
        #[doc = "Checks if the value of the field is `STOP`"]
        #[inline(always)]
        pub fn is_stop(&self) -> bool {
            *self == CMDR::STOP
        }
        #[doc = "Checks if the value of the field is `UPDATE`"]
        #[inline(always)]
        pub fn is_update(&self) -> bool {
            *self == CMDR::UPDATE
        }
        #[doc = "Checks if the value of the field is `READSYNC`"]
        #[inline(always)]
        pub fn is_readsync(&self) -> bool {
            *self == CMDR::READSYNC
        }
    }
    #[doc = r" Proxy"]
    pub struct _DIRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIRW<'a> {
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
    pub struct _LUPDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LUPDW<'a> {
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
    pub struct _ONESHOTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ONESHOTW<'a> {
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
    #[doc = "Values that can be written to the field `IDXCMD`"]
    pub enum IDXCMDW {
        #[doc = "`0`"]
        DISABLE,
        #[doc = "`1`"]
        SET,
        #[doc = "`10`"]
        CLEAR,
        #[doc = "`11`"]
        HOLD,
    }
    impl IDXCMDW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                IDXCMDW::DISABLE => 0,
                IDXCMDW::SET => 1,
                IDXCMDW::CLEAR => 2,
                IDXCMDW::HOLD => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _IDXCMDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDXCMDW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: IDXCMDW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(IDXCMDW::DISABLE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn set(self) -> &'a mut W {
            self.variant(IDXCMDW::SET)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn clear(self) -> &'a mut W {
            self.variant(IDXCMDW::CLEAR)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn hold(self) -> &'a mut W {
            self.variant(IDXCMDW::HOLD)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CMD`"]
    pub enum CMDW {
        #[doc = "`0`"]
        NONE,
        #[doc = "`1`"]
        RETRIGGER,
        #[doc = "`10`"]
        STOP,
        #[doc = "`11`"]
        UPDATE,
        #[doc = "`100`"]
        READSYNC,
    }
    impl CMDW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CMDW::NONE => 0,
                CMDW::RETRIGGER => 1,
                CMDW::STOP => 2,
                CMDW::UPDATE => 3,
                CMDW::READSYNC => 4,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CMDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CMDW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CMDW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(CMDW::NONE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn retrigger(self) -> &'a mut W {
            self.variant(CMDW::RETRIGGER)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn stop(self) -> &'a mut W {
            self.variant(CMDW::STOP)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn update(self) -> &'a mut W {
            self.variant(CMDW::UPDATE)
        }
        #[doc = "`100`"]
        #[inline(always)]
        pub fn readsync(self) -> &'a mut W {
            self.variant(CMDW::READSYNC)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
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
        #[doc = "Bit 0 - Counter Direction"]
        #[inline(always)]
        pub fn dir(&self) -> DIRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            DIRR { bits }
        }
        #[doc = "Bit 1 - Lock update"]
        #[inline(always)]
        pub fn lupd(&self) -> LUPDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            LUPDR { bits }
        }
        #[doc = "Bit 2 - One-Shot"]
        #[inline(always)]
        pub fn oneshot(&self) -> ONESHOTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            ONESHOTR { bits }
        }
        #[doc = "Bits 3:4 - Ramp Index Command"]
        #[inline(always)]
        pub fn idxcmd(&self) -> IDXCMDR {
            IDXCMDR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            })
        }
        #[doc = "Bits 5:7 - TCC Command"]
        #[inline(always)]
        pub fn cmd(&self) -> CMDR {
            CMDR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 5;
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
        #[doc = "Bit 0 - Counter Direction"]
        #[inline(always)]
        pub fn dir(&mut self) -> _DIRW {
            _DIRW { w: self }
        }
        #[doc = "Bit 1 - Lock update"]
        #[inline(always)]
        pub fn lupd(&mut self) -> _LUPDW {
            _LUPDW { w: self }
        }
        #[doc = "Bit 2 - One-Shot"]
        #[inline(always)]
        pub fn oneshot(&mut self) -> _ONESHOTW {
            _ONESHOTW { w: self }
        }
        #[doc = "Bits 3:4 - Ramp Index Command"]
        #[inline(always)]
        pub fn idxcmd(&mut self) -> _IDXCMDW {
            _IDXCMDW { w: self }
        }
        #[doc = "Bits 5:7 - TCC Command"]
        #[inline(always)]
        pub fn cmd(&mut self) -> _CMDW {
            _CMDW { w: self }
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
    #[doc = r" Value of the field"]
    pub struct FDDBDR {
        bits: bool,
    }
    impl FDDBDR {
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
    #[doc = r" Proxy"]
    pub struct _FDDBDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FDDBDW<'a> {
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
        #[doc = "Bit 0 - Debug Running Mode"]
        #[inline(always)]
        pub fn dbgrun(&self) -> DBGRUNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            DBGRUNR { bits }
        }
        #[doc = "Bit 2 - Fault Detection on Debug Break Detection"]
        #[inline(always)]
        pub fn fddbd(&self) -> FDDBDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            FDDBDR { bits }
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
        #[doc = "Bit 0 - Debug Running Mode"]
        #[inline(always)]
        pub fn dbgrun(&mut self) -> _DBGRUNW {
            _DBGRUNW { w: self }
        }
        #[doc = "Bit 2 - Fault Detection on Debug Break Detection"]
        #[inline(always)]
        pub fn fddbd(&mut self) -> _FDDBDW {
            _FDDBDW { w: self }
        }
    }
}
#[doc = "Driver Configuration"]
pub struct DRVCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Driver Configuration"]
pub mod drvctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DRVCTRL {
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
    pub struct NRE0R {
        bits: bool,
    }
    impl NRE0R {
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
    pub struct NRE1R {
        bits: bool,
    }
    impl NRE1R {
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
    pub struct NRE2R {
        bits: bool,
    }
    impl NRE2R {
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
    pub struct NRE3R {
        bits: bool,
    }
    impl NRE3R {
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
    pub struct NRE4R {
        bits: bool,
    }
    impl NRE4R {
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
    pub struct NRE5R {
        bits: bool,
    }
    impl NRE5R {
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
    pub struct NRE6R {
        bits: bool,
    }
    impl NRE6R {
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
    pub struct NRE7R {
        bits: bool,
    }
    impl NRE7R {
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
    pub struct NRV0R {
        bits: bool,
    }
    impl NRV0R {
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
    pub struct NRV1R {
        bits: bool,
    }
    impl NRV1R {
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
    pub struct NRV2R {
        bits: bool,
    }
    impl NRV2R {
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
    pub struct NRV3R {
        bits: bool,
    }
    impl NRV3R {
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
    pub struct NRV4R {
        bits: bool,
    }
    impl NRV4R {
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
    pub struct NRV5R {
        bits: bool,
    }
    impl NRV5R {
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
    pub struct NRV6R {
        bits: bool,
    }
    impl NRV6R {
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
    pub struct NRV7R {
        bits: bool,
    }
    impl NRV7R {
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
    pub struct INVEN0R {
        bits: bool,
    }
    impl INVEN0R {
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
    pub struct INVEN1R {
        bits: bool,
    }
    impl INVEN1R {
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
    pub struct INVEN2R {
        bits: bool,
    }
    impl INVEN2R {
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
    pub struct INVEN3R {
        bits: bool,
    }
    impl INVEN3R {
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
    pub struct INVEN4R {
        bits: bool,
    }
    impl INVEN4R {
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
    pub struct INVEN5R {
        bits: bool,
    }
    impl INVEN5R {
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
    pub struct INVEN6R {
        bits: bool,
    }
    impl INVEN6R {
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
    pub struct INVEN7R {
        bits: bool,
    }
    impl INVEN7R {
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
    pub struct FILTERVAL0R {
        bits: u8,
    }
    impl FILTERVAL0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTERVAL1R {
        bits: u8,
    }
    impl FILTERVAL1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRE0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRE0W<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRE1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRE1W<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRE2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRE2W<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRE3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRE3W<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRE4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRE4W<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRE5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRE5W<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRE6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRE6W<'a> {
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRE7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRE7W<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRV0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRV0W<'a> {
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
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRV1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRV1W<'a> {
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
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRV2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRV2W<'a> {
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
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRV3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRV3W<'a> {
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
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRV4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRV4W<'a> {
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
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRV5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRV5W<'a> {
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
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRV6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRV6W<'a> {
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
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NRV7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _NRV7W<'a> {
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
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _INVEN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _INVEN0W<'a> {
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
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _INVEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _INVEN1W<'a> {
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
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _INVEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _INVEN2W<'a> {
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
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _INVEN3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _INVEN3W<'a> {
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
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _INVEN4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _INVEN4W<'a> {
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
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _INVEN5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _INVEN5W<'a> {
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
            const OFFSET: u8 = 21;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _INVEN6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _INVEN6W<'a> {
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
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _INVEN7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _INVEN7W<'a> {
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
            const OFFSET: u8 = 23;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FILTERVAL0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTERVAL0W<'a> {
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
    #[doc = r" Proxy"]
    pub struct _FILTERVAL1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTERVAL1W<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
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
        #[doc = "Bit 0 - Non-Recoverable State 0 Output Enable"]
        #[inline(always)]
        pub fn nre0(&self) -> NRE0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRE0R { bits }
        }
        #[doc = "Bit 1 - Non-Recoverable State 1 Output Enable"]
        #[inline(always)]
        pub fn nre1(&self) -> NRE1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRE1R { bits }
        }
        #[doc = "Bit 2 - Non-Recoverable State 2 Output Enable"]
        #[inline(always)]
        pub fn nre2(&self) -> NRE2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRE2R { bits }
        }
        #[doc = "Bit 3 - Non-Recoverable State 3 Output Enable"]
        #[inline(always)]
        pub fn nre3(&self) -> NRE3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRE3R { bits }
        }
        #[doc = "Bit 4 - Non-Recoverable State 4 Output Enable"]
        #[inline(always)]
        pub fn nre4(&self) -> NRE4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRE4R { bits }
        }
        #[doc = "Bit 5 - Non-Recoverable State 5 Output Enable"]
        #[inline(always)]
        pub fn nre5(&self) -> NRE5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRE5R { bits }
        }
        #[doc = "Bit 6 - Non-Recoverable State 6 Output Enable"]
        #[inline(always)]
        pub fn nre6(&self) -> NRE6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRE6R { bits }
        }
        #[doc = "Bit 7 - Non-Recoverable State 7 Output Enable"]
        #[inline(always)]
        pub fn nre7(&self) -> NRE7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRE7R { bits }
        }
        #[doc = "Bit 8 - Non-Recoverable State 0 Output Value"]
        #[inline(always)]
        pub fn nrv0(&self) -> NRV0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRV0R { bits }
        }
        #[doc = "Bit 9 - Non-Recoverable State 1 Output Value"]
        #[inline(always)]
        pub fn nrv1(&self) -> NRV1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRV1R { bits }
        }
        #[doc = "Bit 10 - Non-Recoverable State 2 Output Value"]
        #[inline(always)]
        pub fn nrv2(&self) -> NRV2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRV2R { bits }
        }
        #[doc = "Bit 11 - Non-Recoverable State 3 Output Value"]
        #[inline(always)]
        pub fn nrv3(&self) -> NRV3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRV3R { bits }
        }
        #[doc = "Bit 12 - Non-Recoverable State 4 Output Value"]
        #[inline(always)]
        pub fn nrv4(&self) -> NRV4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRV4R { bits }
        }
        #[doc = "Bit 13 - Non-Recoverable State 5 Output Value"]
        #[inline(always)]
        pub fn nrv5(&self) -> NRV5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRV5R { bits }
        }
        #[doc = "Bit 14 - Non-Recoverable State 6 Output Value"]
        #[inline(always)]
        pub fn nrv6(&self) -> NRV6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRV6R { bits }
        }
        #[doc = "Bit 15 - Non-Recoverable State 7 Output Value"]
        #[inline(always)]
        pub fn nrv7(&self) -> NRV7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NRV7R { bits }
        }
        #[doc = "Bit 16 - Output Waveform 0 Inversion"]
        #[inline(always)]
        pub fn inven0(&self) -> INVEN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INVEN0R { bits }
        }
        #[doc = "Bit 17 - Output Waveform 1 Inversion"]
        #[inline(always)]
        pub fn inven1(&self) -> INVEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INVEN1R { bits }
        }
        #[doc = "Bit 18 - Output Waveform 2 Inversion"]
        #[inline(always)]
        pub fn inven2(&self) -> INVEN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INVEN2R { bits }
        }
        #[doc = "Bit 19 - Output Waveform 3 Inversion"]
        #[inline(always)]
        pub fn inven3(&self) -> INVEN3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INVEN3R { bits }
        }
        #[doc = "Bit 20 - Output Waveform 4 Inversion"]
        #[inline(always)]
        pub fn inven4(&self) -> INVEN4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INVEN4R { bits }
        }
        #[doc = "Bit 21 - Output Waveform 5 Inversion"]
        #[inline(always)]
        pub fn inven5(&self) -> INVEN5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INVEN5R { bits }
        }
        #[doc = "Bit 22 - Output Waveform 6 Inversion"]
        #[inline(always)]
        pub fn inven6(&self) -> INVEN6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INVEN6R { bits }
        }
        #[doc = "Bit 23 - Output Waveform 7 Inversion"]
        #[inline(always)]
        pub fn inven7(&self) -> INVEN7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            INVEN7R { bits }
        }
        #[doc = "Bits 24:27 - Non-Recoverable Fault Input 0 Filter Value"]
        #[inline(always)]
        pub fn filterval0(&self) -> FILTERVAL0R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            FILTERVAL0R { bits }
        }
        #[doc = "Bits 28:31 - Non-Recoverable Fault Input 1 Filter Value"]
        #[inline(always)]
        pub fn filterval1(&self) -> FILTERVAL1R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            FILTERVAL1R { bits }
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
        #[doc = "Bit 0 - Non-Recoverable State 0 Output Enable"]
        #[inline(always)]
        pub fn nre0(&mut self) -> _NRE0W {
            _NRE0W { w: self }
        }
        #[doc = "Bit 1 - Non-Recoverable State 1 Output Enable"]
        #[inline(always)]
        pub fn nre1(&mut self) -> _NRE1W {
            _NRE1W { w: self }
        }
        #[doc = "Bit 2 - Non-Recoverable State 2 Output Enable"]
        #[inline(always)]
        pub fn nre2(&mut self) -> _NRE2W {
            _NRE2W { w: self }
        }
        #[doc = "Bit 3 - Non-Recoverable State 3 Output Enable"]
        #[inline(always)]
        pub fn nre3(&mut self) -> _NRE3W {
            _NRE3W { w: self }
        }
        #[doc = "Bit 4 - Non-Recoverable State 4 Output Enable"]
        #[inline(always)]
        pub fn nre4(&mut self) -> _NRE4W {
            _NRE4W { w: self }
        }
        #[doc = "Bit 5 - Non-Recoverable State 5 Output Enable"]
        #[inline(always)]
        pub fn nre5(&mut self) -> _NRE5W {
            _NRE5W { w: self }
        }
        #[doc = "Bit 6 - Non-Recoverable State 6 Output Enable"]
        #[inline(always)]
        pub fn nre6(&mut self) -> _NRE6W {
            _NRE6W { w: self }
        }
        #[doc = "Bit 7 - Non-Recoverable State 7 Output Enable"]
        #[inline(always)]
        pub fn nre7(&mut self) -> _NRE7W {
            _NRE7W { w: self }
        }
        #[doc = "Bit 8 - Non-Recoverable State 0 Output Value"]
        #[inline(always)]
        pub fn nrv0(&mut self) -> _NRV0W {
            _NRV0W { w: self }
        }
        #[doc = "Bit 9 - Non-Recoverable State 1 Output Value"]
        #[inline(always)]
        pub fn nrv1(&mut self) -> _NRV1W {
            _NRV1W { w: self }
        }
        #[doc = "Bit 10 - Non-Recoverable State 2 Output Value"]
        #[inline(always)]
        pub fn nrv2(&mut self) -> _NRV2W {
            _NRV2W { w: self }
        }
        #[doc = "Bit 11 - Non-Recoverable State 3 Output Value"]
        #[inline(always)]
        pub fn nrv3(&mut self) -> _NRV3W {
            _NRV3W { w: self }
        }
        #[doc = "Bit 12 - Non-Recoverable State 4 Output Value"]
        #[inline(always)]
        pub fn nrv4(&mut self) -> _NRV4W {
            _NRV4W { w: self }
        }
        #[doc = "Bit 13 - Non-Recoverable State 5 Output Value"]
        #[inline(always)]
        pub fn nrv5(&mut self) -> _NRV5W {
            _NRV5W { w: self }
        }
        #[doc = "Bit 14 - Non-Recoverable State 6 Output Value"]
        #[inline(always)]
        pub fn nrv6(&mut self) -> _NRV6W {
            _NRV6W { w: self }
        }
        #[doc = "Bit 15 - Non-Recoverable State 7 Output Value"]
        #[inline(always)]
        pub fn nrv7(&mut self) -> _NRV7W {
            _NRV7W { w: self }
        }
        #[doc = "Bit 16 - Output Waveform 0 Inversion"]
        #[inline(always)]
        pub fn inven0(&mut self) -> _INVEN0W {
            _INVEN0W { w: self }
        }
        #[doc = "Bit 17 - Output Waveform 1 Inversion"]
        #[inline(always)]
        pub fn inven1(&mut self) -> _INVEN1W {
            _INVEN1W { w: self }
        }
        #[doc = "Bit 18 - Output Waveform 2 Inversion"]
        #[inline(always)]
        pub fn inven2(&mut self) -> _INVEN2W {
            _INVEN2W { w: self }
        }
        #[doc = "Bit 19 - Output Waveform 3 Inversion"]
        #[inline(always)]
        pub fn inven3(&mut self) -> _INVEN3W {
            _INVEN3W { w: self }
        }
        #[doc = "Bit 20 - Output Waveform 4 Inversion"]
        #[inline(always)]
        pub fn inven4(&mut self) -> _INVEN4W {
            _INVEN4W { w: self }
        }
        #[doc = "Bit 21 - Output Waveform 5 Inversion"]
        #[inline(always)]
        pub fn inven5(&mut self) -> _INVEN5W {
            _INVEN5W { w: self }
        }
        #[doc = "Bit 22 - Output Waveform 6 Inversion"]
        #[inline(always)]
        pub fn inven6(&mut self) -> _INVEN6W {
            _INVEN6W { w: self }
        }
        #[doc = "Bit 23 - Output Waveform 7 Inversion"]
        #[inline(always)]
        pub fn inven7(&mut self) -> _INVEN7W {
            _INVEN7W { w: self }
        }
        #[doc = "Bits 24:27 - Non-Recoverable Fault Input 0 Filter Value"]
        #[inline(always)]
        pub fn filterval0(&mut self) -> _FILTERVAL0W {
            _FILTERVAL0W { w: self }
        }
        #[doc = "Bits 28:31 - Non-Recoverable Fault Input 1 Filter Value"]
        #[inline(always)]
        pub fn filterval1(&mut self) -> _FILTERVAL1W {
            _FILTERVAL1W { w: self }
        }
    }
}
#[doc = "Event Control"]
pub struct EVCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Event Control"]
pub mod evctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
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
    #[doc = "Possible values of the field `EVACT0`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EVACT0R {
        #[doc = "undocumented"]
        OFF,
        #[doc = "undocumented"]
        RETRIGGER,
        #[doc = "undocumented"]
        COUNTEV,
        #[doc = "undocumented"]
        START,
        #[doc = "undocumented"]
        INC,
        #[doc = "undocumented"]
        COUNT,
        #[doc = "undocumented"]
        FAULT,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl EVACT0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                EVACT0R::OFF => 0,
                EVACT0R::RETRIGGER => 1,
                EVACT0R::COUNTEV => 2,
                EVACT0R::START => 3,
                EVACT0R::INC => 4,
                EVACT0R::COUNT => 5,
                EVACT0R::FAULT => 7,
                EVACT0R::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> EVACT0R {
            match value {
                0 => EVACT0R::OFF,
                1 => EVACT0R::RETRIGGER,
                2 => EVACT0R::COUNTEV,
                3 => EVACT0R::START,
                4 => EVACT0R::INC,
                5 => EVACT0R::COUNT,
                7 => EVACT0R::FAULT,
                i => EVACT0R::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `OFF`"]
        #[inline(always)]
        pub fn is_off(&self) -> bool {
            *self == EVACT0R::OFF
        }
        #[doc = "Checks if the value of the field is `RETRIGGER`"]
        #[inline(always)]
        pub fn is_retrigger(&self) -> bool {
            *self == EVACT0R::RETRIGGER
        }
        #[doc = "Checks if the value of the field is `COUNTEV`"]
        #[inline(always)]
        pub fn is_countev(&self) -> bool {
            *self == EVACT0R::COUNTEV
        }
        #[doc = "Checks if the value of the field is `START`"]
        #[inline(always)]
        pub fn is_start(&self) -> bool {
            *self == EVACT0R::START
        }
        #[doc = "Checks if the value of the field is `INC`"]
        #[inline(always)]
        pub fn is_inc(&self) -> bool {
            *self == EVACT0R::INC
        }
        #[doc = "Checks if the value of the field is `COUNT`"]
        #[inline(always)]
        pub fn is_count(&self) -> bool {
            *self == EVACT0R::COUNT
        }
        #[doc = "Checks if the value of the field is `FAULT`"]
        #[inline(always)]
        pub fn is_fault(&self) -> bool {
            *self == EVACT0R::FAULT
        }
    }
    #[doc = "Possible values of the field `EVACT1`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EVACT1R {
        #[doc = "undocumented"]
        OFF,
        #[doc = "undocumented"]
        RETRIGGER,
        #[doc = "undocumented"]
        DIR,
        #[doc = "undocumented"]
        STOP,
        #[doc = "undocumented"]
        DEC,
        #[doc = "undocumented"]
        PPW,
        #[doc = "undocumented"]
        PWP,
        #[doc = "undocumented"]
        FAULT,
    }
    impl EVACT1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                EVACT1R::OFF => 0,
                EVACT1R::RETRIGGER => 1,
                EVACT1R::DIR => 2,
                EVACT1R::STOP => 3,
                EVACT1R::DEC => 4,
                EVACT1R::PPW => 5,
                EVACT1R::PWP => 6,
                EVACT1R::FAULT => 7,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> EVACT1R {
            match value {
                0 => EVACT1R::OFF,
                1 => EVACT1R::RETRIGGER,
                2 => EVACT1R::DIR,
                3 => EVACT1R::STOP,
                4 => EVACT1R::DEC,
                5 => EVACT1R::PPW,
                6 => EVACT1R::PWP,
                7 => EVACT1R::FAULT,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `OFF`"]
        #[inline(always)]
        pub fn is_off(&self) -> bool {
            *self == EVACT1R::OFF
        }
        #[doc = "Checks if the value of the field is `RETRIGGER`"]
        #[inline(always)]
        pub fn is_retrigger(&self) -> bool {
            *self == EVACT1R::RETRIGGER
        }
        #[doc = "Checks if the value of the field is `DIR`"]
        #[inline(always)]
        pub fn is_dir(&self) -> bool {
            *self == EVACT1R::DIR
        }
        #[doc = "Checks if the value of the field is `STOP`"]
        #[inline(always)]
        pub fn is_stop(&self) -> bool {
            *self == EVACT1R::STOP
        }
        #[doc = "Checks if the value of the field is `DEC`"]
        #[inline(always)]
        pub fn is_dec(&self) -> bool {
            *self == EVACT1R::DEC
        }
        #[doc = "Checks if the value of the field is `PPW`"]
        #[inline(always)]
        pub fn is_ppw(&self) -> bool {
            *self == EVACT1R::PPW
        }
        #[doc = "Checks if the value of the field is `PWP`"]
        #[inline(always)]
        pub fn is_pwp(&self) -> bool {
            *self == EVACT1R::PWP
        }
        #[doc = "Checks if the value of the field is `FAULT`"]
        #[inline(always)]
        pub fn is_fault(&self) -> bool {
            *self == EVACT1R::FAULT
        }
    }
    #[doc = "Possible values of the field `CNTSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CNTSELR {
        #[doc = "undocumented"]
        START,
        #[doc = "undocumented"]
        END,
        #[doc = "undocumented"]
        BETWEEN,
        #[doc = "undocumented"]
        BOUNDARY,
    }
    impl CNTSELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CNTSELR::START => 0,
                CNTSELR::END => 1,
                CNTSELR::BETWEEN => 2,
                CNTSELR::BOUNDARY => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CNTSELR {
            match value {
                0 => CNTSELR::START,
                1 => CNTSELR::END,
                2 => CNTSELR::BETWEEN,
                3 => CNTSELR::BOUNDARY,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `START`"]
        #[inline(always)]
        pub fn is_start(&self) -> bool {
            *self == CNTSELR::START
        }
        #[doc = "Checks if the value of the field is `END`"]
        #[inline(always)]
        pub fn is_end(&self) -> bool {
            *self == CNTSELR::END
        }
        #[doc = "Checks if the value of the field is `BETWEEN`"]
        #[inline(always)]
        pub fn is_between(&self) -> bool {
            *self == CNTSELR::BETWEEN
        }
        #[doc = "Checks if the value of the field is `BOUNDARY`"]
        #[inline(always)]
        pub fn is_boundary(&self) -> bool {
            *self == CNTSELR::BOUNDARY
        }
    }
    #[doc = r" Value of the field"]
    pub struct OVFEOR {
        bits: bool,
    }
    impl OVFEOR {
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
    pub struct TRGEOR {
        bits: bool,
    }
    impl TRGEOR {
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
    pub struct CNTEOR {
        bits: bool,
    }
    impl CNTEOR {
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
    pub struct TCINV0R {
        bits: bool,
    }
    impl TCINV0R {
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
    pub struct TCINV1R {
        bits: bool,
    }
    impl TCINV1R {
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
    pub struct TCEI0R {
        bits: bool,
    }
    impl TCEI0R {
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
    pub struct TCEI1R {
        bits: bool,
    }
    impl TCEI1R {
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
    pub struct MCEI0R {
        bits: bool,
    }
    impl MCEI0R {
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
    pub struct MCEI1R {
        bits: bool,
    }
    impl MCEI1R {
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
    pub struct MCEI2R {
        bits: bool,
    }
    impl MCEI2R {
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
    pub struct MCEI3R {
        bits: bool,
    }
    impl MCEI3R {
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
    pub struct MCEO0R {
        bits: bool,
    }
    impl MCEO0R {
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
    pub struct MCEO1R {
        bits: bool,
    }
    impl MCEO1R {
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
    pub struct MCEO2R {
        bits: bool,
    }
    impl MCEO2R {
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
    pub struct MCEO3R {
        bits: bool,
    }
    impl MCEO3R {
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
    #[doc = "Values that can be written to the field `EVACT0`"]
    pub enum EVACT0W {
        #[doc = "`0`"]
        OFF,
        #[doc = "`1`"]
        RETRIGGER,
        #[doc = "`10`"]
        COUNTEV,
        #[doc = "`11`"]
        START,
        #[doc = "`100`"]
        INC,
        #[doc = "`101`"]
        COUNT,
        #[doc = "`111`"]
        FAULT,
    }
    impl EVACT0W {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                EVACT0W::OFF => 0,
                EVACT0W::RETRIGGER => 1,
                EVACT0W::COUNTEV => 2,
                EVACT0W::START => 3,
                EVACT0W::INC => 4,
                EVACT0W::COUNT => 5,
                EVACT0W::FAULT => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _EVACT0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVACT0W<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: EVACT0W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn off(self) -> &'a mut W {
            self.variant(EVACT0W::OFF)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn retrigger(self) -> &'a mut W {
            self.variant(EVACT0W::RETRIGGER)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn countev(self) -> &'a mut W {
            self.variant(EVACT0W::COUNTEV)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn start(self) -> &'a mut W {
            self.variant(EVACT0W::START)
        }
        #[doc = "`100`"]
        #[inline(always)]
        pub fn inc(self) -> &'a mut W {
            self.variant(EVACT0W::INC)
        }
        #[doc = "`101`"]
        #[inline(always)]
        pub fn count(self) -> &'a mut W {
            self.variant(EVACT0W::COUNT)
        }
        #[doc = "`111`"]
        #[inline(always)]
        pub fn fault(self) -> &'a mut W {
            self.variant(EVACT0W::FAULT)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `EVACT1`"]
    pub enum EVACT1W {
        #[doc = "`0`"]
        OFF,
        #[doc = "`1`"]
        RETRIGGER,
        #[doc = "`10`"]
        DIR,
        #[doc = "`11`"]
        STOP,
        #[doc = "`100`"]
        DEC,
        #[doc = "`101`"]
        PPW,
        #[doc = "`110`"]
        PWP,
        #[doc = "`111`"]
        FAULT,
    }
    impl EVACT1W {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                EVACT1W::OFF => 0,
                EVACT1W::RETRIGGER => 1,
                EVACT1W::DIR => 2,
                EVACT1W::STOP => 3,
                EVACT1W::DEC => 4,
                EVACT1W::PPW => 5,
                EVACT1W::PWP => 6,
                EVACT1W::FAULT => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _EVACT1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVACT1W<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: EVACT1W) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn off(self) -> &'a mut W {
            self.variant(EVACT1W::OFF)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn retrigger(self) -> &'a mut W {
            self.variant(EVACT1W::RETRIGGER)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn dir(self) -> &'a mut W {
            self.variant(EVACT1W::DIR)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn stop(self) -> &'a mut W {
            self.variant(EVACT1W::STOP)
        }
        #[doc = "`100`"]
        #[inline(always)]
        pub fn dec(self) -> &'a mut W {
            self.variant(EVACT1W::DEC)
        }
        #[doc = "`101`"]
        #[inline(always)]
        pub fn ppw(self) -> &'a mut W {
            self.variant(EVACT1W::PPW)
        }
        #[doc = "`110`"]
        #[inline(always)]
        pub fn pwp(self) -> &'a mut W {
            self.variant(EVACT1W::PWP)
        }
        #[doc = "`111`"]
        #[inline(always)]
        pub fn fault(self) -> &'a mut W {
            self.variant(EVACT1W::FAULT)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CNTSEL`"]
    pub enum CNTSELW {
        #[doc = "`0`"]
        START,
        #[doc = "`1`"]
        END,
        #[doc = "`10`"]
        BETWEEN,
        #[doc = "`11`"]
        BOUNDARY,
    }
    impl CNTSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CNTSELW::START => 0,
                CNTSELW::END => 1,
                CNTSELW::BETWEEN => 2,
                CNTSELW::BOUNDARY => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CNTSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CNTSELW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn start(self) -> &'a mut W {
            self.variant(CNTSELW::START)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn end(self) -> &'a mut W {
            self.variant(CNTSELW::END)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn between(self) -> &'a mut W {
            self.variant(CNTSELW::BETWEEN)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn boundary(self) -> &'a mut W {
            self.variant(CNTSELW::BOUNDARY)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _OVFEOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVFEOW<'a> {
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
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _TRGEOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TRGEOW<'a> {
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
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CNTEOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTEOW<'a> {
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
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _TCINV0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCINV0W<'a> {
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
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _TCINV1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCINV1W<'a> {
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
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _TCEI0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCEI0W<'a> {
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
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _TCEI1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCEI1W<'a> {
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
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MCEI0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCEI0W<'a> {
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
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MCEI1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCEI1W<'a> {
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
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MCEI2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCEI2W<'a> {
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
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MCEI3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCEI3W<'a> {
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
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MCEO0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCEO0W<'a> {
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
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MCEO1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCEO1W<'a> {
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
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MCEO2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCEO2W<'a> {
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
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MCEO3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCEO3W<'a> {
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
            const OFFSET: u8 = 27;
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
        #[doc = "Bits 0:2 - Timer/counter Input Event0 Action"]
        #[inline(always)]
        pub fn evact0(&self) -> EVACT0R {
            EVACT0R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 3:5 - Timer/counter Input Event1 Action"]
        #[inline(always)]
        pub fn evact1(&self) -> EVACT1R {
            EVACT1R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 6:7 - Timer/counter Output Event Mode"]
        #[inline(always)]
        pub fn cntsel(&self) -> CNTSELR {
            CNTSELR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
        #[inline(always)]
        pub fn ovfeo(&self) -> OVFEOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVFEOR { bits }
        }
        #[doc = "Bit 9 - Retrigger Output Event Enable"]
        #[inline(always)]
        pub fn trgeo(&self) -> TRGEOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TRGEOR { bits }
        }
        #[doc = "Bit 10 - Timer/counter Output Event Enable"]
        #[inline(always)]
        pub fn cnteo(&self) -> CNTEOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CNTEOR { bits }
        }
        #[doc = "Bit 12 - Inverted Event 0 Input Enable"]
        #[inline(always)]
        pub fn tcinv0(&self) -> TCINV0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TCINV0R { bits }
        }
        #[doc = "Bit 13 - Inverted Event 1 Input Enable"]
        #[inline(always)]
        pub fn tcinv1(&self) -> TCINV1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TCINV1R { bits }
        }
        #[doc = "Bit 14 - Timer/counter Event 0 Input Enable"]
        #[inline(always)]
        pub fn tcei0(&self) -> TCEI0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TCEI0R { bits }
        }
        #[doc = "Bit 15 - Timer/counter Event 1 Input Enable"]
        #[inline(always)]
        pub fn tcei1(&self) -> TCEI1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TCEI1R { bits }
        }
        #[doc = "Bit 16 - Match or Capture Channel 0 Event Input Enable"]
        #[inline(always)]
        pub fn mcei0(&self) -> MCEI0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MCEI0R { bits }
        }
        #[doc = "Bit 17 - Match or Capture Channel 1 Event Input Enable"]
        #[inline(always)]
        pub fn mcei1(&self) -> MCEI1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MCEI1R { bits }
        }
        #[doc = "Bit 18 - Match or Capture Channel 2 Event Input Enable"]
        #[inline(always)]
        pub fn mcei2(&self) -> MCEI2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MCEI2R { bits }
        }
        #[doc = "Bit 19 - Match or Capture Channel 3 Event Input Enable"]
        #[inline(always)]
        pub fn mcei3(&self) -> MCEI3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MCEI3R { bits }
        }
        #[doc = "Bit 24 - Match or Capture Channel 0 Event Output Enable"]
        #[inline(always)]
        pub fn mceo0(&self) -> MCEO0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MCEO0R { bits }
        }
        #[doc = "Bit 25 - Match or Capture Channel 1 Event Output Enable"]
        #[inline(always)]
        pub fn mceo1(&self) -> MCEO1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MCEO1R { bits }
        }
        #[doc = "Bit 26 - Match or Capture Channel 2 Event Output Enable"]
        #[inline(always)]
        pub fn mceo2(&self) -> MCEO2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MCEO2R { bits }
        }
        #[doc = "Bit 27 - Match or Capture Channel 3 Event Output Enable"]
        #[inline(always)]
        pub fn mceo3(&self) -> MCEO3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MCEO3R { bits }
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
        #[doc = "Bits 0:2 - Timer/counter Input Event0 Action"]
        #[inline(always)]
        pub fn evact0(&mut self) -> _EVACT0W {
            _EVACT0W { w: self }
        }
        #[doc = "Bits 3:5 - Timer/counter Input Event1 Action"]
        #[inline(always)]
        pub fn evact1(&mut self) -> _EVACT1W {
            _EVACT1W { w: self }
        }
        #[doc = "Bits 6:7 - Timer/counter Output Event Mode"]
        #[inline(always)]
        pub fn cntsel(&mut self) -> _CNTSELW {
            _CNTSELW { w: self }
        }
        #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
        #[inline(always)]
        pub fn ovfeo(&mut self) -> _OVFEOW {
            _OVFEOW { w: self }
        }
        #[doc = "Bit 9 - Retrigger Output Event Enable"]
        #[inline(always)]
        pub fn trgeo(&mut self) -> _TRGEOW {
            _TRGEOW { w: self }
        }
        #[doc = "Bit 10 - Timer/counter Output Event Enable"]
        #[inline(always)]
        pub fn cnteo(&mut self) -> _CNTEOW {
            _CNTEOW { w: self }
        }
        #[doc = "Bit 12 - Inverted Event 0 Input Enable"]
        #[inline(always)]
        pub fn tcinv0(&mut self) -> _TCINV0W {
            _TCINV0W { w: self }
        }
        #[doc = "Bit 13 - Inverted Event 1 Input Enable"]
        #[inline(always)]
        pub fn tcinv1(&mut self) -> _TCINV1W {
            _TCINV1W { w: self }
        }
        #[doc = "Bit 14 - Timer/counter Event 0 Input Enable"]
        #[inline(always)]
        pub fn tcei0(&mut self) -> _TCEI0W {
            _TCEI0W { w: self }
        }
        #[doc = "Bit 15 - Timer/counter Event 1 Input Enable"]
        #[inline(always)]
        pub fn tcei1(&mut self) -> _TCEI1W {
            _TCEI1W { w: self }
        }
        #[doc = "Bit 16 - Match or Capture Channel 0 Event Input Enable"]
        #[inline(always)]
        pub fn mcei0(&mut self) -> _MCEI0W {
            _MCEI0W { w: self }
        }
        #[doc = "Bit 17 - Match or Capture Channel 1 Event Input Enable"]
        #[inline(always)]
        pub fn mcei1(&mut self) -> _MCEI1W {
            _MCEI1W { w: self }
        }
        #[doc = "Bit 18 - Match or Capture Channel 2 Event Input Enable"]
        #[inline(always)]
        pub fn mcei2(&mut self) -> _MCEI2W {
            _MCEI2W { w: self }
        }
        #[doc = "Bit 19 - Match or Capture Channel 3 Event Input Enable"]
        #[inline(always)]
        pub fn mcei3(&mut self) -> _MCEI3W {
            _MCEI3W { w: self }
        }
        #[doc = "Bit 24 - Match or Capture Channel 0 Event Output Enable"]
        #[inline(always)]
        pub fn mceo0(&mut self) -> _MCEO0W {
            _MCEO0W { w: self }
        }
        #[doc = "Bit 25 - Match or Capture Channel 1 Event Output Enable"]
        #[inline(always)]
        pub fn mceo1(&mut self) -> _MCEO1W {
            _MCEO1W { w: self }
        }
        #[doc = "Bit 26 - Match or Capture Channel 2 Event Output Enable"]
        #[inline(always)]
        pub fn mceo2(&mut self) -> _MCEO2W {
            _MCEO2W { w: self }
        }
        #[doc = "Bit 27 - Match or Capture Channel 3 Event Output Enable"]
        #[inline(always)]
        pub fn mceo3(&mut self) -> _MCEO3W {
            _MCEO3W { w: self }
        }
    }
}
#[doc = "Recoverable FaultA Configuration"]
pub struct FCTRLA {
    register: VolatileCell<u32>,
}
#[doc = "Recoverable FaultA Configuration"]
pub mod fctrla {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::FCTRLA {
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
    #[doc = "Possible values of the field `SRC`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SRCR {
        #[doc = "undocumented"]
        DISABLE,
        #[doc = "undocumented"]
        ENABLE,
        #[doc = "undocumented"]
        INVERT,
        #[doc = "undocumented"]
        ALTFAULT,
    }
    impl SRCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SRCR::DISABLE => 0,
                SRCR::ENABLE => 1,
                SRCR::INVERT => 2,
                SRCR::ALTFAULT => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SRCR {
            match value {
                0 => SRCR::DISABLE,
                1 => SRCR::ENABLE,
                2 => SRCR::INVERT,
                3 => SRCR::ALTFAULT,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == SRCR::DISABLE
        }
        #[doc = "Checks if the value of the field is `ENABLE`"]
        #[inline(always)]
        pub fn is_enable(&self) -> bool {
            *self == SRCR::ENABLE
        }
        #[doc = "Checks if the value of the field is `INVERT`"]
        #[inline(always)]
        pub fn is_invert(&self) -> bool {
            *self == SRCR::INVERT
        }
        #[doc = "Checks if the value of the field is `ALTFAULT`"]
        #[inline(always)]
        pub fn is_altfault(&self) -> bool {
            *self == SRCR::ALTFAULT
        }
    }
    #[doc = r" Value of the field"]
    pub struct KEEPR {
        bits: bool,
    }
    impl KEEPR {
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
    pub struct QUALR {
        bits: bool,
    }
    impl QUALR {
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
    #[doc = "Possible values of the field `BLANK`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum BLANKR {
        #[doc = "undocumented"]
        DISABLE,
        #[doc = "undocumented"]
        RISE,
        #[doc = "undocumented"]
        FALL,
        #[doc = "undocumented"]
        BOTH,
    }
    impl BLANKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                BLANKR::DISABLE => 0,
                BLANKR::RISE => 1,
                BLANKR::FALL => 2,
                BLANKR::BOTH => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> BLANKR {
            match value {
                0 => BLANKR::DISABLE,
                1 => BLANKR::RISE,
                2 => BLANKR::FALL,
                3 => BLANKR::BOTH,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == BLANKR::DISABLE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == BLANKR::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == BLANKR::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == BLANKR::BOTH
        }
    }
    #[doc = r" Value of the field"]
    pub struct RESTARTR {
        bits: bool,
    }
    impl RESTARTR {
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
    #[doc = "Possible values of the field `HALT`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum HALTR {
        #[doc = "undocumented"]
        DISABLE,
        #[doc = "undocumented"]
        HW,
        #[doc = "undocumented"]
        SW,
        #[doc = "undocumented"]
        NR,
    }
    impl HALTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                HALTR::DISABLE => 0,
                HALTR::HW => 1,
                HALTR::SW => 2,
                HALTR::NR => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> HALTR {
            match value {
                0 => HALTR::DISABLE,
                1 => HALTR::HW,
                2 => HALTR::SW,
                3 => HALTR::NR,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == HALTR::DISABLE
        }
        #[doc = "Checks if the value of the field is `HW`"]
        #[inline(always)]
        pub fn is_hw(&self) -> bool {
            *self == HALTR::HW
        }
        #[doc = "Checks if the value of the field is `SW`"]
        #[inline(always)]
        pub fn is_sw(&self) -> bool {
            *self == HALTR::SW
        }
        #[doc = "Checks if the value of the field is `NR`"]
        #[inline(always)]
        pub fn is_nr(&self) -> bool {
            *self == HALTR::NR
        }
    }
    #[doc = "Possible values of the field `CHSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CHSELR {
        #[doc = "undocumented"]
        CC0,
        #[doc = "undocumented"]
        CC1,
        #[doc = "undocumented"]
        CC2,
        #[doc = "undocumented"]
        CC3,
    }
    impl CHSELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CHSELR::CC0 => 0,
                CHSELR::CC1 => 1,
                CHSELR::CC2 => 2,
                CHSELR::CC3 => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CHSELR {
            match value {
                0 => CHSELR::CC0,
                1 => CHSELR::CC1,
                2 => CHSELR::CC2,
                3 => CHSELR::CC3,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `CC0`"]
        #[inline(always)]
        pub fn is_cc0(&self) -> bool {
            *self == CHSELR::CC0
        }
        #[doc = "Checks if the value of the field is `CC1`"]
        #[inline(always)]
        pub fn is_cc1(&self) -> bool {
            *self == CHSELR::CC1
        }
        #[doc = "Checks if the value of the field is `CC2`"]
        #[inline(always)]
        pub fn is_cc2(&self) -> bool {
            *self == CHSELR::CC2
        }
        #[doc = "Checks if the value of the field is `CC3`"]
        #[inline(always)]
        pub fn is_cc3(&self) -> bool {
            *self == CHSELR::CC3
        }
    }
    #[doc = "Possible values of the field `CAPTURE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CAPTURER {
        #[doc = "undocumented"]
        DISABLE,
        #[doc = "undocumented"]
        CAPT,
        #[doc = "undocumented"]
        CAPTMIN,
        #[doc = "undocumented"]
        CAPTMAX,
        #[doc = "undocumented"]
        LOCMIN,
        #[doc = "undocumented"]
        LOCMAX,
        #[doc = "undocumented"]
        DERIV0,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CAPTURER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CAPTURER::DISABLE => 0,
                CAPTURER::CAPT => 1,
                CAPTURER::CAPTMIN => 2,
                CAPTURER::CAPTMAX => 3,
                CAPTURER::LOCMIN => 4,
                CAPTURER::LOCMAX => 5,
                CAPTURER::DERIV0 => 6,
                CAPTURER::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CAPTURER {
            match value {
                0 => CAPTURER::DISABLE,
                1 => CAPTURER::CAPT,
                2 => CAPTURER::CAPTMIN,
                3 => CAPTURER::CAPTMAX,
                4 => CAPTURER::LOCMIN,
                5 => CAPTURER::LOCMAX,
                6 => CAPTURER::DERIV0,
                i => CAPTURER::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == CAPTURER::DISABLE
        }
        #[doc = "Checks if the value of the field is `CAPT`"]
        #[inline(always)]
        pub fn is_capt(&self) -> bool {
            *self == CAPTURER::CAPT
        }
        #[doc = "Checks if the value of the field is `CAPTMIN`"]
        #[inline(always)]
        pub fn is_captmin(&self) -> bool {
            *self == CAPTURER::CAPTMIN
        }
        #[doc = "Checks if the value of the field is `CAPTMAX`"]
        #[inline(always)]
        pub fn is_captmax(&self) -> bool {
            *self == CAPTURER::CAPTMAX
        }
        #[doc = "Checks if the value of the field is `LOCMIN`"]
        #[inline(always)]
        pub fn is_locmin(&self) -> bool {
            *self == CAPTURER::LOCMIN
        }
        #[doc = "Checks if the value of the field is `LOCMAX`"]
        #[inline(always)]
        pub fn is_locmax(&self) -> bool {
            *self == CAPTURER::LOCMAX
        }
        #[doc = "Checks if the value of the field is `DERIV0`"]
        #[inline(always)]
        pub fn is_deriv0(&self) -> bool {
            *self == CAPTURER::DERIV0
        }
    }
    #[doc = r" Value of the field"]
    pub struct BLANKVALR {
        bits: u8,
    }
    impl BLANKVALR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTERVALR {
        bits: u8,
    }
    impl FILTERVALR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = "Values that can be written to the field `SRC`"]
    pub enum SRCW {
        #[doc = "`0`"]
        DISABLE,
        #[doc = "`1`"]
        ENABLE,
        #[doc = "`10`"]
        INVERT,
        #[doc = "`11`"]
        ALTFAULT,
    }
    impl SRCW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SRCW::DISABLE => 0,
                SRCW::ENABLE => 1,
                SRCW::INVERT => 2,
                SRCW::ALTFAULT => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SRCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SRCW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SRCW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(SRCW::DISABLE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn enable(self) -> &'a mut W {
            self.variant(SRCW::ENABLE)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn invert(self) -> &'a mut W {
            self.variant(SRCW::INVERT)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn altfault(self) -> &'a mut W {
            self.variant(SRCW::ALTFAULT)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _KEEPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _KEEPW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _QUALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _QUALW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `BLANK`"]
    pub enum BLANKW {
        #[doc = "`0`"]
        DISABLE,
        #[doc = "`1`"]
        RISE,
        #[doc = "`10`"]
        FALL,
        #[doc = "`11`"]
        BOTH,
    }
    impl BLANKW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                BLANKW::DISABLE => 0,
                BLANKW::RISE => 1,
                BLANKW::FALL => 2,
                BLANKW::BOTH => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _BLANKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BLANKW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: BLANKW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(BLANKW::DISABLE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(BLANKW::RISE)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(BLANKW::FALL)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(BLANKW::BOTH)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _RESTARTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESTARTW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `HALT`"]
    pub enum HALTW {
        #[doc = "`0`"]
        DISABLE,
        #[doc = "`1`"]
        HW,
        #[doc = "`10`"]
        SW,
        #[doc = "`11`"]
        NR,
    }
    impl HALTW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                HALTW::DISABLE => 0,
                HALTW::HW => 1,
                HALTW::SW => 2,
                HALTW::NR => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _HALTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HALTW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: HALTW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(HALTW::DISABLE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn hw(self) -> &'a mut W {
            self.variant(HALTW::HW)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn sw(self) -> &'a mut W {
            self.variant(HALTW::SW)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn nr(self) -> &'a mut W {
            self.variant(HALTW::NR)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CHSEL`"]
    pub enum CHSELW {
        #[doc = "`0`"]
        CC0,
        #[doc = "`1`"]
        CC1,
        #[doc = "`10`"]
        CC2,
        #[doc = "`11`"]
        CC3,
    }
    impl CHSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CHSELW::CC0 => 0,
                CHSELW::CC1 => 1,
                CHSELW::CC2 => 2,
                CHSELW::CC3 => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CHSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CHSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CHSELW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn cc0(self) -> &'a mut W {
            self.variant(CHSELW::CC0)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn cc1(self) -> &'a mut W {
            self.variant(CHSELW::CC1)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn cc2(self) -> &'a mut W {
            self.variant(CHSELW::CC2)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn cc3(self) -> &'a mut W {
            self.variant(CHSELW::CC3)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CAPTURE`"]
    pub enum CAPTUREW {
        #[doc = "`0`"]
        DISABLE,
        #[doc = "`1`"]
        CAPT,
        #[doc = "`10`"]
        CAPTMIN,
        #[doc = "`11`"]
        CAPTMAX,
        #[doc = "`100`"]
        LOCMIN,
        #[doc = "`101`"]
        LOCMAX,
        #[doc = "`110`"]
        DERIV0,
    }
    impl CAPTUREW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CAPTUREW::DISABLE => 0,
                CAPTUREW::CAPT => 1,
                CAPTUREW::CAPTMIN => 2,
                CAPTUREW::CAPTMAX => 3,
                CAPTUREW::LOCMIN => 4,
                CAPTUREW::LOCMAX => 5,
                CAPTUREW::DERIV0 => 6,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CAPTUREW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAPTUREW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CAPTUREW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(CAPTUREW::DISABLE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn capt(self) -> &'a mut W {
            self.variant(CAPTUREW::CAPT)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn captmin(self) -> &'a mut W {
            self.variant(CAPTUREW::CAPTMIN)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn captmax(self) -> &'a mut W {
            self.variant(CAPTUREW::CAPTMAX)
        }
        #[doc = "`100`"]
        #[inline(always)]
        pub fn locmin(self) -> &'a mut W {
            self.variant(CAPTUREW::LOCMIN)
        }
        #[doc = "`101`"]
        #[inline(always)]
        pub fn locmax(self) -> &'a mut W {
            self.variant(CAPTUREW::LOCMAX)
        }
        #[doc = "`110`"]
        #[inline(always)]
        pub fn deriv0(self) -> &'a mut W {
            self.variant(CAPTUREW::DERIV0)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _BLANKVALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BLANKVALW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FILTERVALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTERVALW<'a> {
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
        #[doc = "Bits 0:1 - FaultA Source"]
        #[inline(always)]
        pub fn src(&self) -> SRCR {
            SRCR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 3 - FaultA Keeper"]
        #[inline(always)]
        pub fn keep(&self) -> KEEPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            KEEPR { bits }
        }
        #[doc = "Bit 4 - FaultA Qualification"]
        #[inline(always)]
        pub fn qual(&self) -> QUALR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            QUALR { bits }
        }
        #[doc = "Bits 5:6 - FaultA Blanking Mode"]
        #[inline(always)]
        pub fn blank(&self) -> BLANKR {
            BLANKR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 7 - FaultA Restart"]
        #[inline(always)]
        pub fn restart(&self) -> RESTARTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RESTARTR { bits }
        }
        #[doc = "Bits 8:9 - FaultA Halt Mode"]
        #[inline(always)]
        pub fn halt(&self) -> HALTR {
            HALTR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 10:11 - FaultA Capture Channel"]
        #[inline(always)]
        pub fn chsel(&self) -> CHSELR {
            CHSELR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 12:14 - FaultA Capture Action"]
        #[inline(always)]
        pub fn capture(&self) -> CAPTURER {
            CAPTURER::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 16:23 - FaultA Blanking Time"]
        #[inline(always)]
        pub fn blankval(&self) -> BLANKVALR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            BLANKVALR { bits }
        }
        #[doc = "Bits 24:27 - FaultA Filter Value"]
        #[inline(always)]
        pub fn filterval(&self) -> FILTERVALR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            FILTERVALR { bits }
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
        #[doc = "Bits 0:1 - FaultA Source"]
        #[inline(always)]
        pub fn src(&mut self) -> _SRCW {
            _SRCW { w: self }
        }
        #[doc = "Bit 3 - FaultA Keeper"]
        #[inline(always)]
        pub fn keep(&mut self) -> _KEEPW {
            _KEEPW { w: self }
        }
        #[doc = "Bit 4 - FaultA Qualification"]
        #[inline(always)]
        pub fn qual(&mut self) -> _QUALW {
            _QUALW { w: self }
        }
        #[doc = "Bits 5:6 - FaultA Blanking Mode"]
        #[inline(always)]
        pub fn blank(&mut self) -> _BLANKW {
            _BLANKW { w: self }
        }
        #[doc = "Bit 7 - FaultA Restart"]
        #[inline(always)]
        pub fn restart(&mut self) -> _RESTARTW {
            _RESTARTW { w: self }
        }
        #[doc = "Bits 8:9 - FaultA Halt Mode"]
        #[inline(always)]
        pub fn halt(&mut self) -> _HALTW {
            _HALTW { w: self }
        }
        #[doc = "Bits 10:11 - FaultA Capture Channel"]
        #[inline(always)]
        pub fn chsel(&mut self) -> _CHSELW {
            _CHSELW { w: self }
        }
        #[doc = "Bits 12:14 - FaultA Capture Action"]
        #[inline(always)]
        pub fn capture(&mut self) -> _CAPTUREW {
            _CAPTUREW { w: self }
        }
        #[doc = "Bits 16:23 - FaultA Blanking Time"]
        #[inline(always)]
        pub fn blankval(&mut self) -> _BLANKVALW {
            _BLANKVALW { w: self }
        }
        #[doc = "Bits 24:27 - FaultA Filter Value"]
        #[inline(always)]
        pub fn filterval(&mut self) -> _FILTERVALW {
            _FILTERVALW { w: self }
        }
    }
}
#[doc = "Recoverable FaultB Configuration"]
pub struct FCTRLB {
    register: VolatileCell<u32>,
}
#[doc = "Recoverable FaultB Configuration"]
pub mod fctrlb {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::FCTRLB {
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
    #[doc = "Possible values of the field `SRC`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SRCR {
        #[doc = "undocumented"]
        DISABLE,
        #[doc = "undocumented"]
        ENABLE,
        #[doc = "undocumented"]
        INVERT,
        #[doc = "undocumented"]
        ALTFAULT,
    }
    impl SRCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SRCR::DISABLE => 0,
                SRCR::ENABLE => 1,
                SRCR::INVERT => 2,
                SRCR::ALTFAULT => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SRCR {
            match value {
                0 => SRCR::DISABLE,
                1 => SRCR::ENABLE,
                2 => SRCR::INVERT,
                3 => SRCR::ALTFAULT,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == SRCR::DISABLE
        }
        #[doc = "Checks if the value of the field is `ENABLE`"]
        #[inline(always)]
        pub fn is_enable(&self) -> bool {
            *self == SRCR::ENABLE
        }
        #[doc = "Checks if the value of the field is `INVERT`"]
        #[inline(always)]
        pub fn is_invert(&self) -> bool {
            *self == SRCR::INVERT
        }
        #[doc = "Checks if the value of the field is `ALTFAULT`"]
        #[inline(always)]
        pub fn is_altfault(&self) -> bool {
            *self == SRCR::ALTFAULT
        }
    }
    #[doc = r" Value of the field"]
    pub struct KEEPR {
        bits: bool,
    }
    impl KEEPR {
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
    pub struct QUALR {
        bits: bool,
    }
    impl QUALR {
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
    #[doc = "Possible values of the field `BLANK`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum BLANKR {
        #[doc = "undocumented"]
        DISABLE,
        #[doc = "undocumented"]
        RISE,
        #[doc = "undocumented"]
        FALL,
        #[doc = "undocumented"]
        BOTH,
    }
    impl BLANKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                BLANKR::DISABLE => 0,
                BLANKR::RISE => 1,
                BLANKR::FALL => 2,
                BLANKR::BOTH => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> BLANKR {
            match value {
                0 => BLANKR::DISABLE,
                1 => BLANKR::RISE,
                2 => BLANKR::FALL,
                3 => BLANKR::BOTH,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == BLANKR::DISABLE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == BLANKR::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == BLANKR::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == BLANKR::BOTH
        }
    }
    #[doc = r" Value of the field"]
    pub struct RESTARTR {
        bits: bool,
    }
    impl RESTARTR {
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
    #[doc = "Possible values of the field `HALT`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum HALTR {
        #[doc = "undocumented"]
        DISABLE,
        #[doc = "undocumented"]
        HW,
        #[doc = "undocumented"]
        SW,
        #[doc = "undocumented"]
        NR,
    }
    impl HALTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                HALTR::DISABLE => 0,
                HALTR::HW => 1,
                HALTR::SW => 2,
                HALTR::NR => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> HALTR {
            match value {
                0 => HALTR::DISABLE,
                1 => HALTR::HW,
                2 => HALTR::SW,
                3 => HALTR::NR,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == HALTR::DISABLE
        }
        #[doc = "Checks if the value of the field is `HW`"]
        #[inline(always)]
        pub fn is_hw(&self) -> bool {
            *self == HALTR::HW
        }
        #[doc = "Checks if the value of the field is `SW`"]
        #[inline(always)]
        pub fn is_sw(&self) -> bool {
            *self == HALTR::SW
        }
        #[doc = "Checks if the value of the field is `NR`"]
        #[inline(always)]
        pub fn is_nr(&self) -> bool {
            *self == HALTR::NR
        }
    }
    #[doc = "Possible values of the field `CHSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CHSELR {
        #[doc = "undocumented"]
        CC0,
        #[doc = "undocumented"]
        CC1,
        #[doc = "undocumented"]
        CC2,
        #[doc = "undocumented"]
        CC3,
    }
    impl CHSELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CHSELR::CC0 => 0,
                CHSELR::CC1 => 1,
                CHSELR::CC2 => 2,
                CHSELR::CC3 => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CHSELR {
            match value {
                0 => CHSELR::CC0,
                1 => CHSELR::CC1,
                2 => CHSELR::CC2,
                3 => CHSELR::CC3,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `CC0`"]
        #[inline(always)]
        pub fn is_cc0(&self) -> bool {
            *self == CHSELR::CC0
        }
        #[doc = "Checks if the value of the field is `CC1`"]
        #[inline(always)]
        pub fn is_cc1(&self) -> bool {
            *self == CHSELR::CC1
        }
        #[doc = "Checks if the value of the field is `CC2`"]
        #[inline(always)]
        pub fn is_cc2(&self) -> bool {
            *self == CHSELR::CC2
        }
        #[doc = "Checks if the value of the field is `CC3`"]
        #[inline(always)]
        pub fn is_cc3(&self) -> bool {
            *self == CHSELR::CC3
        }
    }
    #[doc = "Possible values of the field `CAPTURE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CAPTURER {
        #[doc = "undocumented"]
        DISABLE,
        #[doc = "undocumented"]
        CAPT,
        #[doc = "undocumented"]
        CAPTMIN,
        #[doc = "undocumented"]
        CAPTMAX,
        #[doc = "undocumented"]
        LOCMIN,
        #[doc = "undocumented"]
        LOCMAX,
        #[doc = "undocumented"]
        DERIV0,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CAPTURER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CAPTURER::DISABLE => 0,
                CAPTURER::CAPT => 1,
                CAPTURER::CAPTMIN => 2,
                CAPTURER::CAPTMAX => 3,
                CAPTURER::LOCMIN => 4,
                CAPTURER::LOCMAX => 5,
                CAPTURER::DERIV0 => 6,
                CAPTURER::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CAPTURER {
            match value {
                0 => CAPTURER::DISABLE,
                1 => CAPTURER::CAPT,
                2 => CAPTURER::CAPTMIN,
                3 => CAPTURER::CAPTMAX,
                4 => CAPTURER::LOCMIN,
                5 => CAPTURER::LOCMAX,
                6 => CAPTURER::DERIV0,
                i => CAPTURER::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == CAPTURER::DISABLE
        }
        #[doc = "Checks if the value of the field is `CAPT`"]
        #[inline(always)]
        pub fn is_capt(&self) -> bool {
            *self == CAPTURER::CAPT
        }
        #[doc = "Checks if the value of the field is `CAPTMIN`"]
        #[inline(always)]
        pub fn is_captmin(&self) -> bool {
            *self == CAPTURER::CAPTMIN
        }
        #[doc = "Checks if the value of the field is `CAPTMAX`"]
        #[inline(always)]
        pub fn is_captmax(&self) -> bool {
            *self == CAPTURER::CAPTMAX
        }
        #[doc = "Checks if the value of the field is `LOCMIN`"]
        #[inline(always)]
        pub fn is_locmin(&self) -> bool {
            *self == CAPTURER::LOCMIN
        }
        #[doc = "Checks if the value of the field is `LOCMAX`"]
        #[inline(always)]
        pub fn is_locmax(&self) -> bool {
            *self == CAPTURER::LOCMAX
        }
        #[doc = "Checks if the value of the field is `DERIV0`"]
        #[inline(always)]
        pub fn is_deriv0(&self) -> bool {
            *self == CAPTURER::DERIV0
        }
    }
    #[doc = r" Value of the field"]
    pub struct BLANKVALR {
        bits: u8,
    }
    impl BLANKVALR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTERVALR {
        bits: u8,
    }
    impl FILTERVALR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = "Values that can be written to the field `SRC`"]
    pub enum SRCW {
        #[doc = "`0`"]
        DISABLE,
        #[doc = "`1`"]
        ENABLE,
        #[doc = "`10`"]
        INVERT,
        #[doc = "`11`"]
        ALTFAULT,
    }
    impl SRCW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SRCW::DISABLE => 0,
                SRCW::ENABLE => 1,
                SRCW::INVERT => 2,
                SRCW::ALTFAULT => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SRCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SRCW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SRCW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(SRCW::DISABLE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn enable(self) -> &'a mut W {
            self.variant(SRCW::ENABLE)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn invert(self) -> &'a mut W {
            self.variant(SRCW::INVERT)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn altfault(self) -> &'a mut W {
            self.variant(SRCW::ALTFAULT)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _KEEPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _KEEPW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _QUALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _QUALW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `BLANK`"]
    pub enum BLANKW {
        #[doc = "`0`"]
        DISABLE,
        #[doc = "`1`"]
        RISE,
        #[doc = "`10`"]
        FALL,
        #[doc = "`11`"]
        BOTH,
    }
    impl BLANKW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                BLANKW::DISABLE => 0,
                BLANKW::RISE => 1,
                BLANKW::FALL => 2,
                BLANKW::BOTH => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _BLANKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BLANKW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: BLANKW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(BLANKW::DISABLE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(BLANKW::RISE)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(BLANKW::FALL)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(BLANKW::BOTH)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _RESTARTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RESTARTW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `HALT`"]
    pub enum HALTW {
        #[doc = "`0`"]
        DISABLE,
        #[doc = "`1`"]
        HW,
        #[doc = "`10`"]
        SW,
        #[doc = "`11`"]
        NR,
    }
    impl HALTW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                HALTW::DISABLE => 0,
                HALTW::HW => 1,
                HALTW::SW => 2,
                HALTW::NR => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _HALTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HALTW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: HALTW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(HALTW::DISABLE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn hw(self) -> &'a mut W {
            self.variant(HALTW::HW)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn sw(self) -> &'a mut W {
            self.variant(HALTW::SW)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn nr(self) -> &'a mut W {
            self.variant(HALTW::NR)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CHSEL`"]
    pub enum CHSELW {
        #[doc = "`0`"]
        CC0,
        #[doc = "`1`"]
        CC1,
        #[doc = "`10`"]
        CC2,
        #[doc = "`11`"]
        CC3,
    }
    impl CHSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CHSELW::CC0 => 0,
                CHSELW::CC1 => 1,
                CHSELW::CC2 => 2,
                CHSELW::CC3 => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CHSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CHSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CHSELW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn cc0(self) -> &'a mut W {
            self.variant(CHSELW::CC0)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn cc1(self) -> &'a mut W {
            self.variant(CHSELW::CC1)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn cc2(self) -> &'a mut W {
            self.variant(CHSELW::CC2)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn cc3(self) -> &'a mut W {
            self.variant(CHSELW::CC3)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CAPTURE`"]
    pub enum CAPTUREW {
        #[doc = "`0`"]
        DISABLE,
        #[doc = "`1`"]
        CAPT,
        #[doc = "`10`"]
        CAPTMIN,
        #[doc = "`11`"]
        CAPTMAX,
        #[doc = "`100`"]
        LOCMIN,
        #[doc = "`101`"]
        LOCMAX,
        #[doc = "`110`"]
        DERIV0,
    }
    impl CAPTUREW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CAPTUREW::DISABLE => 0,
                CAPTUREW::CAPT => 1,
                CAPTUREW::CAPTMIN => 2,
                CAPTUREW::CAPTMAX => 3,
                CAPTUREW::LOCMIN => 4,
                CAPTUREW::LOCMAX => 5,
                CAPTUREW::DERIV0 => 6,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CAPTUREW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CAPTUREW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CAPTUREW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(CAPTUREW::DISABLE)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn capt(self) -> &'a mut W {
            self.variant(CAPTUREW::CAPT)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn captmin(self) -> &'a mut W {
            self.variant(CAPTUREW::CAPTMIN)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn captmax(self) -> &'a mut W {
            self.variant(CAPTUREW::CAPTMAX)
        }
        #[doc = "`100`"]
        #[inline(always)]
        pub fn locmin(self) -> &'a mut W {
            self.variant(CAPTUREW::LOCMIN)
        }
        #[doc = "`101`"]
        #[inline(always)]
        pub fn locmax(self) -> &'a mut W {
            self.variant(CAPTUREW::LOCMAX)
        }
        #[doc = "`110`"]
        #[inline(always)]
        pub fn deriv0(self) -> &'a mut W {
            self.variant(CAPTUREW::DERIV0)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _BLANKVALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BLANKVALW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FILTERVALW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTERVALW<'a> {
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
        #[doc = "Bits 0:1 - FaultB Source"]
        #[inline(always)]
        pub fn src(&self) -> SRCR {
            SRCR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 3 - FaultB Keeper"]
        #[inline(always)]
        pub fn keep(&self) -> KEEPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            KEEPR { bits }
        }
        #[doc = "Bit 4 - FaultB Qualification"]
        #[inline(always)]
        pub fn qual(&self) -> QUALR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            QUALR { bits }
        }
        #[doc = "Bits 5:6 - FaultB Blanking Mode"]
        #[inline(always)]
        pub fn blank(&self) -> BLANKR {
            BLANKR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 7 - FaultB Restart"]
        #[inline(always)]
        pub fn restart(&self) -> RESTARTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RESTARTR { bits }
        }
        #[doc = "Bits 8:9 - FaultB Halt Mode"]
        #[inline(always)]
        pub fn halt(&self) -> HALTR {
            HALTR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 10:11 - FaultB Capture Channel"]
        #[inline(always)]
        pub fn chsel(&self) -> CHSELR {
            CHSELR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 12:14 - FaultB Capture Action"]
        #[inline(always)]
        pub fn capture(&self) -> CAPTURER {
            CAPTURER::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 16:23 - FaultB Blanking Time"]
        #[inline(always)]
        pub fn blankval(&self) -> BLANKVALR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            BLANKVALR { bits }
        }
        #[doc = "Bits 24:27 - FaultB Filter Value"]
        #[inline(always)]
        pub fn filterval(&self) -> FILTERVALR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            FILTERVALR { bits }
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
        #[doc = "Bits 0:1 - FaultB Source"]
        #[inline(always)]
        pub fn src(&mut self) -> _SRCW {
            _SRCW { w: self }
        }
        #[doc = "Bit 3 - FaultB Keeper"]
        #[inline(always)]
        pub fn keep(&mut self) -> _KEEPW {
            _KEEPW { w: self }
        }
        #[doc = "Bit 4 - FaultB Qualification"]
        #[inline(always)]
        pub fn qual(&mut self) -> _QUALW {
            _QUALW { w: self }
        }
        #[doc = "Bits 5:6 - FaultB Blanking Mode"]
        #[inline(always)]
        pub fn blank(&mut self) -> _BLANKW {
            _BLANKW { w: self }
        }
        #[doc = "Bit 7 - FaultB Restart"]
        #[inline(always)]
        pub fn restart(&mut self) -> _RESTARTW {
            _RESTARTW { w: self }
        }
        #[doc = "Bits 8:9 - FaultB Halt Mode"]
        #[inline(always)]
        pub fn halt(&mut self) -> _HALTW {
            _HALTW { w: self }
        }
        #[doc = "Bits 10:11 - FaultB Capture Channel"]
        #[inline(always)]
        pub fn chsel(&mut self) -> _CHSELW {
            _CHSELW { w: self }
        }
        #[doc = "Bits 12:14 - FaultB Capture Action"]
        #[inline(always)]
        pub fn capture(&mut self) -> _CAPTUREW {
            _CAPTUREW { w: self }
        }
        #[doc = "Bits 16:23 - FaultB Blanking Time"]
        #[inline(always)]
        pub fn blankval(&mut self) -> _BLANKVALW {
            _BLANKVALW { w: self }
        }
        #[doc = "Bits 24:27 - FaultB Filter Value"]
        #[inline(always)]
        pub fn filterval(&mut self) -> _FILTERVALW {
            _FILTERVALW { w: self }
        }
    }
}
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
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
    pub struct OVFR {
        bits: bool,
    }
    impl OVFR {
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
    pub struct TRGR {
        bits: bool,
    }
    impl TRGR {
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
    pub struct CNTR {
        bits: bool,
    }
    impl CNTR {
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
    pub struct ERRR {
        bits: bool,
    }
    impl ERRR {
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
    pub struct DFSR {
        bits: bool,
    }
    impl DFSR {
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
    pub struct FAULTAR {
        bits: bool,
    }
    impl FAULTAR {
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
    pub struct FAULTBR {
        bits: bool,
    }
    impl FAULTBR {
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
    pub struct FAULT0R {
        bits: bool,
    }
    impl FAULT0R {
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
    pub struct FAULT1R {
        bits: bool,
    }
    impl FAULT1R {
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
    pub struct MC0R {
        bits: bool,
    }
    impl MC0R {
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
    pub struct MC1R {
        bits: bool,
    }
    impl MC1R {
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
    pub struct MC2R {
        bits: bool,
    }
    impl MC2R {
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
    pub struct MC3R {
        bits: bool,
    }
    impl MC3R {
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
    pub struct _OVFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVFW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _TRGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TRGW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CNTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ERRW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _DFSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFSW<'a> {
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
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULTAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULTAW<'a> {
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
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULTBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULTBW<'a> {
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
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULT0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULT0W<'a> {
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
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULT1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULT1W<'a> {
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
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC0W<'a> {
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
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC1W<'a> {
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
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC2W<'a> {
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
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC3W<'a> {
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
            const OFFSET: u8 = 19;
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
        #[doc = "Bit 0 - Overflow Interrupt Enable"]
        #[inline(always)]
        pub fn ovf(&self) -> OVFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVFR { bits }
        }
        #[doc = "Bit 1 - Retrigger Interrupt Enable"]
        #[inline(always)]
        pub fn trg(&self) -> TRGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TRGR { bits }
        }
        #[doc = "Bit 2 - Counter Interrupt Enable"]
        #[inline(always)]
        pub fn cnt(&self) -> CNTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CNTR { bits }
        }
        #[doc = "Bit 3 - Error Interrupt Enable"]
        #[inline(always)]
        pub fn err(&self) -> ERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ERRR { bits }
        }
        #[doc = "Bit 11 - Non-recoverable Debug Fault Interrupt Enable"]
        #[inline(always)]
        pub fn dfs(&self) -> DFSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFSR { bits }
        }
        #[doc = "Bit 12 - Recoverable FaultA Interrupt Enable"]
        #[inline(always)]
        pub fn faulta(&self) -> FAULTAR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULTAR { bits }
        }
        #[doc = "Bit 13 - Recoverable FaultB Interrupt Enable"]
        #[inline(always)]
        pub fn faultb(&self) -> FAULTBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULTBR { bits }
        }
        #[doc = "Bit 14 - Non-Recoverable Fault 0 Interrupt Enable"]
        #[inline(always)]
        pub fn fault0(&self) -> FAULT0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULT0R { bits }
        }
        #[doc = "Bit 15 - Non-Recoverable Fault 1 Interrupt Enable"]
        #[inline(always)]
        pub fn fault1(&self) -> FAULT1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULT1R { bits }
        }
        #[doc = "Bit 16 - Match or Capture Channel 0 Interrupt Enable"]
        #[inline(always)]
        pub fn mc0(&self) -> MC0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC0R { bits }
        }
        #[doc = "Bit 17 - Match or Capture Channel 1 Interrupt Enable"]
        #[inline(always)]
        pub fn mc1(&self) -> MC1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC1R { bits }
        }
        #[doc = "Bit 18 - Match or Capture Channel 2 Interrupt Enable"]
        #[inline(always)]
        pub fn mc2(&self) -> MC2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC2R { bits }
        }
        #[doc = "Bit 19 - Match or Capture Channel 3 Interrupt Enable"]
        #[inline(always)]
        pub fn mc3(&self) -> MC3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC3R { bits }
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
        #[doc = "Bit 0 - Overflow Interrupt Enable"]
        #[inline(always)]
        pub fn ovf(&mut self) -> _OVFW {
            _OVFW { w: self }
        }
        #[doc = "Bit 1 - Retrigger Interrupt Enable"]
        #[inline(always)]
        pub fn trg(&mut self) -> _TRGW {
            _TRGW { w: self }
        }
        #[doc = "Bit 2 - Counter Interrupt Enable"]
        #[inline(always)]
        pub fn cnt(&mut self) -> _CNTW {
            _CNTW { w: self }
        }
        #[doc = "Bit 3 - Error Interrupt Enable"]
        #[inline(always)]
        pub fn err(&mut self) -> _ERRW {
            _ERRW { w: self }
        }
        #[doc = "Bit 11 - Non-recoverable Debug Fault Interrupt Enable"]
        #[inline(always)]
        pub fn dfs(&mut self) -> _DFSW {
            _DFSW { w: self }
        }
        #[doc = "Bit 12 - Recoverable FaultA Interrupt Enable"]
        #[inline(always)]
        pub fn faulta(&mut self) -> _FAULTAW {
            _FAULTAW { w: self }
        }
        #[doc = "Bit 13 - Recoverable FaultB Interrupt Enable"]
        #[inline(always)]
        pub fn faultb(&mut self) -> _FAULTBW {
            _FAULTBW { w: self }
        }
        #[doc = "Bit 14 - Non-Recoverable Fault 0 Interrupt Enable"]
        #[inline(always)]
        pub fn fault0(&mut self) -> _FAULT0W {
            _FAULT0W { w: self }
        }
        #[doc = "Bit 15 - Non-Recoverable Fault 1 Interrupt Enable"]
        #[inline(always)]
        pub fn fault1(&mut self) -> _FAULT1W {
            _FAULT1W { w: self }
        }
        #[doc = "Bit 16 - Match or Capture Channel 0 Interrupt Enable"]
        #[inline(always)]
        pub fn mc0(&mut self) -> _MC0W {
            _MC0W { w: self }
        }
        #[doc = "Bit 17 - Match or Capture Channel 1 Interrupt Enable"]
        #[inline(always)]
        pub fn mc1(&mut self) -> _MC1W {
            _MC1W { w: self }
        }
        #[doc = "Bit 18 - Match or Capture Channel 2 Interrupt Enable"]
        #[inline(always)]
        pub fn mc2(&mut self) -> _MC2W {
            _MC2W { w: self }
        }
        #[doc = "Bit 19 - Match or Capture Channel 3 Interrupt Enable"]
        #[inline(always)]
        pub fn mc3(&mut self) -> _MC3W {
            _MC3W { w: self }
        }
    }
}
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
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
    pub struct OVFR {
        bits: bool,
    }
    impl OVFR {
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
    pub struct TRGR {
        bits: bool,
    }
    impl TRGR {
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
    pub struct CNTR {
        bits: bool,
    }
    impl CNTR {
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
    pub struct ERRR {
        bits: bool,
    }
    impl ERRR {
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
    pub struct DFSR {
        bits: bool,
    }
    impl DFSR {
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
    pub struct FAULTAR {
        bits: bool,
    }
    impl FAULTAR {
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
    pub struct FAULTBR {
        bits: bool,
    }
    impl FAULTBR {
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
    pub struct FAULT0R {
        bits: bool,
    }
    impl FAULT0R {
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
    pub struct FAULT1R {
        bits: bool,
    }
    impl FAULT1R {
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
    pub struct MC0R {
        bits: bool,
    }
    impl MC0R {
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
    pub struct MC1R {
        bits: bool,
    }
    impl MC1R {
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
    pub struct MC2R {
        bits: bool,
    }
    impl MC2R {
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
    pub struct MC3R {
        bits: bool,
    }
    impl MC3R {
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
    pub struct _OVFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVFW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _TRGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TRGW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CNTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ERRW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _DFSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFSW<'a> {
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
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULTAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULTAW<'a> {
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
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULTBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULTBW<'a> {
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
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULT0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULT0W<'a> {
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
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULT1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULT1W<'a> {
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
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC0W<'a> {
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
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC1W<'a> {
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
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC2W<'a> {
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
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC3W<'a> {
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
            const OFFSET: u8 = 19;
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
        #[doc = "Bit 0 - Overflow Interrupt Enable"]
        #[inline(always)]
        pub fn ovf(&self) -> OVFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVFR { bits }
        }
        #[doc = "Bit 1 - Retrigger Interrupt Enable"]
        #[inline(always)]
        pub fn trg(&self) -> TRGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TRGR { bits }
        }
        #[doc = "Bit 2 - Counter Interrupt Enable"]
        #[inline(always)]
        pub fn cnt(&self) -> CNTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CNTR { bits }
        }
        #[doc = "Bit 3 - Error Interrupt Enable"]
        #[inline(always)]
        pub fn err(&self) -> ERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ERRR { bits }
        }
        #[doc = "Bit 11 - Non-Recoverable Debug Fault Interrupt Enable"]
        #[inline(always)]
        pub fn dfs(&self) -> DFSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFSR { bits }
        }
        #[doc = "Bit 12 - Recoverable FaultA Interrupt Enable"]
        #[inline(always)]
        pub fn faulta(&self) -> FAULTAR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULTAR { bits }
        }
        #[doc = "Bit 13 - Recoverable FaultB Interrupt Enable"]
        #[inline(always)]
        pub fn faultb(&self) -> FAULTBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULTBR { bits }
        }
        #[doc = "Bit 14 - Non-Recoverable Fault 0 Interrupt Enable"]
        #[inline(always)]
        pub fn fault0(&self) -> FAULT0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULT0R { bits }
        }
        #[doc = "Bit 15 - Non-Recoverable Fault 1 Interrupt Enabl"]
        #[inline(always)]
        pub fn fault1(&self) -> FAULT1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULT1R { bits }
        }
        #[doc = "Bit 16 - Match or Capture Channel 0 Interrupt Enable"]
        #[inline(always)]
        pub fn mc0(&self) -> MC0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC0R { bits }
        }
        #[doc = "Bit 17 - Match or Capture Channel 1 Interrupt Enable"]
        #[inline(always)]
        pub fn mc1(&self) -> MC1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC1R { bits }
        }
        #[doc = "Bit 18 - Match or Capture Channel 2 Interrupt Enable"]
        #[inline(always)]
        pub fn mc2(&self) -> MC2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC2R { bits }
        }
        #[doc = "Bit 19 - Match or Capture Channel 3 Interrupt Enable"]
        #[inline(always)]
        pub fn mc3(&self) -> MC3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC3R { bits }
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
        #[doc = "Bit 0 - Overflow Interrupt Enable"]
        #[inline(always)]
        pub fn ovf(&mut self) -> _OVFW {
            _OVFW { w: self }
        }
        #[doc = "Bit 1 - Retrigger Interrupt Enable"]
        #[inline(always)]
        pub fn trg(&mut self) -> _TRGW {
            _TRGW { w: self }
        }
        #[doc = "Bit 2 - Counter Interrupt Enable"]
        #[inline(always)]
        pub fn cnt(&mut self) -> _CNTW {
            _CNTW { w: self }
        }
        #[doc = "Bit 3 - Error Interrupt Enable"]
        #[inline(always)]
        pub fn err(&mut self) -> _ERRW {
            _ERRW { w: self }
        }
        #[doc = "Bit 11 - Non-Recoverable Debug Fault Interrupt Enable"]
        #[inline(always)]
        pub fn dfs(&mut self) -> _DFSW {
            _DFSW { w: self }
        }
        #[doc = "Bit 12 - Recoverable FaultA Interrupt Enable"]
        #[inline(always)]
        pub fn faulta(&mut self) -> _FAULTAW {
            _FAULTAW { w: self }
        }
        #[doc = "Bit 13 - Recoverable FaultB Interrupt Enable"]
        #[inline(always)]
        pub fn faultb(&mut self) -> _FAULTBW {
            _FAULTBW { w: self }
        }
        #[doc = "Bit 14 - Non-Recoverable Fault 0 Interrupt Enable"]
        #[inline(always)]
        pub fn fault0(&mut self) -> _FAULT0W {
            _FAULT0W { w: self }
        }
        #[doc = "Bit 15 - Non-Recoverable Fault 1 Interrupt Enabl"]
        #[inline(always)]
        pub fn fault1(&mut self) -> _FAULT1W {
            _FAULT1W { w: self }
        }
        #[doc = "Bit 16 - Match or Capture Channel 0 Interrupt Enable"]
        #[inline(always)]
        pub fn mc0(&mut self) -> _MC0W {
            _MC0W { w: self }
        }
        #[doc = "Bit 17 - Match or Capture Channel 1 Interrupt Enable"]
        #[inline(always)]
        pub fn mc1(&mut self) -> _MC1W {
            _MC1W { w: self }
        }
        #[doc = "Bit 18 - Match or Capture Channel 2 Interrupt Enable"]
        #[inline(always)]
        pub fn mc2(&mut self) -> _MC2W {
            _MC2W { w: self }
        }
        #[doc = "Bit 19 - Match or Capture Channel 3 Interrupt Enable"]
        #[inline(always)]
        pub fn mc3(&mut self) -> _MC3W {
            _MC3W { w: self }
        }
    }
}
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
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
    pub struct OVFR {
        bits: bool,
    }
    impl OVFR {
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
    pub struct TRGR {
        bits: bool,
    }
    impl TRGR {
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
    pub struct CNTR {
        bits: bool,
    }
    impl CNTR {
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
    pub struct ERRR {
        bits: bool,
    }
    impl ERRR {
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
    pub struct DFSR {
        bits: bool,
    }
    impl DFSR {
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
    pub struct FAULTAR {
        bits: bool,
    }
    impl FAULTAR {
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
    pub struct FAULTBR {
        bits: bool,
    }
    impl FAULTBR {
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
    pub struct FAULT0R {
        bits: bool,
    }
    impl FAULT0R {
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
    pub struct FAULT1R {
        bits: bool,
    }
    impl FAULT1R {
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
    pub struct MC0R {
        bits: bool,
    }
    impl MC0R {
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
    pub struct MC1R {
        bits: bool,
    }
    impl MC1R {
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
    pub struct MC2R {
        bits: bool,
    }
    impl MC2R {
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
    pub struct MC3R {
        bits: bool,
    }
    impl MC3R {
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
    pub struct _OVFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVFW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _TRGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TRGW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CNTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CNTW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ERRW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _DFSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFSW<'a> {
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
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULTAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULTAW<'a> {
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
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULTBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULTBW<'a> {
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
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULT0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULT0W<'a> {
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
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULT1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULT1W<'a> {
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
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC0W<'a> {
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
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC1W<'a> {
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
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC2W<'a> {
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
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MC3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _MC3W<'a> {
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
            const OFFSET: u8 = 19;
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
        #[doc = "Bit 0 - Overflow"]
        #[inline(always)]
        pub fn ovf(&self) -> OVFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVFR { bits }
        }
        #[doc = "Bit 1 - Retrigger"]
        #[inline(always)]
        pub fn trg(&self) -> TRGR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TRGR { bits }
        }
        #[doc = "Bit 2 - Counter"]
        #[inline(always)]
        pub fn cnt(&self) -> CNTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CNTR { bits }
        }
        #[doc = "Bit 3 - Error"]
        #[inline(always)]
        pub fn err(&self) -> ERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ERRR { bits }
        }
        #[doc = "Bit 11 - Non-Recoverable Debug Fault"]
        #[inline(always)]
        pub fn dfs(&self) -> DFSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFSR { bits }
        }
        #[doc = "Bit 12 - Recoverable FaultA"]
        #[inline(always)]
        pub fn faulta(&self) -> FAULTAR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULTAR { bits }
        }
        #[doc = "Bit 13 - Recoverable FaultB"]
        #[inline(always)]
        pub fn faultb(&self) -> FAULTBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULTBR { bits }
        }
        #[doc = "Bit 14 - Non-Recoverable Fault 0"]
        #[inline(always)]
        pub fn fault0(&self) -> FAULT0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULT0R { bits }
        }
        #[doc = "Bit 15 - Non-Recoverable Fault 1"]
        #[inline(always)]
        pub fn fault1(&self) -> FAULT1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULT1R { bits }
        }
        #[doc = "Bit 16 - Match or Capture 0"]
        #[inline(always)]
        pub fn mc0(&self) -> MC0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC0R { bits }
        }
        #[doc = "Bit 17 - Match or Capture 1"]
        #[inline(always)]
        pub fn mc1(&self) -> MC1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC1R { bits }
        }
        #[doc = "Bit 18 - Match or Capture 2"]
        #[inline(always)]
        pub fn mc2(&self) -> MC2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC2R { bits }
        }
        #[doc = "Bit 19 - Match or Capture 3"]
        #[inline(always)]
        pub fn mc3(&self) -> MC3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MC3R { bits }
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
        #[doc = "Bit 0 - Overflow"]
        #[inline(always)]
        pub fn ovf(&mut self) -> _OVFW {
            _OVFW { w: self }
        }
        #[doc = "Bit 1 - Retrigger"]
        #[inline(always)]
        pub fn trg(&mut self) -> _TRGW {
            _TRGW { w: self }
        }
        #[doc = "Bit 2 - Counter"]
        #[inline(always)]
        pub fn cnt(&mut self) -> _CNTW {
            _CNTW { w: self }
        }
        #[doc = "Bit 3 - Error"]
        #[inline(always)]
        pub fn err(&mut self) -> _ERRW {
            _ERRW { w: self }
        }
        #[doc = "Bit 11 - Non-Recoverable Debug Fault"]
        #[inline(always)]
        pub fn dfs(&mut self) -> _DFSW {
            _DFSW { w: self }
        }
        #[doc = "Bit 12 - Recoverable FaultA"]
        #[inline(always)]
        pub fn faulta(&mut self) -> _FAULTAW {
            _FAULTAW { w: self }
        }
        #[doc = "Bit 13 - Recoverable FaultB"]
        #[inline(always)]
        pub fn faultb(&mut self) -> _FAULTBW {
            _FAULTBW { w: self }
        }
        #[doc = "Bit 14 - Non-Recoverable Fault 0"]
        #[inline(always)]
        pub fn fault0(&mut self) -> _FAULT0W {
            _FAULT0W { w: self }
        }
        #[doc = "Bit 15 - Non-Recoverable Fault 1"]
        #[inline(always)]
        pub fn fault1(&mut self) -> _FAULT1W {
            _FAULT1W { w: self }
        }
        #[doc = "Bit 16 - Match or Capture 0"]
        #[inline(always)]
        pub fn mc0(&mut self) -> _MC0W {
            _MC0W { w: self }
        }
        #[doc = "Bit 17 - Match or Capture 1"]
        #[inline(always)]
        pub fn mc1(&mut self) -> _MC1W {
            _MC1W { w: self }
        }
        #[doc = "Bit 18 - Match or Capture 2"]
        #[inline(always)]
        pub fn mc2(&mut self) -> _MC2W {
            _MC2W { w: self }
        }
        #[doc = "Bit 19 - Match or Capture 3"]
        #[inline(always)]
        pub fn mc3(&mut self) -> _MC3W {
            _MC3W { w: self }
        }
    }
}
#[doc = "Pattern"]
pub struct PATT {
    register: VolatileCell<u16>,
}
#[doc = "Pattern"]
pub mod patt {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::PATT {
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
    pub struct PGE0R {
        bits: bool,
    }
    impl PGE0R {
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
    pub struct PGE1R {
        bits: bool,
    }
    impl PGE1R {
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
    pub struct PGE2R {
        bits: bool,
    }
    impl PGE2R {
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
    pub struct PGE3R {
        bits: bool,
    }
    impl PGE3R {
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
    pub struct PGE4R {
        bits: bool,
    }
    impl PGE4R {
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
    pub struct PGE5R {
        bits: bool,
    }
    impl PGE5R {
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
    pub struct PGE6R {
        bits: bool,
    }
    impl PGE6R {
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
    pub struct PGE7R {
        bits: bool,
    }
    impl PGE7R {
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
    pub struct PGV0R {
        bits: bool,
    }
    impl PGV0R {
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
    pub struct PGV1R {
        bits: bool,
    }
    impl PGV1R {
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
    pub struct PGV2R {
        bits: bool,
    }
    impl PGV2R {
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
    pub struct PGV3R {
        bits: bool,
    }
    impl PGV3R {
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
    pub struct PGV4R {
        bits: bool,
    }
    impl PGV4R {
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
    pub struct PGV5R {
        bits: bool,
    }
    impl PGV5R {
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
    pub struct PGV6R {
        bits: bool,
    }
    impl PGV6R {
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
    pub struct PGV7R {
        bits: bool,
    }
    impl PGV7R {
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
    pub struct _PGE0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGE0W<'a> {
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
    pub struct _PGE1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGE1W<'a> {
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
    pub struct _PGE2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGE2W<'a> {
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
    pub struct _PGE3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGE3W<'a> {
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
    #[doc = r" Proxy"]
    pub struct _PGE4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGE4W<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGE5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGE5W<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGE6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGE6W<'a> {
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGE7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGE7W<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGV0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGV0W<'a> {
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
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGV1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGV1W<'a> {
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
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGV2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGV2W<'a> {
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
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGV3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGV3W<'a> {
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
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGV4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGV4W<'a> {
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
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGV5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGV5W<'a> {
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
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGV6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGV6W<'a> {
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
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGV7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGV7W<'a> {
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
            const OFFSET: u8 = 15;
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
        #[doc = "Bit 0 - Pattern Generator 0 Output Enable"]
        #[inline(always)]
        pub fn pge0(&self) -> PGE0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGE0R { bits }
        }
        #[doc = "Bit 1 - Pattern Generator 1 Output Enable"]
        #[inline(always)]
        pub fn pge1(&self) -> PGE1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGE1R { bits }
        }
        #[doc = "Bit 2 - Pattern Generator 2 Output Enable"]
        #[inline(always)]
        pub fn pge2(&self) -> PGE2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGE2R { bits }
        }
        #[doc = "Bit 3 - Pattern Generator 3 Output Enable"]
        #[inline(always)]
        pub fn pge3(&self) -> PGE3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGE3R { bits }
        }
        #[doc = "Bit 4 - Pattern Generator 4 Output Enable"]
        #[inline(always)]
        pub fn pge4(&self) -> PGE4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGE4R { bits }
        }
        #[doc = "Bit 5 - Pattern Generator 5 Output Enable"]
        #[inline(always)]
        pub fn pge5(&self) -> PGE5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGE5R { bits }
        }
        #[doc = "Bit 6 - Pattern Generator 6 Output Enable"]
        #[inline(always)]
        pub fn pge6(&self) -> PGE6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGE6R { bits }
        }
        #[doc = "Bit 7 - Pattern Generator 7 Output Enable"]
        #[inline(always)]
        pub fn pge7(&self) -> PGE7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGE7R { bits }
        }
        #[doc = "Bit 8 - Pattern Generator 0 Output Value"]
        #[inline(always)]
        pub fn pgv0(&self) -> PGV0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGV0R { bits }
        }
        #[doc = "Bit 9 - Pattern Generator 1 Output Value"]
        #[inline(always)]
        pub fn pgv1(&self) -> PGV1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGV1R { bits }
        }
        #[doc = "Bit 10 - Pattern Generator 2 Output Value"]
        #[inline(always)]
        pub fn pgv2(&self) -> PGV2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGV2R { bits }
        }
        #[doc = "Bit 11 - Pattern Generator 3 Output Value"]
        #[inline(always)]
        pub fn pgv3(&self) -> PGV3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGV3R { bits }
        }
        #[doc = "Bit 12 - Pattern Generator 4 Output Value"]
        #[inline(always)]
        pub fn pgv4(&self) -> PGV4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGV4R { bits }
        }
        #[doc = "Bit 13 - Pattern Generator 5 Output Value"]
        #[inline(always)]
        pub fn pgv5(&self) -> PGV5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGV5R { bits }
        }
        #[doc = "Bit 14 - Pattern Generator 6 Output Value"]
        #[inline(always)]
        pub fn pgv6(&self) -> PGV6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGV6R { bits }
        }
        #[doc = "Bit 15 - Pattern Generator 7 Output Value"]
        #[inline(always)]
        pub fn pgv7(&self) -> PGV7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGV7R { bits }
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
        #[doc = "Bit 0 - Pattern Generator 0 Output Enable"]
        #[inline(always)]
        pub fn pge0(&mut self) -> _PGE0W {
            _PGE0W { w: self }
        }
        #[doc = "Bit 1 - Pattern Generator 1 Output Enable"]
        #[inline(always)]
        pub fn pge1(&mut self) -> _PGE1W {
            _PGE1W { w: self }
        }
        #[doc = "Bit 2 - Pattern Generator 2 Output Enable"]
        #[inline(always)]
        pub fn pge2(&mut self) -> _PGE2W {
            _PGE2W { w: self }
        }
        #[doc = "Bit 3 - Pattern Generator 3 Output Enable"]
        #[inline(always)]
        pub fn pge3(&mut self) -> _PGE3W {
            _PGE3W { w: self }
        }
        #[doc = "Bit 4 - Pattern Generator 4 Output Enable"]
        #[inline(always)]
        pub fn pge4(&mut self) -> _PGE4W {
            _PGE4W { w: self }
        }
        #[doc = "Bit 5 - Pattern Generator 5 Output Enable"]
        #[inline(always)]
        pub fn pge5(&mut self) -> _PGE5W {
            _PGE5W { w: self }
        }
        #[doc = "Bit 6 - Pattern Generator 6 Output Enable"]
        #[inline(always)]
        pub fn pge6(&mut self) -> _PGE6W {
            _PGE6W { w: self }
        }
        #[doc = "Bit 7 - Pattern Generator 7 Output Enable"]
        #[inline(always)]
        pub fn pge7(&mut self) -> _PGE7W {
            _PGE7W { w: self }
        }
        #[doc = "Bit 8 - Pattern Generator 0 Output Value"]
        #[inline(always)]
        pub fn pgv0(&mut self) -> _PGV0W {
            _PGV0W { w: self }
        }
        #[doc = "Bit 9 - Pattern Generator 1 Output Value"]
        #[inline(always)]
        pub fn pgv1(&mut self) -> _PGV1W {
            _PGV1W { w: self }
        }
        #[doc = "Bit 10 - Pattern Generator 2 Output Value"]
        #[inline(always)]
        pub fn pgv2(&mut self) -> _PGV2W {
            _PGV2W { w: self }
        }
        #[doc = "Bit 11 - Pattern Generator 3 Output Value"]
        #[inline(always)]
        pub fn pgv3(&mut self) -> _PGV3W {
            _PGV3W { w: self }
        }
        #[doc = "Bit 12 - Pattern Generator 4 Output Value"]
        #[inline(always)]
        pub fn pgv4(&mut self) -> _PGV4W {
            _PGV4W { w: self }
        }
        #[doc = "Bit 13 - Pattern Generator 5 Output Value"]
        #[inline(always)]
        pub fn pgv5(&mut self) -> _PGV5W {
            _PGV5W { w: self }
        }
        #[doc = "Bit 14 - Pattern Generator 6 Output Value"]
        #[inline(always)]
        pub fn pgv6(&mut self) -> _PGV6W {
            _PGV6W { w: self }
        }
        #[doc = "Bit 15 - Pattern Generator 7 Output Value"]
        #[inline(always)]
        pub fn pgv7(&mut self) -> _PGV7W {
            _PGV7W { w: self }
        }
    }
}
#[doc = "Pattern Buffer"]
pub struct PATTB {
    register: VolatileCell<u16>,
}
#[doc = "Pattern Buffer"]
pub mod pattb {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::PATTB {
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
    pub struct PGEB0R {
        bits: bool,
    }
    impl PGEB0R {
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
    pub struct PGEB1R {
        bits: bool,
    }
    impl PGEB1R {
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
    pub struct PGEB2R {
        bits: bool,
    }
    impl PGEB2R {
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
    pub struct PGEB3R {
        bits: bool,
    }
    impl PGEB3R {
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
    pub struct PGEB4R {
        bits: bool,
    }
    impl PGEB4R {
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
    pub struct PGEB5R {
        bits: bool,
    }
    impl PGEB5R {
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
    pub struct PGEB6R {
        bits: bool,
    }
    impl PGEB6R {
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
    pub struct PGEB7R {
        bits: bool,
    }
    impl PGEB7R {
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
    pub struct PGVB0R {
        bits: bool,
    }
    impl PGVB0R {
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
    pub struct PGVB1R {
        bits: bool,
    }
    impl PGVB1R {
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
    pub struct PGVB2R {
        bits: bool,
    }
    impl PGVB2R {
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
    pub struct PGVB3R {
        bits: bool,
    }
    impl PGVB3R {
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
    pub struct PGVB4R {
        bits: bool,
    }
    impl PGVB4R {
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
    pub struct PGVB5R {
        bits: bool,
    }
    impl PGVB5R {
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
    pub struct PGVB6R {
        bits: bool,
    }
    impl PGVB6R {
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
    pub struct PGVB7R {
        bits: bool,
    }
    impl PGVB7R {
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
    pub struct _PGEB0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGEB0W<'a> {
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
    pub struct _PGEB1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGEB1W<'a> {
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
    pub struct _PGEB2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGEB2W<'a> {
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
    pub struct _PGEB3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGEB3W<'a> {
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
    #[doc = r" Proxy"]
    pub struct _PGEB4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGEB4W<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGEB5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGEB5W<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGEB6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGEB6W<'a> {
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGEB7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGEB7W<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGVB0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGVB0W<'a> {
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
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGVB1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGVB1W<'a> {
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
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGVB2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGVB2W<'a> {
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
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGVB3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGVB3W<'a> {
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
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGVB4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGVB4W<'a> {
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
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGVB5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGVB5W<'a> {
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
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGVB6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGVB6W<'a> {
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
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PGVB7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PGVB7W<'a> {
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
            const OFFSET: u8 = 15;
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
        #[doc = "Bit 0 - Pattern Generator 0 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb0(&self) -> PGEB0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGEB0R { bits }
        }
        #[doc = "Bit 1 - Pattern Generator 1 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb1(&self) -> PGEB1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGEB1R { bits }
        }
        #[doc = "Bit 2 - Pattern Generator 2 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb2(&self) -> PGEB2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGEB2R { bits }
        }
        #[doc = "Bit 3 - Pattern Generator 3 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb3(&self) -> PGEB3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGEB3R { bits }
        }
        #[doc = "Bit 4 - Pattern Generator 4 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb4(&self) -> PGEB4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGEB4R { bits }
        }
        #[doc = "Bit 5 - Pattern Generator 5 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb5(&self) -> PGEB5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGEB5R { bits }
        }
        #[doc = "Bit 6 - Pattern Generator 6 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb6(&self) -> PGEB6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGEB6R { bits }
        }
        #[doc = "Bit 7 - Pattern Generator 7 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb7(&self) -> PGEB7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGEB7R { bits }
        }
        #[doc = "Bit 8 - Pattern Generator 0 Output Enable"]
        #[inline(always)]
        pub fn pgvb0(&self) -> PGVB0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGVB0R { bits }
        }
        #[doc = "Bit 9 - Pattern Generator 1 Output Enable"]
        #[inline(always)]
        pub fn pgvb1(&self) -> PGVB1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGVB1R { bits }
        }
        #[doc = "Bit 10 - Pattern Generator 2 Output Enable"]
        #[inline(always)]
        pub fn pgvb2(&self) -> PGVB2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGVB2R { bits }
        }
        #[doc = "Bit 11 - Pattern Generator 3 Output Enable"]
        #[inline(always)]
        pub fn pgvb3(&self) -> PGVB3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGVB3R { bits }
        }
        #[doc = "Bit 12 - Pattern Generator 4 Output Enable"]
        #[inline(always)]
        pub fn pgvb4(&self) -> PGVB4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGVB4R { bits }
        }
        #[doc = "Bit 13 - Pattern Generator 5 Output Enable"]
        #[inline(always)]
        pub fn pgvb5(&self) -> PGVB5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGVB5R { bits }
        }
        #[doc = "Bit 14 - Pattern Generator 6 Output Enable"]
        #[inline(always)]
        pub fn pgvb6(&self) -> PGVB6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGVB6R { bits }
        }
        #[doc = "Bit 15 - Pattern Generator 7 Output Enable"]
        #[inline(always)]
        pub fn pgvb7(&self) -> PGVB7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PGVB7R { bits }
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
        #[doc = "Bit 0 - Pattern Generator 0 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb0(&mut self) -> _PGEB0W {
            _PGEB0W { w: self }
        }
        #[doc = "Bit 1 - Pattern Generator 1 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb1(&mut self) -> _PGEB1W {
            _PGEB1W { w: self }
        }
        #[doc = "Bit 2 - Pattern Generator 2 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb2(&mut self) -> _PGEB2W {
            _PGEB2W { w: self }
        }
        #[doc = "Bit 3 - Pattern Generator 3 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb3(&mut self) -> _PGEB3W {
            _PGEB3W { w: self }
        }
        #[doc = "Bit 4 - Pattern Generator 4 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb4(&mut self) -> _PGEB4W {
            _PGEB4W { w: self }
        }
        #[doc = "Bit 5 - Pattern Generator 5 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb5(&mut self) -> _PGEB5W {
            _PGEB5W { w: self }
        }
        #[doc = "Bit 6 - Pattern Generator 6 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb6(&mut self) -> _PGEB6W {
            _PGEB6W { w: self }
        }
        #[doc = "Bit 7 - Pattern Generator 7 Output Enable Buffer"]
        #[inline(always)]
        pub fn pgeb7(&mut self) -> _PGEB7W {
            _PGEB7W { w: self }
        }
        #[doc = "Bit 8 - Pattern Generator 0 Output Enable"]
        #[inline(always)]
        pub fn pgvb0(&mut self) -> _PGVB0W {
            _PGVB0W { w: self }
        }
        #[doc = "Bit 9 - Pattern Generator 1 Output Enable"]
        #[inline(always)]
        pub fn pgvb1(&mut self) -> _PGVB1W {
            _PGVB1W { w: self }
        }
        #[doc = "Bit 10 - Pattern Generator 2 Output Enable"]
        #[inline(always)]
        pub fn pgvb2(&mut self) -> _PGVB2W {
            _PGVB2W { w: self }
        }
        #[doc = "Bit 11 - Pattern Generator 3 Output Enable"]
        #[inline(always)]
        pub fn pgvb3(&mut self) -> _PGVB3W {
            _PGVB3W { w: self }
        }
        #[doc = "Bit 12 - Pattern Generator 4 Output Enable"]
        #[inline(always)]
        pub fn pgvb4(&mut self) -> _PGVB4W {
            _PGVB4W { w: self }
        }
        #[doc = "Bit 13 - Pattern Generator 5 Output Enable"]
        #[inline(always)]
        pub fn pgvb5(&mut self) -> _PGVB5W {
            _PGVB5W { w: self }
        }
        #[doc = "Bit 14 - Pattern Generator 6 Output Enable"]
        #[inline(always)]
        pub fn pgvb6(&mut self) -> _PGVB6W {
            _PGVB6W { w: self }
        }
        #[doc = "Bit 15 - Pattern Generator 7 Output Enable"]
        #[inline(always)]
        pub fn pgvb7(&mut self) -> _PGVB7W {
            _PGVB7W { w: self }
        }
    }
}
#[doc = "Period"]
pub struct PER {
    register: VolatileCell<u32>,
}
#[doc = "Period"]
pub mod per {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::PER {
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
    pub struct PERR {
        bits: u32,
    }
    impl PERR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _PERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PERW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
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
        #[doc = "Bits 0:23 - Period Value"]
        #[inline(always)]
        pub fn per(&self) -> PERR {
            let bits = {
                const MASK: u32 = 16777215;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            PERR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 4294967295 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:23 - Period Value"]
        #[inline(always)]
        pub fn per(&mut self) -> _PERW {
            _PERW { w: self }
        }
    }
}
#[doc = "Period Buffer"]
pub struct PERB {
    register: VolatileCell<u32>,
}
#[doc = "Period Buffer"]
pub mod perb {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::PERB {
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
    pub struct PERBR {
        bits: u32,
    }
    impl PERBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _PERBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PERBW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
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
        #[doc = "Bits 0:23 - Period Value"]
        #[inline(always)]
        pub fn perb(&self) -> PERBR {
            let bits = {
                const MASK: u32 = 16777215;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            PERBR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 4294967295 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:23 - Period Value"]
        #[inline(always)]
        pub fn perb(&mut self) -> _PERBW {
            _PERBW { w: self }
        }
    }
}
#[doc = "Status"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "Status"]
pub mod status {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::STATUS {
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
    pub struct STOPR {
        bits: bool,
    }
    impl STOPR {
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
    pub struct IDXR {
        bits: bool,
    }
    impl IDXR {
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
    pub struct DFSR {
        bits: bool,
    }
    impl DFSR {
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
    pub struct PATTBVR {
        bits: bool,
    }
    impl PATTBVR {
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
    pub struct WAVEBVR {
        bits: bool,
    }
    impl WAVEBVR {
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
    pub struct PERBVR {
        bits: bool,
    }
    impl PERBVR {
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
    pub struct FAULTAINR {
        bits: bool,
    }
    impl FAULTAINR {
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
    pub struct FAULTBINR {
        bits: bool,
    }
    impl FAULTBINR {
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
    pub struct FAULT0INR {
        bits: bool,
    }
    impl FAULT0INR {
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
    pub struct FAULT1INR {
        bits: bool,
    }
    impl FAULT1INR {
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
    pub struct FAULTAR {
        bits: bool,
    }
    impl FAULTAR {
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
    pub struct FAULTBR {
        bits: bool,
    }
    impl FAULTBR {
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
    pub struct FAULT0R {
        bits: bool,
    }
    impl FAULT0R {
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
    pub struct FAULT1R {
        bits: bool,
    }
    impl FAULT1R {
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
    pub struct CCBV0R {
        bits: bool,
    }
    impl CCBV0R {
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
    pub struct CCBV1R {
        bits: bool,
    }
    impl CCBV1R {
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
    pub struct CCBV2R {
        bits: bool,
    }
    impl CCBV2R {
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
    pub struct CCBV3R {
        bits: bool,
    }
    impl CCBV3R {
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
    pub struct CMP0R {
        bits: bool,
    }
    impl CMP0R {
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
    pub struct CMP1R {
        bits: bool,
    }
    impl CMP1R {
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
    pub struct CMP2R {
        bits: bool,
    }
    impl CMP2R {
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
    pub struct CMP3R {
        bits: bool,
    }
    impl CMP3R {
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
    pub struct _DFSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFSW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PATTBVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PATTBVW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _WAVEBVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAVEBVW<'a> {
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PERBVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PERBVW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULTAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULTAW<'a> {
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
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULTBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULTBW<'a> {
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
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULT0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULT0W<'a> {
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
            const OFFSET: u8 = 14;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FAULT1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAULT1W<'a> {
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
            const OFFSET: u8 = 15;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CCBV0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCBV0W<'a> {
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
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CCBV1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCBV1W<'a> {
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
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CCBV2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCBV2W<'a> {
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
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CCBV3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCBV3W<'a> {
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
            const OFFSET: u8 = 19;
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
        #[doc = "Bit 0 - Stop"]
        #[inline(always)]
        pub fn stop(&self) -> STOPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            STOPR { bits }
        }
        #[doc = "Bit 1 - Ramp"]
        #[inline(always)]
        pub fn idx(&self) -> IDXR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDXR { bits }
        }
        #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
        #[inline(always)]
        pub fn dfs(&self) -> DFSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFSR { bits }
        }
        #[doc = "Bit 5 - Pattern Buffer Valid"]
        #[inline(always)]
        pub fn pattbv(&self) -> PATTBVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PATTBVR { bits }
        }
        #[doc = "Bit 6 - Wave Buffer Valid"]
        #[inline(always)]
        pub fn wavebv(&self) -> WAVEBVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAVEBVR { bits }
        }
        #[doc = "Bit 7 - Period Buffer Valid"]
        #[inline(always)]
        pub fn perbv(&self) -> PERBVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PERBVR { bits }
        }
        #[doc = "Bit 8 - Recoverable FaultA Input"]
        #[inline(always)]
        pub fn faultain(&self) -> FAULTAINR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULTAINR { bits }
        }
        #[doc = "Bit 9 - Recoverable FaultB Input"]
        #[inline(always)]
        pub fn faultbin(&self) -> FAULTBINR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULTBINR { bits }
        }
        #[doc = "Bit 10 - Non-Recoverable Fault0 Input"]
        #[inline(always)]
        pub fn fault0in(&self) -> FAULT0INR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULT0INR { bits }
        }
        #[doc = "Bit 11 - Non-Recoverable Fault1 Input"]
        #[inline(always)]
        pub fn fault1in(&self) -> FAULT1INR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULT1INR { bits }
        }
        #[doc = "Bit 12 - Recoverable FaultA State"]
        #[inline(always)]
        pub fn faulta(&self) -> FAULTAR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULTAR { bits }
        }
        #[doc = "Bit 13 - Recoverable FaultB State"]
        #[inline(always)]
        pub fn faultb(&self) -> FAULTBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULTBR { bits }
        }
        #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
        #[inline(always)]
        pub fn fault0(&self) -> FAULT0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULT0R { bits }
        }
        #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
        #[inline(always)]
        pub fn fault1(&self) -> FAULT1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FAULT1R { bits }
        }
        #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
        #[inline(always)]
        pub fn ccbv0(&self) -> CCBV0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCBV0R { bits }
        }
        #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
        #[inline(always)]
        pub fn ccbv1(&self) -> CCBV1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCBV1R { bits }
        }
        #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
        #[inline(always)]
        pub fn ccbv2(&self) -> CCBV2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCBV2R { bits }
        }
        #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
        #[inline(always)]
        pub fn ccbv3(&self) -> CCBV3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCBV3R { bits }
        }
        #[doc = "Bit 24 - Compare Channel 0 Value"]
        #[inline(always)]
        pub fn cmp0(&self) -> CMP0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CMP0R { bits }
        }
        #[doc = "Bit 25 - Compare Channel 1 Value"]
        #[inline(always)]
        pub fn cmp1(&self) -> CMP1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CMP1R { bits }
        }
        #[doc = "Bit 26 - Compare Channel 2 Value"]
        #[inline(always)]
        pub fn cmp2(&self) -> CMP2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CMP2R { bits }
        }
        #[doc = "Bit 27 - Compare Channel 3 Value"]
        #[inline(always)]
        pub fn cmp3(&self) -> CMP3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CMP3R { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 1 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
        #[inline(always)]
        pub fn dfs(&mut self) -> _DFSW {
            _DFSW { w: self }
        }
        #[doc = "Bit 5 - Pattern Buffer Valid"]
        #[inline(always)]
        pub fn pattbv(&mut self) -> _PATTBVW {
            _PATTBVW { w: self }
        }
        #[doc = "Bit 6 - Wave Buffer Valid"]
        #[inline(always)]
        pub fn wavebv(&mut self) -> _WAVEBVW {
            _WAVEBVW { w: self }
        }
        #[doc = "Bit 7 - Period Buffer Valid"]
        #[inline(always)]
        pub fn perbv(&mut self) -> _PERBVW {
            _PERBVW { w: self }
        }
        #[doc = "Bit 12 - Recoverable FaultA State"]
        #[inline(always)]
        pub fn faulta(&mut self) -> _FAULTAW {
            _FAULTAW { w: self }
        }
        #[doc = "Bit 13 - Recoverable FaultB State"]
        #[inline(always)]
        pub fn faultb(&mut self) -> _FAULTBW {
            _FAULTBW { w: self }
        }
        #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
        #[inline(always)]
        pub fn fault0(&mut self) -> _FAULT0W {
            _FAULT0W { w: self }
        }
        #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
        #[inline(always)]
        pub fn fault1(&mut self) -> _FAULT1W {
            _FAULT1W { w: self }
        }
        #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
        #[inline(always)]
        pub fn ccbv0(&mut self) -> _CCBV0W {
            _CCBV0W { w: self }
        }
        #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
        #[inline(always)]
        pub fn ccbv1(&mut self) -> _CCBV1W {
            _CCBV1W { w: self }
        }
        #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
        #[inline(always)]
        pub fn ccbv2(&mut self) -> _CCBV2W {
            _CCBV2W { w: self }
        }
        #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
        #[inline(always)]
        pub fn ccbv3(&mut self) -> _CCBV3W {
            _CCBV3W { w: self }
        }
    }
}
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: VolatileCell<u32>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::SYNCBUSY {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
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
    pub struct CTRLBR {
        bits: bool,
    }
    impl CTRLBR {
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
    pub struct STATUSR {
        bits: bool,
    }
    impl STATUSR {
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
    pub struct COUNTR {
        bits: bool,
    }
    impl COUNTR {
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
    pub struct PATTR {
        bits: bool,
    }
    impl PATTR {
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
    pub struct WAVER {
        bits: bool,
    }
    impl WAVER {
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
    pub struct PERR {
        bits: bool,
    }
    impl PERR {
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
    pub struct CC0R {
        bits: bool,
    }
    impl CC0R {
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
    pub struct CC1R {
        bits: bool,
    }
    impl CC1R {
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
    pub struct CC2R {
        bits: bool,
    }
    impl CC2R {
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
    pub struct CC3R {
        bits: bool,
    }
    impl CC3R {
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
    pub struct PATTBR {
        bits: bool,
    }
    impl PATTBR {
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
    pub struct WAVEBR {
        bits: bool,
    }
    impl WAVEBR {
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
    pub struct PERBR {
        bits: bool,
    }
    impl PERBR {
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
    pub struct CCB0R {
        bits: bool,
    }
    impl CCB0R {
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
    pub struct CCB1R {
        bits: bool,
    }
    impl CCB1R {
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
    pub struct CCB2R {
        bits: bool,
    }
    impl CCB2R {
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
    pub struct CCB3R {
        bits: bool,
    }
    impl CCB3R {
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
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - Swrst Busy"]
        #[inline(always)]
        pub fn swrst(&self) -> SWRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWRSTR { bits }
        }
        #[doc = "Bit 1 - Enable Busy"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 2 - Ctrlb Busy"]
        #[inline(always)]
        pub fn ctrlb(&self) -> CTRLBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CTRLBR { bits }
        }
        #[doc = "Bit 3 - Status Busy"]
        #[inline(always)]
        pub fn status(&self) -> STATUSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            STATUSR { bits }
        }
        #[doc = "Bit 4 - Count Busy"]
        #[inline(always)]
        pub fn count(&self) -> COUNTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            COUNTR { bits }
        }
        #[doc = "Bit 5 - Pattern Busy"]
        #[inline(always)]
        pub fn patt(&self) -> PATTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PATTR { bits }
        }
        #[doc = "Bit 6 - Wave Busy"]
        #[inline(always)]
        pub fn wave(&self) -> WAVER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAVER { bits }
        }
        #[doc = "Bit 7 - Period busy"]
        #[inline(always)]
        pub fn per(&self) -> PERR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PERR { bits }
        }
        #[doc = "Bit 8 - Compare Channel Buffer 0 Busy"]
        #[inline(always)]
        pub fn cc0(&self) -> CC0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC0R { bits }
        }
        #[doc = "Bit 9 - Compare Channel Buffer 1 Busy"]
        #[inline(always)]
        pub fn cc1(&self) -> CC1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC1R { bits }
        }
        #[doc = "Bit 10 - Compare Channel Buffer 2 Busy"]
        #[inline(always)]
        pub fn cc2(&self) -> CC2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC2R { bits }
        }
        #[doc = "Bit 11 - Compare Channel Buffer 3 Busy"]
        #[inline(always)]
        pub fn cc3(&self) -> CC3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CC3R { bits }
        }
        #[doc = "Bit 16 - Pattern Buffer Busy"]
        #[inline(always)]
        pub fn pattb(&self) -> PATTBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PATTBR { bits }
        }
        #[doc = "Bit 17 - Wave Buffer Busy"]
        #[inline(always)]
        pub fn waveb(&self) -> WAVEBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAVEBR { bits }
        }
        #[doc = "Bit 18 - Period Buffer Busy"]
        #[inline(always)]
        pub fn perb(&self) -> PERBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PERBR { bits }
        }
        #[doc = "Bit 19 - Compare Channel Buffer 0 Busy"]
        #[inline(always)]
        pub fn ccb0(&self) -> CCB0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCB0R { bits }
        }
        #[doc = "Bit 20 - Compare Channel Buffer 1 Busy"]
        #[inline(always)]
        pub fn ccb1(&self) -> CCB1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCB1R { bits }
        }
        #[doc = "Bit 21 - Compare Channel Buffer 2 Busy"]
        #[inline(always)]
        pub fn ccb2(&self) -> CCB2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCB2R { bits }
        }
        #[doc = "Bit 22 - Compare Channel Buffer 3 Busy"]
        #[inline(always)]
        pub fn ccb3(&self) -> CCB3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CCB3R { bits }
        }
    }
}
#[doc = "Waveform Control"]
pub struct WAVE {
    register: VolatileCell<u32>,
}
#[doc = "Waveform Control"]
pub mod wave {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::WAVE {
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
    #[doc = "Possible values of the field `WAVEGEN`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum WAVEGENR {
        #[doc = "undocumented"]
        NFRQ,
        #[doc = "undocumented"]
        MFRQ,
        #[doc = "undocumented"]
        NPWM,
        #[doc = "undocumented"]
        DSCRITICAL,
        #[doc = "undocumented"]
        DSBOTTOM,
        #[doc = "undocumented"]
        DSBOTH,
        #[doc = "undocumented"]
        DSTOP,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl WAVEGENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                WAVEGENR::NFRQ => 0,
                WAVEGENR::MFRQ => 1,
                WAVEGENR::NPWM => 2,
                WAVEGENR::DSCRITICAL => 4,
                WAVEGENR::DSBOTTOM => 5,
                WAVEGENR::DSBOTH => 6,
                WAVEGENR::DSTOP => 7,
                WAVEGENR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> WAVEGENR {
            match value {
                0 => WAVEGENR::NFRQ,
                1 => WAVEGENR::MFRQ,
                2 => WAVEGENR::NPWM,
                4 => WAVEGENR::DSCRITICAL,
                5 => WAVEGENR::DSBOTTOM,
                6 => WAVEGENR::DSBOTH,
                7 => WAVEGENR::DSTOP,
                i => WAVEGENR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NFRQ`"]
        #[inline(always)]
        pub fn is_nfrq(&self) -> bool {
            *self == WAVEGENR::NFRQ
        }
        #[doc = "Checks if the value of the field is `MFRQ`"]
        #[inline(always)]
        pub fn is_mfrq(&self) -> bool {
            *self == WAVEGENR::MFRQ
        }
        #[doc = "Checks if the value of the field is `NPWM`"]
        #[inline(always)]
        pub fn is_npwm(&self) -> bool {
            *self == WAVEGENR::NPWM
        }
        #[doc = "Checks if the value of the field is `DSCRITICAL`"]
        #[inline(always)]
        pub fn is_dscritical(&self) -> bool {
            *self == WAVEGENR::DSCRITICAL
        }
        #[doc = "Checks if the value of the field is `DSBOTTOM`"]
        #[inline(always)]
        pub fn is_dsbottom(&self) -> bool {
            *self == WAVEGENR::DSBOTTOM
        }
        #[doc = "Checks if the value of the field is `DSBOTH`"]
        #[inline(always)]
        pub fn is_dsboth(&self) -> bool {
            *self == WAVEGENR::DSBOTH
        }
        #[doc = "Checks if the value of the field is `DSTOP`"]
        #[inline(always)]
        pub fn is_dstop(&self) -> bool {
            *self == WAVEGENR::DSTOP
        }
    }
    #[doc = "Possible values of the field `RAMP`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum RAMPR {
        #[doc = "undocumented"]
        RAMP1,
        #[doc = "undocumented"]
        RAMP2A,
        #[doc = "undocumented"]
        RAMP2,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl RAMPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                RAMPR::RAMP1 => 0,
                RAMPR::RAMP2A => 1,
                RAMPR::RAMP2 => 2,
                RAMPR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> RAMPR {
            match value {
                0 => RAMPR::RAMP1,
                1 => RAMPR::RAMP2A,
                2 => RAMPR::RAMP2,
                i => RAMPR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `RAMP1`"]
        #[inline(always)]
        pub fn is_ramp1(&self) -> bool {
            *self == RAMPR::RAMP1
        }
        #[doc = "Checks if the value of the field is `RAMP2A`"]
        #[inline(always)]
        pub fn is_ramp2a(&self) -> bool {
            *self == RAMPR::RAMP2A
        }
        #[doc = "Checks if the value of the field is `RAMP2`"]
        #[inline(always)]
        pub fn is_ramp2(&self) -> bool {
            *self == RAMPR::RAMP2
        }
    }
    #[doc = r" Value of the field"]
    pub struct CIPERENR {
        bits: bool,
    }
    impl CIPERENR {
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
    pub struct CICCEN0R {
        bits: bool,
    }
    impl CICCEN0R {
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
    pub struct CICCEN1R {
        bits: bool,
    }
    impl CICCEN1R {
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
    pub struct CICCEN2R {
        bits: bool,
    }
    impl CICCEN2R {
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
    pub struct CICCEN3R {
        bits: bool,
    }
    impl CICCEN3R {
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
    pub struct POL0R {
        bits: bool,
    }
    impl POL0R {
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
    pub struct POL1R {
        bits: bool,
    }
    impl POL1R {
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
    pub struct POL2R {
        bits: bool,
    }
    impl POL2R {
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
    pub struct POL3R {
        bits: bool,
    }
    impl POL3R {
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
    pub struct SWAP0R {
        bits: bool,
    }
    impl SWAP0R {
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
    pub struct SWAP1R {
        bits: bool,
    }
    impl SWAP1R {
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
    pub struct SWAP2R {
        bits: bool,
    }
    impl SWAP2R {
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
    pub struct SWAP3R {
        bits: bool,
    }
    impl SWAP3R {
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
    #[doc = "Values that can be written to the field `WAVEGEN`"]
    pub enum WAVEGENW {
        #[doc = "`0`"]
        NFRQ,
        #[doc = "`1`"]
        MFRQ,
        #[doc = "`10`"]
        NPWM,
        #[doc = "`100`"]
        DSCRITICAL,
        #[doc = "`101`"]
        DSBOTTOM,
        #[doc = "`110`"]
        DSBOTH,
        #[doc = "`111`"]
        DSTOP,
    }
    impl WAVEGENW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                WAVEGENW::NFRQ => 0,
                WAVEGENW::MFRQ => 1,
                WAVEGENW::NPWM => 2,
                WAVEGENW::DSCRITICAL => 4,
                WAVEGENW::DSBOTTOM => 5,
                WAVEGENW::DSBOTH => 6,
                WAVEGENW::DSTOP => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _WAVEGENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAVEGENW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: WAVEGENW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn nfrq(self) -> &'a mut W {
            self.variant(WAVEGENW::NFRQ)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn mfrq(self) -> &'a mut W {
            self.variant(WAVEGENW::MFRQ)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn npwm(self) -> &'a mut W {
            self.variant(WAVEGENW::NPWM)
        }
        #[doc = "`100`"]
        #[inline(always)]
        pub fn dscritical(self) -> &'a mut W {
            self.variant(WAVEGENW::DSCRITICAL)
        }
        #[doc = "`101`"]
        #[inline(always)]
        pub fn dsbottom(self) -> &'a mut W {
            self.variant(WAVEGENW::DSBOTTOM)
        }
        #[doc = "`110`"]
        #[inline(always)]
        pub fn dsboth(self) -> &'a mut W {
            self.variant(WAVEGENW::DSBOTH)
        }
        #[doc = "`111`"]
        #[inline(always)]
        pub fn dstop(self) -> &'a mut W {
            self.variant(WAVEGENW::DSTOP)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `RAMP`"]
    pub enum RAMPW {
        #[doc = "`0`"]
        RAMP1,
        #[doc = "`1`"]
        RAMP2A,
        #[doc = "`10`"]
        RAMP2,
    }
    impl RAMPW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                RAMPW::RAMP1 => 0,
                RAMPW::RAMP2A => 1,
                RAMPW::RAMP2 => 2,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _RAMPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RAMPW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: RAMPW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn ramp1(self) -> &'a mut W {
            self.variant(RAMPW::RAMP1)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn ramp2a(self) -> &'a mut W {
            self.variant(RAMPW::RAMP2A)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn ramp2(self) -> &'a mut W {
            self.variant(RAMPW::RAMP2)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CIPERENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CIPERENW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CICCEN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CICCEN0W<'a> {
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
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CICCEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CICCEN1W<'a> {
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
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CICCEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CICCEN2W<'a> {
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
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CICCEN3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CICCEN3W<'a> {
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
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _POL0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _POL0W<'a> {
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
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _POL1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _POL1W<'a> {
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
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _POL2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _POL2W<'a> {
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
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _POL3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _POL3W<'a> {
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
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SWAP0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWAP0W<'a> {
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
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SWAP1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWAP1W<'a> {
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
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SWAP2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWAP2W<'a> {
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
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SWAP3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWAP3W<'a> {
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
            const OFFSET: u8 = 27;
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
        #[doc = "Bits 0:2 - Waveform Generation"]
        #[inline(always)]
        pub fn wavegen(&self) -> WAVEGENR {
            WAVEGENR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 4:5 - Ramp Mode"]
        #[inline(always)]
        pub fn ramp(&self) -> RAMPR {
            RAMPR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 7 - Circular period Enable"]
        #[inline(always)]
        pub fn ciperen(&self) -> CIPERENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CIPERENR { bits }
        }
        #[doc = "Bit 8 - Circular Channel 0 Enable"]
        #[inline(always)]
        pub fn ciccen0(&self) -> CICCEN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CICCEN0R { bits }
        }
        #[doc = "Bit 9 - Circular Channel 1 Enable"]
        #[inline(always)]
        pub fn ciccen1(&self) -> CICCEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CICCEN1R { bits }
        }
        #[doc = "Bit 10 - Circular Channel 2 Enable"]
        #[inline(always)]
        pub fn ciccen2(&self) -> CICCEN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CICCEN2R { bits }
        }
        #[doc = "Bit 11 - Circular Channel 3 Enable"]
        #[inline(always)]
        pub fn ciccen3(&self) -> CICCEN3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CICCEN3R { bits }
        }
        #[doc = "Bit 16 - Channel 0 Polarity"]
        #[inline(always)]
        pub fn pol0(&self) -> POL0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POL0R { bits }
        }
        #[doc = "Bit 17 - Channel 1 Polarity"]
        #[inline(always)]
        pub fn pol1(&self) -> POL1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POL1R { bits }
        }
        #[doc = "Bit 18 - Channel 2 Polarity"]
        #[inline(always)]
        pub fn pol2(&self) -> POL2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POL2R { bits }
        }
        #[doc = "Bit 19 - Channel 3 Polarity"]
        #[inline(always)]
        pub fn pol3(&self) -> POL3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POL3R { bits }
        }
        #[doc = "Bit 24 - Swap DTI Output Pair 0"]
        #[inline(always)]
        pub fn swap0(&self) -> SWAP0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWAP0R { bits }
        }
        #[doc = "Bit 25 - Swap DTI Output Pair 1"]
        #[inline(always)]
        pub fn swap1(&self) -> SWAP1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWAP1R { bits }
        }
        #[doc = "Bit 26 - Swap DTI Output Pair 2"]
        #[inline(always)]
        pub fn swap2(&self) -> SWAP2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWAP2R { bits }
        }
        #[doc = "Bit 27 - Swap DTI Output Pair 3"]
        #[inline(always)]
        pub fn swap3(&self) -> SWAP3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWAP3R { bits }
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
        #[doc = "Bits 0:2 - Waveform Generation"]
        #[inline(always)]
        pub fn wavegen(&mut self) -> _WAVEGENW {
            _WAVEGENW { w: self }
        }
        #[doc = "Bits 4:5 - Ramp Mode"]
        #[inline(always)]
        pub fn ramp(&mut self) -> _RAMPW {
            _RAMPW { w: self }
        }
        #[doc = "Bit 7 - Circular period Enable"]
        #[inline(always)]
        pub fn ciperen(&mut self) -> _CIPERENW {
            _CIPERENW { w: self }
        }
        #[doc = "Bit 8 - Circular Channel 0 Enable"]
        #[inline(always)]
        pub fn ciccen0(&mut self) -> _CICCEN0W {
            _CICCEN0W { w: self }
        }
        #[doc = "Bit 9 - Circular Channel 1 Enable"]
        #[inline(always)]
        pub fn ciccen1(&mut self) -> _CICCEN1W {
            _CICCEN1W { w: self }
        }
        #[doc = "Bit 10 - Circular Channel 2 Enable"]
        #[inline(always)]
        pub fn ciccen2(&mut self) -> _CICCEN2W {
            _CICCEN2W { w: self }
        }
        #[doc = "Bit 11 - Circular Channel 3 Enable"]
        #[inline(always)]
        pub fn ciccen3(&mut self) -> _CICCEN3W {
            _CICCEN3W { w: self }
        }
        #[doc = "Bit 16 - Channel 0 Polarity"]
        #[inline(always)]
        pub fn pol0(&mut self) -> _POL0W {
            _POL0W { w: self }
        }
        #[doc = "Bit 17 - Channel 1 Polarity"]
        #[inline(always)]
        pub fn pol1(&mut self) -> _POL1W {
            _POL1W { w: self }
        }
        #[doc = "Bit 18 - Channel 2 Polarity"]
        #[inline(always)]
        pub fn pol2(&mut self) -> _POL2W {
            _POL2W { w: self }
        }
        #[doc = "Bit 19 - Channel 3 Polarity"]
        #[inline(always)]
        pub fn pol3(&mut self) -> _POL3W {
            _POL3W { w: self }
        }
        #[doc = "Bit 24 - Swap DTI Output Pair 0"]
        #[inline(always)]
        pub fn swap0(&mut self) -> _SWAP0W {
            _SWAP0W { w: self }
        }
        #[doc = "Bit 25 - Swap DTI Output Pair 1"]
        #[inline(always)]
        pub fn swap1(&mut self) -> _SWAP1W {
            _SWAP1W { w: self }
        }
        #[doc = "Bit 26 - Swap DTI Output Pair 2"]
        #[inline(always)]
        pub fn swap2(&mut self) -> _SWAP2W {
            _SWAP2W { w: self }
        }
        #[doc = "Bit 27 - Swap DTI Output Pair 3"]
        #[inline(always)]
        pub fn swap3(&mut self) -> _SWAP3W {
            _SWAP3W { w: self }
        }
    }
}
#[doc = "Waveform Control Buffer"]
pub struct WAVEB {
    register: VolatileCell<u32>,
}
#[doc = "Waveform Control Buffer"]
pub mod waveb {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::WAVEB {
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
    #[doc = "Possible values of the field `WAVEGENB`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum WAVEGENBR {
        #[doc = "undocumented"]
        NFRQ,
        #[doc = "undocumented"]
        MFRQ,
        #[doc = "undocumented"]
        NPWM,
        #[doc = "undocumented"]
        DSCRITICAL,
        #[doc = "undocumented"]
        DSBOTTOM,
        #[doc = "undocumented"]
        DSBOTH,
        #[doc = "undocumented"]
        DSTOP,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl WAVEGENBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                WAVEGENBR::NFRQ => 0,
                WAVEGENBR::MFRQ => 1,
                WAVEGENBR::NPWM => 2,
                WAVEGENBR::DSCRITICAL => 4,
                WAVEGENBR::DSBOTTOM => 5,
                WAVEGENBR::DSBOTH => 6,
                WAVEGENBR::DSTOP => 7,
                WAVEGENBR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> WAVEGENBR {
            match value {
                0 => WAVEGENBR::NFRQ,
                1 => WAVEGENBR::MFRQ,
                2 => WAVEGENBR::NPWM,
                4 => WAVEGENBR::DSCRITICAL,
                5 => WAVEGENBR::DSBOTTOM,
                6 => WAVEGENBR::DSBOTH,
                7 => WAVEGENBR::DSTOP,
                i => WAVEGENBR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NFRQ`"]
        #[inline(always)]
        pub fn is_nfrq(&self) -> bool {
            *self == WAVEGENBR::NFRQ
        }
        #[doc = "Checks if the value of the field is `MFRQ`"]
        #[inline(always)]
        pub fn is_mfrq(&self) -> bool {
            *self == WAVEGENBR::MFRQ
        }
        #[doc = "Checks if the value of the field is `NPWM`"]
        #[inline(always)]
        pub fn is_npwm(&self) -> bool {
            *self == WAVEGENBR::NPWM
        }
        #[doc = "Checks if the value of the field is `DSCRITICAL`"]
        #[inline(always)]
        pub fn is_dscritical(&self) -> bool {
            *self == WAVEGENBR::DSCRITICAL
        }
        #[doc = "Checks if the value of the field is `DSBOTTOM`"]
        #[inline(always)]
        pub fn is_dsbottom(&self) -> bool {
            *self == WAVEGENBR::DSBOTTOM
        }
        #[doc = "Checks if the value of the field is `DSBOTH`"]
        #[inline(always)]
        pub fn is_dsboth(&self) -> bool {
            *self == WAVEGENBR::DSBOTH
        }
        #[doc = "Checks if the value of the field is `DSTOP`"]
        #[inline(always)]
        pub fn is_dstop(&self) -> bool {
            *self == WAVEGENBR::DSTOP
        }
    }
    #[doc = r" Value of the field"]
    pub struct RAMPBR {
        bits: u8,
    }
    impl RAMPBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct CIPERENBR {
        bits: bool,
    }
    impl CIPERENBR {
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
    pub struct CICCENB0R {
        bits: bool,
    }
    impl CICCENB0R {
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
    pub struct CICCENB1R {
        bits: bool,
    }
    impl CICCENB1R {
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
    pub struct CICCENB2R {
        bits: bool,
    }
    impl CICCENB2R {
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
    pub struct CICCENB3R {
        bits: bool,
    }
    impl CICCENB3R {
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
    pub struct POLB0R {
        bits: bool,
    }
    impl POLB0R {
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
    pub struct POLB1R {
        bits: bool,
    }
    impl POLB1R {
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
    pub struct POLB2R {
        bits: bool,
    }
    impl POLB2R {
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
    pub struct POLB3R {
        bits: bool,
    }
    impl POLB3R {
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
    pub struct SWAPB0R {
        bits: bool,
    }
    impl SWAPB0R {
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
    pub struct SWAPB1R {
        bits: bool,
    }
    impl SWAPB1R {
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
    pub struct SWAPB2R {
        bits: bool,
    }
    impl SWAPB2R {
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
    pub struct SWAPB3R {
        bits: bool,
    }
    impl SWAPB3R {
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
    #[doc = "Values that can be written to the field `WAVEGENB`"]
    pub enum WAVEGENBW {
        #[doc = "`0`"]
        NFRQ,
        #[doc = "`1`"]
        MFRQ,
        #[doc = "`10`"]
        NPWM,
        #[doc = "`100`"]
        DSCRITICAL,
        #[doc = "`101`"]
        DSBOTTOM,
        #[doc = "`110`"]
        DSBOTH,
        #[doc = "`111`"]
        DSTOP,
    }
    impl WAVEGENBW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                WAVEGENBW::NFRQ => 0,
                WAVEGENBW::MFRQ => 1,
                WAVEGENBW::NPWM => 2,
                WAVEGENBW::DSCRITICAL => 4,
                WAVEGENBW::DSBOTTOM => 5,
                WAVEGENBW::DSBOTH => 6,
                WAVEGENBW::DSTOP => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _WAVEGENBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAVEGENBW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: WAVEGENBW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn nfrq(self) -> &'a mut W {
            self.variant(WAVEGENBW::NFRQ)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn mfrq(self) -> &'a mut W {
            self.variant(WAVEGENBW::MFRQ)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn npwm(self) -> &'a mut W {
            self.variant(WAVEGENBW::NPWM)
        }
        #[doc = "`100`"]
        #[inline(always)]
        pub fn dscritical(self) -> &'a mut W {
            self.variant(WAVEGENBW::DSCRITICAL)
        }
        #[doc = "`101`"]
        #[inline(always)]
        pub fn dsbottom(self) -> &'a mut W {
            self.variant(WAVEGENBW::DSBOTTOM)
        }
        #[doc = "`110`"]
        #[inline(always)]
        pub fn dsboth(self) -> &'a mut W {
            self.variant(WAVEGENBW::DSBOTH)
        }
        #[doc = "`111`"]
        #[inline(always)]
        pub fn dstop(self) -> &'a mut W {
            self.variant(WAVEGENBW::DSTOP)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _RAMPBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RAMPBW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CIPERENBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CIPERENBW<'a> {
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CICCENB0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CICCENB0W<'a> {
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
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CICCENB1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CICCENB1W<'a> {
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
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CICCENB2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CICCENB2W<'a> {
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
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CICCENB3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CICCENB3W<'a> {
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
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _POLB0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _POLB0W<'a> {
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
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _POLB1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _POLB1W<'a> {
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
            const OFFSET: u8 = 17;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _POLB2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _POLB2W<'a> {
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
            const OFFSET: u8 = 18;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _POLB3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _POLB3W<'a> {
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
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SWAPB0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWAPB0W<'a> {
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
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SWAPB1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWAPB1W<'a> {
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
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SWAPB2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWAPB2W<'a> {
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
            const OFFSET: u8 = 26;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SWAPB3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWAPB3W<'a> {
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
            const OFFSET: u8 = 27;
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
        #[doc = "Bits 0:2 - Waveform Generation Buffer"]
        #[inline(always)]
        pub fn wavegenb(&self) -> WAVEGENBR {
            WAVEGENBR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 4:5 - Ramp Mode Buffer"]
        #[inline(always)]
        pub fn rampb(&self) -> RAMPBR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            RAMPBR { bits }
        }
        #[doc = "Bit 7 - Circular Period Enable Buffer"]
        #[inline(always)]
        pub fn ciperenb(&self) -> CIPERENBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CIPERENBR { bits }
        }
        #[doc = "Bit 8 - Circular Channel 0 Enable Buffer"]
        #[inline(always)]
        pub fn ciccenb0(&self) -> CICCENB0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CICCENB0R { bits }
        }
        #[doc = "Bit 9 - Circular Channel 1 Enable Buffer"]
        #[inline(always)]
        pub fn ciccenb1(&self) -> CICCENB1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CICCENB1R { bits }
        }
        #[doc = "Bit 10 - Circular Channel 2 Enable Buffer"]
        #[inline(always)]
        pub fn ciccenb2(&self) -> CICCENB2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CICCENB2R { bits }
        }
        #[doc = "Bit 11 - Circular Channel 3 Enable Buffer"]
        #[inline(always)]
        pub fn ciccenb3(&self) -> CICCENB3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CICCENB3R { bits }
        }
        #[doc = "Bit 16 - Channel 0 Polarity Buffer"]
        #[inline(always)]
        pub fn polb0(&self) -> POLB0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POLB0R { bits }
        }
        #[doc = "Bit 17 - Channel 1 Polarity Buffer"]
        #[inline(always)]
        pub fn polb1(&self) -> POLB1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POLB1R { bits }
        }
        #[doc = "Bit 18 - Channel 2 Polarity Buffer"]
        #[inline(always)]
        pub fn polb2(&self) -> POLB2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POLB2R { bits }
        }
        #[doc = "Bit 19 - Channel 3 Polarity Buffer"]
        #[inline(always)]
        pub fn polb3(&self) -> POLB3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            POLB3R { bits }
        }
        #[doc = "Bit 24 - Swap DTI Output Pair 0 Buffer"]
        #[inline(always)]
        pub fn swapb0(&self) -> SWAPB0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWAPB0R { bits }
        }
        #[doc = "Bit 25 - Swap DTI Output Pair 1 Buffer"]
        #[inline(always)]
        pub fn swapb1(&self) -> SWAPB1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWAPB1R { bits }
        }
        #[doc = "Bit 26 - Swap DTI Output Pair 2 Buffer"]
        #[inline(always)]
        pub fn swapb2(&self) -> SWAPB2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWAPB2R { bits }
        }
        #[doc = "Bit 27 - Swap DTI Output Pair 3 Buffer"]
        #[inline(always)]
        pub fn swapb3(&self) -> SWAPB3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWAPB3R { bits }
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
        #[doc = "Bits 0:2 - Waveform Generation Buffer"]
        #[inline(always)]
        pub fn wavegenb(&mut self) -> _WAVEGENBW {
            _WAVEGENBW { w: self }
        }
        #[doc = "Bits 4:5 - Ramp Mode Buffer"]
        #[inline(always)]
        pub fn rampb(&mut self) -> _RAMPBW {
            _RAMPBW { w: self }
        }
        #[doc = "Bit 7 - Circular Period Enable Buffer"]
        #[inline(always)]
        pub fn ciperenb(&mut self) -> _CIPERENBW {
            _CIPERENBW { w: self }
        }
        #[doc = "Bit 8 - Circular Channel 0 Enable Buffer"]
        #[inline(always)]
        pub fn ciccenb0(&mut self) -> _CICCENB0W {
            _CICCENB0W { w: self }
        }
        #[doc = "Bit 9 - Circular Channel 1 Enable Buffer"]
        #[inline(always)]
        pub fn ciccenb1(&mut self) -> _CICCENB1W {
            _CICCENB1W { w: self }
        }
        #[doc = "Bit 10 - Circular Channel 2 Enable Buffer"]
        #[inline(always)]
        pub fn ciccenb2(&mut self) -> _CICCENB2W {
            _CICCENB2W { w: self }
        }
        #[doc = "Bit 11 - Circular Channel 3 Enable Buffer"]
        #[inline(always)]
        pub fn ciccenb3(&mut self) -> _CICCENB3W {
            _CICCENB3W { w: self }
        }
        #[doc = "Bit 16 - Channel 0 Polarity Buffer"]
        #[inline(always)]
        pub fn polb0(&mut self) -> _POLB0W {
            _POLB0W { w: self }
        }
        #[doc = "Bit 17 - Channel 1 Polarity Buffer"]
        #[inline(always)]
        pub fn polb1(&mut self) -> _POLB1W {
            _POLB1W { w: self }
        }
        #[doc = "Bit 18 - Channel 2 Polarity Buffer"]
        #[inline(always)]
        pub fn polb2(&mut self) -> _POLB2W {
            _POLB2W { w: self }
        }
        #[doc = "Bit 19 - Channel 3 Polarity Buffer"]
        #[inline(always)]
        pub fn polb3(&mut self) -> _POLB3W {
            _POLB3W { w: self }
        }
        #[doc = "Bit 24 - Swap DTI Output Pair 0 Buffer"]
        #[inline(always)]
        pub fn swapb0(&mut self) -> _SWAPB0W {
            _SWAPB0W { w: self }
        }
        #[doc = "Bit 25 - Swap DTI Output Pair 1 Buffer"]
        #[inline(always)]
        pub fn swapb1(&mut self) -> _SWAPB1W {
            _SWAPB1W { w: self }
        }
        #[doc = "Bit 26 - Swap DTI Output Pair 2 Buffer"]
        #[inline(always)]
        pub fn swapb2(&mut self) -> _SWAPB2W {
            _SWAPB2W { w: self }
        }
        #[doc = "Bit 27 - Swap DTI Output Pair 3 Buffer"]
        #[inline(always)]
        pub fn swapb3(&mut self) -> _SWAPB3W {
            _SWAPB3W { w: self }
        }
    }
}
#[doc = "Waveform Extension Configuration"]
pub struct WEXCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Waveform Extension Configuration"]
pub mod wexctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::WEXCTRL {
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
    pub struct OTMXR {
        bits: u8,
    }
    impl OTMXR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct DTIEN0R {
        bits: bool,
    }
    impl DTIEN0R {
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
    pub struct DTIEN1R {
        bits: bool,
    }
    impl DTIEN1R {
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
    pub struct DTIEN2R {
        bits: bool,
    }
    impl DTIEN2R {
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
    pub struct DTIEN3R {
        bits: bool,
    }
    impl DTIEN3R {
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
    pub struct DTLSR {
        bits: u8,
    }
    impl DTLSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct DTHSR {
        bits: u8,
    }
    impl DTHSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _OTMXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OTMXW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _DTIEN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTIEN0W<'a> {
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
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _DTIEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTIEN1W<'a> {
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
            const OFFSET: u8 = 9;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _DTIEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTIEN2W<'a> {
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
            const OFFSET: u8 = 10;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _DTIEN3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTIEN3W<'a> {
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
            const OFFSET: u8 = 11;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _DTLSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTLSW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _DTHSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DTHSW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
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
        #[doc = "Bits 0:1 - Output Matrix"]
        #[inline(always)]
        pub fn otmx(&self) -> OTMXR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            OTMXR { bits }
        }
        #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
        #[inline(always)]
        pub fn dtien0(&self) -> DTIEN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTIEN0R { bits }
        }
        #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
        #[inline(always)]
        pub fn dtien1(&self) -> DTIEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTIEN1R { bits }
        }
        #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
        #[inline(always)]
        pub fn dtien2(&self) -> DTIEN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTIEN2R { bits }
        }
        #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
        #[inline(always)]
        pub fn dtien3(&self) -> DTIEN3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DTIEN3R { bits }
        }
        #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
        #[inline(always)]
        pub fn dtls(&self) -> DTLSR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DTLSR { bits }
        }
        #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
        #[inline(always)]
        pub fn dths(&self) -> DTHSR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DTHSR { bits }
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
        #[doc = "Bits 0:1 - Output Matrix"]
        #[inline(always)]
        pub fn otmx(&mut self) -> _OTMXW {
            _OTMXW { w: self }
        }
        #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
        #[inline(always)]
        pub fn dtien0(&mut self) -> _DTIEN0W {
            _DTIEN0W { w: self }
        }
        #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
        #[inline(always)]
        pub fn dtien1(&mut self) -> _DTIEN1W {
            _DTIEN1W { w: self }
        }
        #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
        #[inline(always)]
        pub fn dtien2(&mut self) -> _DTIEN2W {
            _DTIEN2W { w: self }
        }
        #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
        #[inline(always)]
        pub fn dtien3(&mut self) -> _DTIEN3W {
            _DTIEN3W { w: self }
        }
        #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
        #[inline(always)]
        pub fn dtls(&mut self) -> _DTLSW {
            _DTLSW { w: self }
        }
        #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
        #[inline(always)]
        pub fn dths(&mut self) -> _DTHSW {
            _DTHSW { w: self }
        }
    }
}
