use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Configuration"]
    pub config: CONFIG,
    #[doc = "0x02 - Early Warning Interrupt Control"]
    pub ewctrl: EWCTRL,
    _reserved0: [u8; 1usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x07 - Status"]
    pub status: STATUS,
    #[doc = "0x08 - Clear"]
    pub clear: CLEAR,
}
#[doc = "Clear"]
pub struct CLEAR {
    register: VolatileCell<u8>,
}
#[doc = "Clear"]
pub mod clear {
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CLEAR {
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
    #[doc = "Values that can be written to the field `CLEAR`"]
    pub enum CLEARW {
        #[doc = "Clear Key"]
        KEY,
    }
    impl CLEARW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CLEARW::KEY => 165,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CLEARW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLEARW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CLEARW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Clear Key"]
        #[inline(always)]
        pub fn key(self) -> &'a mut W {
            self.variant(CLEARW::KEY)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
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
        #[doc = "Bits 0:7 - Watchdog Clear"]
        #[inline(always)]
        pub fn clear(&mut self) -> _CLEARW {
            _CLEARW { w: self }
        }
    }
}
#[doc = "Configuration"]
pub struct CONFIG {
    register: VolatileCell<u8>,
}
#[doc = "Configuration"]
pub mod config {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
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
    #[doc = "Possible values of the field `PER`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PERR {
        #[doc = "8 clock cycles"]
        _0X0,
        #[doc = "16 clock cycles"]
        _0X1,
        #[doc = "32 clock cycles"]
        _0X2,
        #[doc = "64 clock cycles"]
        _0X3,
        #[doc = "128 clock cycles"]
        _0X4,
        #[doc = "256 clock cycles"]
        _0X5,
        #[doc = "512 clock cycles"]
        _0X6,
        #[doc = "1024 clock cycles"]
        _0X7,
        #[doc = "2048 clock cycles"]
        _0X8,
        #[doc = "4096 clock cycles"]
        _0X9,
        #[doc = "8192 clock cycles"]
        _0XA,
        #[doc = "16384 clock cycles"]
        _0XB,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl PERR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                PERR::_0X0 => 0,
                PERR::_0X1 => 1,
                PERR::_0X2 => 2,
                PERR::_0X3 => 3,
                PERR::_0X4 => 4,
                PERR::_0X5 => 5,
                PERR::_0X6 => 6,
                PERR::_0X7 => 7,
                PERR::_0X8 => 8,
                PERR::_0X9 => 9,
                PERR::_0XA => 10,
                PERR::_0XB => 11,
                PERR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> PERR {
            match value {
                0 => PERR::_0X0,
                1 => PERR::_0X1,
                2 => PERR::_0X2,
                3 => PERR::_0X3,
                4 => PERR::_0X4,
                5 => PERR::_0X5,
                6 => PERR::_0X6,
                7 => PERR::_0X7,
                8 => PERR::_0X8,
                9 => PERR::_0X9,
                10 => PERR::_0XA,
                11 => PERR::_0XB,
                i => PERR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `_0X0`"]
        #[inline(always)]
        pub fn is_0x0(&self) -> bool {
            *self == PERR::_0X0
        }
        #[doc = "Checks if the value of the field is `_0X1`"]
        #[inline(always)]
        pub fn is_0x1(&self) -> bool {
            *self == PERR::_0X1
        }
        #[doc = "Checks if the value of the field is `_0X2`"]
        #[inline(always)]
        pub fn is_0x2(&self) -> bool {
            *self == PERR::_0X2
        }
        #[doc = "Checks if the value of the field is `_0X3`"]
        #[inline(always)]
        pub fn is_0x3(&self) -> bool {
            *self == PERR::_0X3
        }
        #[doc = "Checks if the value of the field is `_0X4`"]
        #[inline(always)]
        pub fn is_0x4(&self) -> bool {
            *self == PERR::_0X4
        }
        #[doc = "Checks if the value of the field is `_0X5`"]
        #[inline(always)]
        pub fn is_0x5(&self) -> bool {
            *self == PERR::_0X5
        }
        #[doc = "Checks if the value of the field is `_0X6`"]
        #[inline(always)]
        pub fn is_0x6(&self) -> bool {
            *self == PERR::_0X6
        }
        #[doc = "Checks if the value of the field is `_0X7`"]
        #[inline(always)]
        pub fn is_0x7(&self) -> bool {
            *self == PERR::_0X7
        }
        #[doc = "Checks if the value of the field is `_0X8`"]
        #[inline(always)]
        pub fn is_0x8(&self) -> bool {
            *self == PERR::_0X8
        }
        #[doc = "Checks if the value of the field is `_0X9`"]
        #[inline(always)]
        pub fn is_0x9(&self) -> bool {
            *self == PERR::_0X9
        }
        #[doc = "Checks if the value of the field is `_0XA`"]
        #[inline(always)]
        pub fn is_0x_a(&self) -> bool {
            *self == PERR::_0XA
        }
        #[doc = "Checks if the value of the field is `_0XB`"]
        #[inline(always)]
        pub fn is_0x_b(&self) -> bool {
            *self == PERR::_0XB
        }
    }
    #[doc = "Possible values of the field `WINDOW`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum WINDOWR {
        #[doc = "8 clock cycles"]
        _0X0,
        #[doc = "16 clock cycles"]
        _0X1,
        #[doc = "32 clock cycles"]
        _0X2,
        #[doc = "64 clock cycles"]
        _0X3,
        #[doc = "128 clock cycles"]
        _0X4,
        #[doc = "256 clock cycles"]
        _0X5,
        #[doc = "512 clock cycles"]
        _0X6,
        #[doc = "1024 clock cycles"]
        _0X7,
        #[doc = "2048 clock cycles"]
        _0X8,
        #[doc = "4096 clock cycles"]
        _0X9,
        #[doc = "8192 clock cycles"]
        _0XA,
        #[doc = "16384 clock cycles"]
        _0XB,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl WINDOWR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                WINDOWR::_0X0 => 0,
                WINDOWR::_0X1 => 1,
                WINDOWR::_0X2 => 2,
                WINDOWR::_0X3 => 3,
                WINDOWR::_0X4 => 4,
                WINDOWR::_0X5 => 5,
                WINDOWR::_0X6 => 6,
                WINDOWR::_0X7 => 7,
                WINDOWR::_0X8 => 8,
                WINDOWR::_0X9 => 9,
                WINDOWR::_0XA => 10,
                WINDOWR::_0XB => 11,
                WINDOWR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> WINDOWR {
            match value {
                0 => WINDOWR::_0X0,
                1 => WINDOWR::_0X1,
                2 => WINDOWR::_0X2,
                3 => WINDOWR::_0X3,
                4 => WINDOWR::_0X4,
                5 => WINDOWR::_0X5,
                6 => WINDOWR::_0X6,
                7 => WINDOWR::_0X7,
                8 => WINDOWR::_0X8,
                9 => WINDOWR::_0X9,
                10 => WINDOWR::_0XA,
                11 => WINDOWR::_0XB,
                i => WINDOWR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `_0X0`"]
        #[inline(always)]
        pub fn is_0x0(&self) -> bool {
            *self == WINDOWR::_0X0
        }
        #[doc = "Checks if the value of the field is `_0X1`"]
        #[inline(always)]
        pub fn is_0x1(&self) -> bool {
            *self == WINDOWR::_0X1
        }
        #[doc = "Checks if the value of the field is `_0X2`"]
        #[inline(always)]
        pub fn is_0x2(&self) -> bool {
            *self == WINDOWR::_0X2
        }
        #[doc = "Checks if the value of the field is `_0X3`"]
        #[inline(always)]
        pub fn is_0x3(&self) -> bool {
            *self == WINDOWR::_0X3
        }
        #[doc = "Checks if the value of the field is `_0X4`"]
        #[inline(always)]
        pub fn is_0x4(&self) -> bool {
            *self == WINDOWR::_0X4
        }
        #[doc = "Checks if the value of the field is `_0X5`"]
        #[inline(always)]
        pub fn is_0x5(&self) -> bool {
            *self == WINDOWR::_0X5
        }
        #[doc = "Checks if the value of the field is `_0X6`"]
        #[inline(always)]
        pub fn is_0x6(&self) -> bool {
            *self == WINDOWR::_0X6
        }
        #[doc = "Checks if the value of the field is `_0X7`"]
        #[inline(always)]
        pub fn is_0x7(&self) -> bool {
            *self == WINDOWR::_0X7
        }
        #[doc = "Checks if the value of the field is `_0X8`"]
        #[inline(always)]
        pub fn is_0x8(&self) -> bool {
            *self == WINDOWR::_0X8
        }
        #[doc = "Checks if the value of the field is `_0X9`"]
        #[inline(always)]
        pub fn is_0x9(&self) -> bool {
            *self == WINDOWR::_0X9
        }
        #[doc = "Checks if the value of the field is `_0XA`"]
        #[inline(always)]
        pub fn is_0x_a(&self) -> bool {
            *self == WINDOWR::_0XA
        }
        #[doc = "Checks if the value of the field is `_0XB`"]
        #[inline(always)]
        pub fn is_0x_b(&self) -> bool {
            *self == WINDOWR::_0XB
        }
    }
    #[doc = "Values that can be written to the field `PER`"]
    pub enum PERW {
        #[doc = "8 clock cycles"]
        _0X0,
        #[doc = "16 clock cycles"]
        _0X1,
        #[doc = "32 clock cycles"]
        _0X2,
        #[doc = "64 clock cycles"]
        _0X3,
        #[doc = "128 clock cycles"]
        _0X4,
        #[doc = "256 clock cycles"]
        _0X5,
        #[doc = "512 clock cycles"]
        _0X6,
        #[doc = "1024 clock cycles"]
        _0X7,
        #[doc = "2048 clock cycles"]
        _0X8,
        #[doc = "4096 clock cycles"]
        _0X9,
        #[doc = "8192 clock cycles"]
        _0XA,
        #[doc = "16384 clock cycles"]
        _0XB,
    }
    impl PERW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                PERW::_0X0 => 0,
                PERW::_0X1 => 1,
                PERW::_0X2 => 2,
                PERW::_0X3 => 3,
                PERW::_0X4 => 4,
                PERW::_0X5 => 5,
                PERW::_0X6 => 6,
                PERW::_0X7 => 7,
                PERW::_0X8 => 8,
                PERW::_0X9 => 9,
                PERW::_0XA => 10,
                PERW::_0XB => 11,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _PERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PERW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: PERW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "8 clock cycles"]
        #[inline(always)]
        pub fn _0x0(self) -> &'a mut W {
            self.variant(PERW::_0X0)
        }
        #[doc = "16 clock cycles"]
        #[inline(always)]
        pub fn _0x1(self) -> &'a mut W {
            self.variant(PERW::_0X1)
        }
        #[doc = "32 clock cycles"]
        #[inline(always)]
        pub fn _0x2(self) -> &'a mut W {
            self.variant(PERW::_0X2)
        }
        #[doc = "64 clock cycles"]
        #[inline(always)]
        pub fn _0x3(self) -> &'a mut W {
            self.variant(PERW::_0X3)
        }
        #[doc = "128 clock cycles"]
        #[inline(always)]
        pub fn _0x4(self) -> &'a mut W {
            self.variant(PERW::_0X4)
        }
        #[doc = "256 clock cycles"]
        #[inline(always)]
        pub fn _0x5(self) -> &'a mut W {
            self.variant(PERW::_0X5)
        }
        #[doc = "512 clock cycles"]
        #[inline(always)]
        pub fn _0x6(self) -> &'a mut W {
            self.variant(PERW::_0X6)
        }
        #[doc = "1024 clock cycles"]
        #[inline(always)]
        pub fn _0x7(self) -> &'a mut W {
            self.variant(PERW::_0X7)
        }
        #[doc = "2048 clock cycles"]
        #[inline(always)]
        pub fn _0x8(self) -> &'a mut W {
            self.variant(PERW::_0X8)
        }
        #[doc = "4096 clock cycles"]
        #[inline(always)]
        pub fn _0x9(self) -> &'a mut W {
            self.variant(PERW::_0X9)
        }
        #[doc = "8192 clock cycles"]
        #[inline(always)]
        pub fn _0x_a(self) -> &'a mut W {
            self.variant(PERW::_0XA)
        }
        #[doc = "16384 clock cycles"]
        #[inline(always)]
        pub fn _0x_b(self) -> &'a mut W {
            self.variant(PERW::_0XB)
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
    #[doc = "Values that can be written to the field `WINDOW`"]
    pub enum WINDOWW {
        #[doc = "8 clock cycles"]
        _0X0,
        #[doc = "16 clock cycles"]
        _0X1,
        #[doc = "32 clock cycles"]
        _0X2,
        #[doc = "64 clock cycles"]
        _0X3,
        #[doc = "128 clock cycles"]
        _0X4,
        #[doc = "256 clock cycles"]
        _0X5,
        #[doc = "512 clock cycles"]
        _0X6,
        #[doc = "1024 clock cycles"]
        _0X7,
        #[doc = "2048 clock cycles"]
        _0X8,
        #[doc = "4096 clock cycles"]
        _0X9,
        #[doc = "8192 clock cycles"]
        _0XA,
        #[doc = "16384 clock cycles"]
        _0XB,
    }
    impl WINDOWW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                WINDOWW::_0X0 => 0,
                WINDOWW::_0X1 => 1,
                WINDOWW::_0X2 => 2,
                WINDOWW::_0X3 => 3,
                WINDOWW::_0X4 => 4,
                WINDOWW::_0X5 => 5,
                WINDOWW::_0X6 => 6,
                WINDOWW::_0X7 => 7,
                WINDOWW::_0X8 => 8,
                WINDOWW::_0X9 => 9,
                WINDOWW::_0XA => 10,
                WINDOWW::_0XB => 11,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _WINDOWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WINDOWW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: WINDOWW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "8 clock cycles"]
        #[inline(always)]
        pub fn _0x0(self) -> &'a mut W {
            self.variant(WINDOWW::_0X0)
        }
        #[doc = "16 clock cycles"]
        #[inline(always)]
        pub fn _0x1(self) -> &'a mut W {
            self.variant(WINDOWW::_0X1)
        }
        #[doc = "32 clock cycles"]
        #[inline(always)]
        pub fn _0x2(self) -> &'a mut W {
            self.variant(WINDOWW::_0X2)
        }
        #[doc = "64 clock cycles"]
        #[inline(always)]
        pub fn _0x3(self) -> &'a mut W {
            self.variant(WINDOWW::_0X3)
        }
        #[doc = "128 clock cycles"]
        #[inline(always)]
        pub fn _0x4(self) -> &'a mut W {
            self.variant(WINDOWW::_0X4)
        }
        #[doc = "256 clock cycles"]
        #[inline(always)]
        pub fn _0x5(self) -> &'a mut W {
            self.variant(WINDOWW::_0X5)
        }
        #[doc = "512 clock cycles"]
        #[inline(always)]
        pub fn _0x6(self) -> &'a mut W {
            self.variant(WINDOWW::_0X6)
        }
        #[doc = "1024 clock cycles"]
        #[inline(always)]
        pub fn _0x7(self) -> &'a mut W {
            self.variant(WINDOWW::_0X7)
        }
        #[doc = "2048 clock cycles"]
        #[inline(always)]
        pub fn _0x8(self) -> &'a mut W {
            self.variant(WINDOWW::_0X8)
        }
        #[doc = "4096 clock cycles"]
        #[inline(always)]
        pub fn _0x9(self) -> &'a mut W {
            self.variant(WINDOWW::_0X9)
        }
        #[doc = "8192 clock cycles"]
        #[inline(always)]
        pub fn _0x_a(self) -> &'a mut W {
            self.variant(WINDOWW::_0XA)
        }
        #[doc = "16384 clock cycles"]
        #[inline(always)]
        pub fn _0x_b(self) -> &'a mut W {
            self.variant(WINDOWW::_0XB)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
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
        #[doc = "Bits 0:3 - Time-Out Period"]
        #[inline(always)]
        pub fn per(&self) -> PERR {
            PERR::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            })
        }
        #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
        #[inline(always)]
        pub fn window(&self) -> WINDOWR {
            WINDOWR::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            })
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 187 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:3 - Time-Out Period"]
        #[inline(always)]
        pub fn per(&mut self) -> _PERW {
            _PERW { w: self }
        }
        #[doc = "Bits 4:7 - Window Mode Time-Out Period"]
        #[inline(always)]
        pub fn window(&mut self) -> _WINDOWW {
            _WINDOWW { w: self }
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
    pub struct WENR {
        bits: bool,
    }
    impl WENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct ALWAYSONR {
        bits: bool,
    }
    impl ALWAYSONR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _WENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ALWAYSONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ALWAYSONW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 2 - Watchdog Timer Window Mode Enable"]
        #[inline(always)]
        pub fn wen(&self) -> WENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            WENR { bits }
        }
        #[doc = "Bit 7 - Always-On"]
        #[inline(always)]
        pub fn alwayson(&self) -> ALWAYSONR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            ALWAYSONR { bits }
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
        #[doc = "Bit 1 - Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
        #[doc = "Bit 2 - Watchdog Timer Window Mode Enable"]
        #[inline(always)]
        pub fn wen(&mut self) -> _WENW {
            _WENW { w: self }
        }
        #[doc = "Bit 7 - Always-On"]
        #[inline(always)]
        pub fn alwayson(&mut self) -> _ALWAYSONW {
            _ALWAYSONW { w: self }
        }
    }
}
#[doc = "Early Warning Interrupt Control"]
pub struct EWCTRL {
    register: VolatileCell<u8>,
}
#[doc = "Early Warning Interrupt Control"]
pub mod ewctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::EWCTRL {
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
    #[doc = "Possible values of the field `EWOFFSET`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EWOFFSETR {
        #[doc = "8 clock cycles"]
        _0X0,
        #[doc = "16 clock cycles"]
        _0X1,
        #[doc = "32 clock cycles"]
        _0X2,
        #[doc = "64 clock cycles"]
        _0X3,
        #[doc = "128 clock cycles"]
        _0X4,
        #[doc = "256 clock cycles"]
        _0X5,
        #[doc = "512 clock cycles"]
        _0X6,
        #[doc = "1024 clock cycles"]
        _0X7,
        #[doc = "2048 clock cycles"]
        _0X8,
        #[doc = "4096 clock cycles"]
        _0X9,
        #[doc = "8192 clock cycles"]
        _0XA,
        #[doc = "16384 clock cycles"]
        _0XB,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl EWOFFSETR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                EWOFFSETR::_0X0 => 0,
                EWOFFSETR::_0X1 => 1,
                EWOFFSETR::_0X2 => 2,
                EWOFFSETR::_0X3 => 3,
                EWOFFSETR::_0X4 => 4,
                EWOFFSETR::_0X5 => 5,
                EWOFFSETR::_0X6 => 6,
                EWOFFSETR::_0X7 => 7,
                EWOFFSETR::_0X8 => 8,
                EWOFFSETR::_0X9 => 9,
                EWOFFSETR::_0XA => 10,
                EWOFFSETR::_0XB => 11,
                EWOFFSETR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> EWOFFSETR {
            match value {
                0 => EWOFFSETR::_0X0,
                1 => EWOFFSETR::_0X1,
                2 => EWOFFSETR::_0X2,
                3 => EWOFFSETR::_0X3,
                4 => EWOFFSETR::_0X4,
                5 => EWOFFSETR::_0X5,
                6 => EWOFFSETR::_0X6,
                7 => EWOFFSETR::_0X7,
                8 => EWOFFSETR::_0X8,
                9 => EWOFFSETR::_0X9,
                10 => EWOFFSETR::_0XA,
                11 => EWOFFSETR::_0XB,
                i => EWOFFSETR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `_0X0`"]
        #[inline(always)]
        pub fn is_0x0(&self) -> bool {
            *self == EWOFFSETR::_0X0
        }
        #[doc = "Checks if the value of the field is `_0X1`"]
        #[inline(always)]
        pub fn is_0x1(&self) -> bool {
            *self == EWOFFSETR::_0X1
        }
        #[doc = "Checks if the value of the field is `_0X2`"]
        #[inline(always)]
        pub fn is_0x2(&self) -> bool {
            *self == EWOFFSETR::_0X2
        }
        #[doc = "Checks if the value of the field is `_0X3`"]
        #[inline(always)]
        pub fn is_0x3(&self) -> bool {
            *self == EWOFFSETR::_0X3
        }
        #[doc = "Checks if the value of the field is `_0X4`"]
        #[inline(always)]
        pub fn is_0x4(&self) -> bool {
            *self == EWOFFSETR::_0X4
        }
        #[doc = "Checks if the value of the field is `_0X5`"]
        #[inline(always)]
        pub fn is_0x5(&self) -> bool {
            *self == EWOFFSETR::_0X5
        }
        #[doc = "Checks if the value of the field is `_0X6`"]
        #[inline(always)]
        pub fn is_0x6(&self) -> bool {
            *self == EWOFFSETR::_0X6
        }
        #[doc = "Checks if the value of the field is `_0X7`"]
        #[inline(always)]
        pub fn is_0x7(&self) -> bool {
            *self == EWOFFSETR::_0X7
        }
        #[doc = "Checks if the value of the field is `_0X8`"]
        #[inline(always)]
        pub fn is_0x8(&self) -> bool {
            *self == EWOFFSETR::_0X8
        }
        #[doc = "Checks if the value of the field is `_0X9`"]
        #[inline(always)]
        pub fn is_0x9(&self) -> bool {
            *self == EWOFFSETR::_0X9
        }
        #[doc = "Checks if the value of the field is `_0XA`"]
        #[inline(always)]
        pub fn is_0x_a(&self) -> bool {
            *self == EWOFFSETR::_0XA
        }
        #[doc = "Checks if the value of the field is `_0XB`"]
        #[inline(always)]
        pub fn is_0x_b(&self) -> bool {
            *self == EWOFFSETR::_0XB
        }
    }
    #[doc = "Values that can be written to the field `EWOFFSET`"]
    pub enum EWOFFSETW {
        #[doc = "8 clock cycles"]
        _0X0,
        #[doc = "16 clock cycles"]
        _0X1,
        #[doc = "32 clock cycles"]
        _0X2,
        #[doc = "64 clock cycles"]
        _0X3,
        #[doc = "128 clock cycles"]
        _0X4,
        #[doc = "256 clock cycles"]
        _0X5,
        #[doc = "512 clock cycles"]
        _0X6,
        #[doc = "1024 clock cycles"]
        _0X7,
        #[doc = "2048 clock cycles"]
        _0X8,
        #[doc = "4096 clock cycles"]
        _0X9,
        #[doc = "8192 clock cycles"]
        _0XA,
        #[doc = "16384 clock cycles"]
        _0XB,
    }
    impl EWOFFSETW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                EWOFFSETW::_0X0 => 0,
                EWOFFSETW::_0X1 => 1,
                EWOFFSETW::_0X2 => 2,
                EWOFFSETW::_0X3 => 3,
                EWOFFSETW::_0X4 => 4,
                EWOFFSETW::_0X5 => 5,
                EWOFFSETW::_0X6 => 6,
                EWOFFSETW::_0X7 => 7,
                EWOFFSETW::_0X8 => 8,
                EWOFFSETW::_0X9 => 9,
                EWOFFSETW::_0XA => 10,
                EWOFFSETW::_0XB => 11,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _EWOFFSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EWOFFSETW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: EWOFFSETW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "8 clock cycles"]
        #[inline(always)]
        pub fn _0x0(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0X0)
        }
        #[doc = "16 clock cycles"]
        #[inline(always)]
        pub fn _0x1(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0X1)
        }
        #[doc = "32 clock cycles"]
        #[inline(always)]
        pub fn _0x2(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0X2)
        }
        #[doc = "64 clock cycles"]
        #[inline(always)]
        pub fn _0x3(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0X3)
        }
        #[doc = "128 clock cycles"]
        #[inline(always)]
        pub fn _0x4(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0X4)
        }
        #[doc = "256 clock cycles"]
        #[inline(always)]
        pub fn _0x5(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0X5)
        }
        #[doc = "512 clock cycles"]
        #[inline(always)]
        pub fn _0x6(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0X6)
        }
        #[doc = "1024 clock cycles"]
        #[inline(always)]
        pub fn _0x7(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0X7)
        }
        #[doc = "2048 clock cycles"]
        #[inline(always)]
        pub fn _0x8(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0X8)
        }
        #[doc = "4096 clock cycles"]
        #[inline(always)]
        pub fn _0x9(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0X9)
        }
        #[doc = "8192 clock cycles"]
        #[inline(always)]
        pub fn _0x_a(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0XA)
        }
        #[doc = "16384 clock cycles"]
        #[inline(always)]
        pub fn _0x_b(self) -> &'a mut W {
            self.variant(EWOFFSETW::_0XB)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
        #[inline(always)]
        pub fn ewoffset(&self) -> EWOFFSETR {
            EWOFFSETR::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            })
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 11 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:3 - Early Warning Interrupt Time Offset"]
        #[inline(always)]
        pub fn ewoffset(&mut self) -> _EWOFFSETW {
            _EWOFFSETW { w: self }
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
    pub struct EWR {
        bits: bool,
    }
    impl EWR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _EWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EWW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Early Warning Interrupt Enable"]
        #[inline(always)]
        pub fn ew(&self) -> EWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            EWR { bits }
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
        #[doc = "Bit 0 - Early Warning Interrupt Enable"]
        #[inline(always)]
        pub fn ew(&mut self) -> _EWW {
            _EWW { w: self }
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
    pub struct EWR {
        bits: bool,
    }
    impl EWR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _EWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EWW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Early Warning Interrupt Enable"]
        #[inline(always)]
        pub fn ew(&self) -> EWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            EWR { bits }
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
        #[doc = "Bit 0 - Early Warning Interrupt Enable"]
        #[inline(always)]
        pub fn ew(&mut self) -> _EWW {
            _EWW { w: self }
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
    pub struct EWR {
        bits: bool,
    }
    impl EWR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _EWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EWW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Early Warning"]
        #[inline(always)]
        pub fn ew(&self) -> EWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            EWR { bits }
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
        #[doc = "Bit 0 - Early Warning"]
        #[inline(always)]
        pub fn ew(&mut self) -> _EWW {
            _EWW { w: self }
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
