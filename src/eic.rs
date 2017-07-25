use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Status"]
    pub status: STATUS,
    #[doc = "0x02 - Non-Maskable Interrupt Control"]
    pub nmictrl: NMICTRL,
    #[doc = "0x03 - Non-Maskable Interrupt Flag Status and Clear"]
    pub nmiflag: NMIFLAG,
    #[doc = "0x04 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x0c - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x10 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x14 - Wake-Up Enable"]
    pub wakeup: WAKEUP,
    #[doc = "0x18 - Configuration n"]
    pub config0: CONFIG,
    #[doc = "0x1c - Configuration n"]
    pub config1: CONFIG,
}
#[doc = "Configuration n"]
pub struct CONFIG {
    register: VolatileCell<u32>,
}
#[doc = "Configuration n"]
pub mod config {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::CONFIG {
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
    #[doc = "Possible values of the field `SENSE0`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SENSE0R {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising-edge detection"]
        RISE,
        #[doc = "Falling-edge detection"]
        FALL,
        #[doc = "Both-edges detection"]
        BOTH,
        #[doc = "High-level detection"]
        HIGH,
        #[doc = "Low-level detection"]
        LOW,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SENSE0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SENSE0R::NONE => 0,
                SENSE0R::RISE => 1,
                SENSE0R::FALL => 2,
                SENSE0R::BOTH => 3,
                SENSE0R::HIGH => 4,
                SENSE0R::LOW => 5,
                SENSE0R::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SENSE0R {
            match value {
                0 => SENSE0R::NONE,
                1 => SENSE0R::RISE,
                2 => SENSE0R::FALL,
                3 => SENSE0R::BOTH,
                4 => SENSE0R::HIGH,
                5 => SENSE0R::LOW,
                i => SENSE0R::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == SENSE0R::NONE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == SENSE0R::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == SENSE0R::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == SENSE0R::BOTH
        }
        #[doc = "Checks if the value of the field is `HIGH`"]
        #[inline(always)]
        pub fn is_high(&self) -> bool {
            *self == SENSE0R::HIGH
        }
        #[doc = "Checks if the value of the field is `LOW`"]
        #[inline(always)]
        pub fn is_low(&self) -> bool {
            *self == SENSE0R::LOW
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTEN0R {
        bits: bool,
    }
    impl FILTEN0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `SENSE1`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SENSE1R {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SENSE1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SENSE1R::NONE => 0,
                SENSE1R::RISE => 1,
                SENSE1R::FALL => 2,
                SENSE1R::BOTH => 3,
                SENSE1R::HIGH => 4,
                SENSE1R::LOW => 5,
                SENSE1R::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SENSE1R {
            match value {
                0 => SENSE1R::NONE,
                1 => SENSE1R::RISE,
                2 => SENSE1R::FALL,
                3 => SENSE1R::BOTH,
                4 => SENSE1R::HIGH,
                5 => SENSE1R::LOW,
                i => SENSE1R::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == SENSE1R::NONE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == SENSE1R::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == SENSE1R::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == SENSE1R::BOTH
        }
        #[doc = "Checks if the value of the field is `HIGH`"]
        #[inline(always)]
        pub fn is_high(&self) -> bool {
            *self == SENSE1R::HIGH
        }
        #[doc = "Checks if the value of the field is `LOW`"]
        #[inline(always)]
        pub fn is_low(&self) -> bool {
            *self == SENSE1R::LOW
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTEN1R {
        bits: bool,
    }
    impl FILTEN1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `SENSE2`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SENSE2R {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SENSE2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SENSE2R::NONE => 0,
                SENSE2R::RISE => 1,
                SENSE2R::FALL => 2,
                SENSE2R::BOTH => 3,
                SENSE2R::HIGH => 4,
                SENSE2R::LOW => 5,
                SENSE2R::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SENSE2R {
            match value {
                0 => SENSE2R::NONE,
                1 => SENSE2R::RISE,
                2 => SENSE2R::FALL,
                3 => SENSE2R::BOTH,
                4 => SENSE2R::HIGH,
                5 => SENSE2R::LOW,
                i => SENSE2R::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == SENSE2R::NONE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == SENSE2R::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == SENSE2R::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == SENSE2R::BOTH
        }
        #[doc = "Checks if the value of the field is `HIGH`"]
        #[inline(always)]
        pub fn is_high(&self) -> bool {
            *self == SENSE2R::HIGH
        }
        #[doc = "Checks if the value of the field is `LOW`"]
        #[inline(always)]
        pub fn is_low(&self) -> bool {
            *self == SENSE2R::LOW
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTEN2R {
        bits: bool,
    }
    impl FILTEN2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `SENSE3`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SENSE3R {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SENSE3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SENSE3R::NONE => 0,
                SENSE3R::RISE => 1,
                SENSE3R::FALL => 2,
                SENSE3R::BOTH => 3,
                SENSE3R::HIGH => 4,
                SENSE3R::LOW => 5,
                SENSE3R::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SENSE3R {
            match value {
                0 => SENSE3R::NONE,
                1 => SENSE3R::RISE,
                2 => SENSE3R::FALL,
                3 => SENSE3R::BOTH,
                4 => SENSE3R::HIGH,
                5 => SENSE3R::LOW,
                i => SENSE3R::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == SENSE3R::NONE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == SENSE3R::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == SENSE3R::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == SENSE3R::BOTH
        }
        #[doc = "Checks if the value of the field is `HIGH`"]
        #[inline(always)]
        pub fn is_high(&self) -> bool {
            *self == SENSE3R::HIGH
        }
        #[doc = "Checks if the value of the field is `LOW`"]
        #[inline(always)]
        pub fn is_low(&self) -> bool {
            *self == SENSE3R::LOW
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTEN3R {
        bits: bool,
    }
    impl FILTEN3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `SENSE4`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SENSE4R {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SENSE4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SENSE4R::NONE => 0,
                SENSE4R::RISE => 1,
                SENSE4R::FALL => 2,
                SENSE4R::BOTH => 3,
                SENSE4R::HIGH => 4,
                SENSE4R::LOW => 5,
                SENSE4R::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SENSE4R {
            match value {
                0 => SENSE4R::NONE,
                1 => SENSE4R::RISE,
                2 => SENSE4R::FALL,
                3 => SENSE4R::BOTH,
                4 => SENSE4R::HIGH,
                5 => SENSE4R::LOW,
                i => SENSE4R::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == SENSE4R::NONE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == SENSE4R::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == SENSE4R::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == SENSE4R::BOTH
        }
        #[doc = "Checks if the value of the field is `HIGH`"]
        #[inline(always)]
        pub fn is_high(&self) -> bool {
            *self == SENSE4R::HIGH
        }
        #[doc = "Checks if the value of the field is `LOW`"]
        #[inline(always)]
        pub fn is_low(&self) -> bool {
            *self == SENSE4R::LOW
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTEN4R {
        bits: bool,
    }
    impl FILTEN4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `SENSE5`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SENSE5R {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SENSE5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SENSE5R::NONE => 0,
                SENSE5R::RISE => 1,
                SENSE5R::FALL => 2,
                SENSE5R::BOTH => 3,
                SENSE5R::HIGH => 4,
                SENSE5R::LOW => 5,
                SENSE5R::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SENSE5R {
            match value {
                0 => SENSE5R::NONE,
                1 => SENSE5R::RISE,
                2 => SENSE5R::FALL,
                3 => SENSE5R::BOTH,
                4 => SENSE5R::HIGH,
                5 => SENSE5R::LOW,
                i => SENSE5R::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == SENSE5R::NONE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == SENSE5R::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == SENSE5R::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == SENSE5R::BOTH
        }
        #[doc = "Checks if the value of the field is `HIGH`"]
        #[inline(always)]
        pub fn is_high(&self) -> bool {
            *self == SENSE5R::HIGH
        }
        #[doc = "Checks if the value of the field is `LOW`"]
        #[inline(always)]
        pub fn is_low(&self) -> bool {
            *self == SENSE5R::LOW
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTEN5R {
        bits: bool,
    }
    impl FILTEN5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `SENSE6`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SENSE6R {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SENSE6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SENSE6R::NONE => 0,
                SENSE6R::RISE => 1,
                SENSE6R::FALL => 2,
                SENSE6R::BOTH => 3,
                SENSE6R::HIGH => 4,
                SENSE6R::LOW => 5,
                SENSE6R::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SENSE6R {
            match value {
                0 => SENSE6R::NONE,
                1 => SENSE6R::RISE,
                2 => SENSE6R::FALL,
                3 => SENSE6R::BOTH,
                4 => SENSE6R::HIGH,
                5 => SENSE6R::LOW,
                i => SENSE6R::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == SENSE6R::NONE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == SENSE6R::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == SENSE6R::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == SENSE6R::BOTH
        }
        #[doc = "Checks if the value of the field is `HIGH`"]
        #[inline(always)]
        pub fn is_high(&self) -> bool {
            *self == SENSE6R::HIGH
        }
        #[doc = "Checks if the value of the field is `LOW`"]
        #[inline(always)]
        pub fn is_low(&self) -> bool {
            *self == SENSE6R::LOW
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTEN6R {
        bits: bool,
    }
    impl FILTEN6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `SENSE7`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SENSE7R {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SENSE7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SENSE7R::NONE => 0,
                SENSE7R::RISE => 1,
                SENSE7R::FALL => 2,
                SENSE7R::BOTH => 3,
                SENSE7R::HIGH => 4,
                SENSE7R::LOW => 5,
                SENSE7R::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SENSE7R {
            match value {
                0 => SENSE7R::NONE,
                1 => SENSE7R::RISE,
                2 => SENSE7R::FALL,
                3 => SENSE7R::BOTH,
                4 => SENSE7R::HIGH,
                5 => SENSE7R::LOW,
                i => SENSE7R::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == SENSE7R::NONE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == SENSE7R::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == SENSE7R::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == SENSE7R::BOTH
        }
        #[doc = "Checks if the value of the field is `HIGH`"]
        #[inline(always)]
        pub fn is_high(&self) -> bool {
            *self == SENSE7R::HIGH
        }
        #[doc = "Checks if the value of the field is `LOW`"]
        #[inline(always)]
        pub fn is_low(&self) -> bool {
            *self == SENSE7R::LOW
        }
    }
    #[doc = r" Value of the field"]
    pub struct FILTEN7R {
        bits: bool,
    }
    impl FILTEN7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Values that can be written to the field `SENSE0`"]
    pub enum SENSE0W {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising-edge detection"]
        RISE,
        #[doc = "Falling-edge detection"]
        FALL,
        #[doc = "Both-edges detection"]
        BOTH,
        #[doc = "High-level detection"]
        HIGH,
        #[doc = "Low-level detection"]
        LOW,
    }
    impl SENSE0W {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SENSE0W::NONE => 0,
                SENSE0W::RISE => 1,
                SENSE0W::FALL => 2,
                SENSE0W::BOTH => 3,
                SENSE0W::HIGH => 4,
                SENSE0W::LOW => 5,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SENSE0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SENSE0W<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SENSE0W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No detection"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(SENSE0W::NONE)
        }
        #[doc = "Rising-edge detection"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(SENSE0W::RISE)
        }
        #[doc = "Falling-edge detection"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(SENSE0W::FALL)
        }
        #[doc = "Both-edges detection"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(SENSE0W::BOTH)
        }
        #[doc = "High-level detection"]
        #[inline(always)]
        pub fn high(self) -> &'a mut W {
            self.variant(SENSE0W::HIGH)
        }
        #[doc = "Low-level detection"]
        #[inline(always)]
        pub fn low(self) -> &'a mut W {
            self.variant(SENSE0W::LOW)
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
    pub struct _FILTEN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTEN0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    #[doc = "Values that can be written to the field `SENSE1`"]
    pub enum SENSE1W {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
    }
    impl SENSE1W {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SENSE1W::NONE => 0,
                SENSE1W::RISE => 1,
                SENSE1W::FALL => 2,
                SENSE1W::BOTH => 3,
                SENSE1W::HIGH => 4,
                SENSE1W::LOW => 5,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SENSE1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SENSE1W<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SENSE1W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No detection"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(SENSE1W::NONE)
        }
        #[doc = "Rising edge detection"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(SENSE1W::RISE)
        }
        #[doc = "Falling edge detection"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(SENSE1W::FALL)
        }
        #[doc = "Both edges detection"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(SENSE1W::BOTH)
        }
        #[doc = "High level detection"]
        #[inline(always)]
        pub fn high(self) -> &'a mut W {
            self.variant(SENSE1W::HIGH)
        }
        #[doc = "Low level detection"]
        #[inline(always)]
        pub fn low(self) -> &'a mut W {
            self.variant(SENSE1W::LOW)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FILTEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTEN1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    #[doc = "Values that can be written to the field `SENSE2`"]
    pub enum SENSE2W {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
    }
    impl SENSE2W {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SENSE2W::NONE => 0,
                SENSE2W::RISE => 1,
                SENSE2W::FALL => 2,
                SENSE2W::BOTH => 3,
                SENSE2W::HIGH => 4,
                SENSE2W::LOW => 5,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SENSE2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SENSE2W<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SENSE2W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No detection"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(SENSE2W::NONE)
        }
        #[doc = "Rising edge detection"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(SENSE2W::RISE)
        }
        #[doc = "Falling edge detection"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(SENSE2W::FALL)
        }
        #[doc = "Both edges detection"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(SENSE2W::BOTH)
        }
        #[doc = "High level detection"]
        #[inline(always)]
        pub fn high(self) -> &'a mut W {
            self.variant(SENSE2W::HIGH)
        }
        #[doc = "Low level detection"]
        #[inline(always)]
        pub fn low(self) -> &'a mut W {
            self.variant(SENSE2W::LOW)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FILTEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTEN2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    #[doc = "Values that can be written to the field `SENSE3`"]
    pub enum SENSE3W {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
    }
    impl SENSE3W {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SENSE3W::NONE => 0,
                SENSE3W::RISE => 1,
                SENSE3W::FALL => 2,
                SENSE3W::BOTH => 3,
                SENSE3W::HIGH => 4,
                SENSE3W::LOW => 5,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SENSE3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SENSE3W<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SENSE3W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No detection"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(SENSE3W::NONE)
        }
        #[doc = "Rising edge detection"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(SENSE3W::RISE)
        }
        #[doc = "Falling edge detection"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(SENSE3W::FALL)
        }
        #[doc = "Both edges detection"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(SENSE3W::BOTH)
        }
        #[doc = "High level detection"]
        #[inline(always)]
        pub fn high(self) -> &'a mut W {
            self.variant(SENSE3W::HIGH)
        }
        #[doc = "Low level detection"]
        #[inline(always)]
        pub fn low(self) -> &'a mut W {
            self.variant(SENSE3W::LOW)
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
    pub struct _FILTEN3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTEN3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    #[doc = "Values that can be written to the field `SENSE4`"]
    pub enum SENSE4W {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
    }
    impl SENSE4W {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SENSE4W::NONE => 0,
                SENSE4W::RISE => 1,
                SENSE4W::FALL => 2,
                SENSE4W::BOTH => 3,
                SENSE4W::HIGH => 4,
                SENSE4W::LOW => 5,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SENSE4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SENSE4W<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SENSE4W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No detection"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(SENSE4W::NONE)
        }
        #[doc = "Rising edge detection"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(SENSE4W::RISE)
        }
        #[doc = "Falling edge detection"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(SENSE4W::FALL)
        }
        #[doc = "Both edges detection"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(SENSE4W::BOTH)
        }
        #[doc = "High level detection"]
        #[inline(always)]
        pub fn high(self) -> &'a mut W {
            self.variant(SENSE4W::HIGH)
        }
        #[doc = "Low level detection"]
        #[inline(always)]
        pub fn low(self) -> &'a mut W {
            self.variant(SENSE4W::LOW)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FILTEN4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTEN4W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    #[doc = "Values that can be written to the field `SENSE5`"]
    pub enum SENSE5W {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
    }
    impl SENSE5W {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SENSE5W::NONE => 0,
                SENSE5W::RISE => 1,
                SENSE5W::FALL => 2,
                SENSE5W::BOTH => 3,
                SENSE5W::HIGH => 4,
                SENSE5W::LOW => 5,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SENSE5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SENSE5W<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SENSE5W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No detection"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(SENSE5W::NONE)
        }
        #[doc = "Rising edge detection"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(SENSE5W::RISE)
        }
        #[doc = "Falling edge detection"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(SENSE5W::FALL)
        }
        #[doc = "Both edges detection"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(SENSE5W::BOTH)
        }
        #[doc = "High level detection"]
        #[inline(always)]
        pub fn high(self) -> &'a mut W {
            self.variant(SENSE5W::HIGH)
        }
        #[doc = "Low level detection"]
        #[inline(always)]
        pub fn low(self) -> &'a mut W {
            self.variant(SENSE5W::LOW)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FILTEN5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTEN5W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    #[doc = "Values that can be written to the field `SENSE6`"]
    pub enum SENSE6W {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
    }
    impl SENSE6W {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SENSE6W::NONE => 0,
                SENSE6W::RISE => 1,
                SENSE6W::FALL => 2,
                SENSE6W::BOTH => 3,
                SENSE6W::HIGH => 4,
                SENSE6W::LOW => 5,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SENSE6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SENSE6W<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SENSE6W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No detection"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(SENSE6W::NONE)
        }
        #[doc = "Rising edge detection"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(SENSE6W::RISE)
        }
        #[doc = "Falling edge detection"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(SENSE6W::FALL)
        }
        #[doc = "Both edges detection"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(SENSE6W::BOTH)
        }
        #[doc = "High level detection"]
        #[inline(always)]
        pub fn high(self) -> &'a mut W {
            self.variant(SENSE6W::HIGH)
        }
        #[doc = "Low level detection"]
        #[inline(always)]
        pub fn low(self) -> &'a mut W {
            self.variant(SENSE6W::LOW)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FILTEN6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTEN6W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    #[doc = "Values that can be written to the field `SENSE7`"]
    pub enum SENSE7W {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising edge detection"]
        RISE,
        #[doc = "Falling edge detection"]
        FALL,
        #[doc = "Both edges detection"]
        BOTH,
        #[doc = "High level detection"]
        HIGH,
        #[doc = "Low level detection"]
        LOW,
    }
    impl SENSE7W {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SENSE7W::NONE => 0,
                SENSE7W::RISE => 1,
                SENSE7W::FALL => 2,
                SENSE7W::BOTH => 3,
                SENSE7W::HIGH => 4,
                SENSE7W::LOW => 5,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SENSE7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SENSE7W<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SENSE7W) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No detection"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(SENSE7W::NONE)
        }
        #[doc = "Rising edge detection"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(SENSE7W::RISE)
        }
        #[doc = "Falling edge detection"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(SENSE7W::FALL)
        }
        #[doc = "Both edges detection"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(SENSE7W::BOTH)
        }
        #[doc = "High level detection"]
        #[inline(always)]
        pub fn high(self) -> &'a mut W {
            self.variant(SENSE7W::HIGH)
        }
        #[doc = "Low level detection"]
        #[inline(always)]
        pub fn low(self) -> &'a mut W {
            self.variant(SENSE7W::LOW)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FILTEN7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTEN7W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
        #[doc = "Bits 0:2 - Input Sense n Configuration"]
        #[inline(always)]
        pub fn sense0(&self) -> SENSE0R {
            SENSE0R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 3 - Filter n Enable"]
        #[inline(always)]
        pub fn filten0(&self) -> FILTEN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FILTEN0R { bits }
        }
        #[doc = "Bits 4:6 - Input Sense 1 Configuration"]
        #[inline(always)]
        pub fn sense1(&self) -> SENSE1R {
            SENSE1R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 7 - Filter 1 Enable"]
        #[inline(always)]
        pub fn filten1(&self) -> FILTEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FILTEN1R { bits }
        }
        #[doc = "Bits 8:10 - Input Sense 2 Configuration"]
        #[inline(always)]
        pub fn sense2(&self) -> SENSE2R {
            SENSE2R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 11 - Filter 2 Enable"]
        #[inline(always)]
        pub fn filten2(&self) -> FILTEN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FILTEN2R { bits }
        }
        #[doc = "Bits 12:14 - Input Sense 3 Configuration"]
        #[inline(always)]
        pub fn sense3(&self) -> SENSE3R {
            SENSE3R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 15 - Filter 3 Enable"]
        #[inline(always)]
        pub fn filten3(&self) -> FILTEN3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FILTEN3R { bits }
        }
        #[doc = "Bits 16:18 - Input Sense 4 Configuration"]
        #[inline(always)]
        pub fn sense4(&self) -> SENSE4R {
            SENSE4R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 19 - Filter 4 Enable"]
        #[inline(always)]
        pub fn filten4(&self) -> FILTEN4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FILTEN4R { bits }
        }
        #[doc = "Bits 20:22 - Input Sense 5 Configuration"]
        #[inline(always)]
        pub fn sense5(&self) -> SENSE5R {
            SENSE5R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 23 - Filter 5 Enable"]
        #[inline(always)]
        pub fn filten5(&self) -> FILTEN5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FILTEN5R { bits }
        }
        #[doc = "Bits 24:26 - Input Sense 6 Configuration"]
        #[inline(always)]
        pub fn sense6(&self) -> SENSE6R {
            SENSE6R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 27 - Filter 6 Enable"]
        #[inline(always)]
        pub fn filten6(&self) -> FILTEN6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 27;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FILTEN6R { bits }
        }
        #[doc = "Bits 28:30 - Input Sense 7 Configuration"]
        #[inline(always)]
        pub fn sense7(&self) -> SENSE7R {
            SENSE7R::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 31 - Filter 7 Enable"]
        #[inline(always)]
        pub fn filten7(&self) -> FILTEN7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FILTEN7R { bits }
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
        #[doc = "Bits 0:2 - Input Sense n Configuration"]
        #[inline(always)]
        pub fn sense0(&mut self) -> _SENSE0W {
            _SENSE0W { w: self }
        }
        #[doc = "Bit 3 - Filter n Enable"]
        #[inline(always)]
        pub fn filten0(&mut self) -> _FILTEN0W {
            _FILTEN0W { w: self }
        }
        #[doc = "Bits 4:6 - Input Sense 1 Configuration"]
        #[inline(always)]
        pub fn sense1(&mut self) -> _SENSE1W {
            _SENSE1W { w: self }
        }
        #[doc = "Bit 7 - Filter 1 Enable"]
        #[inline(always)]
        pub fn filten1(&mut self) -> _FILTEN1W {
            _FILTEN1W { w: self }
        }
        #[doc = "Bits 8:10 - Input Sense 2 Configuration"]
        #[inline(always)]
        pub fn sense2(&mut self) -> _SENSE2W {
            _SENSE2W { w: self }
        }
        #[doc = "Bit 11 - Filter 2 Enable"]
        #[inline(always)]
        pub fn filten2(&mut self) -> _FILTEN2W {
            _FILTEN2W { w: self }
        }
        #[doc = "Bits 12:14 - Input Sense 3 Configuration"]
        #[inline(always)]
        pub fn sense3(&mut self) -> _SENSE3W {
            _SENSE3W { w: self }
        }
        #[doc = "Bit 15 - Filter 3 Enable"]
        #[inline(always)]
        pub fn filten3(&mut self) -> _FILTEN3W {
            _FILTEN3W { w: self }
        }
        #[doc = "Bits 16:18 - Input Sense 4 Configuration"]
        #[inline(always)]
        pub fn sense4(&mut self) -> _SENSE4W {
            _SENSE4W { w: self }
        }
        #[doc = "Bit 19 - Filter 4 Enable"]
        #[inline(always)]
        pub fn filten4(&mut self) -> _FILTEN4W {
            _FILTEN4W { w: self }
        }
        #[doc = "Bits 20:22 - Input Sense 5 Configuration"]
        #[inline(always)]
        pub fn sense5(&mut self) -> _SENSE5W {
            _SENSE5W { w: self }
        }
        #[doc = "Bit 23 - Filter 5 Enable"]
        #[inline(always)]
        pub fn filten5(&mut self) -> _FILTEN5W {
            _FILTEN5W { w: self }
        }
        #[doc = "Bits 24:26 - Input Sense 6 Configuration"]
        #[inline(always)]
        pub fn sense6(&mut self) -> _SENSE6W {
            _SENSE6W { w: self }
        }
        #[doc = "Bit 27 - Filter 6 Enable"]
        #[inline(always)]
        pub fn filten6(&mut self) -> _FILTEN6W {
            _FILTEN6W { w: self }
        }
        #[doc = "Bits 28:30 - Input Sense 7 Configuration"]
        #[inline(always)]
        pub fn sense7(&mut self) -> _SENSE7W {
            _SENSE7W { w: self }
        }
        #[doc = "Bit 31 - Filter 7 Enable"]
        #[inline(always)]
        pub fn filten7(&mut self) -> _FILTEN7W {
            _FILTEN7W { w: self }
        }
    }
}
#[doc = "Control"]
pub struct CTRL {
    register: VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CTRL {
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
    #[doc = r" Value of the field"]
    pub struct EXTINTEO0R {
        bits: bool,
    }
    impl EXTINTEO0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO1R {
        bits: bool,
    }
    impl EXTINTEO1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO2R {
        bits: bool,
    }
    impl EXTINTEO2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO3R {
        bits: bool,
    }
    impl EXTINTEO3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO4R {
        bits: bool,
    }
    impl EXTINTEO4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO5R {
        bits: bool,
    }
    impl EXTINTEO5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO6R {
        bits: bool,
    }
    impl EXTINTEO6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO7R {
        bits: bool,
    }
    impl EXTINTEO7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO8R {
        bits: bool,
    }
    impl EXTINTEO8R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO9R {
        bits: bool,
    }
    impl EXTINTEO9R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO10R {
        bits: bool,
    }
    impl EXTINTEO10R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO11R {
        bits: bool,
    }
    impl EXTINTEO11R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO12R {
        bits: bool,
    }
    impl EXTINTEO12R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO13R {
        bits: bool,
    }
    impl EXTINTEO13R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO14R {
        bits: bool,
    }
    impl EXTINTEO14R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINTEO15R {
        bits: bool,
    }
    impl EXTINTEO15R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _EXTINTEO0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO4W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO5W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO6W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO7W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO8W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO9W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO10W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO11W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO12W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO13W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO14W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINTEO15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINTEO15W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - External Interrupt 0 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo0(&self) -> EXTINTEO0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO0R { bits }
        }
        #[doc = "Bit 1 - External Interrupt 1 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo1(&self) -> EXTINTEO1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO1R { bits }
        }
        #[doc = "Bit 2 - External Interrupt 2 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo2(&self) -> EXTINTEO2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO2R { bits }
        }
        #[doc = "Bit 3 - External Interrupt 3 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo3(&self) -> EXTINTEO3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO3R { bits }
        }
        #[doc = "Bit 4 - External Interrupt 4 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo4(&self) -> EXTINTEO4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO4R { bits }
        }
        #[doc = "Bit 5 - External Interrupt 5 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo5(&self) -> EXTINTEO5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO5R { bits }
        }
        #[doc = "Bit 6 - External Interrupt 6 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo6(&self) -> EXTINTEO6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO6R { bits }
        }
        #[doc = "Bit 7 - External Interrupt 7 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo7(&self) -> EXTINTEO7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO7R { bits }
        }
        #[doc = "Bit 8 - External Interrupt 8 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo8(&self) -> EXTINTEO8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO8R { bits }
        }
        #[doc = "Bit 9 - External Interrupt 9 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo9(&self) -> EXTINTEO9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO9R { bits }
        }
        #[doc = "Bit 10 - External Interrupt 10 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo10(&self) -> EXTINTEO10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO10R { bits }
        }
        #[doc = "Bit 11 - External Interrupt 11 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo11(&self) -> EXTINTEO11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO11R { bits }
        }
        #[doc = "Bit 12 - External Interrupt 12 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo12(&self) -> EXTINTEO12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO12R { bits }
        }
        #[doc = "Bit 13 - External Interrupt 13 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo13(&self) -> EXTINTEO13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO13R { bits }
        }
        #[doc = "Bit 14 - External Interrupt 14 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo14(&self) -> EXTINTEO14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO14R { bits }
        }
        #[doc = "Bit 15 - External Interrupt 15 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo15(&self) -> EXTINTEO15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINTEO15R { bits }
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
        #[doc = "Bit 0 - External Interrupt 0 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo0(&mut self) -> _EXTINTEO0W {
            _EXTINTEO0W { w: self }
        }
        #[doc = "Bit 1 - External Interrupt 1 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo1(&mut self) -> _EXTINTEO1W {
            _EXTINTEO1W { w: self }
        }
        #[doc = "Bit 2 - External Interrupt 2 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo2(&mut self) -> _EXTINTEO2W {
            _EXTINTEO2W { w: self }
        }
        #[doc = "Bit 3 - External Interrupt 3 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo3(&mut self) -> _EXTINTEO3W {
            _EXTINTEO3W { w: self }
        }
        #[doc = "Bit 4 - External Interrupt 4 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo4(&mut self) -> _EXTINTEO4W {
            _EXTINTEO4W { w: self }
        }
        #[doc = "Bit 5 - External Interrupt 5 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo5(&mut self) -> _EXTINTEO5W {
            _EXTINTEO5W { w: self }
        }
        #[doc = "Bit 6 - External Interrupt 6 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo6(&mut self) -> _EXTINTEO6W {
            _EXTINTEO6W { w: self }
        }
        #[doc = "Bit 7 - External Interrupt 7 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo7(&mut self) -> _EXTINTEO7W {
            _EXTINTEO7W { w: self }
        }
        #[doc = "Bit 8 - External Interrupt 8 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo8(&mut self) -> _EXTINTEO8W {
            _EXTINTEO8W { w: self }
        }
        #[doc = "Bit 9 - External Interrupt 9 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo9(&mut self) -> _EXTINTEO9W {
            _EXTINTEO9W { w: self }
        }
        #[doc = "Bit 10 - External Interrupt 10 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo10(&mut self) -> _EXTINTEO10W {
            _EXTINTEO10W { w: self }
        }
        #[doc = "Bit 11 - External Interrupt 11 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo11(&mut self) -> _EXTINTEO11W {
            _EXTINTEO11W { w: self }
        }
        #[doc = "Bit 12 - External Interrupt 12 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo12(&mut self) -> _EXTINTEO12W {
            _EXTINTEO12W { w: self }
        }
        #[doc = "Bit 13 - External Interrupt 13 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo13(&mut self) -> _EXTINTEO13W {
            _EXTINTEO13W { w: self }
        }
        #[doc = "Bit 14 - External Interrupt 14 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo14(&mut self) -> _EXTINTEO14W {
            _EXTINTEO14W { w: self }
        }
        #[doc = "Bit 15 - External Interrupt 15 Event Output Enable"]
        #[inline(always)]
        pub fn extinteo15(&mut self) -> _EXTINTEO15W {
            _EXTINTEO15W { w: self }
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
    pub struct EXTINT0R {
        bits: bool,
    }
    impl EXTINT0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT1R {
        bits: bool,
    }
    impl EXTINT1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT2R {
        bits: bool,
    }
    impl EXTINT2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT3R {
        bits: bool,
    }
    impl EXTINT3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT4R {
        bits: bool,
    }
    impl EXTINT4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT5R {
        bits: bool,
    }
    impl EXTINT5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT6R {
        bits: bool,
    }
    impl EXTINT6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT7R {
        bits: bool,
    }
    impl EXTINT7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT8R {
        bits: bool,
    }
    impl EXTINT8R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT9R {
        bits: bool,
    }
    impl EXTINT9R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT10R {
        bits: bool,
    }
    impl EXTINT10R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT11R {
        bits: bool,
    }
    impl EXTINT11R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT12R {
        bits: bool,
    }
    impl EXTINT12R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT13R {
        bits: bool,
    }
    impl EXTINT13R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT14R {
        bits: bool,
    }
    impl EXTINT14R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT15R {
        bits: bool,
    }
    impl EXTINT15R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _EXTINT0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT4W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT5W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT6W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT7W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT8W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT9W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT10W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT11W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT12W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT13W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT14W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT15W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - External Interrupt 0 Enable"]
        #[inline(always)]
        pub fn extint0(&self) -> EXTINT0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT0R { bits }
        }
        #[doc = "Bit 1 - External Interrupt 1 Enable"]
        #[inline(always)]
        pub fn extint1(&self) -> EXTINT1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT1R { bits }
        }
        #[doc = "Bit 2 - External Interrupt 2 Enable"]
        #[inline(always)]
        pub fn extint2(&self) -> EXTINT2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT2R { bits }
        }
        #[doc = "Bit 3 - External Interrupt 3 Enable"]
        #[inline(always)]
        pub fn extint3(&self) -> EXTINT3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT3R { bits }
        }
        #[doc = "Bit 4 - External Interrupt 4 Enable"]
        #[inline(always)]
        pub fn extint4(&self) -> EXTINT4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT4R { bits }
        }
        #[doc = "Bit 5 - External Interrupt 5 Enable"]
        #[inline(always)]
        pub fn extint5(&self) -> EXTINT5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT5R { bits }
        }
        #[doc = "Bit 6 - External Interrupt 6 Enable"]
        #[inline(always)]
        pub fn extint6(&self) -> EXTINT6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT6R { bits }
        }
        #[doc = "Bit 7 - External Interrupt 7 Enable"]
        #[inline(always)]
        pub fn extint7(&self) -> EXTINT7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT7R { bits }
        }
        #[doc = "Bit 8 - External Interrupt 8 Enable"]
        #[inline(always)]
        pub fn extint8(&self) -> EXTINT8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT8R { bits }
        }
        #[doc = "Bit 9 - External Interrupt 9 Enable"]
        #[inline(always)]
        pub fn extint9(&self) -> EXTINT9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT9R { bits }
        }
        #[doc = "Bit 10 - External Interrupt 10 Enable"]
        #[inline(always)]
        pub fn extint10(&self) -> EXTINT10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT10R { bits }
        }
        #[doc = "Bit 11 - External Interrupt 11 Enable"]
        #[inline(always)]
        pub fn extint11(&self) -> EXTINT11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT11R { bits }
        }
        #[doc = "Bit 12 - External Interrupt 12 Enable"]
        #[inline(always)]
        pub fn extint12(&self) -> EXTINT12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT12R { bits }
        }
        #[doc = "Bit 13 - External Interrupt 13 Enable"]
        #[inline(always)]
        pub fn extint13(&self) -> EXTINT13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT13R { bits }
        }
        #[doc = "Bit 14 - External Interrupt 14 Enable"]
        #[inline(always)]
        pub fn extint14(&self) -> EXTINT14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT14R { bits }
        }
        #[doc = "Bit 15 - External Interrupt 15 Enable"]
        #[inline(always)]
        pub fn extint15(&self) -> EXTINT15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT15R { bits }
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
        #[doc = "Bit 0 - External Interrupt 0 Enable"]
        #[inline(always)]
        pub fn extint0(&mut self) -> _EXTINT0W {
            _EXTINT0W { w: self }
        }
        #[doc = "Bit 1 - External Interrupt 1 Enable"]
        #[inline(always)]
        pub fn extint1(&mut self) -> _EXTINT1W {
            _EXTINT1W { w: self }
        }
        #[doc = "Bit 2 - External Interrupt 2 Enable"]
        #[inline(always)]
        pub fn extint2(&mut self) -> _EXTINT2W {
            _EXTINT2W { w: self }
        }
        #[doc = "Bit 3 - External Interrupt 3 Enable"]
        #[inline(always)]
        pub fn extint3(&mut self) -> _EXTINT3W {
            _EXTINT3W { w: self }
        }
        #[doc = "Bit 4 - External Interrupt 4 Enable"]
        #[inline(always)]
        pub fn extint4(&mut self) -> _EXTINT4W {
            _EXTINT4W { w: self }
        }
        #[doc = "Bit 5 - External Interrupt 5 Enable"]
        #[inline(always)]
        pub fn extint5(&mut self) -> _EXTINT5W {
            _EXTINT5W { w: self }
        }
        #[doc = "Bit 6 - External Interrupt 6 Enable"]
        #[inline(always)]
        pub fn extint6(&mut self) -> _EXTINT6W {
            _EXTINT6W { w: self }
        }
        #[doc = "Bit 7 - External Interrupt 7 Enable"]
        #[inline(always)]
        pub fn extint7(&mut self) -> _EXTINT7W {
            _EXTINT7W { w: self }
        }
        #[doc = "Bit 8 - External Interrupt 8 Enable"]
        #[inline(always)]
        pub fn extint8(&mut self) -> _EXTINT8W {
            _EXTINT8W { w: self }
        }
        #[doc = "Bit 9 - External Interrupt 9 Enable"]
        #[inline(always)]
        pub fn extint9(&mut self) -> _EXTINT9W {
            _EXTINT9W { w: self }
        }
        #[doc = "Bit 10 - External Interrupt 10 Enable"]
        #[inline(always)]
        pub fn extint10(&mut self) -> _EXTINT10W {
            _EXTINT10W { w: self }
        }
        #[doc = "Bit 11 - External Interrupt 11 Enable"]
        #[inline(always)]
        pub fn extint11(&mut self) -> _EXTINT11W {
            _EXTINT11W { w: self }
        }
        #[doc = "Bit 12 - External Interrupt 12 Enable"]
        #[inline(always)]
        pub fn extint12(&mut self) -> _EXTINT12W {
            _EXTINT12W { w: self }
        }
        #[doc = "Bit 13 - External Interrupt 13 Enable"]
        #[inline(always)]
        pub fn extint13(&mut self) -> _EXTINT13W {
            _EXTINT13W { w: self }
        }
        #[doc = "Bit 14 - External Interrupt 14 Enable"]
        #[inline(always)]
        pub fn extint14(&mut self) -> _EXTINT14W {
            _EXTINT14W { w: self }
        }
        #[doc = "Bit 15 - External Interrupt 15 Enable"]
        #[inline(always)]
        pub fn extint15(&mut self) -> _EXTINT15W {
            _EXTINT15W { w: self }
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
    pub struct EXTINT0R {
        bits: bool,
    }
    impl EXTINT0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT1R {
        bits: bool,
    }
    impl EXTINT1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT2R {
        bits: bool,
    }
    impl EXTINT2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT3R {
        bits: bool,
    }
    impl EXTINT3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT4R {
        bits: bool,
    }
    impl EXTINT4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT5R {
        bits: bool,
    }
    impl EXTINT5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT6R {
        bits: bool,
    }
    impl EXTINT6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT7R {
        bits: bool,
    }
    impl EXTINT7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT8R {
        bits: bool,
    }
    impl EXTINT8R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT9R {
        bits: bool,
    }
    impl EXTINT9R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT10R {
        bits: bool,
    }
    impl EXTINT10R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT11R {
        bits: bool,
    }
    impl EXTINT11R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT12R {
        bits: bool,
    }
    impl EXTINT12R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT13R {
        bits: bool,
    }
    impl EXTINT13R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT14R {
        bits: bool,
    }
    impl EXTINT14R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT15R {
        bits: bool,
    }
    impl EXTINT15R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _EXTINT0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT4W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT5W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT6W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT7W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT8W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT9W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT10W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT11W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT12W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT13W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT14W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT15W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - External Interrupt 0 Enable"]
        #[inline(always)]
        pub fn extint0(&self) -> EXTINT0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT0R { bits }
        }
        #[doc = "Bit 1 - External Interrupt 1 Enable"]
        #[inline(always)]
        pub fn extint1(&self) -> EXTINT1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT1R { bits }
        }
        #[doc = "Bit 2 - External Interrupt 2 Enable"]
        #[inline(always)]
        pub fn extint2(&self) -> EXTINT2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT2R { bits }
        }
        #[doc = "Bit 3 - External Interrupt 3 Enable"]
        #[inline(always)]
        pub fn extint3(&self) -> EXTINT3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT3R { bits }
        }
        #[doc = "Bit 4 - External Interrupt 4 Enable"]
        #[inline(always)]
        pub fn extint4(&self) -> EXTINT4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT4R { bits }
        }
        #[doc = "Bit 5 - External Interrupt 5 Enable"]
        #[inline(always)]
        pub fn extint5(&self) -> EXTINT5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT5R { bits }
        }
        #[doc = "Bit 6 - External Interrupt 6 Enable"]
        #[inline(always)]
        pub fn extint6(&self) -> EXTINT6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT6R { bits }
        }
        #[doc = "Bit 7 - External Interrupt 7 Enable"]
        #[inline(always)]
        pub fn extint7(&self) -> EXTINT7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT7R { bits }
        }
        #[doc = "Bit 8 - External Interrupt 8 Enable"]
        #[inline(always)]
        pub fn extint8(&self) -> EXTINT8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT8R { bits }
        }
        #[doc = "Bit 9 - External Interrupt 9 Enable"]
        #[inline(always)]
        pub fn extint9(&self) -> EXTINT9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT9R { bits }
        }
        #[doc = "Bit 10 - External Interrupt 10 Enable"]
        #[inline(always)]
        pub fn extint10(&self) -> EXTINT10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT10R { bits }
        }
        #[doc = "Bit 11 - External Interrupt 11 Enable"]
        #[inline(always)]
        pub fn extint11(&self) -> EXTINT11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT11R { bits }
        }
        #[doc = "Bit 12 - External Interrupt 12 Enable"]
        #[inline(always)]
        pub fn extint12(&self) -> EXTINT12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT12R { bits }
        }
        #[doc = "Bit 13 - External Interrupt 13 Enable"]
        #[inline(always)]
        pub fn extint13(&self) -> EXTINT13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT13R { bits }
        }
        #[doc = "Bit 14 - External Interrupt 14 Enable"]
        #[inline(always)]
        pub fn extint14(&self) -> EXTINT14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT14R { bits }
        }
        #[doc = "Bit 15 - External Interrupt 15 Enable"]
        #[inline(always)]
        pub fn extint15(&self) -> EXTINT15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT15R { bits }
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
        #[doc = "Bit 0 - External Interrupt 0 Enable"]
        #[inline(always)]
        pub fn extint0(&mut self) -> _EXTINT0W {
            _EXTINT0W { w: self }
        }
        #[doc = "Bit 1 - External Interrupt 1 Enable"]
        #[inline(always)]
        pub fn extint1(&mut self) -> _EXTINT1W {
            _EXTINT1W { w: self }
        }
        #[doc = "Bit 2 - External Interrupt 2 Enable"]
        #[inline(always)]
        pub fn extint2(&mut self) -> _EXTINT2W {
            _EXTINT2W { w: self }
        }
        #[doc = "Bit 3 - External Interrupt 3 Enable"]
        #[inline(always)]
        pub fn extint3(&mut self) -> _EXTINT3W {
            _EXTINT3W { w: self }
        }
        #[doc = "Bit 4 - External Interrupt 4 Enable"]
        #[inline(always)]
        pub fn extint4(&mut self) -> _EXTINT4W {
            _EXTINT4W { w: self }
        }
        #[doc = "Bit 5 - External Interrupt 5 Enable"]
        #[inline(always)]
        pub fn extint5(&mut self) -> _EXTINT5W {
            _EXTINT5W { w: self }
        }
        #[doc = "Bit 6 - External Interrupt 6 Enable"]
        #[inline(always)]
        pub fn extint6(&mut self) -> _EXTINT6W {
            _EXTINT6W { w: self }
        }
        #[doc = "Bit 7 - External Interrupt 7 Enable"]
        #[inline(always)]
        pub fn extint7(&mut self) -> _EXTINT7W {
            _EXTINT7W { w: self }
        }
        #[doc = "Bit 8 - External Interrupt 8 Enable"]
        #[inline(always)]
        pub fn extint8(&mut self) -> _EXTINT8W {
            _EXTINT8W { w: self }
        }
        #[doc = "Bit 9 - External Interrupt 9 Enable"]
        #[inline(always)]
        pub fn extint9(&mut self) -> _EXTINT9W {
            _EXTINT9W { w: self }
        }
        #[doc = "Bit 10 - External Interrupt 10 Enable"]
        #[inline(always)]
        pub fn extint10(&mut self) -> _EXTINT10W {
            _EXTINT10W { w: self }
        }
        #[doc = "Bit 11 - External Interrupt 11 Enable"]
        #[inline(always)]
        pub fn extint11(&mut self) -> _EXTINT11W {
            _EXTINT11W { w: self }
        }
        #[doc = "Bit 12 - External Interrupt 12 Enable"]
        #[inline(always)]
        pub fn extint12(&mut self) -> _EXTINT12W {
            _EXTINT12W { w: self }
        }
        #[doc = "Bit 13 - External Interrupt 13 Enable"]
        #[inline(always)]
        pub fn extint13(&mut self) -> _EXTINT13W {
            _EXTINT13W { w: self }
        }
        #[doc = "Bit 14 - External Interrupt 14 Enable"]
        #[inline(always)]
        pub fn extint14(&mut self) -> _EXTINT14W {
            _EXTINT14W { w: self }
        }
        #[doc = "Bit 15 - External Interrupt 15 Enable"]
        #[inline(always)]
        pub fn extint15(&mut self) -> _EXTINT15W {
            _EXTINT15W { w: self }
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
    pub struct EXTINT0R {
        bits: bool,
    }
    impl EXTINT0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT1R {
        bits: bool,
    }
    impl EXTINT1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT2R {
        bits: bool,
    }
    impl EXTINT2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT3R {
        bits: bool,
    }
    impl EXTINT3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT4R {
        bits: bool,
    }
    impl EXTINT4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT5R {
        bits: bool,
    }
    impl EXTINT5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT6R {
        bits: bool,
    }
    impl EXTINT6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT7R {
        bits: bool,
    }
    impl EXTINT7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT8R {
        bits: bool,
    }
    impl EXTINT8R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT9R {
        bits: bool,
    }
    impl EXTINT9R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT10R {
        bits: bool,
    }
    impl EXTINT10R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT11R {
        bits: bool,
    }
    impl EXTINT11R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT12R {
        bits: bool,
    }
    impl EXTINT12R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT13R {
        bits: bool,
    }
    impl EXTINT13R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT14R {
        bits: bool,
    }
    impl EXTINT14R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTINT15R {
        bits: bool,
    }
    impl EXTINT15R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _EXTINT0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT4W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT5W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT6W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT7W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT8W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT9W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT10W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT11W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT12W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT13W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT14W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EXTINT15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTINT15W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - External Interrupt 0"]
        #[inline(always)]
        pub fn extint0(&self) -> EXTINT0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT0R { bits }
        }
        #[doc = "Bit 1 - External Interrupt 1"]
        #[inline(always)]
        pub fn extint1(&self) -> EXTINT1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT1R { bits }
        }
        #[doc = "Bit 2 - External Interrupt 2"]
        #[inline(always)]
        pub fn extint2(&self) -> EXTINT2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT2R { bits }
        }
        #[doc = "Bit 3 - External Interrupt 3"]
        #[inline(always)]
        pub fn extint3(&self) -> EXTINT3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT3R { bits }
        }
        #[doc = "Bit 4 - External Interrupt 4"]
        #[inline(always)]
        pub fn extint4(&self) -> EXTINT4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT4R { bits }
        }
        #[doc = "Bit 5 - External Interrupt 5"]
        #[inline(always)]
        pub fn extint5(&self) -> EXTINT5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT5R { bits }
        }
        #[doc = "Bit 6 - External Interrupt 6"]
        #[inline(always)]
        pub fn extint6(&self) -> EXTINT6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT6R { bits }
        }
        #[doc = "Bit 7 - External Interrupt 7"]
        #[inline(always)]
        pub fn extint7(&self) -> EXTINT7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT7R { bits }
        }
        #[doc = "Bit 8 - External Interrupt 8"]
        #[inline(always)]
        pub fn extint8(&self) -> EXTINT8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT8R { bits }
        }
        #[doc = "Bit 9 - External Interrupt 9"]
        #[inline(always)]
        pub fn extint9(&self) -> EXTINT9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT9R { bits }
        }
        #[doc = "Bit 10 - External Interrupt 10"]
        #[inline(always)]
        pub fn extint10(&self) -> EXTINT10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT10R { bits }
        }
        #[doc = "Bit 11 - External Interrupt 11"]
        #[inline(always)]
        pub fn extint11(&self) -> EXTINT11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT11R { bits }
        }
        #[doc = "Bit 12 - External Interrupt 12"]
        #[inline(always)]
        pub fn extint12(&self) -> EXTINT12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT12R { bits }
        }
        #[doc = "Bit 13 - External Interrupt 13"]
        #[inline(always)]
        pub fn extint13(&self) -> EXTINT13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT13R { bits }
        }
        #[doc = "Bit 14 - External Interrupt 14"]
        #[inline(always)]
        pub fn extint14(&self) -> EXTINT14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT14R { bits }
        }
        #[doc = "Bit 15 - External Interrupt 15"]
        #[inline(always)]
        pub fn extint15(&self) -> EXTINT15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EXTINT15R { bits }
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
        #[doc = "Bit 0 - External Interrupt 0"]
        #[inline(always)]
        pub fn extint0(&mut self) -> _EXTINT0W {
            _EXTINT0W { w: self }
        }
        #[doc = "Bit 1 - External Interrupt 1"]
        #[inline(always)]
        pub fn extint1(&mut self) -> _EXTINT1W {
            _EXTINT1W { w: self }
        }
        #[doc = "Bit 2 - External Interrupt 2"]
        #[inline(always)]
        pub fn extint2(&mut self) -> _EXTINT2W {
            _EXTINT2W { w: self }
        }
        #[doc = "Bit 3 - External Interrupt 3"]
        #[inline(always)]
        pub fn extint3(&mut self) -> _EXTINT3W {
            _EXTINT3W { w: self }
        }
        #[doc = "Bit 4 - External Interrupt 4"]
        #[inline(always)]
        pub fn extint4(&mut self) -> _EXTINT4W {
            _EXTINT4W { w: self }
        }
        #[doc = "Bit 5 - External Interrupt 5"]
        #[inline(always)]
        pub fn extint5(&mut self) -> _EXTINT5W {
            _EXTINT5W { w: self }
        }
        #[doc = "Bit 6 - External Interrupt 6"]
        #[inline(always)]
        pub fn extint6(&mut self) -> _EXTINT6W {
            _EXTINT6W { w: self }
        }
        #[doc = "Bit 7 - External Interrupt 7"]
        #[inline(always)]
        pub fn extint7(&mut self) -> _EXTINT7W {
            _EXTINT7W { w: self }
        }
        #[doc = "Bit 8 - External Interrupt 8"]
        #[inline(always)]
        pub fn extint8(&mut self) -> _EXTINT8W {
            _EXTINT8W { w: self }
        }
        #[doc = "Bit 9 - External Interrupt 9"]
        #[inline(always)]
        pub fn extint9(&mut self) -> _EXTINT9W {
            _EXTINT9W { w: self }
        }
        #[doc = "Bit 10 - External Interrupt 10"]
        #[inline(always)]
        pub fn extint10(&mut self) -> _EXTINT10W {
            _EXTINT10W { w: self }
        }
        #[doc = "Bit 11 - External Interrupt 11"]
        #[inline(always)]
        pub fn extint11(&mut self) -> _EXTINT11W {
            _EXTINT11W { w: self }
        }
        #[doc = "Bit 12 - External Interrupt 12"]
        #[inline(always)]
        pub fn extint12(&mut self) -> _EXTINT12W {
            _EXTINT12W { w: self }
        }
        #[doc = "Bit 13 - External Interrupt 13"]
        #[inline(always)]
        pub fn extint13(&mut self) -> _EXTINT13W {
            _EXTINT13W { w: self }
        }
        #[doc = "Bit 14 - External Interrupt 14"]
        #[inline(always)]
        pub fn extint14(&mut self) -> _EXTINT14W {
            _EXTINT14W { w: self }
        }
        #[doc = "Bit 15 - External Interrupt 15"]
        #[inline(always)]
        pub fn extint15(&mut self) -> _EXTINT15W {
            _EXTINT15W { w: self }
        }
    }
}
#[doc = "Non-Maskable Interrupt Control"]
pub struct NMICTRL {
    register: VolatileCell<u8>,
}
#[doc = "Non-Maskable Interrupt Control"]
pub mod nmictrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::NMICTRL {
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
    #[doc = "Possible values of the field `NMISENSE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum NMISENSER {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising-edge detection"]
        RISE,
        #[doc = "Falling-edge detection"]
        FALL,
        #[doc = "Both-edges detection"]
        BOTH,
        #[doc = "High-level detection"]
        HIGH,
        #[doc = "Low-level detection"]
        LOW,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl NMISENSER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                NMISENSER::NONE => 0,
                NMISENSER::RISE => 1,
                NMISENSER::FALL => 2,
                NMISENSER::BOTH => 3,
                NMISENSER::HIGH => 4,
                NMISENSER::LOW => 5,
                NMISENSER::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> NMISENSER {
            match value {
                0 => NMISENSER::NONE,
                1 => NMISENSER::RISE,
                2 => NMISENSER::FALL,
                3 => NMISENSER::BOTH,
                4 => NMISENSER::HIGH,
                5 => NMISENSER::LOW,
                i => NMISENSER::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == NMISENSER::NONE
        }
        #[doc = "Checks if the value of the field is `RISE`"]
        #[inline(always)]
        pub fn is_rise(&self) -> bool {
            *self == NMISENSER::RISE
        }
        #[doc = "Checks if the value of the field is `FALL`"]
        #[inline(always)]
        pub fn is_fall(&self) -> bool {
            *self == NMISENSER::FALL
        }
        #[doc = "Checks if the value of the field is `BOTH`"]
        #[inline(always)]
        pub fn is_both(&self) -> bool {
            *self == NMISENSER::BOTH
        }
        #[doc = "Checks if the value of the field is `HIGH`"]
        #[inline(always)]
        pub fn is_high(&self) -> bool {
            *self == NMISENSER::HIGH
        }
        #[doc = "Checks if the value of the field is `LOW`"]
        #[inline(always)]
        pub fn is_low(&self) -> bool {
            *self == NMISENSER::LOW
        }
    }
    #[doc = r" Value of the field"]
    pub struct NMIFILTENR {
        bits: bool,
    }
    impl NMIFILTENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Values that can be written to the field `NMISENSE`"]
    pub enum NMISENSEW {
        #[doc = "No detection"]
        NONE,
        #[doc = "Rising-edge detection"]
        RISE,
        #[doc = "Falling-edge detection"]
        FALL,
        #[doc = "Both-edges detection"]
        BOTH,
        #[doc = "High-level detection"]
        HIGH,
        #[doc = "Low-level detection"]
        LOW,
    }
    impl NMISENSEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                NMISENSEW::NONE => 0,
                NMISENSEW::RISE => 1,
                NMISENSEW::FALL => 2,
                NMISENSEW::BOTH => 3,
                NMISENSEW::HIGH => 4,
                NMISENSEW::LOW => 5,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _NMISENSEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _NMISENSEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: NMISENSEW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No detection"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(NMISENSEW::NONE)
        }
        #[doc = "Rising-edge detection"]
        #[inline(always)]
        pub fn rise(self) -> &'a mut W {
            self.variant(NMISENSEW::RISE)
        }
        #[doc = "Falling-edge detection"]
        #[inline(always)]
        pub fn fall(self) -> &'a mut W {
            self.variant(NMISENSEW::FALL)
        }
        #[doc = "Both-edges detection"]
        #[inline(always)]
        pub fn both(self) -> &'a mut W {
            self.variant(NMISENSEW::BOTH)
        }
        #[doc = "High-level detection"]
        #[inline(always)]
        pub fn high(self) -> &'a mut W {
            self.variant(NMISENSEW::HIGH)
        }
        #[doc = "Low-level detection"]
        #[inline(always)]
        pub fn low(self) -> &'a mut W {
            self.variant(NMISENSEW::LOW)
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
    #[doc = r" Proxy"]
    pub struct _NMIFILTENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _NMIFILTENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense"]
        #[inline(always)]
        pub fn nmisense(&self) -> NMISENSER {
            NMISENSER::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            })
        }
        #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
        #[inline(always)]
        pub fn nmifilten(&self) -> NMIFILTENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            NMIFILTENR { bits }
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
        #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense"]
        #[inline(always)]
        pub fn nmisense(&mut self) -> _NMISENSEW {
            _NMISENSEW { w: self }
        }
        #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
        #[inline(always)]
        pub fn nmifilten(&mut self) -> _NMIFILTENW {
            _NMIFILTENW { w: self }
        }
    }
}
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub struct NMIFLAG {
    register: VolatileCell<u8>,
}
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub mod nmiflag {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::NMIFLAG {
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
    pub struct NMIR {
        bits: bool,
    }
    impl NMIR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _NMIW<'a> {
        w: &'a mut W,
    }
    impl<'a> _NMIW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Non-Maskable Interrupt"]
        #[inline(always)]
        pub fn nmi(&self) -> NMIR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            NMIR { bits }
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
        #[doc = "Bit 0 - Non-Maskable Interrupt"]
        #[inline(always)]
        pub fn nmi(&mut self) -> _NMIW {
            _NMIW { w: self }
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
#[doc = "Wake-Up Enable"]
pub struct WAKEUP {
    register: VolatileCell<u32>,
}
#[doc = "Wake-Up Enable"]
pub mod wakeup {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::WAKEUP {
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
    pub struct WAKEUPEN0R {
        bits: bool,
    }
    impl WAKEUPEN0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN1R {
        bits: bool,
    }
    impl WAKEUPEN1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN2R {
        bits: bool,
    }
    impl WAKEUPEN2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN3R {
        bits: bool,
    }
    impl WAKEUPEN3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN4R {
        bits: bool,
    }
    impl WAKEUPEN4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN5R {
        bits: bool,
    }
    impl WAKEUPEN5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN6R {
        bits: bool,
    }
    impl WAKEUPEN6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN7R {
        bits: bool,
    }
    impl WAKEUPEN7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN8R {
        bits: bool,
    }
    impl WAKEUPEN8R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN9R {
        bits: bool,
    }
    impl WAKEUPEN9R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN10R {
        bits: bool,
    }
    impl WAKEUPEN10R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN11R {
        bits: bool,
    }
    impl WAKEUPEN11R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN12R {
        bits: bool,
    }
    impl WAKEUPEN12R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN13R {
        bits: bool,
    }
    impl WAKEUPEN13R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN14R {
        bits: bool,
    }
    impl WAKEUPEN14R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WAKEUPEN15R {
        bits: bool,
    }
    impl WAKEUPEN15R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _WAKEUPEN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN4W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN5W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN6W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN7W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN8W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN9W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN10W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN11W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN12W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN12W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN13W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN13W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN14W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN14W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WAKEUPEN15W<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAKEUPEN15W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen0(&self) -> WAKEUPEN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN0R { bits }
        }
        #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen1(&self) -> WAKEUPEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN1R { bits }
        }
        #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen2(&self) -> WAKEUPEN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN2R { bits }
        }
        #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen3(&self) -> WAKEUPEN3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN3R { bits }
        }
        #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen4(&self) -> WAKEUPEN4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN4R { bits }
        }
        #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen5(&self) -> WAKEUPEN5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN5R { bits }
        }
        #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen6(&self) -> WAKEUPEN6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN6R { bits }
        }
        #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen7(&self) -> WAKEUPEN7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN7R { bits }
        }
        #[doc = "Bit 8 - External Interrupt 8 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen8(&self) -> WAKEUPEN8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN8R { bits }
        }
        #[doc = "Bit 9 - External Interrupt 9 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen9(&self) -> WAKEUPEN9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN9R { bits }
        }
        #[doc = "Bit 10 - External Interrupt 10 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen10(&self) -> WAKEUPEN10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN10R { bits }
        }
        #[doc = "Bit 11 - External Interrupt 11 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen11(&self) -> WAKEUPEN11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN11R { bits }
        }
        #[doc = "Bit 12 - External Interrupt 12 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen12(&self) -> WAKEUPEN12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN12R { bits }
        }
        #[doc = "Bit 13 - External Interrupt 13 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen13(&self) -> WAKEUPEN13R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN13R { bits }
        }
        #[doc = "Bit 14 - External Interrupt 14 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen14(&self) -> WAKEUPEN14R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN14R { bits }
        }
        #[doc = "Bit 15 - External Interrupt 15 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen15(&self) -> WAKEUPEN15R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WAKEUPEN15R { bits }
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
        #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen0(&mut self) -> _WAKEUPEN0W {
            _WAKEUPEN0W { w: self }
        }
        #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen1(&mut self) -> _WAKEUPEN1W {
            _WAKEUPEN1W { w: self }
        }
        #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen2(&mut self) -> _WAKEUPEN2W {
            _WAKEUPEN2W { w: self }
        }
        #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen3(&mut self) -> _WAKEUPEN3W {
            _WAKEUPEN3W { w: self }
        }
        #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen4(&mut self) -> _WAKEUPEN4W {
            _WAKEUPEN4W { w: self }
        }
        #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen5(&mut self) -> _WAKEUPEN5W {
            _WAKEUPEN5W { w: self }
        }
        #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen6(&mut self) -> _WAKEUPEN6W {
            _WAKEUPEN6W { w: self }
        }
        #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen7(&mut self) -> _WAKEUPEN7W {
            _WAKEUPEN7W { w: self }
        }
        #[doc = "Bit 8 - External Interrupt 8 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen8(&mut self) -> _WAKEUPEN8W {
            _WAKEUPEN8W { w: self }
        }
        #[doc = "Bit 9 - External Interrupt 9 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen9(&mut self) -> _WAKEUPEN9W {
            _WAKEUPEN9W { w: self }
        }
        #[doc = "Bit 10 - External Interrupt 10 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen10(&mut self) -> _WAKEUPEN10W {
            _WAKEUPEN10W { w: self }
        }
        #[doc = "Bit 11 - External Interrupt 11 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen11(&mut self) -> _WAKEUPEN11W {
            _WAKEUPEN11W { w: self }
        }
        #[doc = "Bit 12 - External Interrupt 12 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen12(&mut self) -> _WAKEUPEN12W {
            _WAKEUPEN12W { w: self }
        }
        #[doc = "Bit 13 - External Interrupt 13 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen13(&mut self) -> _WAKEUPEN13W {
            _WAKEUPEN13W { w: self }
        }
        #[doc = "Bit 14 - External Interrupt 14 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen14(&mut self) -> _WAKEUPEN14W {
            _WAKEUPEN14W { w: self }
        }
        #[doc = "Bit 15 - External Interrupt 15 Wake-up Enable"]
        #[inline(always)]
        pub fn wakeupen15(&mut self) -> _WAKEUPEN15W {
            _WAKEUPEN15W { w: self }
        }
    }
}
