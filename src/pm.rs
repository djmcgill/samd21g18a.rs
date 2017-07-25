use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Sleep Mode"]
    pub sleep: SLEEP,
    _reserved0: [u8; 6usize],
    #[doc = "0x08 - CPU Clock Select"]
    pub cpusel: CPUSEL,
    #[doc = "0x09 - APBA Clock Select"]
    pub apbasel: APBASEL,
    #[doc = "0x0a - APBB Clock Select"]
    pub apbbsel: APBBSEL,
    #[doc = "0x0b - APBC Clock Select"]
    pub apbcsel: APBCSEL,
    _reserved1: [u8; 8usize],
    #[doc = "0x14 - AHB Mask"]
    pub ahbmask: AHBMASK,
    #[doc = "0x18 - APBA Mask"]
    pub apbamask: APBAMASK,
    #[doc = "0x1c - APBB Mask"]
    pub apbbmask: APBBMASK,
    #[doc = "0x20 - APBC Mask"]
    pub apbcmask: APBCMASK,
    _reserved2: [u8; 16usize],
    #[doc = "0x34 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x35 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x36 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved3: [u8; 1usize],
    #[doc = "0x38 - Reset Cause"]
    pub rcause: RCAUSE,
}
#[doc = "AHB Mask"]
pub struct AHBMASK {
    register: VolatileCell<u32>,
}
#[doc = "AHB Mask"]
pub mod ahbmask {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::AHBMASK {
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
    pub struct HPB0R {
        bits: bool,
    }
    impl HPB0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct HPB1R {
        bits: bool,
    }
    impl HPB1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct HPB2R {
        bits: bool,
    }
    impl HPB2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct DSUR {
        bits: bool,
    }
    impl DSUR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct NVMCTRLR {
        bits: bool,
    }
    impl NVMCTRLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct DMACR {
        bits: bool,
    }
    impl DMACR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct USBR {
        bits: bool,
    }
    impl USBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _HPB0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _HPB0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _HPB1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _HPB1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _HPB2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _HPB2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DSUW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DSUW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _NVMCTRLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _NVMCTRLW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DMACW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMACW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _USBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USBW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - HPB0 AHB Clock Enable"]
        #[inline(always)]
        pub fn hpb0(&self) -> HPB0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HPB0R { bits }
        }
        #[doc = "Bit 1 - HPB1 AHB Clock Enable"]
        #[inline(always)]
        pub fn hpb1(&self) -> HPB1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HPB1R { bits }
        }
        #[doc = "Bit 2 - HPB2 AHB Clock Enable"]
        #[inline(always)]
        pub fn hpb2(&self) -> HPB2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HPB2R { bits }
        }
        #[doc = "Bit 3 - DSU AHB Clock Enable"]
        #[inline(always)]
        pub fn dsu(&self) -> DSUR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DSUR { bits }
        }
        #[doc = "Bit 4 - NVMCTRL AHB Clock Enable"]
        #[inline(always)]
        pub fn nvmctrl(&self) -> NVMCTRLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NVMCTRLR { bits }
        }
        #[doc = "Bit 5 - DMAC AHB Clock Enable"]
        #[inline(always)]
        pub fn dmac(&self) -> DMACR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMACR { bits }
        }
        #[doc = "Bit 6 - USB AHB Clock Enable"]
        #[inline(always)]
        pub fn usb(&self) -> USBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USBR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 127 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - HPB0 AHB Clock Enable"]
        #[inline(always)]
        pub fn hpb0(&mut self) -> _HPB0W {
            _HPB0W { w: self }
        }
        #[doc = "Bit 1 - HPB1 AHB Clock Enable"]
        #[inline(always)]
        pub fn hpb1(&mut self) -> _HPB1W {
            _HPB1W { w: self }
        }
        #[doc = "Bit 2 - HPB2 AHB Clock Enable"]
        #[inline(always)]
        pub fn hpb2(&mut self) -> _HPB2W {
            _HPB2W { w: self }
        }
        #[doc = "Bit 3 - DSU AHB Clock Enable"]
        #[inline(always)]
        pub fn dsu(&mut self) -> _DSUW {
            _DSUW { w: self }
        }
        #[doc = "Bit 4 - NVMCTRL AHB Clock Enable"]
        #[inline(always)]
        pub fn nvmctrl(&mut self) -> _NVMCTRLW {
            _NVMCTRLW { w: self }
        }
        #[doc = "Bit 5 - DMAC AHB Clock Enable"]
        #[inline(always)]
        pub fn dmac(&mut self) -> _DMACW {
            _DMACW { w: self }
        }
        #[doc = "Bit 6 - USB AHB Clock Enable"]
        #[inline(always)]
        pub fn usb(&mut self) -> _USBW {
            _USBW { w: self }
        }
    }
}
#[doc = "APBA Mask"]
pub struct APBAMASK {
    register: VolatileCell<u32>,
}
#[doc = "APBA Mask"]
pub mod apbamask {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::APBAMASK {
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
    pub struct PAC0R {
        bits: bool,
    }
    impl PAC0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PMR {
        bits: bool,
    }
    impl PMR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SYSCTRLR {
        bits: bool,
    }
    impl SYSCTRLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct GCLKR {
        bits: bool,
    }
    impl GCLKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WDTR {
        bits: bool,
    }
    impl WDTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct RTCR {
        bits: bool,
    }
    impl RTCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EICR {
        bits: bool,
    }
    impl EICR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _PAC0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PAC0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PMW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SYSCTRLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SYSCTRLW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _GCLKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _GCLKW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WDTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WDTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RTCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RTCW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EICW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EICW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - PAC0 APB Clock Enable"]
        #[inline(always)]
        pub fn pac0(&self) -> PAC0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PAC0R { bits }
        }
        #[doc = "Bit 1 - PM APB Clock Enable"]
        #[inline(always)]
        pub fn pm(&self) -> PMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PMR { bits }
        }
        #[doc = "Bit 2 - SYSCTRL APB Clock Enable"]
        #[inline(always)]
        pub fn sysctrl(&self) -> SYSCTRLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SYSCTRLR { bits }
        }
        #[doc = "Bit 3 - GCLK APB Clock Enable"]
        #[inline(always)]
        pub fn gclk(&self) -> GCLKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            GCLKR { bits }
        }
        #[doc = "Bit 4 - WDT APB Clock Enable"]
        #[inline(always)]
        pub fn wdt(&self) -> WDTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WDTR { bits }
        }
        #[doc = "Bit 5 - RTC APB Clock Enable"]
        #[inline(always)]
        pub fn rtc(&self) -> RTCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RTCR { bits }
        }
        #[doc = "Bit 6 - EIC APB Clock Enable"]
        #[inline(always)]
        pub fn eic(&self) -> EICR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EICR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 127 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - PAC0 APB Clock Enable"]
        #[inline(always)]
        pub fn pac0(&mut self) -> _PAC0W {
            _PAC0W { w: self }
        }
        #[doc = "Bit 1 - PM APB Clock Enable"]
        #[inline(always)]
        pub fn pm(&mut self) -> _PMW {
            _PMW { w: self }
        }
        #[doc = "Bit 2 - SYSCTRL APB Clock Enable"]
        #[inline(always)]
        pub fn sysctrl(&mut self) -> _SYSCTRLW {
            _SYSCTRLW { w: self }
        }
        #[doc = "Bit 3 - GCLK APB Clock Enable"]
        #[inline(always)]
        pub fn gclk(&mut self) -> _GCLKW {
            _GCLKW { w: self }
        }
        #[doc = "Bit 4 - WDT APB Clock Enable"]
        #[inline(always)]
        pub fn wdt(&mut self) -> _WDTW {
            _WDTW { w: self }
        }
        #[doc = "Bit 5 - RTC APB Clock Enable"]
        #[inline(always)]
        pub fn rtc(&mut self) -> _RTCW {
            _RTCW { w: self }
        }
        #[doc = "Bit 6 - EIC APB Clock Enable"]
        #[inline(always)]
        pub fn eic(&mut self) -> _EICW {
            _EICW { w: self }
        }
    }
}
#[doc = "APBA Clock Select"]
pub struct APBASEL {
    register: VolatileCell<u8>,
}
#[doc = "APBA Clock Select"]
pub mod apbasel {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::APBASEL {
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
    #[doc = "Possible values of the field `APBADIV`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum APBADIVR {
        #[doc = "Divide by 1"]
        DIV1,
        #[doc = "Divide by 2"]
        DIV2,
        #[doc = "Divide by 4"]
        DIV4,
        #[doc = "Divide by 8"]
        DIV8,
        #[doc = "Divide by 16"]
        DIV16,
        #[doc = "Divide by 32"]
        DIV32,
        #[doc = "Divide by 64"]
        DIV64,
        #[doc = "Divide by 128"]
        DIV128,
    }
    impl APBADIVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                APBADIVR::DIV1 => 0,
                APBADIVR::DIV2 => 1,
                APBADIVR::DIV4 => 2,
                APBADIVR::DIV8 => 3,
                APBADIVR::DIV16 => 4,
                APBADIVR::DIV32 => 5,
                APBADIVR::DIV64 => 6,
                APBADIVR::DIV128 => 7,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> APBADIVR {
            match value {
                0 => APBADIVR::DIV1,
                1 => APBADIVR::DIV2,
                2 => APBADIVR::DIV4,
                3 => APBADIVR::DIV8,
                4 => APBADIVR::DIV16,
                5 => APBADIVR::DIV32,
                6 => APBADIVR::DIV64,
                7 => APBADIVR::DIV128,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DIV1`"]
        #[inline(always)]
        pub fn is_div1(&self) -> bool {
            *self == APBADIVR::DIV1
        }
        #[doc = "Checks if the value of the field is `DIV2`"]
        #[inline(always)]
        pub fn is_div2(&self) -> bool {
            *self == APBADIVR::DIV2
        }
        #[doc = "Checks if the value of the field is `DIV4`"]
        #[inline(always)]
        pub fn is_div4(&self) -> bool {
            *self == APBADIVR::DIV4
        }
        #[doc = "Checks if the value of the field is `DIV8`"]
        #[inline(always)]
        pub fn is_div8(&self) -> bool {
            *self == APBADIVR::DIV8
        }
        #[doc = "Checks if the value of the field is `DIV16`"]
        #[inline(always)]
        pub fn is_div16(&self) -> bool {
            *self == APBADIVR::DIV16
        }
        #[doc = "Checks if the value of the field is `DIV32`"]
        #[inline(always)]
        pub fn is_div32(&self) -> bool {
            *self == APBADIVR::DIV32
        }
        #[doc = "Checks if the value of the field is `DIV64`"]
        #[inline(always)]
        pub fn is_div64(&self) -> bool {
            *self == APBADIVR::DIV64
        }
        #[doc = "Checks if the value of the field is `DIV128`"]
        #[inline(always)]
        pub fn is_div128(&self) -> bool {
            *self == APBADIVR::DIV128
        }
    }
    #[doc = "Values that can be written to the field `APBADIV`"]
    pub enum APBADIVW {
        #[doc = "Divide by 1"]
        DIV1,
        #[doc = "Divide by 2"]
        DIV2,
        #[doc = "Divide by 4"]
        DIV4,
        #[doc = "Divide by 8"]
        DIV8,
        #[doc = "Divide by 16"]
        DIV16,
        #[doc = "Divide by 32"]
        DIV32,
        #[doc = "Divide by 64"]
        DIV64,
        #[doc = "Divide by 128"]
        DIV128,
    }
    impl APBADIVW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                APBADIVW::DIV1 => 0,
                APBADIVW::DIV2 => 1,
                APBADIVW::DIV4 => 2,
                APBADIVW::DIV8 => 3,
                APBADIVW::DIV16 => 4,
                APBADIVW::DIV32 => 5,
                APBADIVW::DIV64 => 6,
                APBADIVW::DIV128 => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _APBADIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _APBADIVW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: APBADIVW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "Divide by 1"]
        #[inline(always)]
        pub fn div1(self) -> &'a mut W {
            self.variant(APBADIVW::DIV1)
        }
        #[doc = "Divide by 2"]
        #[inline(always)]
        pub fn div2(self) -> &'a mut W {
            self.variant(APBADIVW::DIV2)
        }
        #[doc = "Divide by 4"]
        #[inline(always)]
        pub fn div4(self) -> &'a mut W {
            self.variant(APBADIVW::DIV4)
        }
        #[doc = "Divide by 8"]
        #[inline(always)]
        pub fn div8(self) -> &'a mut W {
            self.variant(APBADIVW::DIV8)
        }
        #[doc = "Divide by 16"]
        #[inline(always)]
        pub fn div16(self) -> &'a mut W {
            self.variant(APBADIVW::DIV16)
        }
        #[doc = "Divide by 32"]
        #[inline(always)]
        pub fn div32(self) -> &'a mut W {
            self.variant(APBADIVW::DIV32)
        }
        #[doc = "Divide by 64"]
        #[inline(always)]
        pub fn div64(self) -> &'a mut W {
            self.variant(APBADIVW::DIV64)
        }
        #[doc = "Divide by 128"]
        #[inline(always)]
        pub fn div128(self) -> &'a mut W {
            self.variant(APBADIVW::DIV128)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
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
        #[doc = "Bits 0:2 - APBA Prescaler Selection"]
        #[inline(always)]
        pub fn apbadiv(&self) -> APBADIVR {
            APBADIVR::_from({
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
        #[doc = "Bits 0:2 - APBA Prescaler Selection"]
        #[inline(always)]
        pub fn apbadiv(&mut self) -> _APBADIVW {
            _APBADIVW { w: self }
        }
    }
}
#[doc = "APBB Mask"]
pub struct APBBMASK {
    register: VolatileCell<u32>,
}
#[doc = "APBB Mask"]
pub mod apbbmask {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::APBBMASK {
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
    pub struct PAC1R {
        bits: bool,
    }
    impl PAC1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct DSUR {
        bits: bool,
    }
    impl DSUR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct NVMCTRLR {
        bits: bool,
    }
    impl NVMCTRLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PORTR {
        bits: bool,
    }
    impl PORTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct DMACR {
        bits: bool,
    }
    impl DMACR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct USBR {
        bits: bool,
    }
    impl USBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _PAC1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PAC1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DSUW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DSUW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _NVMCTRLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _NVMCTRLW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PORTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PORTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DMACW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMACW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _USBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USBW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - PAC1 APB Clock Enable"]
        #[inline(always)]
        pub fn pac1(&self) -> PAC1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PAC1R { bits }
        }
        #[doc = "Bit 1 - DSU APB Clock Enable"]
        #[inline(always)]
        pub fn dsu(&self) -> DSUR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DSUR { bits }
        }
        #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
        #[inline(always)]
        pub fn nvmctrl(&self) -> NVMCTRLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            NVMCTRLR { bits }
        }
        #[doc = "Bit 3 - PORT APB Clock Enable"]
        #[inline(always)]
        pub fn port(&self) -> PORTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PORTR { bits }
        }
        #[doc = "Bit 4 - DMAC APB Clock Enable"]
        #[inline(always)]
        pub fn dmac(&self) -> DMACR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DMACR { bits }
        }
        #[doc = "Bit 5 - USB APB Clock Enable"]
        #[inline(always)]
        pub fn usb(&self) -> USBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            USBR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 127 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - PAC1 APB Clock Enable"]
        #[inline(always)]
        pub fn pac1(&mut self) -> _PAC1W {
            _PAC1W { w: self }
        }
        #[doc = "Bit 1 - DSU APB Clock Enable"]
        #[inline(always)]
        pub fn dsu(&mut self) -> _DSUW {
            _DSUW { w: self }
        }
        #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
        #[inline(always)]
        pub fn nvmctrl(&mut self) -> _NVMCTRLW {
            _NVMCTRLW { w: self }
        }
        #[doc = "Bit 3 - PORT APB Clock Enable"]
        #[inline(always)]
        pub fn port(&mut self) -> _PORTW {
            _PORTW { w: self }
        }
        #[doc = "Bit 4 - DMAC APB Clock Enable"]
        #[inline(always)]
        pub fn dmac(&mut self) -> _DMACW {
            _DMACW { w: self }
        }
        #[doc = "Bit 5 - USB APB Clock Enable"]
        #[inline(always)]
        pub fn usb(&mut self) -> _USBW {
            _USBW { w: self }
        }
    }
}
#[doc = "APBB Clock Select"]
pub struct APBBSEL {
    register: VolatileCell<u8>,
}
#[doc = "APBB Clock Select"]
pub mod apbbsel {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::APBBSEL {
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
    #[doc = "Possible values of the field `APBBDIV`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum APBBDIVR {
        #[doc = "Divide by 1"]
        DIV1,
        #[doc = "Divide by 2"]
        DIV2,
        #[doc = "Divide by 4"]
        DIV4,
        #[doc = "Divide by 8"]
        DIV8,
        #[doc = "Divide by 16"]
        DIV16,
        #[doc = "Divide by 32"]
        DIV32,
        #[doc = "Divide by 64"]
        DIV64,
        #[doc = "Divide by 128"]
        DIV128,
    }
    impl APBBDIVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                APBBDIVR::DIV1 => 0,
                APBBDIVR::DIV2 => 1,
                APBBDIVR::DIV4 => 2,
                APBBDIVR::DIV8 => 3,
                APBBDIVR::DIV16 => 4,
                APBBDIVR::DIV32 => 5,
                APBBDIVR::DIV64 => 6,
                APBBDIVR::DIV128 => 7,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> APBBDIVR {
            match value {
                0 => APBBDIVR::DIV1,
                1 => APBBDIVR::DIV2,
                2 => APBBDIVR::DIV4,
                3 => APBBDIVR::DIV8,
                4 => APBBDIVR::DIV16,
                5 => APBBDIVR::DIV32,
                6 => APBBDIVR::DIV64,
                7 => APBBDIVR::DIV128,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DIV1`"]
        #[inline(always)]
        pub fn is_div1(&self) -> bool {
            *self == APBBDIVR::DIV1
        }
        #[doc = "Checks if the value of the field is `DIV2`"]
        #[inline(always)]
        pub fn is_div2(&self) -> bool {
            *self == APBBDIVR::DIV2
        }
        #[doc = "Checks if the value of the field is `DIV4`"]
        #[inline(always)]
        pub fn is_div4(&self) -> bool {
            *self == APBBDIVR::DIV4
        }
        #[doc = "Checks if the value of the field is `DIV8`"]
        #[inline(always)]
        pub fn is_div8(&self) -> bool {
            *self == APBBDIVR::DIV8
        }
        #[doc = "Checks if the value of the field is `DIV16`"]
        #[inline(always)]
        pub fn is_div16(&self) -> bool {
            *self == APBBDIVR::DIV16
        }
        #[doc = "Checks if the value of the field is `DIV32`"]
        #[inline(always)]
        pub fn is_div32(&self) -> bool {
            *self == APBBDIVR::DIV32
        }
        #[doc = "Checks if the value of the field is `DIV64`"]
        #[inline(always)]
        pub fn is_div64(&self) -> bool {
            *self == APBBDIVR::DIV64
        }
        #[doc = "Checks if the value of the field is `DIV128`"]
        #[inline(always)]
        pub fn is_div128(&self) -> bool {
            *self == APBBDIVR::DIV128
        }
    }
    #[doc = "Values that can be written to the field `APBBDIV`"]
    pub enum APBBDIVW {
        #[doc = "Divide by 1"]
        DIV1,
        #[doc = "Divide by 2"]
        DIV2,
        #[doc = "Divide by 4"]
        DIV4,
        #[doc = "Divide by 8"]
        DIV8,
        #[doc = "Divide by 16"]
        DIV16,
        #[doc = "Divide by 32"]
        DIV32,
        #[doc = "Divide by 64"]
        DIV64,
        #[doc = "Divide by 128"]
        DIV128,
    }
    impl APBBDIVW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                APBBDIVW::DIV1 => 0,
                APBBDIVW::DIV2 => 1,
                APBBDIVW::DIV4 => 2,
                APBBDIVW::DIV8 => 3,
                APBBDIVW::DIV16 => 4,
                APBBDIVW::DIV32 => 5,
                APBBDIVW::DIV64 => 6,
                APBBDIVW::DIV128 => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _APBBDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _APBBDIVW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: APBBDIVW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "Divide by 1"]
        #[inline(always)]
        pub fn div1(self) -> &'a mut W {
            self.variant(APBBDIVW::DIV1)
        }
        #[doc = "Divide by 2"]
        #[inline(always)]
        pub fn div2(self) -> &'a mut W {
            self.variant(APBBDIVW::DIV2)
        }
        #[doc = "Divide by 4"]
        #[inline(always)]
        pub fn div4(self) -> &'a mut W {
            self.variant(APBBDIVW::DIV4)
        }
        #[doc = "Divide by 8"]
        #[inline(always)]
        pub fn div8(self) -> &'a mut W {
            self.variant(APBBDIVW::DIV8)
        }
        #[doc = "Divide by 16"]
        #[inline(always)]
        pub fn div16(self) -> &'a mut W {
            self.variant(APBBDIVW::DIV16)
        }
        #[doc = "Divide by 32"]
        #[inline(always)]
        pub fn div32(self) -> &'a mut W {
            self.variant(APBBDIVW::DIV32)
        }
        #[doc = "Divide by 64"]
        #[inline(always)]
        pub fn div64(self) -> &'a mut W {
            self.variant(APBBDIVW::DIV64)
        }
        #[doc = "Divide by 128"]
        #[inline(always)]
        pub fn div128(self) -> &'a mut W {
            self.variant(APBBDIVW::DIV128)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
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
        #[doc = "Bits 0:2 - APBB Prescaler Selection"]
        #[inline(always)]
        pub fn apbbdiv(&self) -> APBBDIVR {
            APBBDIVR::_from({
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
        #[doc = "Bits 0:2 - APBB Prescaler Selection"]
        #[inline(always)]
        pub fn apbbdiv(&mut self) -> _APBBDIVW {
            _APBBDIVW { w: self }
        }
    }
}
#[doc = "APBC Mask"]
pub struct APBCMASK {
    register: VolatileCell<u32>,
}
#[doc = "APBC Mask"]
pub mod apbcmask {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::APBCMASK {
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
    pub struct PAC2R {
        bits: bool,
    }
    impl PAC2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EVSYSR {
        bits: bool,
    }
    impl EVSYSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SERCOM0R {
        bits: bool,
    }
    impl SERCOM0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SERCOM1R {
        bits: bool,
    }
    impl SERCOM1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SERCOM2R {
        bits: bool,
    }
    impl SERCOM2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SERCOM3R {
        bits: bool,
    }
    impl SERCOM3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SERCOM4R {
        bits: bool,
    }
    impl SERCOM4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SERCOM5R {
        bits: bool,
    }
    impl SERCOM5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TCC0R {
        bits: bool,
    }
    impl TCC0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TCC1R {
        bits: bool,
    }
    impl TCC1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TCC2R {
        bits: bool,
    }
    impl TCC2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TC3R {
        bits: bool,
    }
    impl TC3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TC4R {
        bits: bool,
    }
    impl TC4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TC5R {
        bits: bool,
    }
    impl TC5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct ADCR {
        bits: bool,
    }
    impl ADCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct ACR {
        bits: bool,
    }
    impl ACR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct DACR {
        bits: bool,
    }
    impl DACR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct I2SR {
        bits: bool,
    }
    impl I2SR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct ATWR {
        bits: bool,
    }
    impl ATWR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _PAC2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _PAC2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EVSYSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVSYSW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SERCOM0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SERCOM0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SERCOM1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SERCOM1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SERCOM2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SERCOM2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SERCOM3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SERCOM3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SERCOM4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SERCOM4W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SERCOM5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SERCOM5W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TCC0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCC0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TCC1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCC1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TCC2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCC2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC4W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TC5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TC5W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ADCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADCW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ACW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _DACW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DACW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _I2SW<'a> {
        w: &'a mut W,
    }
    impl<'a> _I2SW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ATWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ATWW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - PAC2 APB Clock Enable"]
        #[inline(always)]
        pub fn pac2(&self) -> PAC2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PAC2R { bits }
        }
        #[doc = "Bit 1 - EVSYS APB Clock Enable"]
        #[inline(always)]
        pub fn evsys(&self) -> EVSYSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVSYSR { bits }
        }
        #[doc = "Bit 2 - SERCOM0 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom0(&self) -> SERCOM0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SERCOM0R { bits }
        }
        #[doc = "Bit 3 - SERCOM1 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom1(&self) -> SERCOM1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SERCOM1R { bits }
        }
        #[doc = "Bit 4 - SERCOM2 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom2(&self) -> SERCOM2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SERCOM2R { bits }
        }
        #[doc = "Bit 5 - SERCOM3 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom3(&self) -> SERCOM3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SERCOM3R { bits }
        }
        #[doc = "Bit 6 - SERCOM4 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom4(&self) -> SERCOM4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SERCOM4R { bits }
        }
        #[doc = "Bit 7 - SERCOM5 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom5(&self) -> SERCOM5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SERCOM5R { bits }
        }
        #[doc = "Bit 8 - TCC0 APB Clock Enable"]
        #[inline(always)]
        pub fn tcc0(&self) -> TCC0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TCC0R { bits }
        }
        #[doc = "Bit 9 - TCC1 APB Clock Enable"]
        #[inline(always)]
        pub fn tcc1(&self) -> TCC1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TCC1R { bits }
        }
        #[doc = "Bit 10 - TCC2 APB Clock Enable"]
        #[inline(always)]
        pub fn tcc2(&self) -> TCC2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TCC2R { bits }
        }
        #[doc = "Bit 11 - TC3 APB Clock Enable"]
        #[inline(always)]
        pub fn tc3(&self) -> TC3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TC3R { bits }
        }
        #[doc = "Bit 12 - TC4 APB Clock Enable"]
        #[inline(always)]
        pub fn tc4(&self) -> TC4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TC4R { bits }
        }
        #[doc = "Bit 13 - TC5 APB Clock Enable"]
        #[inline(always)]
        pub fn tc5(&self) -> TC5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TC5R { bits }
        }
        #[doc = "Bit 16 - ADC APB Clock Enable"]
        #[inline(always)]
        pub fn adc(&self) -> ADCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ADCR { bits }
        }
        #[doc = "Bit 17 - AC APB Clock Enable"]
        #[inline(always)]
        pub fn ac(&self) -> ACR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ACR { bits }
        }
        #[doc = "Bit 18 - DAC APB Clock Enable"]
        #[inline(always)]
        pub fn dac(&self) -> DACR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DACR { bits }
        }
        #[doc = "Bit 20 - I2S APB Clock Enable"]
        #[inline(always)]
        pub fn i2s(&self) -> I2SR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            I2SR { bits }
        }
        #[doc = "Bit 23 - ATW APB Clock Enable"]
        #[inline(always)]
        pub fn atw(&self) -> ATWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ATWR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 65536 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 0 - PAC2 APB Clock Enable"]
        #[inline(always)]
        pub fn pac2(&mut self) -> _PAC2W {
            _PAC2W { w: self }
        }
        #[doc = "Bit 1 - EVSYS APB Clock Enable"]
        #[inline(always)]
        pub fn evsys(&mut self) -> _EVSYSW {
            _EVSYSW { w: self }
        }
        #[doc = "Bit 2 - SERCOM0 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom0(&mut self) -> _SERCOM0W {
            _SERCOM0W { w: self }
        }
        #[doc = "Bit 3 - SERCOM1 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom1(&mut self) -> _SERCOM1W {
            _SERCOM1W { w: self }
        }
        #[doc = "Bit 4 - SERCOM2 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom2(&mut self) -> _SERCOM2W {
            _SERCOM2W { w: self }
        }
        #[doc = "Bit 5 - SERCOM3 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom3(&mut self) -> _SERCOM3W {
            _SERCOM3W { w: self }
        }
        #[doc = "Bit 6 - SERCOM4 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom4(&mut self) -> _SERCOM4W {
            _SERCOM4W { w: self }
        }
        #[doc = "Bit 7 - SERCOM5 APB Clock Enable"]
        #[inline(always)]
        pub fn sercom5(&mut self) -> _SERCOM5W {
            _SERCOM5W { w: self }
        }
        #[doc = "Bit 8 - TCC0 APB Clock Enable"]
        #[inline(always)]
        pub fn tcc0(&mut self) -> _TCC0W {
            _TCC0W { w: self }
        }
        #[doc = "Bit 9 - TCC1 APB Clock Enable"]
        #[inline(always)]
        pub fn tcc1(&mut self) -> _TCC1W {
            _TCC1W { w: self }
        }
        #[doc = "Bit 10 - TCC2 APB Clock Enable"]
        #[inline(always)]
        pub fn tcc2(&mut self) -> _TCC2W {
            _TCC2W { w: self }
        }
        #[doc = "Bit 11 - TC3 APB Clock Enable"]
        #[inline(always)]
        pub fn tc3(&mut self) -> _TC3W {
            _TC3W { w: self }
        }
        #[doc = "Bit 12 - TC4 APB Clock Enable"]
        #[inline(always)]
        pub fn tc4(&mut self) -> _TC4W {
            _TC4W { w: self }
        }
        #[doc = "Bit 13 - TC5 APB Clock Enable"]
        #[inline(always)]
        pub fn tc5(&mut self) -> _TC5W {
            _TC5W { w: self }
        }
        #[doc = "Bit 16 - ADC APB Clock Enable"]
        #[inline(always)]
        pub fn adc(&mut self) -> _ADCW {
            _ADCW { w: self }
        }
        #[doc = "Bit 17 - AC APB Clock Enable"]
        #[inline(always)]
        pub fn ac(&mut self) -> _ACW {
            _ACW { w: self }
        }
        #[doc = "Bit 18 - DAC APB Clock Enable"]
        #[inline(always)]
        pub fn dac(&mut self) -> _DACW {
            _DACW { w: self }
        }
        #[doc = "Bit 20 - I2S APB Clock Enable"]
        #[inline(always)]
        pub fn i2s(&mut self) -> _I2SW {
            _I2SW { w: self }
        }
        #[doc = "Bit 23 - ATW APB Clock Enable"]
        #[inline(always)]
        pub fn atw(&mut self) -> _ATWW {
            _ATWW { w: self }
        }
    }
}
#[doc = "APBC Clock Select"]
pub struct APBCSEL {
    register: VolatileCell<u8>,
}
#[doc = "APBC Clock Select"]
pub mod apbcsel {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::APBCSEL {
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
    #[doc = "Possible values of the field `APBCDIV`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum APBCDIVR {
        #[doc = "Divide by 1"]
        DIV1,
        #[doc = "Divide by 2"]
        DIV2,
        #[doc = "Divide by 4"]
        DIV4,
        #[doc = "Divide by 8"]
        DIV8,
        #[doc = "Divide by 16"]
        DIV16,
        #[doc = "Divide by 32"]
        DIV32,
        #[doc = "Divide by 64"]
        DIV64,
        #[doc = "Divide by 128"]
        DIV128,
    }
    impl APBCDIVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                APBCDIVR::DIV1 => 0,
                APBCDIVR::DIV2 => 1,
                APBCDIVR::DIV4 => 2,
                APBCDIVR::DIV8 => 3,
                APBCDIVR::DIV16 => 4,
                APBCDIVR::DIV32 => 5,
                APBCDIVR::DIV64 => 6,
                APBCDIVR::DIV128 => 7,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> APBCDIVR {
            match value {
                0 => APBCDIVR::DIV1,
                1 => APBCDIVR::DIV2,
                2 => APBCDIVR::DIV4,
                3 => APBCDIVR::DIV8,
                4 => APBCDIVR::DIV16,
                5 => APBCDIVR::DIV32,
                6 => APBCDIVR::DIV64,
                7 => APBCDIVR::DIV128,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DIV1`"]
        #[inline(always)]
        pub fn is_div1(&self) -> bool {
            *self == APBCDIVR::DIV1
        }
        #[doc = "Checks if the value of the field is `DIV2`"]
        #[inline(always)]
        pub fn is_div2(&self) -> bool {
            *self == APBCDIVR::DIV2
        }
        #[doc = "Checks if the value of the field is `DIV4`"]
        #[inline(always)]
        pub fn is_div4(&self) -> bool {
            *self == APBCDIVR::DIV4
        }
        #[doc = "Checks if the value of the field is `DIV8`"]
        #[inline(always)]
        pub fn is_div8(&self) -> bool {
            *self == APBCDIVR::DIV8
        }
        #[doc = "Checks if the value of the field is `DIV16`"]
        #[inline(always)]
        pub fn is_div16(&self) -> bool {
            *self == APBCDIVR::DIV16
        }
        #[doc = "Checks if the value of the field is `DIV32`"]
        #[inline(always)]
        pub fn is_div32(&self) -> bool {
            *self == APBCDIVR::DIV32
        }
        #[doc = "Checks if the value of the field is `DIV64`"]
        #[inline(always)]
        pub fn is_div64(&self) -> bool {
            *self == APBCDIVR::DIV64
        }
        #[doc = "Checks if the value of the field is `DIV128`"]
        #[inline(always)]
        pub fn is_div128(&self) -> bool {
            *self == APBCDIVR::DIV128
        }
    }
    #[doc = "Values that can be written to the field `APBCDIV`"]
    pub enum APBCDIVW {
        #[doc = "Divide by 1"]
        DIV1,
        #[doc = "Divide by 2"]
        DIV2,
        #[doc = "Divide by 4"]
        DIV4,
        #[doc = "Divide by 8"]
        DIV8,
        #[doc = "Divide by 16"]
        DIV16,
        #[doc = "Divide by 32"]
        DIV32,
        #[doc = "Divide by 64"]
        DIV64,
        #[doc = "Divide by 128"]
        DIV128,
    }
    impl APBCDIVW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                APBCDIVW::DIV1 => 0,
                APBCDIVW::DIV2 => 1,
                APBCDIVW::DIV4 => 2,
                APBCDIVW::DIV8 => 3,
                APBCDIVW::DIV16 => 4,
                APBCDIVW::DIV32 => 5,
                APBCDIVW::DIV64 => 6,
                APBCDIVW::DIV128 => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _APBCDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _APBCDIVW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: APBCDIVW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "Divide by 1"]
        #[inline(always)]
        pub fn div1(self) -> &'a mut W {
            self.variant(APBCDIVW::DIV1)
        }
        #[doc = "Divide by 2"]
        #[inline(always)]
        pub fn div2(self) -> &'a mut W {
            self.variant(APBCDIVW::DIV2)
        }
        #[doc = "Divide by 4"]
        #[inline(always)]
        pub fn div4(self) -> &'a mut W {
            self.variant(APBCDIVW::DIV4)
        }
        #[doc = "Divide by 8"]
        #[inline(always)]
        pub fn div8(self) -> &'a mut W {
            self.variant(APBCDIVW::DIV8)
        }
        #[doc = "Divide by 16"]
        #[inline(always)]
        pub fn div16(self) -> &'a mut W {
            self.variant(APBCDIVW::DIV16)
        }
        #[doc = "Divide by 32"]
        #[inline(always)]
        pub fn div32(self) -> &'a mut W {
            self.variant(APBCDIVW::DIV32)
        }
        #[doc = "Divide by 64"]
        #[inline(always)]
        pub fn div64(self) -> &'a mut W {
            self.variant(APBCDIVW::DIV64)
        }
        #[doc = "Divide by 128"]
        #[inline(always)]
        pub fn div128(self) -> &'a mut W {
            self.variant(APBCDIVW::DIV128)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
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
        #[doc = "Bits 0:2 - APBC Prescaler Selection"]
        #[inline(always)]
        pub fn apbcdiv(&self) -> APBCDIVR {
            APBCDIVR::_from({
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
        #[doc = "Bits 0:2 - APBC Prescaler Selection"]
        #[inline(always)]
        pub fn apbcdiv(&mut self) -> _APBCDIVW {
            _APBCDIVW { w: self }
        }
    }
}
#[doc = "CPU Clock Select"]
pub struct CPUSEL {
    register: VolatileCell<u8>,
}
#[doc = "CPU Clock Select"]
pub mod cpusel {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CPUSEL {
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
    #[doc = "Possible values of the field `CPUDIV`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CPUDIVR {
        #[doc = "Divide by 1"]
        DIV1,
        #[doc = "Divide by 2"]
        DIV2,
        #[doc = "Divide by 4"]
        DIV4,
        #[doc = "Divide by 8"]
        DIV8,
        #[doc = "Divide by 16"]
        DIV16,
        #[doc = "Divide by 32"]
        DIV32,
        #[doc = "Divide by 64"]
        DIV64,
        #[doc = "Divide by 128"]
        DIV128,
    }
    impl CPUDIVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CPUDIVR::DIV1 => 0,
                CPUDIVR::DIV2 => 1,
                CPUDIVR::DIV4 => 2,
                CPUDIVR::DIV8 => 3,
                CPUDIVR::DIV16 => 4,
                CPUDIVR::DIV32 => 5,
                CPUDIVR::DIV64 => 6,
                CPUDIVR::DIV128 => 7,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CPUDIVR {
            match value {
                0 => CPUDIVR::DIV1,
                1 => CPUDIVR::DIV2,
                2 => CPUDIVR::DIV4,
                3 => CPUDIVR::DIV8,
                4 => CPUDIVR::DIV16,
                5 => CPUDIVR::DIV32,
                6 => CPUDIVR::DIV64,
                7 => CPUDIVR::DIV128,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DIV1`"]
        #[inline(always)]
        pub fn is_div1(&self) -> bool {
            *self == CPUDIVR::DIV1
        }
        #[doc = "Checks if the value of the field is `DIV2`"]
        #[inline(always)]
        pub fn is_div2(&self) -> bool {
            *self == CPUDIVR::DIV2
        }
        #[doc = "Checks if the value of the field is `DIV4`"]
        #[inline(always)]
        pub fn is_div4(&self) -> bool {
            *self == CPUDIVR::DIV4
        }
        #[doc = "Checks if the value of the field is `DIV8`"]
        #[inline(always)]
        pub fn is_div8(&self) -> bool {
            *self == CPUDIVR::DIV8
        }
        #[doc = "Checks if the value of the field is `DIV16`"]
        #[inline(always)]
        pub fn is_div16(&self) -> bool {
            *self == CPUDIVR::DIV16
        }
        #[doc = "Checks if the value of the field is `DIV32`"]
        #[inline(always)]
        pub fn is_div32(&self) -> bool {
            *self == CPUDIVR::DIV32
        }
        #[doc = "Checks if the value of the field is `DIV64`"]
        #[inline(always)]
        pub fn is_div64(&self) -> bool {
            *self == CPUDIVR::DIV64
        }
        #[doc = "Checks if the value of the field is `DIV128`"]
        #[inline(always)]
        pub fn is_div128(&self) -> bool {
            *self == CPUDIVR::DIV128
        }
    }
    #[doc = "Values that can be written to the field `CPUDIV`"]
    pub enum CPUDIVW {
        #[doc = "Divide by 1"]
        DIV1,
        #[doc = "Divide by 2"]
        DIV2,
        #[doc = "Divide by 4"]
        DIV4,
        #[doc = "Divide by 8"]
        DIV8,
        #[doc = "Divide by 16"]
        DIV16,
        #[doc = "Divide by 32"]
        DIV32,
        #[doc = "Divide by 64"]
        DIV64,
        #[doc = "Divide by 128"]
        DIV128,
    }
    impl CPUDIVW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CPUDIVW::DIV1 => 0,
                CPUDIVW::DIV2 => 1,
                CPUDIVW::DIV4 => 2,
                CPUDIVW::DIV8 => 3,
                CPUDIVW::DIV16 => 4,
                CPUDIVW::DIV32 => 5,
                CPUDIVW::DIV64 => 6,
                CPUDIVW::DIV128 => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CPUDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CPUDIVW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CPUDIVW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "Divide by 1"]
        #[inline(always)]
        pub fn div1(self) -> &'a mut W {
            self.variant(CPUDIVW::DIV1)
        }
        #[doc = "Divide by 2"]
        #[inline(always)]
        pub fn div2(self) -> &'a mut W {
            self.variant(CPUDIVW::DIV2)
        }
        #[doc = "Divide by 4"]
        #[inline(always)]
        pub fn div4(self) -> &'a mut W {
            self.variant(CPUDIVW::DIV4)
        }
        #[doc = "Divide by 8"]
        #[inline(always)]
        pub fn div8(self) -> &'a mut W {
            self.variant(CPUDIVW::DIV8)
        }
        #[doc = "Divide by 16"]
        #[inline(always)]
        pub fn div16(self) -> &'a mut W {
            self.variant(CPUDIVW::DIV16)
        }
        #[doc = "Divide by 32"]
        #[inline(always)]
        pub fn div32(self) -> &'a mut W {
            self.variant(CPUDIVW::DIV32)
        }
        #[doc = "Divide by 64"]
        #[inline(always)]
        pub fn div64(self) -> &'a mut W {
            self.variant(CPUDIVW::DIV64)
        }
        #[doc = "Divide by 128"]
        #[inline(always)]
        pub fn div128(self) -> &'a mut W {
            self.variant(CPUDIVW::DIV128)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
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
        #[doc = "Bits 0:2 - CPU Prescaler Selection"]
        #[inline(always)]
        pub fn cpudiv(&self) -> CPUDIVR {
            CPUDIVR::_from({
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
        #[doc = "Bits 0:2 - CPU Prescaler Selection"]
        #[inline(always)]
        pub fn cpudiv(&mut self) -> _CPUDIVW {
            _CPUDIVW { w: self }
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
    pub struct CFDENR {
        bits: bool,
    }
    impl CFDENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BKUPCLKR {
        bits: bool,
    }
    impl BKUPCLKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _CFDENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CFDENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BKUPCLKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BKUPCLKW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bit 2 - Clock Failure Detector Enable"]
        #[inline(always)]
        pub fn cfden(&self) -> CFDENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CFDENR { bits }
        }
        #[doc = "Bit 4 - Backup Clock Select"]
        #[inline(always)]
        pub fn bkupclk(&self) -> BKUPCLKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            BKUPCLKR { bits }
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
        #[doc = "Bit 2 - Clock Failure Detector Enable"]
        #[inline(always)]
        pub fn cfden(&mut self) -> _CFDENW {
            _CFDENW { w: self }
        }
        #[doc = "Bit 4 - Backup Clock Select"]
        #[inline(always)]
        pub fn bkupclk(&mut self) -> _BKUPCLKW {
            _BKUPCLKW { w: self }
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
    pub struct CKRDYR {
        bits: bool,
    }
    impl CKRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CFDR {
        bits: bool,
    }
    impl CFDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _CKRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CKRDYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CFDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CFDW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Clock Ready Interrupt Enable"]
        #[inline(always)]
        pub fn ckrdy(&self) -> CKRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CKRDYR { bits }
        }
        #[doc = "Bit 1 - Clock Failure Detector Interrupt Enable"]
        #[inline(always)]
        pub fn cfd(&self) -> CFDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CFDR { bits }
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
        #[doc = "Bit 0 - Clock Ready Interrupt Enable"]
        #[inline(always)]
        pub fn ckrdy(&mut self) -> _CKRDYW {
            _CKRDYW { w: self }
        }
        #[doc = "Bit 1 - Clock Failure Detector Interrupt Enable"]
        #[inline(always)]
        pub fn cfd(&mut self) -> _CFDW {
            _CFDW { w: self }
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
    pub struct CKRDYR {
        bits: bool,
    }
    impl CKRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CFDR {
        bits: bool,
    }
    impl CFDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _CKRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CKRDYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CFDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CFDW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Clock Ready Interrupt Enable"]
        #[inline(always)]
        pub fn ckrdy(&self) -> CKRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CKRDYR { bits }
        }
        #[doc = "Bit 1 - Clock Failure Detector Interrupt Enable"]
        #[inline(always)]
        pub fn cfd(&self) -> CFDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CFDR { bits }
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
        #[doc = "Bit 0 - Clock Ready Interrupt Enable"]
        #[inline(always)]
        pub fn ckrdy(&mut self) -> _CKRDYW {
            _CKRDYW { w: self }
        }
        #[doc = "Bit 1 - Clock Failure Detector Interrupt Enable"]
        #[inline(always)]
        pub fn cfd(&mut self) -> _CFDW {
            _CFDW { w: self }
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
    pub struct CKRDYR {
        bits: bool,
    }
    impl CKRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CFDR {
        bits: bool,
    }
    impl CFDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _CKRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CKRDYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CFDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CFDW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Clock Ready"]
        #[inline(always)]
        pub fn ckrdy(&self) -> CKRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CKRDYR { bits }
        }
        #[doc = "Bit 1 - Clock Failure Detector"]
        #[inline(always)]
        pub fn cfd(&self) -> CFDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CFDR { bits }
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
        #[doc = "Bit 0 - Clock Ready"]
        #[inline(always)]
        pub fn ckrdy(&mut self) -> _CKRDYW {
            _CKRDYW { w: self }
        }
        #[doc = "Bit 1 - Clock Failure Detector"]
        #[inline(always)]
        pub fn cfd(&mut self) -> _CFDW {
            _CFDW { w: self }
        }
    }
}
#[doc = "Reset Cause"]
pub struct RCAUSE {
    register: VolatileCell<u8>,
}
#[doc = "Reset Cause"]
pub mod rcause {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    impl super::RCAUSE {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct PORR {
        bits: bool,
    }
    impl PORR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BOD12R {
        bits: bool,
    }
    impl BOD12R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BOD33R {
        bits: bool,
    }
    impl BOD33R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EXTR {
        bits: bool,
    }
    impl EXTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WDTR {
        bits: bool,
    }
    impl WDTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SYSTR {
        bits: bool,
    }
    impl SYSTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
        #[doc = "Bit 0 - Power On Reset"]
        #[inline(always)]
        pub fn por(&self) -> PORR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            PORR { bits }
        }
        #[doc = "Bit 1 - Brown Out 12 Detector Reset"]
        #[inline(always)]
        pub fn bod12(&self) -> BOD12R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            BOD12R { bits }
        }
        #[doc = "Bit 2 - Brown Out 33 Detector Reset"]
        #[inline(always)]
        pub fn bod33(&self) -> BOD33R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            BOD33R { bits }
        }
        #[doc = "Bit 4 - External Reset"]
        #[inline(always)]
        pub fn ext(&self) -> EXTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            EXTR { bits }
        }
        #[doc = "Bit 5 - Watchdog Reset"]
        #[inline(always)]
        pub fn wdt(&self) -> WDTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            WDTR { bits }
        }
        #[doc = "Bit 6 - System Reset Request"]
        #[inline(always)]
        pub fn syst(&self) -> SYSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SYSTR { bits }
        }
    }
}
#[doc = "Sleep Mode"]
pub struct SLEEP {
    register: VolatileCell<u8>,
}
#[doc = "Sleep Mode"]
pub mod sleep {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::SLEEP {
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
    #[doc = "Possible values of the field `IDLE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum IDLER {
        #[doc = "The CPU clock domain is stopped"]
        CPU,
        #[doc = "The CPU and AHB clock domains are stopped"]
        AHB,
        #[doc = "The CPU, AHB and APB clock domains are stopped"]
        APB,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl IDLER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                IDLER::CPU => 0,
                IDLER::AHB => 1,
                IDLER::APB => 2,
                IDLER::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> IDLER {
            match value {
                0 => IDLER::CPU,
                1 => IDLER::AHB,
                2 => IDLER::APB,
                i => IDLER::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `CPU`"]
        #[inline(always)]
        pub fn is_cpu(&self) -> bool {
            *self == IDLER::CPU
        }
        #[doc = "Checks if the value of the field is `AHB`"]
        #[inline(always)]
        pub fn is_ahb(&self) -> bool {
            *self == IDLER::AHB
        }
        #[doc = "Checks if the value of the field is `APB`"]
        #[inline(always)]
        pub fn is_apb(&self) -> bool {
            *self == IDLER::APB
        }
    }
    #[doc = "Values that can be written to the field `IDLE`"]
    pub enum IDLEW {
        #[doc = "The CPU clock domain is stopped"]
        CPU,
        #[doc = "The CPU and AHB clock domains are stopped"]
        AHB,
        #[doc = "The CPU, AHB and APB clock domains are stopped"]
        APB,
    }
    impl IDLEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                IDLEW::CPU => 0,
                IDLEW::AHB => 1,
                IDLEW::APB => 2,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _IDLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _IDLEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: IDLEW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "The CPU clock domain is stopped"]
        #[inline(always)]
        pub fn cpu(self) -> &'a mut W {
            self.variant(IDLEW::CPU)
        }
        #[doc = "The CPU and AHB clock domains are stopped"]
        #[inline(always)]
        pub fn ahb(self) -> &'a mut W {
            self.variant(IDLEW::AHB)
        }
        #[doc = "The CPU, AHB and APB clock domains are stopped"]
        #[inline(always)]
        pub fn apb(self) -> &'a mut W {
            self.variant(IDLEW::APB)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
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
        #[doc = "Bits 0:1 - Idle Mode Configuration"]
        #[inline(always)]
        pub fn idle(&self) -> IDLER {
            IDLER::_from({
                const MASK: u8 = 3;
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
        #[doc = "Bits 0:1 - Idle Mode Configuration"]
        #[inline(always)]
        pub fn idle(&mut self) -> _IDLEW {
            _IDLEW { w: self }
        }
    }
}
