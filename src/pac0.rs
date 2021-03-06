use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write Protection Clear"]
    pub wpclr: WPCLR,
    #[doc = "0x04 - Write Protection Set"]
    pub wpset: WPSET,
}
#[doc = "Write Protection Clear"]
pub struct WPCLR {
    register: VolatileCell<u32>,
}
#[doc = "Write Protection Clear"]
pub mod wpclr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::WPCLR {
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
    pub struct WPR {
        bits: u32,
    }
    impl WPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _WPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WPW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 2147483647;
            const OFFSET: u8 = 1;
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
        #[doc = "Bits 1:31 - Write Protection Clear"]
        #[inline(always)]
        pub fn wp(&self) -> WPR {
            let bits = {
                const MASK: u32 = 2147483647;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            WPR { bits }
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
        #[doc = "Bits 1:31 - Write Protection Clear"]
        #[inline(always)]
        pub fn wp(&mut self) -> _WPW {
            _WPW { w: self }
        }
    }
}
#[doc = "Write Protection Set"]
pub struct WPSET {
    register: VolatileCell<u32>,
}
#[doc = "Write Protection Set"]
pub mod wpset {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::WPSET {
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
    pub struct WPR {
        bits: u32,
    }
    impl WPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _WPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WPW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 2147483647;
            const OFFSET: u8 = 1;
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
        #[doc = "Bits 1:31 - Write Protection Set"]
        #[inline(always)]
        pub fn wp(&self) -> WPR {
            let bits = {
                const MASK: u32 = 2147483647;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            WPR { bits }
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
        #[doc = "Bits 1:31 - Write Protection Set"]
        #[inline(always)]
        pub fn wp(&mut self) -> _WPW {
            _WPW { w: self }
        }
    }
}
