use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - Channel"]
    pub channel: CHANNEL,
    #[doc = "0x08 - User Multiplexer"]
    pub user: USER,
    _reserved1: [u8; 2usize],
    #[doc = "0x0c - Channel Status"]
    pub chstatus: CHSTATUS,
    #[doc = "0x10 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x14 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x18 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
}
#[doc = "Channel"]
pub struct CHANNEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel"]
pub mod channel {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::CHANNEL {
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
    pub struct CHANNELR {
        bits: u8,
    }
    impl CHANNELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWEVTR {
        bits: bool,
    }
    impl SWEVTR {
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
    pub struct EVGENR {
        bits: u8,
    }
    impl EVGENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = "Possible values of the field `PATH`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PATHR {
        #[doc = "Synchronous path"]
        SYNCHRONOUS,
        #[doc = "Resynchronized path"]
        RESYNCHRONIZED,
        #[doc = "Asynchronous path"]
        ASYNCHRONOUS,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl PATHR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                PATHR::SYNCHRONOUS => 0,
                PATHR::RESYNCHRONIZED => 1,
                PATHR::ASYNCHRONOUS => 2,
                PATHR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> PATHR {
            match value {
                0 => PATHR::SYNCHRONOUS,
                1 => PATHR::RESYNCHRONIZED,
                2 => PATHR::ASYNCHRONOUS,
                i => PATHR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
        #[inline(always)]
        pub fn is_synchronous(&self) -> bool {
            *self == PATHR::SYNCHRONOUS
        }
        #[doc = "Checks if the value of the field is `RESYNCHRONIZED`"]
        #[inline(always)]
        pub fn is_resynchronized(&self) -> bool {
            *self == PATHR::RESYNCHRONIZED
        }
        #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
        #[inline(always)]
        pub fn is_asynchronous(&self) -> bool {
            *self == PATHR::ASYNCHRONOUS
        }
    }
    #[doc = "Possible values of the field `EDGSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EDGSELR {
        #[doc = "No event output when using the resynchronized or synchronous path"]
        NO_EVT_OUTPUT,
        #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
        RISING_EDGE,
        #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
        FALLING_EDGE,
        #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
        BOTH_EDGES,
    }
    impl EDGSELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                EDGSELR::NO_EVT_OUTPUT => 0,
                EDGSELR::RISING_EDGE => 1,
                EDGSELR::FALLING_EDGE => 2,
                EDGSELR::BOTH_EDGES => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> EDGSELR {
            match value {
                0 => EDGSELR::NO_EVT_OUTPUT,
                1 => EDGSELR::RISING_EDGE,
                2 => EDGSELR::FALLING_EDGE,
                3 => EDGSELR::BOTH_EDGES,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `NO_EVT_OUTPUT`"]
        #[inline(always)]
        pub fn is_no_evt_output(&self) -> bool {
            *self == EDGSELR::NO_EVT_OUTPUT
        }
        #[doc = "Checks if the value of the field is `RISING_EDGE`"]
        #[inline(always)]
        pub fn is_rising_edge(&self) -> bool {
            *self == EDGSELR::RISING_EDGE
        }
        #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
        #[inline(always)]
        pub fn is_falling_edge(&self) -> bool {
            *self == EDGSELR::FALLING_EDGE
        }
        #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
        #[inline(always)]
        pub fn is_both_edges(&self) -> bool {
            *self == EDGSELR::BOTH_EDGES
        }
    }
    #[doc = r" Proxy"]
    pub struct _CHANNELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CHANNELW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SWEVTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWEVTW<'a> {
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
    pub struct _EVGENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVGENW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `PATH`"]
    pub enum PATHW {
        #[doc = "Synchronous path"]
        SYNCHRONOUS,
        #[doc = "Resynchronized path"]
        RESYNCHRONIZED,
        #[doc = "Asynchronous path"]
        ASYNCHRONOUS,
    }
    impl PATHW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                PATHW::SYNCHRONOUS => 0,
                PATHW::RESYNCHRONIZED => 1,
                PATHW::ASYNCHRONOUS => 2,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _PATHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PATHW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: PATHW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Synchronous path"]
        #[inline(always)]
        pub fn synchronous(self) -> &'a mut W {
            self.variant(PATHW::SYNCHRONOUS)
        }
        #[doc = "Resynchronized path"]
        #[inline(always)]
        pub fn resynchronized(self) -> &'a mut W {
            self.variant(PATHW::RESYNCHRONIZED)
        }
        #[doc = "Asynchronous path"]
        #[inline(always)]
        pub fn asynchronous(self) -> &'a mut W {
            self.variant(PATHW::ASYNCHRONOUS)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `EDGSEL`"]
    pub enum EDGSELW {
        #[doc = "No event output when using the resynchronized or synchronous path"]
        NO_EVT_OUTPUT,
        #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
        RISING_EDGE,
        #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
        FALLING_EDGE,
        #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
        BOTH_EDGES,
    }
    impl EDGSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                EDGSELW::NO_EVT_OUTPUT => 0,
                EDGSELW::RISING_EDGE => 1,
                EDGSELW::FALLING_EDGE => 2,
                EDGSELW::BOTH_EDGES => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _EDGSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EDGSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: EDGSELW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "No event output when using the resynchronized or synchronous path"]
        #[inline(always)]
        pub fn no_evt_output(self) -> &'a mut W {
            self.variant(EDGSELW::NO_EVT_OUTPUT)
        }
        #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
        #[inline(always)]
        pub fn rising_edge(self) -> &'a mut W {
            self.variant(EDGSELW::RISING_EDGE)
        }
        #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
        #[inline(always)]
        pub fn falling_edge(self) -> &'a mut W {
            self.variant(EDGSELW::FALLING_EDGE)
        }
        #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
        #[inline(always)]
        pub fn both_edges(self) -> &'a mut W {
            self.variant(EDGSELW::BOTH_EDGES)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
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
        #[doc = "Bits 0:3 - Channel Selection"]
        #[inline(always)]
        pub fn channel(&self) -> CHANNELR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CHANNELR { bits }
        }
        #[doc = "Bit 8 - Software Event"]
        #[inline(always)]
        pub fn swevt(&self) -> SWEVTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWEVTR { bits }
        }
        #[doc = "Bits 16:22 - Event Generator Selection"]
        #[inline(always)]
        pub fn evgen(&self) -> EVGENR {
            let bits = {
                const MASK: u8 = 127;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            EVGENR { bits }
        }
        #[doc = "Bits 24:25 - Path Selection"]
        #[inline(always)]
        pub fn path(&self) -> PATHR {
            PATHR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 26:27 - Edge Detection Selection"]
        #[inline(always)]
        pub fn edgsel(&self) -> EDGSELR {
            EDGSELR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 26;
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
        #[doc = "Bits 0:3 - Channel Selection"]
        #[inline(always)]
        pub fn channel(&mut self) -> _CHANNELW {
            _CHANNELW { w: self }
        }
        #[doc = "Bit 8 - Software Event"]
        #[inline(always)]
        pub fn swevt(&mut self) -> _SWEVTW {
            _SWEVTW { w: self }
        }
        #[doc = "Bits 16:22 - Event Generator Selection"]
        #[inline(always)]
        pub fn evgen(&mut self) -> _EVGENW {
            _EVGENW { w: self }
        }
        #[doc = "Bits 24:25 - Path Selection"]
        #[inline(always)]
        pub fn path(&mut self) -> _PATHW {
            _PATHW { w: self }
        }
        #[doc = "Bits 26:27 - Edge Detection Selection"]
        #[inline(always)]
        pub fn edgsel(&mut self) -> _EDGSELW {
            _EDGSELW { w: self }
        }
    }
}
#[doc = "Channel Status"]
pub struct CHSTATUS {
    register: VolatileCell<u32>,
}
#[doc = "Channel Status"]
pub mod chstatus {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::CHSTATUS {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct USRRDY0R {
        bits: bool,
    }
    impl USRRDY0R {
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
    pub struct USRRDY1R {
        bits: bool,
    }
    impl USRRDY1R {
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
    pub struct USRRDY2R {
        bits: bool,
    }
    impl USRRDY2R {
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
    pub struct USRRDY3R {
        bits: bool,
    }
    impl USRRDY3R {
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
    pub struct USRRDY4R {
        bits: bool,
    }
    impl USRRDY4R {
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
    pub struct USRRDY5R {
        bits: bool,
    }
    impl USRRDY5R {
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
    pub struct USRRDY6R {
        bits: bool,
    }
    impl USRRDY6R {
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
    pub struct USRRDY7R {
        bits: bool,
    }
    impl USRRDY7R {
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
    pub struct CHBUSY0R {
        bits: bool,
    }
    impl CHBUSY0R {
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
    pub struct CHBUSY1R {
        bits: bool,
    }
    impl CHBUSY1R {
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
    pub struct CHBUSY2R {
        bits: bool,
    }
    impl CHBUSY2R {
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
    pub struct CHBUSY3R {
        bits: bool,
    }
    impl CHBUSY3R {
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
    pub struct CHBUSY4R {
        bits: bool,
    }
    impl CHBUSY4R {
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
    pub struct CHBUSY5R {
        bits: bool,
    }
    impl CHBUSY5R {
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
    pub struct CHBUSY6R {
        bits: bool,
    }
    impl CHBUSY6R {
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
    pub struct CHBUSY7R {
        bits: bool,
    }
    impl CHBUSY7R {
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
    pub struct USRRDY8R {
        bits: bool,
    }
    impl USRRDY8R {
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
    pub struct USRRDY9R {
        bits: bool,
    }
    impl USRRDY9R {
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
    pub struct USRRDY10R {
        bits: bool,
    }
    impl USRRDY10R {
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
    pub struct USRRDY11R {
        bits: bool,
    }
    impl USRRDY11R {
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
    pub struct CHBUSY8R {
        bits: bool,
    }
    impl CHBUSY8R {
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
    pub struct CHBUSY9R {
        bits: bool,
    }
    impl CHBUSY9R {
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
    pub struct CHBUSY10R {
        bits: bool,
    }
    impl CHBUSY10R {
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
    pub struct CHBUSY11R {
        bits: bool,
    }
    impl CHBUSY11R {
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
        #[doc = "Bit 0 - Channel 0 User Ready"]
        #[inline(always)]
        pub fn usrrdy0(&self) -> USRRDY0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY0R { bits }
        }
        #[doc = "Bit 1 - Channel 1 User Ready"]
        #[inline(always)]
        pub fn usrrdy1(&self) -> USRRDY1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY1R { bits }
        }
        #[doc = "Bit 2 - Channel 2 User Ready"]
        #[inline(always)]
        pub fn usrrdy2(&self) -> USRRDY2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY2R { bits }
        }
        #[doc = "Bit 3 - Channel 3 User Ready"]
        #[inline(always)]
        pub fn usrrdy3(&self) -> USRRDY3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY3R { bits }
        }
        #[doc = "Bit 4 - Channel 4 User Ready"]
        #[inline(always)]
        pub fn usrrdy4(&self) -> USRRDY4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY4R { bits }
        }
        #[doc = "Bit 5 - Channel 5 User Ready"]
        #[inline(always)]
        pub fn usrrdy5(&self) -> USRRDY5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY5R { bits }
        }
        #[doc = "Bit 6 - Channel 6 User Ready"]
        #[inline(always)]
        pub fn usrrdy6(&self) -> USRRDY6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY6R { bits }
        }
        #[doc = "Bit 7 - Channel 7 User Ready"]
        #[inline(always)]
        pub fn usrrdy7(&self) -> USRRDY7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY7R { bits }
        }
        #[doc = "Bit 8 - Channel 0 Busy"]
        #[inline(always)]
        pub fn chbusy0(&self) -> CHBUSY0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY0R { bits }
        }
        #[doc = "Bit 9 - Channel 1 Busy"]
        #[inline(always)]
        pub fn chbusy1(&self) -> CHBUSY1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY1R { bits }
        }
        #[doc = "Bit 10 - Channel 2 Busy"]
        #[inline(always)]
        pub fn chbusy2(&self) -> CHBUSY2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY2R { bits }
        }
        #[doc = "Bit 11 - Channel 3 Busy"]
        #[inline(always)]
        pub fn chbusy3(&self) -> CHBUSY3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY3R { bits }
        }
        #[doc = "Bit 12 - Channel 4 Busy"]
        #[inline(always)]
        pub fn chbusy4(&self) -> CHBUSY4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY4R { bits }
        }
        #[doc = "Bit 13 - Channel 5 Busy"]
        #[inline(always)]
        pub fn chbusy5(&self) -> CHBUSY5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY5R { bits }
        }
        #[doc = "Bit 14 - Channel 6 Busy"]
        #[inline(always)]
        pub fn chbusy6(&self) -> CHBUSY6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY6R { bits }
        }
        #[doc = "Bit 15 - Channel 7 Busy"]
        #[inline(always)]
        pub fn chbusy7(&self) -> CHBUSY7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY7R { bits }
        }
        #[doc = "Bit 16 - Channel 8 User Ready"]
        #[inline(always)]
        pub fn usrrdy8(&self) -> USRRDY8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY8R { bits }
        }
        #[doc = "Bit 17 - Channel 9 User Ready"]
        #[inline(always)]
        pub fn usrrdy9(&self) -> USRRDY9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY9R { bits }
        }
        #[doc = "Bit 18 - Channel 10 User Ready"]
        #[inline(always)]
        pub fn usrrdy10(&self) -> USRRDY10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY10R { bits }
        }
        #[doc = "Bit 19 - Channel 11 User Ready"]
        #[inline(always)]
        pub fn usrrdy11(&self) -> USRRDY11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USRRDY11R { bits }
        }
        #[doc = "Bit 24 - Channel 8 Busy"]
        #[inline(always)]
        pub fn chbusy8(&self) -> CHBUSY8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY8R { bits }
        }
        #[doc = "Bit 25 - Channel 9 Busy"]
        #[inline(always)]
        pub fn chbusy9(&self) -> CHBUSY9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY9R { bits }
        }
        #[doc = "Bit 26 - Channel 10 Busy"]
        #[inline(always)]
        pub fn chbusy10(&self) -> CHBUSY10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY10R { bits }
        }
        #[doc = "Bit 27 - Channel 11 Busy"]
        #[inline(always)]
        pub fn chbusy11(&self) -> CHBUSY11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHBUSY11R { bits }
        }
    }
}
#[doc = "Control"]
pub struct CTRL {
    register: VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrl {
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CTRL {
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
    pub struct _GCLKREQW<'a> {
        w: &'a mut W,
    }
    impl<'a> _GCLKREQW<'a> {
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
        #[doc = "Bit 4 - Generic Clock Requests"]
        #[inline(always)]
        pub fn gclkreq(&mut self) -> _GCLKREQW {
            _GCLKREQW { w: self }
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
    pub struct OVR0R {
        bits: bool,
    }
    impl OVR0R {
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
    pub struct OVR1R {
        bits: bool,
    }
    impl OVR1R {
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
    pub struct OVR2R {
        bits: bool,
    }
    impl OVR2R {
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
    pub struct OVR3R {
        bits: bool,
    }
    impl OVR3R {
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
    pub struct OVR4R {
        bits: bool,
    }
    impl OVR4R {
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
    pub struct OVR5R {
        bits: bool,
    }
    impl OVR5R {
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
    pub struct OVR6R {
        bits: bool,
    }
    impl OVR6R {
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
    pub struct OVR7R {
        bits: bool,
    }
    impl OVR7R {
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
    pub struct EVD0R {
        bits: bool,
    }
    impl EVD0R {
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
    pub struct EVD1R {
        bits: bool,
    }
    impl EVD1R {
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
    pub struct EVD2R {
        bits: bool,
    }
    impl EVD2R {
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
    pub struct EVD3R {
        bits: bool,
    }
    impl EVD3R {
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
    pub struct EVD4R {
        bits: bool,
    }
    impl EVD4R {
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
    pub struct EVD5R {
        bits: bool,
    }
    impl EVD5R {
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
    pub struct EVD6R {
        bits: bool,
    }
    impl EVD6R {
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
    pub struct EVD7R {
        bits: bool,
    }
    impl EVD7R {
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
    pub struct OVR8R {
        bits: bool,
    }
    impl OVR8R {
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
    pub struct OVR9R {
        bits: bool,
    }
    impl OVR9R {
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
    pub struct OVR10R {
        bits: bool,
    }
    impl OVR10R {
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
    pub struct OVR11R {
        bits: bool,
    }
    impl OVR11R {
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
    pub struct EVD8R {
        bits: bool,
    }
    impl EVD8R {
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
    pub struct EVD9R {
        bits: bool,
    }
    impl EVD9R {
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
    pub struct EVD10R {
        bits: bool,
    }
    impl EVD10R {
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
    pub struct EVD11R {
        bits: bool,
    }
    impl EVD11R {
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
    pub struct _OVR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR0W<'a> {
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
    pub struct _OVR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR1W<'a> {
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
    pub struct _OVR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR2W<'a> {
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
    pub struct _OVR3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR3W<'a> {
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
    pub struct _OVR4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR4W<'a> {
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
    pub struct _OVR5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR5W<'a> {
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
    pub struct _OVR6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR6W<'a> {
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
    pub struct _OVR7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR7W<'a> {
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
    pub struct _EVD0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD0W<'a> {
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
    pub struct _EVD1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD1W<'a> {
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
    pub struct _EVD2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD2W<'a> {
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
    pub struct _EVD3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD3W<'a> {
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
    pub struct _EVD4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD4W<'a> {
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
    pub struct _EVD5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD5W<'a> {
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
    pub struct _EVD6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD6W<'a> {
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
    pub struct _EVD7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD7W<'a> {
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
    pub struct _OVR8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR8W<'a> {
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
    pub struct _OVR9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR9W<'a> {
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
    pub struct _OVR10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR10W<'a> {
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
    pub struct _OVR11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR11W<'a> {
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
    pub struct _EVD8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD8W<'a> {
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
    pub struct _EVD9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD9W<'a> {
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
    pub struct _EVD10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD10W<'a> {
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
    pub struct _EVD11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD11W<'a> {
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
        #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr0(&self) -> OVR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR0R { bits }
        }
        #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr1(&self) -> OVR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR1R { bits }
        }
        #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr2(&self) -> OVR2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR2R { bits }
        }
        #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr3(&self) -> OVR3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR3R { bits }
        }
        #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr4(&self) -> OVR4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR4R { bits }
        }
        #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr5(&self) -> OVR5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR5R { bits }
        }
        #[doc = "Bit 6 - Channel 6 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr6(&self) -> OVR6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR6R { bits }
        }
        #[doc = "Bit 7 - Channel 7 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr7(&self) -> OVR7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR7R { bits }
        }
        #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd0(&self) -> EVD0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD0R { bits }
        }
        #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd1(&self) -> EVD1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD1R { bits }
        }
        #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd2(&self) -> EVD2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD2R { bits }
        }
        #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd3(&self) -> EVD3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD3R { bits }
        }
        #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd4(&self) -> EVD4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD4R { bits }
        }
        #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd5(&self) -> EVD5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD5R { bits }
        }
        #[doc = "Bit 14 - Channel 6 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd6(&self) -> EVD6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD6R { bits }
        }
        #[doc = "Bit 15 - Channel 7 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd7(&self) -> EVD7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD7R { bits }
        }
        #[doc = "Bit 16 - Channel 8 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr8(&self) -> OVR8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR8R { bits }
        }
        #[doc = "Bit 17 - Channel 9 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr9(&self) -> OVR9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR9R { bits }
        }
        #[doc = "Bit 18 - Channel 10 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr10(&self) -> OVR10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR10R { bits }
        }
        #[doc = "Bit 19 - Channel 11 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr11(&self) -> OVR11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR11R { bits }
        }
        #[doc = "Bit 24 - Channel 8 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd8(&self) -> EVD8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD8R { bits }
        }
        #[doc = "Bit 25 - Channel 9 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd9(&self) -> EVD9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD9R { bits }
        }
        #[doc = "Bit 26 - Channel 10 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd10(&self) -> EVD10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD10R { bits }
        }
        #[doc = "Bit 27 - Channel 11 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd11(&self) -> EVD11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD11R { bits }
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
        #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr0(&mut self) -> _OVR0W {
            _OVR0W { w: self }
        }
        #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr1(&mut self) -> _OVR1W {
            _OVR1W { w: self }
        }
        #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr2(&mut self) -> _OVR2W {
            _OVR2W { w: self }
        }
        #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr3(&mut self) -> _OVR3W {
            _OVR3W { w: self }
        }
        #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr4(&mut self) -> _OVR4W {
            _OVR4W { w: self }
        }
        #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr5(&mut self) -> _OVR5W {
            _OVR5W { w: self }
        }
        #[doc = "Bit 6 - Channel 6 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr6(&mut self) -> _OVR6W {
            _OVR6W { w: self }
        }
        #[doc = "Bit 7 - Channel 7 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr7(&mut self) -> _OVR7W {
            _OVR7W { w: self }
        }
        #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd0(&mut self) -> _EVD0W {
            _EVD0W { w: self }
        }
        #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd1(&mut self) -> _EVD1W {
            _EVD1W { w: self }
        }
        #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd2(&mut self) -> _EVD2W {
            _EVD2W { w: self }
        }
        #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd3(&mut self) -> _EVD3W {
            _EVD3W { w: self }
        }
        #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd4(&mut self) -> _EVD4W {
            _EVD4W { w: self }
        }
        #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd5(&mut self) -> _EVD5W {
            _EVD5W { w: self }
        }
        #[doc = "Bit 14 - Channel 6 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd6(&mut self) -> _EVD6W {
            _EVD6W { w: self }
        }
        #[doc = "Bit 15 - Channel 7 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd7(&mut self) -> _EVD7W {
            _EVD7W { w: self }
        }
        #[doc = "Bit 16 - Channel 8 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr8(&mut self) -> _OVR8W {
            _OVR8W { w: self }
        }
        #[doc = "Bit 17 - Channel 9 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr9(&mut self) -> _OVR9W {
            _OVR9W { w: self }
        }
        #[doc = "Bit 18 - Channel 10 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr10(&mut self) -> _OVR10W {
            _OVR10W { w: self }
        }
        #[doc = "Bit 19 - Channel 11 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr11(&mut self) -> _OVR11W {
            _OVR11W { w: self }
        }
        #[doc = "Bit 24 - Channel 8 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd8(&mut self) -> _EVD8W {
            _EVD8W { w: self }
        }
        #[doc = "Bit 25 - Channel 9 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd9(&mut self) -> _EVD9W {
            _EVD9W { w: self }
        }
        #[doc = "Bit 26 - Channel 10 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd10(&mut self) -> _EVD10W {
            _EVD10W { w: self }
        }
        #[doc = "Bit 27 - Channel 11 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd11(&mut self) -> _EVD11W {
            _EVD11W { w: self }
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
    pub struct OVR0R {
        bits: bool,
    }
    impl OVR0R {
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
    pub struct OVR1R {
        bits: bool,
    }
    impl OVR1R {
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
    pub struct OVR2R {
        bits: bool,
    }
    impl OVR2R {
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
    pub struct OVR3R {
        bits: bool,
    }
    impl OVR3R {
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
    pub struct OVR4R {
        bits: bool,
    }
    impl OVR4R {
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
    pub struct OVR5R {
        bits: bool,
    }
    impl OVR5R {
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
    pub struct OVR6R {
        bits: bool,
    }
    impl OVR6R {
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
    pub struct OVR7R {
        bits: bool,
    }
    impl OVR7R {
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
    pub struct EVD0R {
        bits: bool,
    }
    impl EVD0R {
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
    pub struct EVD1R {
        bits: bool,
    }
    impl EVD1R {
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
    pub struct EVD2R {
        bits: bool,
    }
    impl EVD2R {
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
    pub struct EVD3R {
        bits: bool,
    }
    impl EVD3R {
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
    pub struct EVD4R {
        bits: bool,
    }
    impl EVD4R {
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
    pub struct EVD5R {
        bits: bool,
    }
    impl EVD5R {
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
    pub struct EVD6R {
        bits: bool,
    }
    impl EVD6R {
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
    pub struct EVD7R {
        bits: bool,
    }
    impl EVD7R {
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
    pub struct OVR8R {
        bits: bool,
    }
    impl OVR8R {
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
    pub struct OVR9R {
        bits: bool,
    }
    impl OVR9R {
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
    pub struct OVR10R {
        bits: bool,
    }
    impl OVR10R {
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
    pub struct OVR11R {
        bits: bool,
    }
    impl OVR11R {
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
    pub struct EVD8R {
        bits: bool,
    }
    impl EVD8R {
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
    pub struct EVD9R {
        bits: bool,
    }
    impl EVD9R {
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
    pub struct EVD10R {
        bits: bool,
    }
    impl EVD10R {
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
    pub struct EVD11R {
        bits: bool,
    }
    impl EVD11R {
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
    pub struct _OVR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR0W<'a> {
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
    pub struct _OVR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR1W<'a> {
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
    pub struct _OVR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR2W<'a> {
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
    pub struct _OVR3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR3W<'a> {
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
    pub struct _OVR4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR4W<'a> {
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
    pub struct _OVR5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR5W<'a> {
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
    pub struct _OVR6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR6W<'a> {
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
    pub struct _OVR7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR7W<'a> {
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
    pub struct _EVD0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD0W<'a> {
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
    pub struct _EVD1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD1W<'a> {
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
    pub struct _EVD2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD2W<'a> {
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
    pub struct _EVD3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD3W<'a> {
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
    pub struct _EVD4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD4W<'a> {
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
    pub struct _EVD5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD5W<'a> {
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
    pub struct _EVD6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD6W<'a> {
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
    pub struct _EVD7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD7W<'a> {
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
    pub struct _OVR8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR8W<'a> {
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
    pub struct _OVR9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR9W<'a> {
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
    pub struct _OVR10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR10W<'a> {
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
    pub struct _OVR11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR11W<'a> {
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
    pub struct _EVD8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD8W<'a> {
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
    pub struct _EVD9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD9W<'a> {
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
    pub struct _EVD10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD10W<'a> {
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
    pub struct _EVD11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD11W<'a> {
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
        #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr0(&self) -> OVR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR0R { bits }
        }
        #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr1(&self) -> OVR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR1R { bits }
        }
        #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr2(&self) -> OVR2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR2R { bits }
        }
        #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr3(&self) -> OVR3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR3R { bits }
        }
        #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr4(&self) -> OVR4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR4R { bits }
        }
        #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr5(&self) -> OVR5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR5R { bits }
        }
        #[doc = "Bit 6 - Channel 6 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr6(&self) -> OVR6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR6R { bits }
        }
        #[doc = "Bit 7 - Channel 7 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr7(&self) -> OVR7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR7R { bits }
        }
        #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd0(&self) -> EVD0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD0R { bits }
        }
        #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd1(&self) -> EVD1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD1R { bits }
        }
        #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd2(&self) -> EVD2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD2R { bits }
        }
        #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd3(&self) -> EVD3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD3R { bits }
        }
        #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd4(&self) -> EVD4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD4R { bits }
        }
        #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd5(&self) -> EVD5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD5R { bits }
        }
        #[doc = "Bit 14 - Channel 6 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd6(&self) -> EVD6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD6R { bits }
        }
        #[doc = "Bit 15 - Channel 7 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd7(&self) -> EVD7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD7R { bits }
        }
        #[doc = "Bit 16 - Channel 8 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr8(&self) -> OVR8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR8R { bits }
        }
        #[doc = "Bit 17 - Channel 9 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr9(&self) -> OVR9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR9R { bits }
        }
        #[doc = "Bit 18 - Channel 10 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr10(&self) -> OVR10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR10R { bits }
        }
        #[doc = "Bit 19 - Channel 11 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr11(&self) -> OVR11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR11R { bits }
        }
        #[doc = "Bit 24 - Channel 8 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd8(&self) -> EVD8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD8R { bits }
        }
        #[doc = "Bit 25 - Channel 9 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd9(&self) -> EVD9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD9R { bits }
        }
        #[doc = "Bit 26 - Channel 10 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd10(&self) -> EVD10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD10R { bits }
        }
        #[doc = "Bit 27 - Channel 11 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd11(&self) -> EVD11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD11R { bits }
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
        #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr0(&mut self) -> _OVR0W {
            _OVR0W { w: self }
        }
        #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr1(&mut self) -> _OVR1W {
            _OVR1W { w: self }
        }
        #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr2(&mut self) -> _OVR2W {
            _OVR2W { w: self }
        }
        #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr3(&mut self) -> _OVR3W {
            _OVR3W { w: self }
        }
        #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr4(&mut self) -> _OVR4W {
            _OVR4W { w: self }
        }
        #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr5(&mut self) -> _OVR5W {
            _OVR5W { w: self }
        }
        #[doc = "Bit 6 - Channel 6 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr6(&mut self) -> _OVR6W {
            _OVR6W { w: self }
        }
        #[doc = "Bit 7 - Channel 7 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr7(&mut self) -> _OVR7W {
            _OVR7W { w: self }
        }
        #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd0(&mut self) -> _EVD0W {
            _EVD0W { w: self }
        }
        #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd1(&mut self) -> _EVD1W {
            _EVD1W { w: self }
        }
        #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd2(&mut self) -> _EVD2W {
            _EVD2W { w: self }
        }
        #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd3(&mut self) -> _EVD3W {
            _EVD3W { w: self }
        }
        #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd4(&mut self) -> _EVD4W {
            _EVD4W { w: self }
        }
        #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd5(&mut self) -> _EVD5W {
            _EVD5W { w: self }
        }
        #[doc = "Bit 14 - Channel 6 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd6(&mut self) -> _EVD6W {
            _EVD6W { w: self }
        }
        #[doc = "Bit 15 - Channel 7 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd7(&mut self) -> _EVD7W {
            _EVD7W { w: self }
        }
        #[doc = "Bit 16 - Channel 8 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr8(&mut self) -> _OVR8W {
            _OVR8W { w: self }
        }
        #[doc = "Bit 17 - Channel 9 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr9(&mut self) -> _OVR9W {
            _OVR9W { w: self }
        }
        #[doc = "Bit 18 - Channel 10 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr10(&mut self) -> _OVR10W {
            _OVR10W { w: self }
        }
        #[doc = "Bit 19 - Channel 11 Overrun Interrupt Enable"]
        #[inline(always)]
        pub fn ovr11(&mut self) -> _OVR11W {
            _OVR11W { w: self }
        }
        #[doc = "Bit 24 - Channel 8 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd8(&mut self) -> _EVD8W {
            _EVD8W { w: self }
        }
        #[doc = "Bit 25 - Channel 9 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd9(&mut self) -> _EVD9W {
            _EVD9W { w: self }
        }
        #[doc = "Bit 26 - Channel 10 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd10(&mut self) -> _EVD10W {
            _EVD10W { w: self }
        }
        #[doc = "Bit 27 - Channel 11 Event Detection Interrupt Enable"]
        #[inline(always)]
        pub fn evd11(&mut self) -> _EVD11W {
            _EVD11W { w: self }
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
    pub struct OVR0R {
        bits: bool,
    }
    impl OVR0R {
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
    pub struct OVR1R {
        bits: bool,
    }
    impl OVR1R {
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
    pub struct OVR2R {
        bits: bool,
    }
    impl OVR2R {
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
    pub struct OVR3R {
        bits: bool,
    }
    impl OVR3R {
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
    pub struct OVR4R {
        bits: bool,
    }
    impl OVR4R {
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
    pub struct OVR5R {
        bits: bool,
    }
    impl OVR5R {
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
    pub struct OVR6R {
        bits: bool,
    }
    impl OVR6R {
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
    pub struct OVR7R {
        bits: bool,
    }
    impl OVR7R {
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
    pub struct EVD0R {
        bits: bool,
    }
    impl EVD0R {
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
    pub struct EVD1R {
        bits: bool,
    }
    impl EVD1R {
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
    pub struct EVD2R {
        bits: bool,
    }
    impl EVD2R {
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
    pub struct EVD3R {
        bits: bool,
    }
    impl EVD3R {
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
    pub struct EVD4R {
        bits: bool,
    }
    impl EVD4R {
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
    pub struct EVD5R {
        bits: bool,
    }
    impl EVD5R {
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
    pub struct EVD6R {
        bits: bool,
    }
    impl EVD6R {
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
    pub struct EVD7R {
        bits: bool,
    }
    impl EVD7R {
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
    pub struct OVR8R {
        bits: bool,
    }
    impl OVR8R {
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
    pub struct OVR9R {
        bits: bool,
    }
    impl OVR9R {
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
    pub struct OVR10R {
        bits: bool,
    }
    impl OVR10R {
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
    pub struct OVR11R {
        bits: bool,
    }
    impl OVR11R {
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
    pub struct EVD8R {
        bits: bool,
    }
    impl EVD8R {
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
    pub struct EVD9R {
        bits: bool,
    }
    impl EVD9R {
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
    pub struct EVD10R {
        bits: bool,
    }
    impl EVD10R {
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
    pub struct EVD11R {
        bits: bool,
    }
    impl EVD11R {
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
    pub struct _OVR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR0W<'a> {
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
    pub struct _OVR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR1W<'a> {
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
    pub struct _OVR2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR2W<'a> {
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
    pub struct _OVR3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR3W<'a> {
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
    pub struct _OVR4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR4W<'a> {
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
    pub struct _OVR5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR5W<'a> {
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
    pub struct _OVR6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR6W<'a> {
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
    pub struct _OVR7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR7W<'a> {
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
    pub struct _EVD0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD0W<'a> {
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
    pub struct _EVD1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD1W<'a> {
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
    pub struct _EVD2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD2W<'a> {
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
    pub struct _EVD3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD3W<'a> {
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
    pub struct _EVD4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD4W<'a> {
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
    pub struct _EVD5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD5W<'a> {
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
    pub struct _EVD6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD6W<'a> {
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
    pub struct _EVD7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD7W<'a> {
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
    pub struct _OVR8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR8W<'a> {
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
    pub struct _OVR9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR9W<'a> {
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
    pub struct _OVR10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR10W<'a> {
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
    pub struct _OVR11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _OVR11W<'a> {
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
    pub struct _EVD8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD8W<'a> {
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
    pub struct _EVD9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD9W<'a> {
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
    pub struct _EVD10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD10W<'a> {
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
    pub struct _EVD11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVD11W<'a> {
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
        #[doc = "Bit 0 - Channel 0 Overrun"]
        #[inline(always)]
        pub fn ovr0(&self) -> OVR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR0R { bits }
        }
        #[doc = "Bit 1 - Channel 1 Overrun"]
        #[inline(always)]
        pub fn ovr1(&self) -> OVR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR1R { bits }
        }
        #[doc = "Bit 2 - Channel 2 Overrun"]
        #[inline(always)]
        pub fn ovr2(&self) -> OVR2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR2R { bits }
        }
        #[doc = "Bit 3 - Channel 3 Overrun"]
        #[inline(always)]
        pub fn ovr3(&self) -> OVR3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR3R { bits }
        }
        #[doc = "Bit 4 - Channel 4 Overrun"]
        #[inline(always)]
        pub fn ovr4(&self) -> OVR4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR4R { bits }
        }
        #[doc = "Bit 5 - Channel 5 Overrun"]
        #[inline(always)]
        pub fn ovr5(&self) -> OVR5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR5R { bits }
        }
        #[doc = "Bit 6 - Channel 6 Overrun"]
        #[inline(always)]
        pub fn ovr6(&self) -> OVR6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR6R { bits }
        }
        #[doc = "Bit 7 - Channel 7 Overrun"]
        #[inline(always)]
        pub fn ovr7(&self) -> OVR7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR7R { bits }
        }
        #[doc = "Bit 8 - Channel 0 Event Detection"]
        #[inline(always)]
        pub fn evd0(&self) -> EVD0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD0R { bits }
        }
        #[doc = "Bit 9 - Channel 1 Event Detection"]
        #[inline(always)]
        pub fn evd1(&self) -> EVD1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD1R { bits }
        }
        #[doc = "Bit 10 - Channel 2 Event Detection"]
        #[inline(always)]
        pub fn evd2(&self) -> EVD2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD2R { bits }
        }
        #[doc = "Bit 11 - Channel 3 Event Detection"]
        #[inline(always)]
        pub fn evd3(&self) -> EVD3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD3R { bits }
        }
        #[doc = "Bit 12 - Channel 4 Event Detection"]
        #[inline(always)]
        pub fn evd4(&self) -> EVD4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD4R { bits }
        }
        #[doc = "Bit 13 - Channel 5 Event Detection"]
        #[inline(always)]
        pub fn evd5(&self) -> EVD5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD5R { bits }
        }
        #[doc = "Bit 14 - Channel 6 Event Detection"]
        #[inline(always)]
        pub fn evd6(&self) -> EVD6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD6R { bits }
        }
        #[doc = "Bit 15 - Channel 7 Event Detection"]
        #[inline(always)]
        pub fn evd7(&self) -> EVD7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD7R { bits }
        }
        #[doc = "Bit 16 - Channel 8 Overrun"]
        #[inline(always)]
        pub fn ovr8(&self) -> OVR8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR8R { bits }
        }
        #[doc = "Bit 17 - Channel 9 Overrun"]
        #[inline(always)]
        pub fn ovr9(&self) -> OVR9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR9R { bits }
        }
        #[doc = "Bit 18 - Channel 10 Overrun"]
        #[inline(always)]
        pub fn ovr10(&self) -> OVR10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR10R { bits }
        }
        #[doc = "Bit 19 - Channel 11 Overrun"]
        #[inline(always)]
        pub fn ovr11(&self) -> OVR11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OVR11R { bits }
        }
        #[doc = "Bit 24 - Channel 8 Event Detection"]
        #[inline(always)]
        pub fn evd8(&self) -> EVD8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD8R { bits }
        }
        #[doc = "Bit 25 - Channel 9 Event Detection"]
        #[inline(always)]
        pub fn evd9(&self) -> EVD9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD9R { bits }
        }
        #[doc = "Bit 26 - Channel 10 Event Detection"]
        #[inline(always)]
        pub fn evd10(&self) -> EVD10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD10R { bits }
        }
        #[doc = "Bit 27 - Channel 11 Event Detection"]
        #[inline(always)]
        pub fn evd11(&self) -> EVD11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVD11R { bits }
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
        #[doc = "Bit 0 - Channel 0 Overrun"]
        #[inline(always)]
        pub fn ovr0(&mut self) -> _OVR0W {
            _OVR0W { w: self }
        }
        #[doc = "Bit 1 - Channel 1 Overrun"]
        #[inline(always)]
        pub fn ovr1(&mut self) -> _OVR1W {
            _OVR1W { w: self }
        }
        #[doc = "Bit 2 - Channel 2 Overrun"]
        #[inline(always)]
        pub fn ovr2(&mut self) -> _OVR2W {
            _OVR2W { w: self }
        }
        #[doc = "Bit 3 - Channel 3 Overrun"]
        #[inline(always)]
        pub fn ovr3(&mut self) -> _OVR3W {
            _OVR3W { w: self }
        }
        #[doc = "Bit 4 - Channel 4 Overrun"]
        #[inline(always)]
        pub fn ovr4(&mut self) -> _OVR4W {
            _OVR4W { w: self }
        }
        #[doc = "Bit 5 - Channel 5 Overrun"]
        #[inline(always)]
        pub fn ovr5(&mut self) -> _OVR5W {
            _OVR5W { w: self }
        }
        #[doc = "Bit 6 - Channel 6 Overrun"]
        #[inline(always)]
        pub fn ovr6(&mut self) -> _OVR6W {
            _OVR6W { w: self }
        }
        #[doc = "Bit 7 - Channel 7 Overrun"]
        #[inline(always)]
        pub fn ovr7(&mut self) -> _OVR7W {
            _OVR7W { w: self }
        }
        #[doc = "Bit 8 - Channel 0 Event Detection"]
        #[inline(always)]
        pub fn evd0(&mut self) -> _EVD0W {
            _EVD0W { w: self }
        }
        #[doc = "Bit 9 - Channel 1 Event Detection"]
        #[inline(always)]
        pub fn evd1(&mut self) -> _EVD1W {
            _EVD1W { w: self }
        }
        #[doc = "Bit 10 - Channel 2 Event Detection"]
        #[inline(always)]
        pub fn evd2(&mut self) -> _EVD2W {
            _EVD2W { w: self }
        }
        #[doc = "Bit 11 - Channel 3 Event Detection"]
        #[inline(always)]
        pub fn evd3(&mut self) -> _EVD3W {
            _EVD3W { w: self }
        }
        #[doc = "Bit 12 - Channel 4 Event Detection"]
        #[inline(always)]
        pub fn evd4(&mut self) -> _EVD4W {
            _EVD4W { w: self }
        }
        #[doc = "Bit 13 - Channel 5 Event Detection"]
        #[inline(always)]
        pub fn evd5(&mut self) -> _EVD5W {
            _EVD5W { w: self }
        }
        #[doc = "Bit 14 - Channel 6 Event Detection"]
        #[inline(always)]
        pub fn evd6(&mut self) -> _EVD6W {
            _EVD6W { w: self }
        }
        #[doc = "Bit 15 - Channel 7 Event Detection"]
        #[inline(always)]
        pub fn evd7(&mut self) -> _EVD7W {
            _EVD7W { w: self }
        }
        #[doc = "Bit 16 - Channel 8 Overrun"]
        #[inline(always)]
        pub fn ovr8(&mut self) -> _OVR8W {
            _OVR8W { w: self }
        }
        #[doc = "Bit 17 - Channel 9 Overrun"]
        #[inline(always)]
        pub fn ovr9(&mut self) -> _OVR9W {
            _OVR9W { w: self }
        }
        #[doc = "Bit 18 - Channel 10 Overrun"]
        #[inline(always)]
        pub fn ovr10(&mut self) -> _OVR10W {
            _OVR10W { w: self }
        }
        #[doc = "Bit 19 - Channel 11 Overrun"]
        #[inline(always)]
        pub fn ovr11(&mut self) -> _OVR11W {
            _OVR11W { w: self }
        }
        #[doc = "Bit 24 - Channel 8 Event Detection"]
        #[inline(always)]
        pub fn evd8(&mut self) -> _EVD8W {
            _EVD8W { w: self }
        }
        #[doc = "Bit 25 - Channel 9 Event Detection"]
        #[inline(always)]
        pub fn evd9(&mut self) -> _EVD9W {
            _EVD9W { w: self }
        }
        #[doc = "Bit 26 - Channel 10 Event Detection"]
        #[inline(always)]
        pub fn evd10(&mut self) -> _EVD10W {
            _EVD10W { w: self }
        }
        #[doc = "Bit 27 - Channel 11 Event Detection"]
        #[inline(always)]
        pub fn evd11(&mut self) -> _EVD11W {
            _EVD11W { w: self }
        }
    }
}
#[doc = "User Multiplexer"]
pub struct USER {
    register: VolatileCell<u16>,
}
#[doc = "User Multiplexer"]
pub mod user {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::USER {
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
    pub struct USERR {
        bits: u8,
    }
    impl USERR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = "Possible values of the field `CHANNEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CHANNELR {
        #[doc = "No Channel Output Selected"]
        _0,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CHANNELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CHANNELR::_0 => 0,
                CHANNELR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CHANNELR {
            match value {
                0 => CHANNELR::_0,
                i => CHANNELR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `_0`"]
        #[inline(always)]
        pub fn is_0(&self) -> bool {
            *self == CHANNELR::_0
        }
    }
    #[doc = r" Proxy"]
    pub struct _USERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USERW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CHANNEL`"]
    pub enum CHANNELW {
        #[doc = "No Channel Output Selected"]
        _0,
    }
    impl CHANNELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CHANNELW::_0 => 0,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CHANNELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CHANNELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CHANNELW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No Channel Output Selected"]
        #[inline(always)]
        pub fn _0(self) -> &'a mut W {
            self.variant(CHANNELW::_0)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
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
        #[doc = "Bits 0:4 - User Multiplexer Selection"]
        #[inline(always)]
        pub fn user(&self) -> USERR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            };
            USERR { bits }
        }
        #[doc = "Bits 8:12 - Channel Event Selection"]
        #[inline(always)]
        pub fn channel(&self) -> CHANNELR {
            CHANNELR::_from({
                const MASK: u8 = 31;
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
        #[doc = "Bits 0:4 - User Multiplexer Selection"]
        #[inline(always)]
        pub fn user(&mut self) -> _USERW {
            _USERW { w: self }
        }
        #[doc = "Bits 8:12 - Channel Event Selection"]
        #[inline(always)]
        pub fn channel(&mut self) -> _CHANNELW {
            _CHANNELW { w: self }
        }
    }
}
