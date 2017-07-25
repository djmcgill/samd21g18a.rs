use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Status A"]
    pub statusa: STATUSA,
    #[doc = "0x02 - Status B"]
    pub statusb: STATUSB,
    _reserved0: [u8; 1usize],
    #[doc = "0x04 - Address"]
    pub addr: ADDR,
    #[doc = "0x08 - Length"]
    pub length: LENGTH,
    #[doc = "0x0c - Data"]
    pub data: DATA,
    #[doc = "0x10 - Debug Communication Channel n"]
    pub dcc0: DCC,
    #[doc = "0x14 - Debug Communication Channel n"]
    pub dcc1: DCC,
    #[doc = "0x18 - Device Identification"]
    pub did: DID,
    _reserved1: [u8; 4068usize],
    #[doc = "0x1000 - Coresight ROM Table Entry n"]
    pub entry0: ENTRY,
    #[doc = "0x1004 - Coresight ROM Table Entry n"]
    pub entry1: ENTRY,
    #[doc = "0x1008 - Coresight ROM Table End"]
    pub end: END,
    _reserved2: [u8; 4032usize],
    #[doc = "0x1fcc - Coresight ROM Table Memory Type"]
    pub memtype: MEMTYPE,
    #[doc = "0x1fd0 - Peripheral Identification 4"]
    pub pid4: PID4,
    _reserved3: [u8; 12usize],
    #[doc = "0x1fe0 - Peripheral Identification 0"]
    pub pid0: PID0,
    #[doc = "0x1fe4 - Peripheral Identification 1"]
    pub pid1: PID1,
    #[doc = "0x1fe8 - Peripheral Identification 2"]
    pub pid2: PID2,
    #[doc = "0x1fec - Peripheral Identification 3"]
    pub pid3: PID3,
    #[doc = "0x1ff0 - Component Identification 0"]
    pub cid0: CID0,
    #[doc = "0x1ff4 - Component Identification 1"]
    pub cid1: CID1,
    #[doc = "0x1ff8 - Component Identification 2"]
    pub cid2: CID2,
    #[doc = "0x1ffc - Component Identification 3"]
    pub cid3: CID3,
}
#[doc = "Address"]
pub struct ADDR {
    register: VolatileCell<u32>,
}
#[doc = "Address"]
pub mod addr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::ADDR {
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
    pub struct ADDRR {
        bits: u32,
    }
    impl ADDRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _ADDRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ADDRW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 2;
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
        #[doc = "Bits 2:31 - Address"]
        #[inline(always)]
        pub fn addr(&self) -> ADDRR {
            let bits = {
                const MASK: u32 = 1073741823;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            ADDRR { bits }
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
        #[doc = "Bits 2:31 - Address"]
        #[inline(always)]
        pub fn addr(&mut self) -> _ADDRW {
            _ADDRW { w: self }
        }
    }
}
#[doc = "Component Identification 0"]
pub struct CID0 {
    register: VolatileCell<u32>,
}
#[doc = "Component Identification 0"]
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
    #[doc = r" Value of the field"]
    pub struct PREAMBLEB0R {
        bits: u8,
    }
    impl PREAMBLEB0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:7 - Preamble Byte 0"]
        #[inline(always)]
        pub fn preambleb0(&self) -> PREAMBLEB0R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PREAMBLEB0R { bits }
        }
    }
}
#[doc = "Component Identification 1"]
pub struct CID1 {
    register: VolatileCell<u32>,
}
#[doc = "Component Identification 1"]
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
    #[doc = r" Value of the field"]
    pub struct PREAMBLER {
        bits: u8,
    }
    impl PREAMBLER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct CCLASSR {
        bits: u8,
    }
    impl CCLASSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:3 - Preamble"]
        #[inline(always)]
        pub fn preamble(&self) -> PREAMBLER {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PREAMBLER { bits }
        }
        #[doc = "Bits 4:7 - Component Class"]
        #[inline(always)]
        pub fn cclass(&self) -> CCLASSR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CCLASSR { bits }
        }
    }
}
#[doc = "Component Identification 2"]
pub struct CID2 {
    register: VolatileCell<u32>,
}
#[doc = "Component Identification 2"]
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
    #[doc = r" Value of the field"]
    pub struct PREAMBLEB2R {
        bits: u8,
    }
    impl PREAMBLEB2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:7 - Preamble Byte 2"]
        #[inline(always)]
        pub fn preambleb2(&self) -> PREAMBLEB2R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PREAMBLEB2R { bits }
        }
    }
}
#[doc = "Component Identification 3"]
pub struct CID3 {
    register: VolatileCell<u32>,
}
#[doc = "Component Identification 3"]
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
    #[doc = r" Value of the field"]
    pub struct PREAMBLEB3R {
        bits: u8,
    }
    impl PREAMBLEB3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:7 - Preamble Byte 3"]
        #[inline(always)]
        pub fn preambleb3(&self) -> PREAMBLEB3R {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PREAMBLEB3R { bits }
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
    pub struct _CRCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRCW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _MBISTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MBISTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    #[doc = r" Proxy"]
    pub struct _CEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 2 - 32-bit Cyclic Redundancy Check"]
        #[inline(always)]
        pub fn crc(&mut self) -> _CRCW {
            _CRCW { w: self }
        }
        #[doc = "Bit 3 - Memory Built-In Self-Test"]
        #[inline(always)]
        pub fn mbist(&mut self) -> _MBISTW {
            _MBISTW { w: self }
        }
        #[doc = "Bit 4 - Chip Erase"]
        #[inline(always)]
        pub fn ce(&mut self) -> _CEW {
            _CEW { w: self }
        }
    }
}
#[doc = "Data"]
pub struct DATA {
    register: VolatileCell<u32>,
}
#[doc = "Data"]
pub mod data {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DATA {
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
    pub struct DATAR {
        bits: u32,
    }
    impl DATAR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _DATAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATAW<'a> {
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
        #[doc = "Bits 0:31 - Data"]
        #[inline(always)]
        pub fn data(&self) -> DATAR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            DATAR { bits }
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
        #[doc = "Bits 0:31 - Data"]
        #[inline(always)]
        pub fn data(&mut self) -> _DATAW {
            _DATAW { w: self }
        }
    }
}
#[doc = "Debug Communication Channel n"]
pub struct DCC {
    register: VolatileCell<u32>,
}
#[doc = "Debug Communication Channel n"]
pub mod dcc {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DCC {
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
    pub struct DATAR {
        bits: u32,
    }
    impl DATAR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _DATAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATAW<'a> {
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
        #[doc = "Bits 0:31 - Data"]
        #[inline(always)]
        pub fn data(&self) -> DATAR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            DATAR { bits }
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
        #[doc = "Bits 0:31 - Data"]
        #[inline(always)]
        pub fn data(&mut self) -> _DATAW {
            _DATAW { w: self }
        }
    }
}
#[doc = "Device Identification"]
pub struct DID {
    register: VolatileCell<u32>,
}
#[doc = "Device Identification"]
pub mod did {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::DID {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct DEVSELR {
        bits: u8,
    }
    impl DEVSELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct REVISIONR {
        bits: u8,
    }
    impl REVISIONR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct DIER {
        bits: u8,
    }
    impl DIER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct SERIESR {
        bits: u8,
    }
    impl SERIESR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct FAMILYR {
        bits: u8,
    }
    impl FAMILYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct PROCESSORR {
        bits: u8,
    }
    impl PROCESSORR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:7 - Device Select"]
        #[inline(always)]
        pub fn devsel(&self) -> DEVSELR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DEVSELR { bits }
        }
        #[doc = "Bits 8:11 - Revision"]
        #[inline(always)]
        pub fn revision(&self) -> REVISIONR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            REVISIONR { bits }
        }
        #[doc = "Bits 12:15 - Die Identification"]
        #[inline(always)]
        pub fn die(&self) -> DIER {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            DIER { bits }
        }
        #[doc = "Bits 16:21 - Product Series"]
        #[inline(always)]
        pub fn series(&self) -> SERIESR {
            let bits = {
                const MASK: u8 = 63;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            SERIESR { bits }
        }
        #[doc = "Bits 23:27 - Product Family"]
        #[inline(always)]
        pub fn family(&self) -> FAMILYR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            FAMILYR { bits }
        }
        #[doc = "Bits 28:31 - Processor"]
        #[inline(always)]
        pub fn processor(&self) -> PROCESSORR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 28;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PROCESSORR { bits }
        }
    }
}
#[doc = "Coresight ROM Table End"]
pub struct END {
    register: VolatileCell<u32>,
}
#[doc = "Coresight ROM Table End"]
pub mod end {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::END {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct ENDR {
        bits: u32,
    }
    impl ENDR {
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
        #[doc = "Bits 0:31 - End Marker"]
        #[inline(always)]
        pub fn end(&self) -> ENDR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            ENDR { bits }
        }
    }
}
#[doc = "Coresight ROM Table Entry n"]
pub struct ENTRY {
    register: VolatileCell<u32>,
}
#[doc = "Coresight ROM Table Entry n"]
pub mod entry {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::ENTRY {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct EPRESR {
        bits: bool,
    }
    impl EPRESR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct FMTR {
        bits: bool,
    }
    impl FMTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct ADDOFFR {
        bits: u32,
    }
    impl ADDOFFR {
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
        #[doc = "Bit 0 - Entry Present"]
        #[inline(always)]
        pub fn epres(&self) -> EPRESR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EPRESR { bits }
        }
        #[doc = "Bit 1 - Format"]
        #[inline(always)]
        pub fn fmt(&self) -> FMTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FMTR { bits }
        }
        #[doc = "Bits 12:31 - Address Offset"]
        #[inline(always)]
        pub fn addoff(&self) -> ADDOFFR {
            let bits = {
                const MASK: u32 = 1048575;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            ADDOFFR { bits }
        }
    }
}
#[doc = "Length"]
pub struct LENGTH {
    register: VolatileCell<u32>,
}
#[doc = "Length"]
pub mod length {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::LENGTH {
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
    pub struct LENGTHR {
        bits: u32,
    }
    impl LENGTHR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _LENGTHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LENGTHW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 2;
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
        #[doc = "Bits 2:31 - Length"]
        #[inline(always)]
        pub fn length(&self) -> LENGTHR {
            let bits = {
                const MASK: u32 = 1073741823;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            LENGTHR { bits }
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
        #[doc = "Bits 2:31 - Length"]
        #[inline(always)]
        pub fn length(&mut self) -> _LENGTHW {
            _LENGTHW { w: self }
        }
    }
}
#[doc = "Coresight ROM Table Memory Type"]
pub struct MEMTYPE {
    register: VolatileCell<u32>,
}
#[doc = "Coresight ROM Table Memory Type"]
pub mod memtype {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::MEMTYPE {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct SMEMPR {
        bits: bool,
    }
    impl SMEMPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
        #[doc = "Bit 0 - System Memory Present"]
        #[inline(always)]
        pub fn smemp(&self) -> SMEMPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SMEMPR { bits }
        }
    }
}
#[doc = "Peripheral Identification 0"]
pub struct PID0 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Identification 0"]
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
    #[doc = r" Value of the field"]
    pub struct PARTNBLR {
        bits: u8,
    }
    impl PARTNBLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:7 - Part Number Low"]
        #[inline(always)]
        pub fn partnbl(&self) -> PARTNBLR {
            let bits = {
                const MASK: u8 = 255;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PARTNBLR { bits }
        }
    }
}
#[doc = "Peripheral Identification 1"]
pub struct PID1 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Identification 1"]
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
    #[doc = r" Value of the field"]
    pub struct PARTNBHR {
        bits: u8,
    }
    impl PARTNBHR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct JEPIDCLR {
        bits: u8,
    }
    impl JEPIDCLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:3 - Part Number High"]
        #[inline(always)]
        pub fn partnbh(&self) -> PARTNBHR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            PARTNBHR { bits }
        }
        #[doc = "Bits 4:7 - Low part of the JEP-106 Identity Code"]
        #[inline(always)]
        pub fn jepidcl(&self) -> JEPIDCLR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            JEPIDCLR { bits }
        }
    }
}
#[doc = "Peripheral Identification 2"]
pub struct PID2 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Identification 2"]
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
    #[doc = r" Value of the field"]
    pub struct JEPIDCHR {
        bits: u8,
    }
    impl JEPIDCHR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct JEPUR {
        bits: bool,
    }
    impl JEPUR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct REVISIONR {
        bits: u8,
    }
    impl REVISIONR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:2 - JEP-106 Identity Code High"]
        #[inline(always)]
        pub fn jepidch(&self) -> JEPIDCHR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            JEPIDCHR { bits }
        }
        #[doc = "Bit 3 - JEP-106 Identity Code is used"]
        #[inline(always)]
        pub fn jepu(&self) -> JEPUR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            JEPUR { bits }
        }
        #[doc = "Bits 4:7 - Revision Number"]
        #[inline(always)]
        pub fn revision(&self) -> REVISIONR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            REVISIONR { bits }
        }
    }
}
#[doc = "Peripheral Identification 3"]
pub struct PID3 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Identification 3"]
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
    #[doc = r" Value of the field"]
    pub struct CUSMODR {
        bits: u8,
    }
    impl CUSMODR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct REVANDR {
        bits: u8,
    }
    impl REVANDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:3 - ARM CUSMOD"]
        #[inline(always)]
        pub fn cusmod(&self) -> CUSMODR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CUSMODR { bits }
        }
        #[doc = "Bits 4:7 - Revision Number"]
        #[inline(always)]
        pub fn revand(&self) -> REVANDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            REVANDR { bits }
        }
    }
}
#[doc = "Peripheral Identification 4"]
pub struct PID4 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Identification 4"]
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
    #[doc = r" Value of the field"]
    pub struct JEPCCR {
        bits: u8,
    }
    impl JEPCCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct FKBCR {
        bits: u8,
    }
    impl FKBCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:3 - JEP-106 Continuation Code"]
        #[inline(always)]
        pub fn jepcc(&self) -> JEPCCR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            JEPCCR { bits }
        }
        #[doc = "Bits 4:7 - 4KB Count"]
        #[inline(always)]
        pub fn fkbc(&self) -> FKBCR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            FKBCR { bits }
        }
    }
}
#[doc = "Status A"]
pub struct STATUSA {
    register: VolatileCell<u8>,
}
#[doc = "Status A"]
pub mod statusa {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::STATUSA {
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
    pub struct DONER {
        bits: bool,
    }
    impl DONER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CRSTEXTR {
        bits: bool,
    }
    impl CRSTEXTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BERRR {
        bits: bool,
    }
    impl BERRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct FAILR {
        bits: bool,
    }
    impl FAILR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PERRR {
        bits: bool,
    }
    impl PERRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _DONEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DONEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CRSTEXTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRSTEXTW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _BERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BERRW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _FAILW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FAILW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    #[doc = r" Proxy"]
    pub struct _PERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PERRW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Done"]
        #[inline(always)]
        pub fn done(&self) -> DONER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            DONER { bits }
        }
        #[doc = "Bit 1 - CPU Reset Phase Extension"]
        #[inline(always)]
        pub fn crstext(&self) -> CRSTEXTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CRSTEXTR { bits }
        }
        #[doc = "Bit 2 - Bus Error"]
        #[inline(always)]
        pub fn berr(&self) -> BERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            BERRR { bits }
        }
        #[doc = "Bit 3 - Failure"]
        #[inline(always)]
        pub fn fail(&self) -> FAILR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            FAILR { bits }
        }
        #[doc = "Bit 4 - Protection Error"]
        #[inline(always)]
        pub fn perr(&self) -> PERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            PERRR { bits }
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
        #[doc = "Bit 0 - Done"]
        #[inline(always)]
        pub fn done(&mut self) -> _DONEW {
            _DONEW { w: self }
        }
        #[doc = "Bit 1 - CPU Reset Phase Extension"]
        #[inline(always)]
        pub fn crstext(&mut self) -> _CRSTEXTW {
            _CRSTEXTW { w: self }
        }
        #[doc = "Bit 2 - Bus Error"]
        #[inline(always)]
        pub fn berr(&mut self) -> _BERRW {
            _BERRW { w: self }
        }
        #[doc = "Bit 3 - Failure"]
        #[inline(always)]
        pub fn fail(&mut self) -> _FAILW {
            _FAILW { w: self }
        }
        #[doc = "Bit 4 - Protection Error"]
        #[inline(always)]
        pub fn perr(&mut self) -> _PERRW {
            _PERRW { w: self }
        }
    }
}
#[doc = "Status B"]
pub struct STATUSB {
    register: VolatileCell<u8>,
}
#[doc = "Status B"]
pub mod statusb {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    impl super::STATUSB {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct PROTR {
        bits: bool,
    }
    impl PROTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct DBGPRESR {
        bits: bool,
    }
    impl DBGPRESR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct DCCD0R {
        bits: bool,
    }
    impl DCCD0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct DCCD1R {
        bits: bool,
    }
    impl DCCD1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct HPER {
        bits: bool,
    }
    impl HPER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
        #[doc = "Bit 0 - Protected"]
        #[inline(always)]
        pub fn prot(&self) -> PROTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            PROTR { bits }
        }
        #[doc = "Bit 1 - Debugger Present"]
        #[inline(always)]
        pub fn dbgpres(&self) -> DBGPRESR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            DBGPRESR { bits }
        }
        #[doc = "Bit 2 - Debug Communication Channel 0 Dirty"]
        #[inline(always)]
        pub fn dccd0(&self) -> DCCD0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            DCCD0R { bits }
        }
        #[doc = "Bit 3 - Debug Communication Channel 1 Dirty"]
        #[inline(always)]
        pub fn dccd1(&self) -> DCCD1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            DCCD1R { bits }
        }
        #[doc = "Bit 4 - Hot-Plugging Enable"]
        #[inline(always)]
        pub fn hpe(&self) -> HPER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            HPER { bits }
        }
    }
}
