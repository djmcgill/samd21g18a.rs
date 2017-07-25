use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Direction"]
    pub dir0: DIR,
    #[doc = "0x04 - Data Direction Clear"]
    pub dirclr0: DIRCLR,
    #[doc = "0x08 - Data Direction Set"]
    pub dirset0: DIRSET,
    #[doc = "0x0c - Data Direction Toggle"]
    pub dirtgl0: DIRTGL,
    #[doc = "0x10 - Data Output Value"]
    pub out0: OUT,
    #[doc = "0x14 - Data Output Value Clear"]
    pub outclr0: OUTCLR,
    #[doc = "0x18 - Data Output Value Set"]
    pub outset0: OUTSET,
    #[doc = "0x1c - Data Output Value Toggle"]
    pub outtgl0: OUTTGL,
    #[doc = "0x20 - Data Input Value"]
    pub in0: IN,
    #[doc = "0x24 - Control"]
    pub ctrl0: CTRL,
    #[doc = "0x28 - Write Configuration"]
    pub wrconfig0: WRCONFIG,
    _reserved0: [u8; 4usize],
    #[doc = "0x30 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_0: PMUX0_,
    #[doc = "0x31 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_1: PMUX0_,
    #[doc = "0x32 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_2: PMUX0_,
    #[doc = "0x33 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_3: PMUX0_,
    #[doc = "0x34 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_4: PMUX0_,
    #[doc = "0x35 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_5: PMUX0_,
    #[doc = "0x36 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_6: PMUX0_,
    #[doc = "0x37 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_7: PMUX0_,
    #[doc = "0x38 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_8: PMUX0_,
    #[doc = "0x39 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_9: PMUX0_,
    #[doc = "0x3a - Peripheral Multiplexing n - Group 0"]
    pub pmux0_10: PMUX0_,
    #[doc = "0x3b - Peripheral Multiplexing n - Group 0"]
    pub pmux0_11: PMUX0_,
    #[doc = "0x3c - Peripheral Multiplexing n - Group 0"]
    pub pmux0_12: PMUX0_,
    #[doc = "0x3d - Peripheral Multiplexing n - Group 0"]
    pub pmux0_13: PMUX0_,
    #[doc = "0x3e - Peripheral Multiplexing n - Group 0"]
    pub pmux0_14: PMUX0_,
    #[doc = "0x3f - Peripheral Multiplexing n - Group 0"]
    pub pmux0_15: PMUX0_,
    #[doc = "0x40 - Pin Configuration n - Group 0"]
    pub pincfg0_0: PINCFG0_,
    #[doc = "0x41 - Pin Configuration n - Group 0"]
    pub pincfg0_1: PINCFG0_,
    #[doc = "0x42 - Pin Configuration n - Group 0"]
    pub pincfg0_2: PINCFG0_,
    #[doc = "0x43 - Pin Configuration n - Group 0"]
    pub pincfg0_3: PINCFG0_,
    #[doc = "0x44 - Pin Configuration n - Group 0"]
    pub pincfg0_4: PINCFG0_,
    #[doc = "0x45 - Pin Configuration n - Group 0"]
    pub pincfg0_5: PINCFG0_,
    #[doc = "0x46 - Pin Configuration n - Group 0"]
    pub pincfg0_6: PINCFG0_,
    #[doc = "0x47 - Pin Configuration n - Group 0"]
    pub pincfg0_7: PINCFG0_,
    #[doc = "0x48 - Pin Configuration n - Group 0"]
    pub pincfg0_8: PINCFG0_,
    #[doc = "0x49 - Pin Configuration n - Group 0"]
    pub pincfg0_9: PINCFG0_,
    #[doc = "0x4a - Pin Configuration n - Group 0"]
    pub pincfg0_10: PINCFG0_,
    #[doc = "0x4b - Pin Configuration n - Group 0"]
    pub pincfg0_11: PINCFG0_,
    #[doc = "0x4c - Pin Configuration n - Group 0"]
    pub pincfg0_12: PINCFG0_,
    #[doc = "0x4d - Pin Configuration n - Group 0"]
    pub pincfg0_13: PINCFG0_,
    #[doc = "0x4e - Pin Configuration n - Group 0"]
    pub pincfg0_14: PINCFG0_,
    #[doc = "0x4f - Pin Configuration n - Group 0"]
    pub pincfg0_15: PINCFG0_,
    #[doc = "0x50 - Pin Configuration n - Group 0"]
    pub pincfg0_16: PINCFG0_,
    #[doc = "0x51 - Pin Configuration n - Group 0"]
    pub pincfg0_17: PINCFG0_,
    #[doc = "0x52 - Pin Configuration n - Group 0"]
    pub pincfg0_18: PINCFG0_,
    #[doc = "0x53 - Pin Configuration n - Group 0"]
    pub pincfg0_19: PINCFG0_,
    #[doc = "0x54 - Pin Configuration n - Group 0"]
    pub pincfg0_20: PINCFG0_,
    #[doc = "0x55 - Pin Configuration n - Group 0"]
    pub pincfg0_21: PINCFG0_,
    #[doc = "0x56 - Pin Configuration n - Group 0"]
    pub pincfg0_22: PINCFG0_,
    #[doc = "0x57 - Pin Configuration n - Group 0"]
    pub pincfg0_23: PINCFG0_,
    #[doc = "0x58 - Pin Configuration n - Group 0"]
    pub pincfg0_24: PINCFG0_,
    #[doc = "0x59 - Pin Configuration n - Group 0"]
    pub pincfg0_25: PINCFG0_,
    #[doc = "0x5a - Pin Configuration n - Group 0"]
    pub pincfg0_26: PINCFG0_,
    #[doc = "0x5b - Pin Configuration n - Group 0"]
    pub pincfg0_27: PINCFG0_,
    #[doc = "0x5c - Pin Configuration n - Group 0"]
    pub pincfg0_28: PINCFG0_,
    #[doc = "0x5d - Pin Configuration n - Group 0"]
    pub pincfg0_29: PINCFG0_,
    #[doc = "0x5e - Pin Configuration n - Group 0"]
    pub pincfg0_30: PINCFG0_,
    #[doc = "0x5f - Pin Configuration n - Group 0"]
    pub pincfg0_31: PINCFG0_,
    _reserved1: [u8; 32usize],
    #[doc = "0x80 - Data Direction"]
    pub dir1: DIR,
    #[doc = "0x84 - Data Direction Clear"]
    pub dirclr1: DIRCLR,
    #[doc = "0x88 - Data Direction Set"]
    pub dirset1: DIRSET,
    #[doc = "0x8c - Data Direction Toggle"]
    pub dirtgl1: DIRTGL,
    #[doc = "0x90 - Data Output Value"]
    pub out1: OUT,
    #[doc = "0x94 - Data Output Value Clear"]
    pub outclr1: OUTCLR,
    #[doc = "0x98 - Data Output Value Set"]
    pub outset1: OUTSET,
    #[doc = "0x9c - Data Output Value Toggle"]
    pub outtgl1: OUTTGL,
    #[doc = "0xa0 - Data Input Value"]
    pub in1: IN,
    #[doc = "0xa4 - Control"]
    pub ctrl1: CTRL,
    #[doc = "0xa8 - Write Configuration"]
    pub wrconfig1: WRCONFIG,
    _reserved2: [u8; 4usize],
    #[doc = "0xb0 - Peripheral Multiplexing n - Group 1"]
    pub pmux1_0: PMUX1_,
    #[doc = "0xb4 - Peripheral Multiplexing n - Group 1"]
    pub pmux1_4: PMUX1_,
    #[doc = "0xb8 - Peripheral Multiplexing n - Group 1"]
    pub pmux1_8: PMUX1_,
    #[doc = "0xbc - Peripheral Multiplexing n - Group 1"]
    pub pmux1_12: PMUX1_,
    #[doc = "0xc0 - Pin Configuration n - Group 1"]
    pub pincfg1_0: PINCFG1_,
    #[doc = "0xc4 - Pin Configuration n - Group 1"]
    pub pincfg1_4: PINCFG1_,
    #[doc = "0xc8 - Pin Configuration n - Group 1"]
    pub pincfg1_8: PINCFG1_,
    #[doc = "0xcc - Pin Configuration n - Group 1"]
    pub pincfg1_12: PINCFG1_,
    #[doc = "0xd0 - Pin Configuration n - Group 1"]
    pub pincfg1_16: PINCFG1_,
    #[doc = "0xd4 - Pin Configuration n - Group 1"]
    pub pincfg1_20: PINCFG1_,
    #[doc = "0xd8 - Pin Configuration n - Group 1"]
    pub pincfg1_24: PINCFG1_,
    #[doc = "0xdc - Pin Configuration n - Group 1"]
    pub pincfg1_28: PINCFG1_,
    _reserved3: [u8; 32usize],
    #[doc = "0x100 - Data Direction"]
    pub dir2: DIR,
    #[doc = "0x104 - Data Direction Clear"]
    pub dirclr2: DIRCLR,
    #[doc = "0x108 - Data Direction Set"]
    pub dirset2: DIRSET,
    #[doc = "0x10c - Data Direction Toggle"]
    pub dirtgl2: DIRTGL,
    #[doc = "0x110 - Data Output Value"]
    pub out2: OUT,
    #[doc = "0x114 - Data Output Value Clear"]
    pub outclr2: OUTCLR,
    #[doc = "0x118 - Data Output Value Set"]
    pub outset2: OUTSET,
    #[doc = "0x11c - Data Output Value Toggle"]
    pub outtgl2: OUTTGL,
    #[doc = "0x120 - Data Input Value"]
    pub in2: IN,
    #[doc = "0x124 - Control"]
    pub ctrl2: CTRL,
    #[doc = "0x128 - Write Configuration"]
    pub wrconfig2: WRCONFIG,
    _reserved4: [u8; 4usize],
    #[doc = "0x130 - Peripheral Multiplexing n - Group 2"]
    pub pmux2_0: PMUX2_,
    #[doc = "0x134 - Peripheral Multiplexing n - Group 2"]
    pub pmux2_4: PMUX2_,
    #[doc = "0x138 - Peripheral Multiplexing n - Group 2"]
    pub pmux2_8: PMUX2_,
    #[doc = "0x13c - Peripheral Multiplexing n - Group 2"]
    pub pmux2_12: PMUX2_,
    #[doc = "0x140 - Pin Configuration n - Group 2"]
    pub pincfg2_0: PINCFG2_,
    #[doc = "0x144 - Pin Configuration n - Group 2"]
    pub pincfg2_4: PINCFG2_,
    #[doc = "0x148 - Pin Configuration n - Group 2"]
    pub pincfg2_8: PINCFG2_,
    #[doc = "0x14c - Pin Configuration n - Group 2"]
    pub pincfg2_12: PINCFG2_,
    #[doc = "0x150 - Pin Configuration n - Group 2"]
    pub pincfg2_16: PINCFG2_,
    #[doc = "0x154 - Pin Configuration n - Group 2"]
    pub pincfg2_20: PINCFG2_,
    #[doc = "0x158 - Pin Configuration n - Group 2"]
    pub pincfg2_24: PINCFG2_,
    #[doc = "0x15c - Pin Configuration n - Group 2"]
    pub pincfg2_28: PINCFG2_,
}
#[doc = "Control"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Control"]
pub mod ctrl {
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
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
    pub struct _SAMPLINGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SAMPLINGW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:31 - Input Sampling Mode"]
        #[inline(always)]
        pub fn sampling(&mut self) -> _SAMPLINGW {
            _SAMPLINGW { w: self }
        }
    }
}
#[doc = "Data Direction"]
pub struct DIR {
    register: VolatileCell<u32>,
}
#[doc = "Data Direction"]
pub mod dir {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DIR {
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
        bits: u32,
    }
    impl DIRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _DIRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIRW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        #[doc = "Bits 0:31 - Port Data Direction"]
        #[inline(always)]
        pub fn dir(&self) -> DIRR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            DIRR { bits }
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
        #[doc = "Bits 0:31 - Port Data Direction"]
        #[inline(always)]
        pub fn dir(&mut self) -> _DIRW {
            _DIRW { w: self }
        }
    }
}
#[doc = "Data Direction Clear"]
pub struct DIRCLR {
    register: VolatileCell<u32>,
}
#[doc = "Data Direction Clear"]
pub mod dirclr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DIRCLR {
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
    pub struct DIRCLRR {
        bits: u32,
    }
    impl DIRCLRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _DIRCLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIRCLRW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        #[doc = "Bits 0:31 - Port Data Direction Clear"]
        #[inline(always)]
        pub fn dirclr(&self) -> DIRCLRR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            DIRCLRR { bits }
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
        #[doc = "Bits 0:31 - Port Data Direction Clear"]
        #[inline(always)]
        pub fn dirclr(&mut self) -> _DIRCLRW {
            _DIRCLRW { w: self }
        }
    }
}
#[doc = "Data Direction Set"]
pub struct DIRSET {
    register: VolatileCell<u32>,
}
#[doc = "Data Direction Set"]
pub mod dirset {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DIRSET {
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
    pub struct DIRSETR {
        bits: u32,
    }
    impl DIRSETR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _DIRSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIRSETW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        #[doc = "Bits 0:31 - Port Data Direction Set"]
        #[inline(always)]
        pub fn dirset(&self) -> DIRSETR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            DIRSETR { bits }
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
        #[doc = "Bits 0:31 - Port Data Direction Set"]
        #[inline(always)]
        pub fn dirset(&mut self) -> _DIRSETW {
            _DIRSETW { w: self }
        }
    }
}
#[doc = "Data Direction Toggle"]
pub struct DIRTGL {
    register: VolatileCell<u32>,
}
#[doc = "Data Direction Toggle"]
pub mod dirtgl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DIRTGL {
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
    pub struct DIRTGLR {
        bits: u32,
    }
    impl DIRTGLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _DIRTGLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIRTGLW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        #[doc = "Bits 0:31 - Port Data Direction Toggle"]
        #[inline(always)]
        pub fn dirtgl(&self) -> DIRTGLR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            DIRTGLR { bits }
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
        #[doc = "Bits 0:31 - Port Data Direction Toggle"]
        #[inline(always)]
        pub fn dirtgl(&mut self) -> _DIRTGLW {
            _DIRTGLW { w: self }
        }
    }
}
#[doc = "Data Input Value"]
pub struct IN {
    register: VolatileCell<u32>,
}
#[doc = "Data Input Value"]
pub mod in_ {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::IN {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct INR {
        bits: u32,
    }
    impl INR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:31 - Port Data Input Value"]
        #[inline(always)]
        pub fn in_(&self) -> INR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            INR { bits }
        }
    }
}
#[doc = "Data Output Value"]
pub struct OUT {
    register: VolatileCell<u32>,
}
#[doc = "Data Output Value"]
pub mod out {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::OUT {
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
    pub struct OUTR {
        bits: u32,
    }
    impl OUTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _OUTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OUTW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        #[doc = "Bits 0:31 - Port Data Output Value"]
        #[inline(always)]
        pub fn out(&self) -> OUTR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            OUTR { bits }
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
        #[doc = "Bits 0:31 - Port Data Output Value"]
        #[inline(always)]
        pub fn out(&mut self) -> _OUTW {
            _OUTW { w: self }
        }
    }
}
#[doc = "Data Output Value Clear"]
pub struct OUTCLR {
    register: VolatileCell<u32>,
}
#[doc = "Data Output Value Clear"]
pub mod outclr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::OUTCLR {
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
    pub struct OUTCLRR {
        bits: u32,
    }
    impl OUTCLRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _OUTCLRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OUTCLRW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        #[doc = "Bits 0:31 - Port Data Output Value Clear"]
        #[inline(always)]
        pub fn outclr(&self) -> OUTCLRR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            OUTCLRR { bits }
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
        #[doc = "Bits 0:31 - Port Data Output Value Clear"]
        #[inline(always)]
        pub fn outclr(&mut self) -> _OUTCLRW {
            _OUTCLRW { w: self }
        }
    }
}
#[doc = "Data Output Value Set"]
pub struct OUTSET {
    register: VolatileCell<u32>,
}
#[doc = "Data Output Value Set"]
pub mod outset {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::OUTSET {
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
    pub struct OUTSETR {
        bits: u32,
    }
    impl OUTSETR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _OUTSETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OUTSETW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        #[doc = "Bits 0:31 - Port Data Output Value Set"]
        #[inline(always)]
        pub fn outset(&self) -> OUTSETR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            OUTSETR { bits }
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
        #[doc = "Bits 0:31 - Port Data Output Value Set"]
        #[inline(always)]
        pub fn outset(&mut self) -> _OUTSETW {
            _OUTSETW { w: self }
        }
    }
}
#[doc = "Data Output Value Toggle"]
pub struct OUTTGL {
    register: VolatileCell<u32>,
}
#[doc = "Data Output Value Toggle"]
pub mod outtgl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::OUTTGL {
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
    pub struct OUTTGLR {
        bits: u32,
    }
    impl OUTTGLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _OUTTGLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OUTTGLW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
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
        #[doc = "Bits 0:31 - Port Data Output Value Toggle"]
        #[inline(always)]
        pub fn outtgl(&self) -> OUTTGLR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            OUTTGLR { bits }
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
        #[doc = "Bits 0:31 - Port Data Output Value Toggle"]
        #[inline(always)]
        pub fn outtgl(&mut self) -> _OUTTGLW {
            _OUTTGLW { w: self }
        }
    }
}
#[doc = "Pin Configuration n - Group 0"]
pub struct PINCFG0_ {
    register: VolatileCell<u8>,
}
#[doc = "Pin Configuration n - Group 0"]
pub mod pincfg0_ {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::PINCFG0_ {
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
    pub struct PMUXENR {
        bits: bool,
    }
    impl PMUXENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct INENR {
        bits: bool,
    }
    impl INENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PULLENR {
        bits: bool,
    }
    impl PULLENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _PMUXENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PMUXENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _INENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _INENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PULLENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PULLENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DRVSTRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DRVSTRW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
        #[inline(always)]
        pub fn pmuxen(&self) -> PMUXENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            PMUXENR { bits }
        }
        #[doc = "Bit 1 - Input Enable"]
        #[inline(always)]
        pub fn inen(&self) -> INENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            INENR { bits }
        }
        #[doc = "Bit 2 - Pull Enable"]
        #[inline(always)]
        pub fn pullen(&self) -> PULLENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            PULLENR { bits }
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
        #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
        #[inline(always)]
        pub fn pmuxen(&mut self) -> _PMUXENW {
            _PMUXENW { w: self }
        }
        #[doc = "Bit 1 - Input Enable"]
        #[inline(always)]
        pub fn inen(&mut self) -> _INENW {
            _INENW { w: self }
        }
        #[doc = "Bit 2 - Pull Enable"]
        #[inline(always)]
        pub fn pullen(&mut self) -> _PULLENW {
            _PULLENW { w: self }
        }
        #[doc = "Bit 6 - Output Driver Strength Selection"]
        #[inline(always)]
        pub fn drvstr(&mut self) -> _DRVSTRW {
            _DRVSTRW { w: self }
        }
    }
}
#[doc = "Pin Configuration n - Group 1"]
pub struct PINCFG1_ {
    register: VolatileCell<u32>,
}
#[doc = "Pin Configuration n - Group 1"]
pub mod pincfg1_ {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::PINCFG1_ {
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
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
    }
}
#[doc = "Pin Configuration n - Group 2"]
pub struct PINCFG2_ {
    register: VolatileCell<u32>,
}
#[doc = "Pin Configuration n - Group 2"]
pub mod pincfg2_ {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::PINCFG2_ {
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
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
    }
}
#[doc = "Peripheral Multiplexing n - Group 0"]
pub struct PMUX0_ {
    register: VolatileCell<u8>,
}
#[doc = "Peripheral Multiplexing n - Group 0"]
pub mod pmux0_ {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::PMUX0_ {
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
    #[doc = "Possible values of the field `PMUXE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PMUXER {
        #[doc = "Peripheral function A selected"]
        A,
        #[doc = "Peripheral function B selected"]
        B,
        #[doc = "Peripheral function C selected"]
        C,
        #[doc = "Peripheral function D selected"]
        D,
        #[doc = "Peripheral function E selected"]
        E,
        #[doc = "Peripheral function F selected"]
        F,
        #[doc = "Peripheral function G selected"]
        G,
        #[doc = "Peripheral function H selected"]
        H,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl PMUXER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                PMUXER::A => 0,
                PMUXER::B => 1,
                PMUXER::C => 2,
                PMUXER::D => 3,
                PMUXER::E => 4,
                PMUXER::F => 5,
                PMUXER::G => 6,
                PMUXER::H => 7,
                PMUXER::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> PMUXER {
            match value {
                0 => PMUXER::A,
                1 => PMUXER::B,
                2 => PMUXER::C,
                3 => PMUXER::D,
                4 => PMUXER::E,
                5 => PMUXER::F,
                6 => PMUXER::G,
                7 => PMUXER::H,
                i => PMUXER::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `A`"]
        #[inline(always)]
        pub fn is_a(&self) -> bool {
            *self == PMUXER::A
        }
        #[doc = "Checks if the value of the field is `B`"]
        #[inline(always)]
        pub fn is_b(&self) -> bool {
            *self == PMUXER::B
        }
        #[doc = "Checks if the value of the field is `C`"]
        #[inline(always)]
        pub fn is_c(&self) -> bool {
            *self == PMUXER::C
        }
        #[doc = "Checks if the value of the field is `D`"]
        #[inline(always)]
        pub fn is_d(&self) -> bool {
            *self == PMUXER::D
        }
        #[doc = "Checks if the value of the field is `E`"]
        #[inline(always)]
        pub fn is_e(&self) -> bool {
            *self == PMUXER::E
        }
        #[doc = "Checks if the value of the field is `F`"]
        #[inline(always)]
        pub fn is_f(&self) -> bool {
            *self == PMUXER::F
        }
        #[doc = "Checks if the value of the field is `G`"]
        #[inline(always)]
        pub fn is_g(&self) -> bool {
            *self == PMUXER::G
        }
        #[doc = "Checks if the value of the field is `H`"]
        #[inline(always)]
        pub fn is_h(&self) -> bool {
            *self == PMUXER::H
        }
    }
    #[doc = "Possible values of the field `PMUXO`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PMUXOR {
        #[doc = "Peripheral function A selected"]
        A,
        #[doc = "Peripheral function B selected"]
        B,
        #[doc = "Peripheral function C selected"]
        C,
        #[doc = "Peripheral function D selected"]
        D,
        #[doc = "Peripheral function E selected"]
        E,
        #[doc = "Peripheral function F selected"]
        F,
        #[doc = "Peripheral function G selected"]
        G,
        #[doc = "Peripheral function H selected"]
        H,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl PMUXOR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                PMUXOR::A => 0,
                PMUXOR::B => 1,
                PMUXOR::C => 2,
                PMUXOR::D => 3,
                PMUXOR::E => 4,
                PMUXOR::F => 5,
                PMUXOR::G => 6,
                PMUXOR::H => 7,
                PMUXOR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> PMUXOR {
            match value {
                0 => PMUXOR::A,
                1 => PMUXOR::B,
                2 => PMUXOR::C,
                3 => PMUXOR::D,
                4 => PMUXOR::E,
                5 => PMUXOR::F,
                6 => PMUXOR::G,
                7 => PMUXOR::H,
                i => PMUXOR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `A`"]
        #[inline(always)]
        pub fn is_a(&self) -> bool {
            *self == PMUXOR::A
        }
        #[doc = "Checks if the value of the field is `B`"]
        #[inline(always)]
        pub fn is_b(&self) -> bool {
            *self == PMUXOR::B
        }
        #[doc = "Checks if the value of the field is `C`"]
        #[inline(always)]
        pub fn is_c(&self) -> bool {
            *self == PMUXOR::C
        }
        #[doc = "Checks if the value of the field is `D`"]
        #[inline(always)]
        pub fn is_d(&self) -> bool {
            *self == PMUXOR::D
        }
        #[doc = "Checks if the value of the field is `E`"]
        #[inline(always)]
        pub fn is_e(&self) -> bool {
            *self == PMUXOR::E
        }
        #[doc = "Checks if the value of the field is `F`"]
        #[inline(always)]
        pub fn is_f(&self) -> bool {
            *self == PMUXOR::F
        }
        #[doc = "Checks if the value of the field is `G`"]
        #[inline(always)]
        pub fn is_g(&self) -> bool {
            *self == PMUXOR::G
        }
        #[doc = "Checks if the value of the field is `H`"]
        #[inline(always)]
        pub fn is_h(&self) -> bool {
            *self == PMUXOR::H
        }
    }
    #[doc = "Values that can be written to the field `PMUXE`"]
    pub enum PMUXEW {
        #[doc = "Peripheral function A selected"]
        A,
        #[doc = "Peripheral function B selected"]
        B,
        #[doc = "Peripheral function C selected"]
        C,
        #[doc = "Peripheral function D selected"]
        D,
        #[doc = "Peripheral function E selected"]
        E,
        #[doc = "Peripheral function F selected"]
        F,
        #[doc = "Peripheral function G selected"]
        G,
        #[doc = "Peripheral function H selected"]
        H,
    }
    impl PMUXEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                PMUXEW::A => 0,
                PMUXEW::B => 1,
                PMUXEW::C => 2,
                PMUXEW::D => 3,
                PMUXEW::E => 4,
                PMUXEW::F => 5,
                PMUXEW::G => 6,
                PMUXEW::H => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _PMUXEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PMUXEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: PMUXEW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Peripheral function A selected"]
        #[inline(always)]
        pub fn a(self) -> &'a mut W {
            self.variant(PMUXEW::A)
        }
        #[doc = "Peripheral function B selected"]
        #[inline(always)]
        pub fn b(self) -> &'a mut W {
            self.variant(PMUXEW::B)
        }
        #[doc = "Peripheral function C selected"]
        #[inline(always)]
        pub fn c(self) -> &'a mut W {
            self.variant(PMUXEW::C)
        }
        #[doc = "Peripheral function D selected"]
        #[inline(always)]
        pub fn d(self) -> &'a mut W {
            self.variant(PMUXEW::D)
        }
        #[doc = "Peripheral function E selected"]
        #[inline(always)]
        pub fn e(self) -> &'a mut W {
            self.variant(PMUXEW::E)
        }
        #[doc = "Peripheral function F selected"]
        #[inline(always)]
        pub fn f(self) -> &'a mut W {
            self.variant(PMUXEW::F)
        }
        #[doc = "Peripheral function G selected"]
        #[inline(always)]
        pub fn g(self) -> &'a mut W {
            self.variant(PMUXEW::G)
        }
        #[doc = "Peripheral function H selected"]
        #[inline(always)]
        pub fn h(self) -> &'a mut W {
            self.variant(PMUXEW::H)
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
    #[doc = "Values that can be written to the field `PMUXO`"]
    pub enum PMUXOW {
        #[doc = "Peripheral function A selected"]
        A,
        #[doc = "Peripheral function B selected"]
        B,
        #[doc = "Peripheral function C selected"]
        C,
        #[doc = "Peripheral function D selected"]
        D,
        #[doc = "Peripheral function E selected"]
        E,
        #[doc = "Peripheral function F selected"]
        F,
        #[doc = "Peripheral function G selected"]
        G,
        #[doc = "Peripheral function H selected"]
        H,
    }
    impl PMUXOW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                PMUXOW::A => 0,
                PMUXOW::B => 1,
                PMUXOW::C => 2,
                PMUXOW::D => 3,
                PMUXOW::E => 4,
                PMUXOW::F => 5,
                PMUXOW::G => 6,
                PMUXOW::H => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _PMUXOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PMUXOW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: PMUXOW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Peripheral function A selected"]
        #[inline(always)]
        pub fn a(self) -> &'a mut W {
            self.variant(PMUXOW::A)
        }
        #[doc = "Peripheral function B selected"]
        #[inline(always)]
        pub fn b(self) -> &'a mut W {
            self.variant(PMUXOW::B)
        }
        #[doc = "Peripheral function C selected"]
        #[inline(always)]
        pub fn c(self) -> &'a mut W {
            self.variant(PMUXOW::C)
        }
        #[doc = "Peripheral function D selected"]
        #[inline(always)]
        pub fn d(self) -> &'a mut W {
            self.variant(PMUXOW::D)
        }
        #[doc = "Peripheral function E selected"]
        #[inline(always)]
        pub fn e(self) -> &'a mut W {
            self.variant(PMUXOW::E)
        }
        #[doc = "Peripheral function F selected"]
        #[inline(always)]
        pub fn f(self) -> &'a mut W {
            self.variant(PMUXOW::F)
        }
        #[doc = "Peripheral function G selected"]
        #[inline(always)]
        pub fn g(self) -> &'a mut W {
            self.variant(PMUXOW::G)
        }
        #[doc = "Peripheral function H selected"]
        #[inline(always)]
        pub fn h(self) -> &'a mut W {
            self.variant(PMUXOW::H)
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
        #[doc = "Bits 0:3 - Peripheral Multiplexing Even"]
        #[inline(always)]
        pub fn pmuxe(&self) -> PMUXER {
            PMUXER::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            })
        }
        #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
        #[inline(always)]
        pub fn pmuxo(&self) -> PMUXOR {
            PMUXOR::_from({
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
            W { bits: 0 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:3 - Peripheral Multiplexing Even"]
        #[inline(always)]
        pub fn pmuxe(&mut self) -> _PMUXEW {
            _PMUXEW { w: self }
        }
        #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
        #[inline(always)]
        pub fn pmuxo(&mut self) -> _PMUXOW {
            _PMUXOW { w: self }
        }
    }
}
#[doc = "Peripheral Multiplexing n - Group 1"]
pub struct PMUX1_ {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Multiplexing n - Group 1"]
pub mod pmux1_ {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::PMUX1_ {
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
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
    }
}
#[doc = "Peripheral Multiplexing n - Group 2"]
pub struct PMUX2_ {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Multiplexing n - Group 2"]
pub mod pmux2_ {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::PMUX2_ {
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
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
    }
}
#[doc = "Write Configuration"]
pub struct WRCONFIG {
    register: VolatileCell<u32>,
}
#[doc = "Write Configuration"]
pub mod wrconfig {
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::WRCONFIG {
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
    pub struct _PINMASKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PINMASKW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _PMUXENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PMUXENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _INENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _INENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PULLENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PULLENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DRVSTRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DRVSTRW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PMUXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PMUXW<'a> {
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
    pub struct _WRPMUXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRPMUXW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _WRPINCFGW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRPINCFGW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
        }
        #[doc = r" Clears the field bit"]
        pub fn clear_bit(self) -> &'a mut W {
            self.bit(false)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bit(self, value: bool) -> &'a mut W {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _HWSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HWSELW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bits 0:15 - Pin Mask for Multiple Pin Configuration"]
        #[inline(always)]
        pub fn pinmask(&mut self) -> _PINMASKW {
            _PINMASKW { w: self }
        }
        #[doc = "Bit 16 - Peripheral Multiplexer Enable"]
        #[inline(always)]
        pub fn pmuxen(&mut self) -> _PMUXENW {
            _PMUXENW { w: self }
        }
        #[doc = "Bit 17 - Input Enable"]
        #[inline(always)]
        pub fn inen(&mut self) -> _INENW {
            _INENW { w: self }
        }
        #[doc = "Bit 18 - Pull Enable"]
        #[inline(always)]
        pub fn pullen(&mut self) -> _PULLENW {
            _PULLENW { w: self }
        }
        #[doc = "Bit 22 - Output Driver Strength Selection"]
        #[inline(always)]
        pub fn drvstr(&mut self) -> _DRVSTRW {
            _DRVSTRW { w: self }
        }
        #[doc = "Bits 24:27 - Peripheral Multiplexing"]
        #[inline(always)]
        pub fn pmux(&mut self) -> _PMUXW {
            _PMUXW { w: self }
        }
        #[doc = "Bit 28 - Write PMUX"]
        #[inline(always)]
        pub fn wrpmux(&mut self) -> _WRPMUXW {
            _WRPMUXW { w: self }
        }
        #[doc = "Bit 30 - Write PINCFG"]
        #[inline(always)]
        pub fn wrpincfg(&mut self) -> _WRPINCFGW {
            _WRPINCFGW { w: self }
        }
        #[doc = "Bit 31 - Half-Word Select"]
        #[inline(always)]
        pub fn hwsel(&mut self) -> _HWSELW {
            _HWSELW { w: self }
        }
    }
}
