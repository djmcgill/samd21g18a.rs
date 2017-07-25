use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Status"]
    pub status: STATUS,
    #[doc = "0x02 - Generic Clock Control"]
    pub clkctrl: CLKCTRL,
    #[doc = "0x04 - Generic Clock Generator Control"]
    pub genctrl: GENCTRL,
    #[doc = "0x08 - Generic Clock Generator Division"]
    pub gendiv: GENDIV,
}
#[doc = "Generic Clock Control"]
pub struct CLKCTRL {
    register: VolatileCell<u16>,
}
#[doc = "Generic Clock Control"]
pub mod clkctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::CLKCTRL {
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
    pub struct IDR {
        bits: u8,
    }
    impl IDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = "Possible values of the field `GEN`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum GENR {
        #[doc = "Generic clock generator 0"]
        GCLK0,
        #[doc = "Generic clock generator 1"]
        GCLK1,
        #[doc = "Generic clock generator 2"]
        GCLK2,
        #[doc = "Generic clock generator 3"]
        GCLK3,
        #[doc = "Generic clock generator 4"]
        GCLK4,
        #[doc = "Generic clock generator 5"]
        GCLK5,
        #[doc = "Generic clock generator 6"]
        GCLK6,
        #[doc = "Generic clock generator 7"]
        GCLK7,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl GENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                GENR::GCLK0 => 0,
                GENR::GCLK1 => 1,
                GENR::GCLK2 => 2,
                GENR::GCLK3 => 3,
                GENR::GCLK4 => 4,
                GENR::GCLK5 => 5,
                GENR::GCLK6 => 6,
                GENR::GCLK7 => 7,
                GENR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> GENR {
            match value {
                0 => GENR::GCLK0,
                1 => GENR::GCLK1,
                2 => GENR::GCLK2,
                3 => GENR::GCLK3,
                4 => GENR::GCLK4,
                5 => GENR::GCLK5,
                6 => GENR::GCLK6,
                7 => GENR::GCLK7,
                i => GENR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `GCLK0`"]
        #[inline(always)]
        pub fn is_gclk0(&self) -> bool {
            *self == GENR::GCLK0
        }
        #[doc = "Checks if the value of the field is `GCLK1`"]
        #[inline(always)]
        pub fn is_gclk1(&self) -> bool {
            *self == GENR::GCLK1
        }
        #[doc = "Checks if the value of the field is `GCLK2`"]
        #[inline(always)]
        pub fn is_gclk2(&self) -> bool {
            *self == GENR::GCLK2
        }
        #[doc = "Checks if the value of the field is `GCLK3`"]
        #[inline(always)]
        pub fn is_gclk3(&self) -> bool {
            *self == GENR::GCLK3
        }
        #[doc = "Checks if the value of the field is `GCLK4`"]
        #[inline(always)]
        pub fn is_gclk4(&self) -> bool {
            *self == GENR::GCLK4
        }
        #[doc = "Checks if the value of the field is `GCLK5`"]
        #[inline(always)]
        pub fn is_gclk5(&self) -> bool {
            *self == GENR::GCLK5
        }
        #[doc = "Checks if the value of the field is `GCLK6`"]
        #[inline(always)]
        pub fn is_gclk6(&self) -> bool {
            *self == GENR::GCLK6
        }
        #[doc = "Checks if the value of the field is `GCLK7`"]
        #[inline(always)]
        pub fn is_gclk7(&self) -> bool {
            *self == GENR::GCLK7
        }
    }
    #[doc = r" Value of the field"]
    pub struct CLKENR {
        bits: bool,
    }
    impl CLKENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WRTLOCKR {
        bits: bool,
    }
    impl WRTLOCKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _IDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `GEN`"]
    pub enum GENW {
        #[doc = "Generic clock generator 0"]
        GCLK0,
        #[doc = "Generic clock generator 1"]
        GCLK1,
        #[doc = "Generic clock generator 2"]
        GCLK2,
        #[doc = "Generic clock generator 3"]
        GCLK3,
        #[doc = "Generic clock generator 4"]
        GCLK4,
        #[doc = "Generic clock generator 5"]
        GCLK5,
        #[doc = "Generic clock generator 6"]
        GCLK6,
        #[doc = "Generic clock generator 7"]
        GCLK7,
    }
    impl GENW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                GENW::GCLK0 => 0,
                GENW::GCLK1 => 1,
                GENW::GCLK2 => 2,
                GENW::GCLK3 => 3,
                GENW::GCLK4 => 4,
                GENW::GCLK5 => 5,
                GENW::GCLK6 => 6,
                GENW::GCLK7 => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _GENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _GENW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: GENW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Generic clock generator 0"]
        #[inline(always)]
        pub fn gclk0(self) -> &'a mut W {
            self.variant(GENW::GCLK0)
        }
        #[doc = "Generic clock generator 1"]
        #[inline(always)]
        pub fn gclk1(self) -> &'a mut W {
            self.variant(GENW::GCLK1)
        }
        #[doc = "Generic clock generator 2"]
        #[inline(always)]
        pub fn gclk2(self) -> &'a mut W {
            self.variant(GENW::GCLK2)
        }
        #[doc = "Generic clock generator 3"]
        #[inline(always)]
        pub fn gclk3(self) -> &'a mut W {
            self.variant(GENW::GCLK3)
        }
        #[doc = "Generic clock generator 4"]
        #[inline(always)]
        pub fn gclk4(self) -> &'a mut W {
            self.variant(GENW::GCLK4)
        }
        #[doc = "Generic clock generator 5"]
        #[inline(always)]
        pub fn gclk5(self) -> &'a mut W {
            self.variant(GENW::GCLK5)
        }
        #[doc = "Generic clock generator 6"]
        #[inline(always)]
        pub fn gclk6(self) -> &'a mut W {
            self.variant(GENW::GCLK6)
        }
        #[doc = "Generic clock generator 7"]
        #[inline(always)]
        pub fn gclk7(self) -> &'a mut W {
            self.variant(GENW::GCLK7)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CLKENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WRTLOCKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRTLOCKW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bits 0:5 - Generic Clock Selection ID"]
        #[inline(always)]
        pub fn id(&self) -> IDR {
            let bits = {
                const MASK: u8 = 63;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            };
            IDR { bits }
        }
        #[doc = "Bits 8:11 - Generic Clock Generator"]
        #[inline(always)]
        pub fn gen(&self) -> GENR {
            GENR::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            })
        }
        #[doc = "Bit 14 - Clock Enable"]
        #[inline(always)]
        pub fn clken(&self) -> CLKENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            CLKENR { bits }
        }
        #[doc = "Bit 15 - Write Lock"]
        #[inline(always)]
        pub fn wrtlock(&self) -> WRTLOCKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            WRTLOCKR { bits }
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
        #[doc = "Bits 0:5 - Generic Clock Selection ID"]
        #[inline(always)]
        pub fn id(&mut self) -> _IDW {
            _IDW { w: self }
        }
        #[doc = "Bits 8:11 - Generic Clock Generator"]
        #[inline(always)]
        pub fn gen(&mut self) -> _GENW {
            _GENW { w: self }
        }
        #[doc = "Bit 14 - Clock Enable"]
        #[inline(always)]
        pub fn clken(&mut self) -> _CLKENW {
            _CLKENW { w: self }
        }
        #[doc = "Bit 15 - Write Lock"]
        #[inline(always)]
        pub fn wrtlock(&mut self) -> _WRTLOCKW {
            _WRTLOCKW { w: self }
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
    }
}
#[doc = "Generic Clock Generator Control"]
pub struct GENCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Generic Clock Generator Control"]
pub mod genctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::GENCTRL {
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
    pub struct IDR {
        bits: u8,
    }
    impl IDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = "Possible values of the field `SRC`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SRCR {
        #[doc = "XOSC oscillator output"]
        XOSC,
        #[doc = "Generator input pad"]
        GCLKIN,
        #[doc = "Generic clock generator 1 output"]
        GCLKGEN1,
        #[doc = "OSCULP32K oscillator output"]
        OSCULP32K,
        #[doc = "OSC32K oscillator output"]
        OSC32K,
        #[doc = "XOSC32K oscillator output"]
        XOSC32K,
        #[doc = "OSC8M oscillator output"]
        OSC8M,
        #[doc = "DFLL48M output"]
        DFLL48M,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SRCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SRCR::XOSC => 0,
                SRCR::GCLKIN => 1,
                SRCR::GCLKGEN1 => 2,
                SRCR::OSCULP32K => 3,
                SRCR::OSC32K => 4,
                SRCR::XOSC32K => 5,
                SRCR::OSC8M => 6,
                SRCR::DFLL48M => 7,
                SRCR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SRCR {
            match value {
                0 => SRCR::XOSC,
                1 => SRCR::GCLKIN,
                2 => SRCR::GCLKGEN1,
                3 => SRCR::OSCULP32K,
                4 => SRCR::OSC32K,
                5 => SRCR::XOSC32K,
                6 => SRCR::OSC8M,
                7 => SRCR::DFLL48M,
                i => SRCR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `XOSC`"]
        #[inline(always)]
        pub fn is_xosc(&self) -> bool {
            *self == SRCR::XOSC
        }
        #[doc = "Checks if the value of the field is `GCLKIN`"]
        #[inline(always)]
        pub fn is_gclkin(&self) -> bool {
            *self == SRCR::GCLKIN
        }
        #[doc = "Checks if the value of the field is `GCLKGEN1`"]
        #[inline(always)]
        pub fn is_gclkgen1(&self) -> bool {
            *self == SRCR::GCLKGEN1
        }
        #[doc = "Checks if the value of the field is `OSCULP32K`"]
        #[inline(always)]
        pub fn is_osculp32k(&self) -> bool {
            *self == SRCR::OSCULP32K
        }
        #[doc = "Checks if the value of the field is `OSC32K`"]
        #[inline(always)]
        pub fn is_osc32k(&self) -> bool {
            *self == SRCR::OSC32K
        }
        #[doc = "Checks if the value of the field is `XOSC32K`"]
        #[inline(always)]
        pub fn is_xosc32k(&self) -> bool {
            *self == SRCR::XOSC32K
        }
        #[doc = "Checks if the value of the field is `OSC8M`"]
        #[inline(always)]
        pub fn is_osc8m(&self) -> bool {
            *self == SRCR::OSC8M
        }
        #[doc = "Checks if the value of the field is `DFLL48M`"]
        #[inline(always)]
        pub fn is_dfll48m(&self) -> bool {
            *self == SRCR::DFLL48M
        }
    }
    #[doc = r" Value of the field"]
    pub struct GENENR {
        bits: bool,
    }
    impl GENENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct IDCR {
        bits: bool,
    }
    impl IDCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct OOVR {
        bits: bool,
    }
    impl OOVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct OER {
        bits: bool,
    }
    impl OER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct DIVSELR {
        bits: bool,
    }
    impl DIVSELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
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
    pub struct _IDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDW<'a> {
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
    #[doc = "Values that can be written to the field `SRC`"]
    pub enum SRCW {
        #[doc = "XOSC oscillator output"]
        XOSC,
        #[doc = "Generator input pad"]
        GCLKIN,
        #[doc = "Generic clock generator 1 output"]
        GCLKGEN1,
        #[doc = "OSCULP32K oscillator output"]
        OSCULP32K,
        #[doc = "OSC32K oscillator output"]
        OSC32K,
        #[doc = "XOSC32K oscillator output"]
        XOSC32K,
        #[doc = "OSC8M oscillator output"]
        OSC8M,
        #[doc = "DFLL48M output"]
        DFLL48M,
    }
    impl SRCW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SRCW::XOSC => 0,
                SRCW::GCLKIN => 1,
                SRCW::GCLKGEN1 => 2,
                SRCW::OSCULP32K => 3,
                SRCW::OSC32K => 4,
                SRCW::XOSC32K => 5,
                SRCW::OSC8M => 6,
                SRCW::DFLL48M => 7,
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
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "XOSC oscillator output"]
        #[inline(always)]
        pub fn xosc(self) -> &'a mut W {
            self.variant(SRCW::XOSC)
        }
        #[doc = "Generator input pad"]
        #[inline(always)]
        pub fn gclkin(self) -> &'a mut W {
            self.variant(SRCW::GCLKIN)
        }
        #[doc = "Generic clock generator 1 output"]
        #[inline(always)]
        pub fn gclkgen1(self) -> &'a mut W {
            self.variant(SRCW::GCLKGEN1)
        }
        #[doc = "OSCULP32K oscillator output"]
        #[inline(always)]
        pub fn osculp32k(self) -> &'a mut W {
            self.variant(SRCW::OSCULP32K)
        }
        #[doc = "OSC32K oscillator output"]
        #[inline(always)]
        pub fn osc32k(self) -> &'a mut W {
            self.variant(SRCW::OSC32K)
        }
        #[doc = "XOSC32K oscillator output"]
        #[inline(always)]
        pub fn xosc32k(self) -> &'a mut W {
            self.variant(SRCW::XOSC32K)
        }
        #[doc = "OSC8M oscillator output"]
        #[inline(always)]
        pub fn osc8m(self) -> &'a mut W {
            self.variant(SRCW::OSC8M)
        }
        #[doc = "DFLL48M output"]
        #[inline(always)]
        pub fn dfll48m(self) -> &'a mut W {
            self.variant(SRCW::DFLL48M)
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
    pub struct _GENENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _GENENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _IDCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDCW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _OOVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OOVW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _OEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DIVSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIVSELW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
            const OFFSET: u8 = 21;
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
        #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
        #[inline(always)]
        pub fn id(&self) -> IDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IDR { bits }
        }
        #[doc = "Bits 8:12 - Source Select"]
        #[inline(always)]
        pub fn src(&self) -> SRCR {
            SRCR::_from({
                const MASK: u8 = 31;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 16 - Generic Clock Generator Enable"]
        #[inline(always)]
        pub fn genen(&self) -> GENENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            GENENR { bits }
        }
        #[doc = "Bit 17 - Improve Duty Cycle"]
        #[inline(always)]
        pub fn idc(&self) -> IDCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            IDCR { bits }
        }
        #[doc = "Bit 18 - Output Off Value"]
        #[inline(always)]
        pub fn oov(&self) -> OOVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OOVR { bits }
        }
        #[doc = "Bit 19 - Output Enable"]
        #[inline(always)]
        pub fn oe(&self) -> OER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OER { bits }
        }
        #[doc = "Bit 20 - Divide Selection"]
        #[inline(always)]
        pub fn divsel(&self) -> DIVSELR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DIVSELR { bits }
        }
        #[doc = "Bit 21 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&self) -> RUNSTDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
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
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
        #[inline(always)]
        pub fn id(&mut self) -> _IDW {
            _IDW { w: self }
        }
        #[doc = "Bits 8:12 - Source Select"]
        #[inline(always)]
        pub fn src(&mut self) -> _SRCW {
            _SRCW { w: self }
        }
        #[doc = "Bit 16 - Generic Clock Generator Enable"]
        #[inline(always)]
        pub fn genen(&mut self) -> _GENENW {
            _GENENW { w: self }
        }
        #[doc = "Bit 17 - Improve Duty Cycle"]
        #[inline(always)]
        pub fn idc(&mut self) -> _IDCW {
            _IDCW { w: self }
        }
        #[doc = "Bit 18 - Output Off Value"]
        #[inline(always)]
        pub fn oov(&mut self) -> _OOVW {
            _OOVW { w: self }
        }
        #[doc = "Bit 19 - Output Enable"]
        #[inline(always)]
        pub fn oe(&mut self) -> _OEW {
            _OEW { w: self }
        }
        #[doc = "Bit 20 - Divide Selection"]
        #[inline(always)]
        pub fn divsel(&mut self) -> _DIVSELW {
            _DIVSELW { w: self }
        }
        #[doc = "Bit 21 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&mut self) -> _RUNSTDBYW {
            _RUNSTDBYW { w: self }
        }
    }
}
#[doc = "Generic Clock Generator Division"]
pub struct GENDIV {
    register: VolatileCell<u32>,
}
#[doc = "Generic Clock Generator Division"]
pub mod gendiv {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::GENDIV {
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
    pub struct IDR {
        bits: u8,
    }
    impl IDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct DIVR {
        bits: u16,
    }
    impl DIVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _IDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDW<'a> {
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
    pub struct _DIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIVW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 8;
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
        #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
        #[inline(always)]
        pub fn id(&self) -> IDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IDR { bits }
        }
        #[doc = "Bits 8:23 - Division Factor"]
        #[inline(always)]
        pub fn div(&self) -> DIVR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DIVR { bits }
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
        #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
        #[inline(always)]
        pub fn id(&mut self) -> _IDW {
            _IDW { w: self }
        }
        #[doc = "Bits 8:23 - Division Factor"]
        #[inline(always)]
        pub fn div(&mut self) -> _DIVW {
            _DIVW { w: self }
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
        #[doc = "Bit 7 - Synchronization Busy Status"]
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
