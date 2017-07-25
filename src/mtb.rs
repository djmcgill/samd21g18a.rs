use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB Position"]
    pub position: POSITION,
    #[doc = "0x04 - MTB Master"]
    pub master: MASTER,
    #[doc = "0x08 - MTB Flow"]
    pub flow: FLOW,
    #[doc = "0x0c - MTB Base"]
    pub base: BASE,
    _reserved0: [u8; 3824usize],
    #[doc = "0xf00 - MTB Integration Mode Control"]
    pub itctrl: ITCTRL,
    _reserved1: [u8; 156usize],
    #[doc = "0xfa0 - MTB Claim Set"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - MTB Claim Clear"]
    pub claimclr: CLAIMCLR,
    _reserved2: [u8; 8usize],
    #[doc = "0xfb0 - MTB Lock Access"]
    pub lockaccess: LOCKACCESS,
    #[doc = "0xfb4 - MTB Lock Status"]
    pub lockstatus: LOCKSTATUS,
    #[doc = "0xfb8 - MTB Authentication Status"]
    pub authstatus: AUTHSTATUS,
    #[doc = "0xfbc - MTB Device Architecture"]
    pub devarch: DEVARCH,
    _reserved3: [u8; 8usize],
    #[doc = "0xfc8 - MTB Device Configuration"]
    pub devid: DEVID,
    #[doc = "0xfcc - MTB Device Type"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - CoreSight"]
    pub pid4: PID4,
    #[doc = "0xfd4 - CoreSight"]
    pub pid5: PID5,
    #[doc = "0xfd8 - CoreSight"]
    pub pid6: PID6,
    #[doc = "0xfdc - CoreSight"]
    pub pid7: PID7,
    #[doc = "0xfe0 - CoreSight"]
    pub pid0: PID0,
    #[doc = "0xfe4 - CoreSight"]
    pub pid1: PID1,
    #[doc = "0xfe8 - CoreSight"]
    pub pid2: PID2,
    #[doc = "0xfec - CoreSight"]
    pub pid3: PID3,
    #[doc = "0xff0 - CoreSight"]
    pub cid0: CID0,
    #[doc = "0xff4 - CoreSight"]
    pub cid1: CID1,
    #[doc = "0xff8 - CoreSight"]
    pub cid2: CID2,
    #[doc = "0xffc - CoreSight"]
    pub cid3: CID3,
}
#[doc = "MTB Authentication Status"]
pub struct AUTHSTATUS {
    register: VolatileCell<u32>,
}
#[doc = "MTB Authentication Status"]
pub mod authstatus {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::AUTHSTATUS {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "MTB Base"]
pub struct BASE {
    register: VolatileCell<u32>,
}
#[doc = "MTB Base"]
pub mod base {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::BASE {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct CID0 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod cid0 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::CID0 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct CID1 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod cid1 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::CID1 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct CID2 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod cid2 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::CID2 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct CID3 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod cid3 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::CID3 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "MTB Claim Clear"]
pub struct CLAIMCLR {
    register: VolatileCell<u32>,
}
#[doc = "MTB Claim Clear"]
pub mod claimclr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::CLAIMCLR {
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
#[doc = "MTB Claim Set"]
pub struct CLAIMSET {
    register: VolatileCell<u32>,
}
#[doc = "MTB Claim Set"]
pub mod claimset {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::CLAIMSET {
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
#[doc = "MTB Device Architecture"]
pub struct DEVARCH {
    register: VolatileCell<u32>,
}
#[doc = "MTB Device Architecture"]
pub mod devarch {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::DEVARCH {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "MTB Device Configuration"]
pub struct DEVID {
    register: VolatileCell<u32>,
}
#[doc = "MTB Device Configuration"]
pub mod devid {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::DEVID {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "MTB Device Type"]
pub struct DEVTYPE {
    register: VolatileCell<u32>,
}
#[doc = "MTB Device Type"]
pub mod devtype {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::DEVTYPE {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "MTB Flow"]
pub struct FLOW {
    register: VolatileCell<u32>,
}
#[doc = "MTB Flow"]
pub mod flow {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::FLOW {
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
    pub struct AUTOSTOPR {
        bits: bool,
    }
    impl AUTOSTOPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct AUTOHALTR {
        bits: bool,
    }
    impl AUTOHALTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct WATERMARKR {
        bits: u32,
    }
    impl WATERMARKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _AUTOSTOPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AUTOSTOPW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _AUTOHALTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AUTOHALTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _WATERMARKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WATERMARKW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
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
        #[doc = "Bit 0 - Auto Stop Tracing"]
        #[inline(always)]
        pub fn autostop(&self) -> AUTOSTOPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            AUTOSTOPR { bits }
        }
        #[doc = "Bit 1 - Auto Halt Request"]
        #[inline(always)]
        pub fn autohalt(&self) -> AUTOHALTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            AUTOHALTR { bits }
        }
        #[doc = "Bits 3:31 - Watermark value"]
        #[inline(always)]
        pub fn watermark(&self) -> WATERMARKR {
            let bits = {
                const MASK: u32 = 536870911;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            WATERMARKR { bits }
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
        #[doc = "Bit 0 - Auto Stop Tracing"]
        #[inline(always)]
        pub fn autostop(&mut self) -> _AUTOSTOPW {
            _AUTOSTOPW { w: self }
        }
        #[doc = "Bit 1 - Auto Halt Request"]
        #[inline(always)]
        pub fn autohalt(&mut self) -> _AUTOHALTW {
            _AUTOHALTW { w: self }
        }
        #[doc = "Bits 3:31 - Watermark value"]
        #[inline(always)]
        pub fn watermark(&mut self) -> _WATERMARKW {
            _WATERMARKW { w: self }
        }
    }
}
#[doc = "MTB Integration Mode Control"]
pub struct ITCTRL {
    register: VolatileCell<u32>,
}
#[doc = "MTB Integration Mode Control"]
pub mod itctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::ITCTRL {
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
#[doc = "MTB Lock Access"]
pub struct LOCKACCESS {
    register: VolatileCell<u32>,
}
#[doc = "MTB Lock Access"]
pub mod lockaccess {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::LOCKACCESS {
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
#[doc = "MTB Lock Status"]
pub struct LOCKSTATUS {
    register: VolatileCell<u32>,
}
#[doc = "MTB Lock Status"]
pub mod lockstatus {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::LOCKSTATUS {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "MTB Master"]
pub struct MASTER {
    register: VolatileCell<u32>,
}
#[doc = "MTB Master"]
pub mod master {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::MASTER {
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
    pub struct MASKR {
        bits: u8,
    }
    impl MASKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct TSTARTENR {
        bits: bool,
    }
    impl TSTARTENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TSTOPENR {
        bits: bool,
    }
    impl TSTOPENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SFRWPRIVR {
        bits: bool,
    }
    impl SFRWPRIVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct RAMPRIVR {
        bits: bool,
    }
    impl RAMPRIVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct HALTREQR {
        bits: bool,
    }
    impl HALTREQR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct ENR {
        bits: bool,
    }
    impl ENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _MASKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MASKW<'a> {
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
    #[doc = r" Proxy"]
    pub struct _TSTARTENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TSTARTENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TSTOPENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TSTOPENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SFRWPRIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SFRWPRIVW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _RAMPRIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RAMPRIVW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _HALTREQW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HALTREQW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ENW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bits 0:4 - Maximum Value of the Trace Buffer in SRAM"]
        #[inline(always)]
        pub fn mask(&self) -> MASKR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MASKR { bits }
        }
        #[doc = "Bit 5 - Trace Start Input Enable"]
        #[inline(always)]
        pub fn tstarten(&self) -> TSTARTENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TSTARTENR { bits }
        }
        #[doc = "Bit 6 - Trace Stop Input Enable"]
        #[inline(always)]
        pub fn tstopen(&self) -> TSTOPENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TSTOPENR { bits }
        }
        #[doc = "Bit 7 - Special Function Register Write Privilege"]
        #[inline(always)]
        pub fn sfrwpriv(&self) -> SFRWPRIVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SFRWPRIVR { bits }
        }
        #[doc = "Bit 8 - SRAM Privilege"]
        #[inline(always)]
        pub fn rampriv(&self) -> RAMPRIVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RAMPRIVR { bits }
        }
        #[doc = "Bit 9 - Halt Request"]
        #[inline(always)]
        pub fn haltreq(&self) -> HALTREQR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HALTREQR { bits }
        }
        #[doc = "Bit 31 - Main Trace Enable"]
        #[inline(always)]
        pub fn en(&self) -> ENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ENR { bits }
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
        #[doc = "Bits 0:4 - Maximum Value of the Trace Buffer in SRAM"]
        #[inline(always)]
        pub fn mask(&mut self) -> _MASKW {
            _MASKW { w: self }
        }
        #[doc = "Bit 5 - Trace Start Input Enable"]
        #[inline(always)]
        pub fn tstarten(&mut self) -> _TSTARTENW {
            _TSTARTENW { w: self }
        }
        #[doc = "Bit 6 - Trace Stop Input Enable"]
        #[inline(always)]
        pub fn tstopen(&mut self) -> _TSTOPENW {
            _TSTOPENW { w: self }
        }
        #[doc = "Bit 7 - Special Function Register Write Privilege"]
        #[inline(always)]
        pub fn sfrwpriv(&mut self) -> _SFRWPRIVW {
            _SFRWPRIVW { w: self }
        }
        #[doc = "Bit 8 - SRAM Privilege"]
        #[inline(always)]
        pub fn rampriv(&mut self) -> _RAMPRIVW {
            _RAMPRIVW { w: self }
        }
        #[doc = "Bit 9 - Halt Request"]
        #[inline(always)]
        pub fn haltreq(&mut self) -> _HALTREQW {
            _HALTREQW { w: self }
        }
        #[doc = "Bit 31 - Main Trace Enable"]
        #[inline(always)]
        pub fn en(&mut self) -> _ENW {
            _ENW { w: self }
        }
    }
}
#[doc = "CoreSight"]
pub struct PID0 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid0 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PID0 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct PID1 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid1 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PID1 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct PID2 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid2 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PID2 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct PID3 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid3 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PID3 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct PID4 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid4 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PID4 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct PID5 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid5 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PID5 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct PID6 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid6 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PID6 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "CoreSight"]
pub struct PID7 {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight"]
pub mod pid7 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PID7 {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
}
#[doc = "MTB Position"]
pub struct POSITION {
    register: VolatileCell<u32>,
}
#[doc = "MTB Position"]
pub mod position {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::POSITION {
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
    pub struct WRAPR {
        bits: bool,
    }
    impl WRAPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct POINTERR {
        bits: u32,
    }
    impl POINTERR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _WRAPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRAPW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _POINTERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _POINTERW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
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
        #[doc = "Bit 2 - Pointer Value Wraps"]
        #[inline(always)]
        pub fn wrap(&self) -> WRAPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WRAPR { bits }
        }
        #[doc = "Bits 3:31 - Trace Packet Location Pointer"]
        #[inline(always)]
        pub fn pointer(&self) -> POINTERR {
            let bits = {
                const MASK: u32 = 536870911;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            POINTERR { bits }
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
        #[doc = "Bit 2 - Pointer Value Wraps"]
        #[inline(always)]
        pub fn wrap(&mut self) -> _WRAPW {
            _WRAPW { w: self }
        }
        #[doc = "Bits 3:31 - Trace Packet Location Pointer"]
        #[inline(always)]
        pub fn pointer(&mut self) -> _POINTERW {
            _POINTERW { w: self }
        }
    }
}
