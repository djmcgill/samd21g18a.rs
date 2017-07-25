use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x02 - CRC Control"]
    pub crcctrl: CRCCTRL,
    #[doc = "0x04 - CRC Data Input"]
    pub crcdatain: CRCDATAIN,
    #[doc = "0x08 - CRC Checksum"]
    pub crcchksum: CRCCHKSUM,
    #[doc = "0x0c - CRC Status"]
    pub crcstatus: CRCSTATUS,
    #[doc = "0x0d - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved0: [u8; 2usize],
    #[doc = "0x10 - Software Trigger Control"]
    pub swtrigctrl: SWTRIGCTRL,
    #[doc = "0x14 - Priority Control 0"]
    pub prictrl0: PRICTRL0,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - Interrupt Pending"]
    pub intpend: INTPEND,
    _reserved2: [u8; 2usize],
    #[doc = "0x24 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x28 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x2c - Pending Channels"]
    pub pendch: PENDCH,
    #[doc = "0x30 - Active Channel and Levels"]
    pub active: ACTIVE,
    #[doc = "0x34 - Descriptor Memory Section Base Address"]
    pub baseaddr: BASEADDR,
    #[doc = "0x38 - Write-Back Memory Section Base Address"]
    pub wrbaddr: WRBADDR,
    _reserved3: [u8; 3usize],
    #[doc = "0x3f - Channel ID"]
    pub chid: CHID,
    #[doc = "0x40 - Channel Control A"]
    pub chctrla: CHCTRLA,
    _reserved4: [u8; 3usize],
    #[doc = "0x44 - Channel Control B"]
    pub chctrlb: CHCTRLB,
    _reserved5: [u8; 4usize],
    #[doc = "0x4c - Channel Interrupt Enable Clear"]
    pub chintenclr: CHINTENCLR,
    #[doc = "0x4d - Channel Interrupt Enable Set"]
    pub chintenset: CHINTENSET,
    #[doc = "0x4e - Channel Interrupt Flag Status and Clear"]
    pub chintflag: CHINTFLAG,
    #[doc = "0x4f - Channel Status"]
    pub chstatus: CHSTATUS,
}
#[doc = "Active Channel and Levels"]
pub struct ACTIVE {
    register: VolatileCell<u32>,
}
#[doc = "Active Channel and Levels"]
pub mod active {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::ACTIVE {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLEX0R {
        bits: bool,
    }
    impl LVLEX0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLEX1R {
        bits: bool,
    }
    impl LVLEX1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLEX2R {
        bits: bool,
    }
    impl LVLEX2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLEX3R {
        bits: bool,
    }
    impl LVLEX3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
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
    pub struct ABUSYR {
        bits: bool,
    }
    impl ABUSYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BTCNTR {
        bits: u16,
    }
    impl BTCNTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - Level 0 Channel Trigger Request Executing"]
        #[inline(always)]
        pub fn lvlex0(&self) -> LVLEX0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LVLEX0R { bits }
        }
        #[doc = "Bit 1 - Level 1 Channel Trigger Request Executing"]
        #[inline(always)]
        pub fn lvlex1(&self) -> LVLEX1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LVLEX1R { bits }
        }
        #[doc = "Bit 2 - Level 2 Channel Trigger Request Executing"]
        #[inline(always)]
        pub fn lvlex2(&self) -> LVLEX2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LVLEX2R { bits }
        }
        #[doc = "Bit 3 - Level 3 Channel Trigger Request Executing"]
        #[inline(always)]
        pub fn lvlex3(&self) -> LVLEX3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LVLEX3R { bits }
        }
        #[doc = "Bits 8:12 - Active Channel ID"]
        #[inline(always)]
        pub fn id(&self) -> IDR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            IDR { bits }
        }
        #[doc = "Bit 15 - Active Channel Busy"]
        #[inline(always)]
        pub fn abusy(&self) -> ABUSYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ABUSYR { bits }
        }
        #[doc = "Bits 16:31 - Active Channel Block Transfer Count"]
        #[inline(always)]
        pub fn btcnt(&self) -> BTCNTR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            BTCNTR { bits }
        }
    }
}
#[doc = "Descriptor Memory Section Base Address"]
pub struct BASEADDR {
    register: VolatileCell<u32>,
}
#[doc = "Descriptor Memory Section Base Address"]
pub mod baseaddr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::BASEADDR {
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
    pub struct BASEADDRR {
        bits: u32,
    }
    impl BASEADDRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _BASEADDRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BASEADDRW<'a> {
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
        #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
        #[inline(always)]
        pub fn baseaddr(&self) -> BASEADDRR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            BASEADDRR { bits }
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
        #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
        #[inline(always)]
        pub fn baseaddr(&mut self) -> _BASEADDRW {
            _BASEADDRW { w: self }
        }
    }
}
#[doc = "Busy Channels"]
pub struct BUSYCH {
    register: VolatileCell<u32>,
}
#[doc = "Busy Channels"]
pub mod busych {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::BUSYCH {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH0R {
        bits: bool,
    }
    impl BUSYCH0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH1R {
        bits: bool,
    }
    impl BUSYCH1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH2R {
        bits: bool,
    }
    impl BUSYCH2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH3R {
        bits: bool,
    }
    impl BUSYCH3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH4R {
        bits: bool,
    }
    impl BUSYCH4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH5R {
        bits: bool,
    }
    impl BUSYCH5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH6R {
        bits: bool,
    }
    impl BUSYCH6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH7R {
        bits: bool,
    }
    impl BUSYCH7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH8R {
        bits: bool,
    }
    impl BUSYCH8R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH9R {
        bits: bool,
    }
    impl BUSYCH9R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH10R {
        bits: bool,
    }
    impl BUSYCH10R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYCH11R {
        bits: bool,
    }
    impl BUSYCH11R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
        #[doc = "Bit 0 - Busy Channel 0"]
        #[inline(always)]
        pub fn busych0(&self) -> BUSYCH0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH0R { bits }
        }
        #[doc = "Bit 1 - Busy Channel 1"]
        #[inline(always)]
        pub fn busych1(&self) -> BUSYCH1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH1R { bits }
        }
        #[doc = "Bit 2 - Busy Channel 2"]
        #[inline(always)]
        pub fn busych2(&self) -> BUSYCH2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH2R { bits }
        }
        #[doc = "Bit 3 - Busy Channel 3"]
        #[inline(always)]
        pub fn busych3(&self) -> BUSYCH3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH3R { bits }
        }
        #[doc = "Bit 4 - Busy Channel 4"]
        #[inline(always)]
        pub fn busych4(&self) -> BUSYCH4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH4R { bits }
        }
        #[doc = "Bit 5 - Busy Channel 5"]
        #[inline(always)]
        pub fn busych5(&self) -> BUSYCH5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH5R { bits }
        }
        #[doc = "Bit 6 - Busy Channel 6"]
        #[inline(always)]
        pub fn busych6(&self) -> BUSYCH6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH6R { bits }
        }
        #[doc = "Bit 7 - Busy Channel 7"]
        #[inline(always)]
        pub fn busych7(&self) -> BUSYCH7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH7R { bits }
        }
        #[doc = "Bit 8 - Busy Channel 8"]
        #[inline(always)]
        pub fn busych8(&self) -> BUSYCH8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH8R { bits }
        }
        #[doc = "Bit 9 - Busy Channel 9"]
        #[inline(always)]
        pub fn busych9(&self) -> BUSYCH9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH9R { bits }
        }
        #[doc = "Bit 10 - Busy Channel 10"]
        #[inline(always)]
        pub fn busych10(&self) -> BUSYCH10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH10R { bits }
        }
        #[doc = "Bit 11 - Busy Channel 11"]
        #[inline(always)]
        pub fn busych11(&self) -> BUSYCH11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BUSYCH11R { bits }
        }
    }
}
#[doc = "Channel Control A"]
pub struct CHCTRLA {
    register: VolatileCell<u8>,
}
#[doc = "Channel Control A"]
pub mod chctrla {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CHCTRLA {
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
        #[doc = "Bit 0 - Channel Software Reset"]
        #[inline(always)]
        pub fn swrst(&self) -> SWRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SWRSTR { bits }
        }
        #[doc = "Bit 1 - Channel Enable"]
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
        #[doc = "Bit 0 - Channel Software Reset"]
        #[inline(always)]
        pub fn swrst(&mut self) -> _SWRSTW {
            _SWRSTW { w: self }
        }
        #[doc = "Bit 1 - Channel Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
    }
}
#[doc = "Channel Control B"]
pub struct CHCTRLB {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control B"]
pub mod chctrlb {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::CHCTRLB {
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
    #[doc = "Possible values of the field `EVACT`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EVACTR {
        #[doc = "No action"]
        NOACT,
        #[doc = "Transfer and periodic transfer trigger"]
        TRIG,
        #[doc = "Conditional transfer trigger"]
        CTRIG,
        #[doc = "Conditional block transfer"]
        CBLOCK,
        #[doc = "Channel suspend operation"]
        SUSPEND,
        #[doc = "Channel resume operation"]
        RESUME,
        #[doc = "Skip next block suspend action"]
        SSKIP,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl EVACTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                EVACTR::NOACT => 0,
                EVACTR::TRIG => 1,
                EVACTR::CTRIG => 2,
                EVACTR::CBLOCK => 3,
                EVACTR::SUSPEND => 4,
                EVACTR::RESUME => 5,
                EVACTR::SSKIP => 6,
                EVACTR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> EVACTR {
            match value {
                0 => EVACTR::NOACT,
                1 => EVACTR::TRIG,
                2 => EVACTR::CTRIG,
                3 => EVACTR::CBLOCK,
                4 => EVACTR::SUSPEND,
                5 => EVACTR::RESUME,
                6 => EVACTR::SSKIP,
                i => EVACTR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NOACT`"]
        #[inline(always)]
        pub fn is_noact(&self) -> bool {
            *self == EVACTR::NOACT
        }
        #[doc = "Checks if the value of the field is `TRIG`"]
        #[inline(always)]
        pub fn is_trig(&self) -> bool {
            *self == EVACTR::TRIG
        }
        #[doc = "Checks if the value of the field is `CTRIG`"]
        #[inline(always)]
        pub fn is_ctrig(&self) -> bool {
            *self == EVACTR::CTRIG
        }
        #[doc = "Checks if the value of the field is `CBLOCK`"]
        #[inline(always)]
        pub fn is_cblock(&self) -> bool {
            *self == EVACTR::CBLOCK
        }
        #[doc = "Checks if the value of the field is `SUSPEND`"]
        #[inline(always)]
        pub fn is_suspend(&self) -> bool {
            *self == EVACTR::SUSPEND
        }
        #[doc = "Checks if the value of the field is `RESUME`"]
        #[inline(always)]
        pub fn is_resume(&self) -> bool {
            *self == EVACTR::RESUME
        }
        #[doc = "Checks if the value of the field is `SSKIP`"]
        #[inline(always)]
        pub fn is_sskip(&self) -> bool {
            *self == EVACTR::SSKIP
        }
    }
    #[doc = r" Value of the field"]
    pub struct EVIER {
        bits: bool,
    }
    impl EVIER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct EVOER {
        bits: bool,
    }
    impl EVOER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLR {
        bits: u8,
    }
    impl LVLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = "Possible values of the field `TRIGSRC`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum TRIGSRCR {
        #[doc = "Only software/event triggers"]
        DISABLE,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl TRIGSRCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                TRIGSRCR::DISABLE => 0,
                TRIGSRCR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> TRIGSRCR {
            match value {
                0 => TRIGSRCR::DISABLE,
                i => TRIGSRCR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `DISABLE`"]
        #[inline(always)]
        pub fn is_disable(&self) -> bool {
            *self == TRIGSRCR::DISABLE
        }
    }
    #[doc = "Possible values of the field `TRIGACT`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum TRIGACTR {
        #[doc = "One trigger required for each block transfer"]
        BLOCK,
        #[doc = "One trigger required for each beat transfer"]
        BEAT,
        #[doc = "One trigger required for each transaction"]
        TRANSACTION,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl TRIGACTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                TRIGACTR::BLOCK => 0,
                TRIGACTR::BEAT => 2,
                TRIGACTR::TRANSACTION => 3,
                TRIGACTR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> TRIGACTR {
            match value {
                0 => TRIGACTR::BLOCK,
                2 => TRIGACTR::BEAT,
                3 => TRIGACTR::TRANSACTION,
                i => TRIGACTR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `BLOCK`"]
        #[inline(always)]
        pub fn is_block(&self) -> bool {
            *self == TRIGACTR::BLOCK
        }
        #[doc = "Checks if the value of the field is `BEAT`"]
        #[inline(always)]
        pub fn is_beat(&self) -> bool {
            *self == TRIGACTR::BEAT
        }
        #[doc = "Checks if the value of the field is `TRANSACTION`"]
        #[inline(always)]
        pub fn is_transaction(&self) -> bool {
            *self == TRIGACTR::TRANSACTION
        }
    }
    #[doc = "Possible values of the field `CMD`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CMDR {
        #[doc = "No action"]
        NOACT,
        #[doc = "Channel suspend operation"]
        SUSPEND,
        #[doc = "Channel resume operation"]
        RESUME,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CMDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CMDR::NOACT => 0,
                CMDR::SUSPEND => 1,
                CMDR::RESUME => 2,
                CMDR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CMDR {
            match value {
                0 => CMDR::NOACT,
                1 => CMDR::SUSPEND,
                2 => CMDR::RESUME,
                i => CMDR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NOACT`"]
        #[inline(always)]
        pub fn is_noact(&self) -> bool {
            *self == CMDR::NOACT
        }
        #[doc = "Checks if the value of the field is `SUSPEND`"]
        #[inline(always)]
        pub fn is_suspend(&self) -> bool {
            *self == CMDR::SUSPEND
        }
        #[doc = "Checks if the value of the field is `RESUME`"]
        #[inline(always)]
        pub fn is_resume(&self) -> bool {
            *self == CMDR::RESUME
        }
    }
    #[doc = "Values that can be written to the field `EVACT`"]
    pub enum EVACTW {
        #[doc = "No action"]
        NOACT,
        #[doc = "Transfer and periodic transfer trigger"]
        TRIG,
        #[doc = "Conditional transfer trigger"]
        CTRIG,
        #[doc = "Conditional block transfer"]
        CBLOCK,
        #[doc = "Channel suspend operation"]
        SUSPEND,
        #[doc = "Channel resume operation"]
        RESUME,
        #[doc = "Skip next block suspend action"]
        SSKIP,
    }
    impl EVACTW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                EVACTW::NOACT => 0,
                EVACTW::TRIG => 1,
                EVACTW::CTRIG => 2,
                EVACTW::CBLOCK => 3,
                EVACTW::SUSPEND => 4,
                EVACTW::RESUME => 5,
                EVACTW::SSKIP => 6,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _EVACTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVACTW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: EVACTW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No action"]
        #[inline(always)]
        pub fn noact(self) -> &'a mut W {
            self.variant(EVACTW::NOACT)
        }
        #[doc = "Transfer and periodic transfer trigger"]
        #[inline(always)]
        pub fn trig(self) -> &'a mut W {
            self.variant(EVACTW::TRIG)
        }
        #[doc = "Conditional transfer trigger"]
        #[inline(always)]
        pub fn ctrig(self) -> &'a mut W {
            self.variant(EVACTW::CTRIG)
        }
        #[doc = "Conditional block transfer"]
        #[inline(always)]
        pub fn cblock(self) -> &'a mut W {
            self.variant(EVACTW::CBLOCK)
        }
        #[doc = "Channel suspend operation"]
        #[inline(always)]
        pub fn suspend(self) -> &'a mut W {
            self.variant(EVACTW::SUSPEND)
        }
        #[doc = "Channel resume operation"]
        #[inline(always)]
        pub fn resume(self) -> &'a mut W {
            self.variant(EVACTW::RESUME)
        }
        #[doc = "Skip next block suspend action"]
        #[inline(always)]
        pub fn sskip(self) -> &'a mut W {
            self.variant(EVACTW::SSKIP)
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
    pub struct _EVIEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVIEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _EVOEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EVOEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LVLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LVLW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `TRIGSRC`"]
    pub enum TRIGSRCW {
        #[doc = "Only software/event triggers"]
        DISABLE,
    }
    impl TRIGSRCW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                TRIGSRCW::DISABLE => 0,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _TRIGSRCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TRIGSRCW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: TRIGSRCW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Only software/event triggers"]
        #[inline(always)]
        pub fn disable(self) -> &'a mut W {
            self.variant(TRIGSRCW::DISABLE)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `TRIGACT`"]
    pub enum TRIGACTW {
        #[doc = "One trigger required for each block transfer"]
        BLOCK,
        #[doc = "One trigger required for each beat transfer"]
        BEAT,
        #[doc = "One trigger required for each transaction"]
        TRANSACTION,
    }
    impl TRIGACTW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                TRIGACTW::BLOCK => 0,
                TRIGACTW::BEAT => 2,
                TRIGACTW::TRANSACTION => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _TRIGACTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TRIGACTW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: TRIGACTW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "One trigger required for each block transfer"]
        #[inline(always)]
        pub fn block(self) -> &'a mut W {
            self.variant(TRIGACTW::BLOCK)
        }
        #[doc = "One trigger required for each beat transfer"]
        #[inline(always)]
        pub fn beat(self) -> &'a mut W {
            self.variant(TRIGACTW::BEAT)
        }
        #[doc = "One trigger required for each transaction"]
        #[inline(always)]
        pub fn transaction(self) -> &'a mut W {
            self.variant(TRIGACTW::TRANSACTION)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CMD`"]
    pub enum CMDW {
        #[doc = "No action"]
        NOACT,
        #[doc = "Channel suspend operation"]
        SUSPEND,
        #[doc = "Channel resume operation"]
        RESUME,
    }
    impl CMDW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CMDW::NOACT => 0,
                CMDW::SUSPEND => 1,
                CMDW::RESUME => 2,
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
        #[doc = "No action"]
        #[inline(always)]
        pub fn noact(self) -> &'a mut W {
            self.variant(CMDW::NOACT)
        }
        #[doc = "Channel suspend operation"]
        #[inline(always)]
        pub fn suspend(self) -> &'a mut W {
            self.variant(CMDW::SUSPEND)
        }
        #[doc = "Channel resume operation"]
        #[inline(always)]
        pub fn resume(self) -> &'a mut W {
            self.variant(CMDW::RESUME)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:2 - Event Input Action"]
        #[inline(always)]
        pub fn evact(&self) -> EVACTR {
            EVACTR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 3 - Channel Event Input Enable"]
        #[inline(always)]
        pub fn evie(&self) -> EVIER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVIER { bits }
        }
        #[doc = "Bit 4 - Channel Event Output Enable"]
        #[inline(always)]
        pub fn evoe(&self) -> EVOER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EVOER { bits }
        }
        #[doc = "Bits 5:6 - Channel Arbitration Level"]
        #[inline(always)]
        pub fn lvl(&self) -> LVLR {
            let bits = {
                const MASK: u8 = 3;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            LVLR { bits }
        }
        #[doc = "Bits 8:13 - Peripheral Trigger Source"]
        #[inline(always)]
        pub fn trigsrc(&self) -> TRIGSRCR {
            TRIGSRCR::_from({
                const MASK: u8 = 63;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 22:23 - Trigger Action"]
        #[inline(always)]
        pub fn trigact(&self) -> TRIGACTR {
            TRIGACTR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 24:25 - Software Command"]
        #[inline(always)]
        pub fn cmd(&self) -> CMDR {
            CMDR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 24;
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
        #[doc = "Bits 0:2 - Event Input Action"]
        #[inline(always)]
        pub fn evact(&mut self) -> _EVACTW {
            _EVACTW { w: self }
        }
        #[doc = "Bit 3 - Channel Event Input Enable"]
        #[inline(always)]
        pub fn evie(&mut self) -> _EVIEW {
            _EVIEW { w: self }
        }
        #[doc = "Bit 4 - Channel Event Output Enable"]
        #[inline(always)]
        pub fn evoe(&mut self) -> _EVOEW {
            _EVOEW { w: self }
        }
        #[doc = "Bits 5:6 - Channel Arbitration Level"]
        #[inline(always)]
        pub fn lvl(&mut self) -> _LVLW {
            _LVLW { w: self }
        }
        #[doc = "Bits 8:13 - Peripheral Trigger Source"]
        #[inline(always)]
        pub fn trigsrc(&mut self) -> _TRIGSRCW {
            _TRIGSRCW { w: self }
        }
        #[doc = "Bits 22:23 - Trigger Action"]
        #[inline(always)]
        pub fn trigact(&mut self) -> _TRIGACTW {
            _TRIGACTW { w: self }
        }
        #[doc = "Bits 24:25 - Software Command"]
        #[inline(always)]
        pub fn cmd(&mut self) -> _CMDW {
            _CMDW { w: self }
        }
    }
}
#[doc = "Channel ID"]
pub struct CHID {
    register: VolatileCell<u8>,
}
#[doc = "Channel ID"]
pub mod chid {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CHID {
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
        #[doc = "Bits 0:3 - Channel ID"]
        #[inline(always)]
        pub fn id(&self) -> IDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            };
            IDR { bits }
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
        #[doc = "Bits 0:3 - Channel ID"]
        #[inline(always)]
        pub fn id(&mut self) -> _IDW {
            _IDW { w: self }
        }
    }
}
#[doc = "Channel Interrupt Enable Clear"]
pub struct CHINTENCLR {
    register: VolatileCell<u8>,
}
#[doc = "Channel Interrupt Enable Clear"]
pub mod chintenclr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CHINTENCLR {
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
    pub struct TERRR {
        bits: bool,
    }
    impl TERRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TCMPLR {
        bits: bool,
    }
    impl TCMPLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SUSPR {
        bits: bool,
    }
    impl SUSPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _TERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TERRW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TCMPLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCMPLW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SUSPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SUSPW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Transfer Error Interrupt Enable"]
        #[inline(always)]
        pub fn terr(&self) -> TERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            TERRR { bits }
        }
        #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
        #[inline(always)]
        pub fn tcmpl(&self) -> TCMPLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            TCMPLR { bits }
        }
        #[doc = "Bit 2 - Channel Suspend Interrupt Enable"]
        #[inline(always)]
        pub fn susp(&self) -> SUSPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SUSPR { bits }
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
        #[doc = "Bit 0 - Transfer Error Interrupt Enable"]
        #[inline(always)]
        pub fn terr(&mut self) -> _TERRW {
            _TERRW { w: self }
        }
        #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
        #[inline(always)]
        pub fn tcmpl(&mut self) -> _TCMPLW {
            _TCMPLW { w: self }
        }
        #[doc = "Bit 2 - Channel Suspend Interrupt Enable"]
        #[inline(always)]
        pub fn susp(&mut self) -> _SUSPW {
            _SUSPW { w: self }
        }
    }
}
#[doc = "Channel Interrupt Enable Set"]
pub struct CHINTENSET {
    register: VolatileCell<u8>,
}
#[doc = "Channel Interrupt Enable Set"]
pub mod chintenset {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CHINTENSET {
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
    pub struct TERRR {
        bits: bool,
    }
    impl TERRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TCMPLR {
        bits: bool,
    }
    impl TCMPLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SUSPR {
        bits: bool,
    }
    impl SUSPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _TERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TERRW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TCMPLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCMPLW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SUSPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SUSPW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Transfer Error Interrupt Enable"]
        #[inline(always)]
        pub fn terr(&self) -> TERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            TERRR { bits }
        }
        #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
        #[inline(always)]
        pub fn tcmpl(&self) -> TCMPLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            TCMPLR { bits }
        }
        #[doc = "Bit 2 - Channel Suspend Interrupt Enable"]
        #[inline(always)]
        pub fn susp(&self) -> SUSPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SUSPR { bits }
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
        #[doc = "Bit 0 - Transfer Error Interrupt Enable"]
        #[inline(always)]
        pub fn terr(&mut self) -> _TERRW {
            _TERRW { w: self }
        }
        #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
        #[inline(always)]
        pub fn tcmpl(&mut self) -> _TCMPLW {
            _TCMPLW { w: self }
        }
        #[doc = "Bit 2 - Channel Suspend Interrupt Enable"]
        #[inline(always)]
        pub fn susp(&mut self) -> _SUSPW {
            _SUSPW { w: self }
        }
    }
}
#[doc = "Channel Interrupt Flag Status and Clear"]
pub struct CHINTFLAG {
    register: VolatileCell<u8>,
}
#[doc = "Channel Interrupt Flag Status and Clear"]
pub mod chintflag {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CHINTFLAG {
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
    pub struct TERRR {
        bits: bool,
    }
    impl TERRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TCMPLR {
        bits: bool,
    }
    impl TCMPLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SUSPR {
        bits: bool,
    }
    impl SUSPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _TERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TERRW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TCMPLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCMPLW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SUSPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SUSPW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Transfer Error"]
        #[inline(always)]
        pub fn terr(&self) -> TERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            TERRR { bits }
        }
        #[doc = "Bit 1 - Transfer Complete"]
        #[inline(always)]
        pub fn tcmpl(&self) -> TCMPLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            TCMPLR { bits }
        }
        #[doc = "Bit 2 - Channel Suspend"]
        #[inline(always)]
        pub fn susp(&self) -> SUSPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SUSPR { bits }
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
        #[doc = "Bit 0 - Transfer Error"]
        #[inline(always)]
        pub fn terr(&mut self) -> _TERRW {
            _TERRW { w: self }
        }
        #[doc = "Bit 1 - Transfer Complete"]
        #[inline(always)]
        pub fn tcmpl(&mut self) -> _TCMPLW {
            _TCMPLW { w: self }
        }
        #[doc = "Bit 2 - Channel Suspend"]
        #[inline(always)]
        pub fn susp(&mut self) -> _SUSPW {
            _SUSPW { w: self }
        }
    }
}
#[doc = "Channel Status"]
pub struct CHSTATUS {
    register: VolatileCell<u8>,
}
#[doc = "Channel Status"]
pub mod chstatus {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    impl super::CHSTATUS {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDR {
        bits: bool,
    }
    impl PENDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYR {
        bits: bool,
    }
    impl BUSYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct FERRR {
        bits: bool,
    }
    impl FERRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
        #[doc = "Bit 0 - Channel Pending"]
        #[inline(always)]
        pub fn pend(&self) -> PENDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            PENDR { bits }
        }
        #[doc = "Bit 1 - Channel Busy"]
        #[inline(always)]
        pub fn busy(&self) -> BUSYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            BUSYR { bits }
        }
        #[doc = "Bit 2 - Fetch Error"]
        #[inline(always)]
        pub fn ferr(&self) -> FERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            FERRR { bits }
        }
    }
}
#[doc = "CRC Checksum"]
pub struct CRCCHKSUM {
    register: VolatileCell<u32>,
}
#[doc = "CRC Checksum"]
pub mod crcchksum {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::CRCCHKSUM {
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
    pub struct CRCCHKSUMR {
        bits: u32,
    }
    impl CRCCHKSUMR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _CRCCHKSUMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRCCHKSUMW<'a> {
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
        #[doc = "Bits 0:31 - CRC Checksum"]
        #[inline(always)]
        pub fn crcchksum(&self) -> CRCCHKSUMR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            CRCCHKSUMR { bits }
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
        #[doc = "Bits 0:31 - CRC Checksum"]
        #[inline(always)]
        pub fn crcchksum(&mut self) -> _CRCCHKSUMW {
            _CRCCHKSUMW { w: self }
        }
    }
}
#[doc = "CRC Control"]
pub struct CRCCTRL {
    register: VolatileCell<u16>,
}
#[doc = "CRC Control"]
pub mod crcctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::CRCCTRL {
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
    #[doc = "Possible values of the field `CRCBEATSIZE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CRCBEATSIZER {
        #[doc = "Byte bus access"]
        BYTE,
        #[doc = "Half-word bus access"]
        HWORD,
        #[doc = "Word bus access"]
        WORD,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CRCBEATSIZER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CRCBEATSIZER::BYTE => 0,
                CRCBEATSIZER::HWORD => 1,
                CRCBEATSIZER::WORD => 2,
                CRCBEATSIZER::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CRCBEATSIZER {
            match value {
                0 => CRCBEATSIZER::BYTE,
                1 => CRCBEATSIZER::HWORD,
                2 => CRCBEATSIZER::WORD,
                i => CRCBEATSIZER::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `BYTE`"]
        #[inline(always)]
        pub fn is_byte(&self) -> bool {
            *self == CRCBEATSIZER::BYTE
        }
        #[doc = "Checks if the value of the field is `HWORD`"]
        #[inline(always)]
        pub fn is_hword(&self) -> bool {
            *self == CRCBEATSIZER::HWORD
        }
        #[doc = "Checks if the value of the field is `WORD`"]
        #[inline(always)]
        pub fn is_word(&self) -> bool {
            *self == CRCBEATSIZER::WORD
        }
    }
    #[doc = "Possible values of the field `CRCPOLY`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CRCPOLYR {
        #[doc = "CRC-16 (CRC-CCITT)"]
        CRC16,
        #[doc = "CRC32 (IEEE 802.3)"]
        CRC32,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CRCPOLYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CRCPOLYR::CRC16 => 0,
                CRCPOLYR::CRC32 => 1,
                CRCPOLYR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CRCPOLYR {
            match value {
                0 => CRCPOLYR::CRC16,
                1 => CRCPOLYR::CRC32,
                i => CRCPOLYR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `CRC16`"]
        #[inline(always)]
        pub fn is_crc16(&self) -> bool {
            *self == CRCPOLYR::CRC16
        }
        #[doc = "Checks if the value of the field is `CRC32`"]
        #[inline(always)]
        pub fn is_crc32(&self) -> bool {
            *self == CRCPOLYR::CRC32
        }
    }
    #[doc = "Possible values of the field `CRCSRC`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CRCSRCR {
        #[doc = "No action"]
        NOACT,
        #[doc = "I/O interface"]
        IO,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CRCSRCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CRCSRCR::NOACT => 0,
                CRCSRCR::IO => 1,
                CRCSRCR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CRCSRCR {
            match value {
                0 => CRCSRCR::NOACT,
                1 => CRCSRCR::IO,
                i => CRCSRCR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NOACT`"]
        #[inline(always)]
        pub fn is_noact(&self) -> bool {
            *self == CRCSRCR::NOACT
        }
        #[doc = "Checks if the value of the field is `IO`"]
        #[inline(always)]
        pub fn is_io(&self) -> bool {
            *self == CRCSRCR::IO
        }
    }
    #[doc = "Values that can be written to the field `CRCBEATSIZE`"]
    pub enum CRCBEATSIZEW {
        #[doc = "Byte bus access"]
        BYTE,
        #[doc = "Half-word bus access"]
        HWORD,
        #[doc = "Word bus access"]
        WORD,
    }
    impl CRCBEATSIZEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CRCBEATSIZEW::BYTE => 0,
                CRCBEATSIZEW::HWORD => 1,
                CRCBEATSIZEW::WORD => 2,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CRCBEATSIZEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRCBEATSIZEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CRCBEATSIZEW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Byte bus access"]
        #[inline(always)]
        pub fn byte(self) -> &'a mut W {
            self.variant(CRCBEATSIZEW::BYTE)
        }
        #[doc = "Half-word bus access"]
        #[inline(always)]
        pub fn hword(self) -> &'a mut W {
            self.variant(CRCBEATSIZEW::HWORD)
        }
        #[doc = "Word bus access"]
        #[inline(always)]
        pub fn word(self) -> &'a mut W {
            self.variant(CRCBEATSIZEW::WORD)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CRCPOLY`"]
    pub enum CRCPOLYW {
        #[doc = "CRC-16 (CRC-CCITT)"]
        CRC16,
        #[doc = "CRC32 (IEEE 802.3)"]
        CRC32,
    }
    impl CRCPOLYW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CRCPOLYW::CRC16 => 0,
                CRCPOLYW::CRC32 => 1,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CRCPOLYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRCPOLYW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CRCPOLYW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "CRC-16 (CRC-CCITT)"]
        #[inline(always)]
        pub fn crc16(self) -> &'a mut W {
            self.variant(CRCPOLYW::CRC16)
        }
        #[doc = "CRC32 (IEEE 802.3)"]
        #[inline(always)]
        pub fn crc32(self) -> &'a mut W {
            self.variant(CRCPOLYW::CRC32)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CRCSRC`"]
    pub enum CRCSRCW {
        #[doc = "No action"]
        NOACT,
        #[doc = "I/O interface"]
        IO,
    }
    impl CRCSRCW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CRCSRCW::NOACT => 0,
                CRCSRCW::IO => 1,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CRCSRCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRCSRCW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CRCSRCW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No action"]
        #[inline(always)]
        pub fn noact(self) -> &'a mut W {
            self.variant(CRCSRCW::NOACT)
        }
        #[doc = "I/O interface"]
        #[inline(always)]
        pub fn io(self) -> &'a mut W {
            self.variant(CRCSRCW::IO)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 63;
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
        #[doc = "Bits 0:1 - CRC Beat Size"]
        #[inline(always)]
        pub fn crcbeatsize(&self) -> CRCBEATSIZER {
            CRCBEATSIZER::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            })
        }
        #[doc = "Bits 2:3 - CRC Polynomial Type"]
        #[inline(always)]
        pub fn crcpoly(&self) -> CRCPOLYR {
            CRCPOLYR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            })
        }
        #[doc = "Bits 8:13 - CRC Input Source"]
        #[inline(always)]
        pub fn crcsrc(&self) -> CRCSRCR {
            CRCSRCR::_from({
                const MASK: u8 = 63;
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
        #[doc = "Bits 0:1 - CRC Beat Size"]
        #[inline(always)]
        pub fn crcbeatsize(&mut self) -> _CRCBEATSIZEW {
            _CRCBEATSIZEW { w: self }
        }
        #[doc = "Bits 2:3 - CRC Polynomial Type"]
        #[inline(always)]
        pub fn crcpoly(&mut self) -> _CRCPOLYW {
            _CRCPOLYW { w: self }
        }
        #[doc = "Bits 8:13 - CRC Input Source"]
        #[inline(always)]
        pub fn crcsrc(&mut self) -> _CRCSRCW {
            _CRCSRCW { w: self }
        }
    }
}
#[doc = "CRC Data Input"]
pub struct CRCDATAIN {
    register: VolatileCell<u32>,
}
#[doc = "CRC Data Input"]
pub mod crcdatain {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::CRCDATAIN {
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
    pub struct CRCDATAINR {
        bits: u32,
    }
    impl CRCDATAINR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _CRCDATAINW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRCDATAINW<'a> {
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
        #[doc = "Bits 0:31 - CRC Data Input"]
        #[inline(always)]
        pub fn crcdatain(&self) -> CRCDATAINR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            CRCDATAINR { bits }
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
        #[doc = "Bits 0:31 - CRC Data Input"]
        #[inline(always)]
        pub fn crcdatain(&mut self) -> _CRCDATAINW {
            _CRCDATAINW { w: self }
        }
    }
}
#[doc = "CRC Status"]
pub struct CRCSTATUS {
    register: VolatileCell<u8>,
}
#[doc = "CRC Status"]
pub mod crcstatus {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::CRCSTATUS {
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
    pub struct CRCBUSYR {
        bits: bool,
    }
    impl CRCBUSYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CRCZEROR {
        bits: bool,
    }
    impl CRCZEROR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _CRCBUSYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRCBUSYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - CRC Module Busy"]
        #[inline(always)]
        pub fn crcbusy(&self) -> CRCBUSYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CRCBUSYR { bits }
        }
        #[doc = "Bit 1 - CRC Zero"]
        #[inline(always)]
        pub fn crczero(&self) -> CRCZEROR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CRCZEROR { bits }
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
        #[doc = "Bit 0 - CRC Module Busy"]
        #[inline(always)]
        pub fn crcbusy(&mut self) -> _CRCBUSYW {
            _CRCBUSYW { w: self }
        }
    }
}
#[doc = "Control"]
pub struct CTRL {
    register: VolatileCell<u16>,
}
#[doc = "Control"]
pub mod ctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
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
    pub struct DMAENABLER {
        bits: bool,
    }
    impl DMAENABLER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CRCENABLER {
        bits: bool,
    }
    impl CRCENABLER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLEN0R {
        bits: bool,
    }
    impl LVLEN0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLEN1R {
        bits: bool,
    }
    impl LVLEN1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLEN2R {
        bits: bool,
    }
    impl LVLEN2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLEN3R {
        bits: bool,
    }
    impl LVLEN3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _DMAENABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMAENABLEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _CRCENABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CRCENABLEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LVLEN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LVLEN0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LVLEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LVLEN1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LVLEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LVLEN2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LVLEN3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LVLEN3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bit 0 - Software Reset"]
        #[inline(always)]
        pub fn swrst(&self) -> SWRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            SWRSTR { bits }
        }
        #[doc = "Bit 1 - DMA Enable"]
        #[inline(always)]
        pub fn dmaenable(&self) -> DMAENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            DMAENABLER { bits }
        }
        #[doc = "Bit 2 - CRC Enable"]
        #[inline(always)]
        pub fn crcenable(&self) -> CRCENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            CRCENABLER { bits }
        }
        #[doc = "Bit 8 - Priority Level 0 Enable"]
        #[inline(always)]
        pub fn lvlen0(&self) -> LVLEN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            LVLEN0R { bits }
        }
        #[doc = "Bit 9 - Priority Level 1 Enable"]
        #[inline(always)]
        pub fn lvlen1(&self) -> LVLEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            LVLEN1R { bits }
        }
        #[doc = "Bit 10 - Priority Level 2 Enable"]
        #[inline(always)]
        pub fn lvlen2(&self) -> LVLEN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            LVLEN2R { bits }
        }
        #[doc = "Bit 11 - Priority Level 3 Enable"]
        #[inline(always)]
        pub fn lvlen3(&self) -> LVLEN3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            LVLEN3R { bits }
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
        #[doc = "Bit 0 - Software Reset"]
        #[inline(always)]
        pub fn swrst(&mut self) -> _SWRSTW {
            _SWRSTW { w: self }
        }
        #[doc = "Bit 1 - DMA Enable"]
        #[inline(always)]
        pub fn dmaenable(&mut self) -> _DMAENABLEW {
            _DMAENABLEW { w: self }
        }
        #[doc = "Bit 2 - CRC Enable"]
        #[inline(always)]
        pub fn crcenable(&mut self) -> _CRCENABLEW {
            _CRCENABLEW { w: self }
        }
        #[doc = "Bit 8 - Priority Level 0 Enable"]
        #[inline(always)]
        pub fn lvlen0(&mut self) -> _LVLEN0W {
            _LVLEN0W { w: self }
        }
        #[doc = "Bit 9 - Priority Level 1 Enable"]
        #[inline(always)]
        pub fn lvlen1(&mut self) -> _LVLEN1W {
            _LVLEN1W { w: self }
        }
        #[doc = "Bit 10 - Priority Level 2 Enable"]
        #[inline(always)]
        pub fn lvlen2(&mut self) -> _LVLEN2W {
            _LVLEN2W { w: self }
        }
        #[doc = "Bit 11 - Priority Level 3 Enable"]
        #[inline(always)]
        pub fn lvlen3(&mut self) -> _LVLEN3W {
            _LVLEN3W { w: self }
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
        #[doc = "Bit 0 - Debug Run"]
        #[inline(always)]
        pub fn dbgrun(&self) -> DBGRUNR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            DBGRUNR { bits }
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
        #[doc = "Bit 0 - Debug Run"]
        #[inline(always)]
        pub fn dbgrun(&mut self) -> _DBGRUNW {
            _DBGRUNW { w: self }
        }
    }
}
#[doc = "Interrupt Pending"]
pub struct INTPEND {
    register: VolatileCell<u16>,
}
#[doc = "Interrupt Pending"]
pub mod intpend {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::INTPEND {
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
    pub struct TERRR {
        bits: bool,
    }
    impl TERRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct TCMPLR {
        bits: bool,
    }
    impl TCMPLR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SUSPR {
        bits: bool,
    }
    impl SUSPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct FERRR {
        bits: bool,
    }
    impl FERRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct BUSYR {
        bits: bool,
    }
    impl BUSYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDR {
        bits: bool,
    }
    impl PENDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _TERRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TERRW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _TCMPLW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TCMPLW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SUSPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SUSPW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bits 0:3 - Channel ID"]
        #[inline(always)]
        pub fn id(&self) -> IDR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            };
            IDR { bits }
        }
        #[doc = "Bit 8 - Transfer Error"]
        #[inline(always)]
        pub fn terr(&self) -> TERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TERRR { bits }
        }
        #[doc = "Bit 9 - Transfer Complete"]
        #[inline(always)]
        pub fn tcmpl(&self) -> TCMPLR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TCMPLR { bits }
        }
        #[doc = "Bit 10 - Channel Suspend"]
        #[inline(always)]
        pub fn susp(&self) -> SUSPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            SUSPR { bits }
        }
        #[doc = "Bit 13 - Fetch Error"]
        #[inline(always)]
        pub fn ferr(&self) -> FERRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            FERRR { bits }
        }
        #[doc = "Bit 14 - Busy"]
        #[inline(always)]
        pub fn busy(&self) -> BUSYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 14;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            BUSYR { bits }
        }
        #[doc = "Bit 15 - Pending"]
        #[inline(always)]
        pub fn pend(&self) -> PENDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PENDR { bits }
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
        #[doc = "Bits 0:3 - Channel ID"]
        #[inline(always)]
        pub fn id(&mut self) -> _IDW {
            _IDW { w: self }
        }
        #[doc = "Bit 8 - Transfer Error"]
        #[inline(always)]
        pub fn terr(&mut self) -> _TERRW {
            _TERRW { w: self }
        }
        #[doc = "Bit 9 - Transfer Complete"]
        #[inline(always)]
        pub fn tcmpl(&mut self) -> _TCMPLW {
            _TCMPLW { w: self }
        }
        #[doc = "Bit 10 - Channel Suspend"]
        #[inline(always)]
        pub fn susp(&mut self) -> _SUSPW {
            _SUSPW { w: self }
        }
    }
}
#[doc = "Interrupt Status"]
pub struct INTSTATUS {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod intstatus {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::INTSTATUS {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT0R {
        bits: bool,
    }
    impl CHINT0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT1R {
        bits: bool,
    }
    impl CHINT1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT2R {
        bits: bool,
    }
    impl CHINT2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT3R {
        bits: bool,
    }
    impl CHINT3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT4R {
        bits: bool,
    }
    impl CHINT4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT5R {
        bits: bool,
    }
    impl CHINT5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT6R {
        bits: bool,
    }
    impl CHINT6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT7R {
        bits: bool,
    }
    impl CHINT7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT8R {
        bits: bool,
    }
    impl CHINT8R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT9R {
        bits: bool,
    }
    impl CHINT9R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT10R {
        bits: bool,
    }
    impl CHINT10R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct CHINT11R {
        bits: bool,
    }
    impl CHINT11R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
        #[doc = "Bit 0 - Channel 0 Pending Interrupt"]
        #[inline(always)]
        pub fn chint0(&self) -> CHINT0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT0R { bits }
        }
        #[doc = "Bit 1 - Channel 1 Pending Interrupt"]
        #[inline(always)]
        pub fn chint1(&self) -> CHINT1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT1R { bits }
        }
        #[doc = "Bit 2 - Channel 2 Pending Interrupt"]
        #[inline(always)]
        pub fn chint2(&self) -> CHINT2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT2R { bits }
        }
        #[doc = "Bit 3 - Channel 3 Pending Interrupt"]
        #[inline(always)]
        pub fn chint3(&self) -> CHINT3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT3R { bits }
        }
        #[doc = "Bit 4 - Channel 4 Pending Interrupt"]
        #[inline(always)]
        pub fn chint4(&self) -> CHINT4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT4R { bits }
        }
        #[doc = "Bit 5 - Channel 5 Pending Interrupt"]
        #[inline(always)]
        pub fn chint5(&self) -> CHINT5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT5R { bits }
        }
        #[doc = "Bit 6 - Channel 6 Pending Interrupt"]
        #[inline(always)]
        pub fn chint6(&self) -> CHINT6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT6R { bits }
        }
        #[doc = "Bit 7 - Channel 7 Pending Interrupt"]
        #[inline(always)]
        pub fn chint7(&self) -> CHINT7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT7R { bits }
        }
        #[doc = "Bit 8 - Channel 8 Pending Interrupt"]
        #[inline(always)]
        pub fn chint8(&self) -> CHINT8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT8R { bits }
        }
        #[doc = "Bit 9 - Channel 9 Pending Interrupt"]
        #[inline(always)]
        pub fn chint9(&self) -> CHINT9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT9R { bits }
        }
        #[doc = "Bit 10 - Channel 10 Pending Interrupt"]
        #[inline(always)]
        pub fn chint10(&self) -> CHINT10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT10R { bits }
        }
        #[doc = "Bit 11 - Channel 11 Pending Interrupt"]
        #[inline(always)]
        pub fn chint11(&self) -> CHINT11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CHINT11R { bits }
        }
    }
}
#[doc = "Pending Channels"]
pub struct PENDCH {
    register: VolatileCell<u32>,
}
#[doc = "Pending Channels"]
pub mod pendch {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PENDCH {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH0R {
        bits: bool,
    }
    impl PENDCH0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH1R {
        bits: bool,
    }
    impl PENDCH1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH2R {
        bits: bool,
    }
    impl PENDCH2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH3R {
        bits: bool,
    }
    impl PENDCH3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH4R {
        bits: bool,
    }
    impl PENDCH4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH5R {
        bits: bool,
    }
    impl PENDCH5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH6R {
        bits: bool,
    }
    impl PENDCH6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH7R {
        bits: bool,
    }
    impl PENDCH7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH8R {
        bits: bool,
    }
    impl PENDCH8R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH9R {
        bits: bool,
    }
    impl PENDCH9R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH10R {
        bits: bool,
    }
    impl PENDCH10R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PENDCH11R {
        bits: bool,
    }
    impl PENDCH11R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
        #[doc = "Bit 0 - Pending Channel 0"]
        #[inline(always)]
        pub fn pendch0(&self) -> PENDCH0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH0R { bits }
        }
        #[doc = "Bit 1 - Pending Channel 1"]
        #[inline(always)]
        pub fn pendch1(&self) -> PENDCH1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH1R { bits }
        }
        #[doc = "Bit 2 - Pending Channel 2"]
        #[inline(always)]
        pub fn pendch2(&self) -> PENDCH2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH2R { bits }
        }
        #[doc = "Bit 3 - Pending Channel 3"]
        #[inline(always)]
        pub fn pendch3(&self) -> PENDCH3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH3R { bits }
        }
        #[doc = "Bit 4 - Pending Channel 4"]
        #[inline(always)]
        pub fn pendch4(&self) -> PENDCH4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH4R { bits }
        }
        #[doc = "Bit 5 - Pending Channel 5"]
        #[inline(always)]
        pub fn pendch5(&self) -> PENDCH5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH5R { bits }
        }
        #[doc = "Bit 6 - Pending Channel 6"]
        #[inline(always)]
        pub fn pendch6(&self) -> PENDCH6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH6R { bits }
        }
        #[doc = "Bit 7 - Pending Channel 7"]
        #[inline(always)]
        pub fn pendch7(&self) -> PENDCH7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH7R { bits }
        }
        #[doc = "Bit 8 - Pending Channel 8"]
        #[inline(always)]
        pub fn pendch8(&self) -> PENDCH8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH8R { bits }
        }
        #[doc = "Bit 9 - Pending Channel 9"]
        #[inline(always)]
        pub fn pendch9(&self) -> PENDCH9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH9R { bits }
        }
        #[doc = "Bit 10 - Pending Channel 10"]
        #[inline(always)]
        pub fn pendch10(&self) -> PENDCH10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH10R { bits }
        }
        #[doc = "Bit 11 - Pending Channel 11"]
        #[inline(always)]
        pub fn pendch11(&self) -> PENDCH11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            PENDCH11R { bits }
        }
    }
}
#[doc = "Priority Control 0"]
pub struct PRICTRL0 {
    register: VolatileCell<u32>,
}
#[doc = "Priority Control 0"]
pub mod prictrl0 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::PRICTRL0 {
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
    pub struct LVLPRI0R {
        bits: u8,
    }
    impl LVLPRI0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct RRLVLEN0R {
        bits: bool,
    }
    impl RRLVLEN0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLPRI1R {
        bits: u8,
    }
    impl LVLPRI1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct RRLVLEN1R {
        bits: bool,
    }
    impl RRLVLEN1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLPRI2R {
        bits: u8,
    }
    impl LVLPRI2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct RRLVLEN2R {
        bits: bool,
    }
    impl RRLVLEN2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LVLPRI3R {
        bits: u8,
    }
    impl LVLPRI3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct RRLVLEN3R {
        bits: bool,
    }
    impl RRLVLEN3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _LVLPRI0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LVLPRI0W<'a> {
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
    pub struct _RRLVLEN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RRLVLEN0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LVLPRI1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LVLPRI1W<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _RRLVLEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RRLVLEN1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LVLPRI2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LVLPRI2W<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _RRLVLEN2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RRLVLEN2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LVLPRI3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _LVLPRI3W<'a> {
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
    pub struct _RRLVLEN3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RRLVLEN3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bits 0:3 - Level 0 Channel Priority Number"]
        #[inline(always)]
        pub fn lvlpri0(&self) -> LVLPRI0R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            LVLPRI0R { bits }
        }
        #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
        #[inline(always)]
        pub fn rrlvlen0(&self) -> RRLVLEN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RRLVLEN0R { bits }
        }
        #[doc = "Bits 8:11 - Level 1 Channel Priority Number"]
        #[inline(always)]
        pub fn lvlpri1(&self) -> LVLPRI1R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            LVLPRI1R { bits }
        }
        #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
        #[inline(always)]
        pub fn rrlvlen1(&self) -> RRLVLEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RRLVLEN1R { bits }
        }
        #[doc = "Bits 16:19 - Level 2 Channel Priority Number"]
        #[inline(always)]
        pub fn lvlpri2(&self) -> LVLPRI2R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            LVLPRI2R { bits }
        }
        #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
        #[inline(always)]
        pub fn rrlvlen2(&self) -> RRLVLEN2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RRLVLEN2R { bits }
        }
        #[doc = "Bits 24:27 - Level 3 Channel Priority Number"]
        #[inline(always)]
        pub fn lvlpri3(&self) -> LVLPRI3R {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            LVLPRI3R { bits }
        }
        #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
        #[inline(always)]
        pub fn rrlvlen3(&self) -> RRLVLEN3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RRLVLEN3R { bits }
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
        #[doc = "Bits 0:3 - Level 0 Channel Priority Number"]
        #[inline(always)]
        pub fn lvlpri0(&mut self) -> _LVLPRI0W {
            _LVLPRI0W { w: self }
        }
        #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
        #[inline(always)]
        pub fn rrlvlen0(&mut self) -> _RRLVLEN0W {
            _RRLVLEN0W { w: self }
        }
        #[doc = "Bits 8:11 - Level 1 Channel Priority Number"]
        #[inline(always)]
        pub fn lvlpri1(&mut self) -> _LVLPRI1W {
            _LVLPRI1W { w: self }
        }
        #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
        #[inline(always)]
        pub fn rrlvlen1(&mut self) -> _RRLVLEN1W {
            _RRLVLEN1W { w: self }
        }
        #[doc = "Bits 16:19 - Level 2 Channel Priority Number"]
        #[inline(always)]
        pub fn lvlpri2(&mut self) -> _LVLPRI2W {
            _LVLPRI2W { w: self }
        }
        #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
        #[inline(always)]
        pub fn rrlvlen2(&mut self) -> _RRLVLEN2W {
            _RRLVLEN2W { w: self }
        }
        #[doc = "Bits 24:27 - Level 3 Channel Priority Number"]
        #[inline(always)]
        pub fn lvlpri3(&mut self) -> _LVLPRI3W {
            _LVLPRI3W { w: self }
        }
        #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
        #[inline(always)]
        pub fn rrlvlen3(&mut self) -> _RRLVLEN3W {
            _RRLVLEN3W { w: self }
        }
    }
}
#[doc = "Software Trigger Control"]
pub struct SWTRIGCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Software Trigger Control"]
pub mod swtrigctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::SWTRIGCTRL {
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
    pub struct SWTRIG0R {
        bits: bool,
    }
    impl SWTRIG0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG1R {
        bits: bool,
    }
    impl SWTRIG1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG2R {
        bits: bool,
    }
    impl SWTRIG2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG3R {
        bits: bool,
    }
    impl SWTRIG3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG4R {
        bits: bool,
    }
    impl SWTRIG4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG5R {
        bits: bool,
    }
    impl SWTRIG5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG6R {
        bits: bool,
    }
    impl SWTRIG6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG7R {
        bits: bool,
    }
    impl SWTRIG7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG8R {
        bits: bool,
    }
    impl SWTRIG8R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG9R {
        bits: bool,
    }
    impl SWTRIG9R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG10R {
        bits: bool,
    }
    impl SWTRIG10R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SWTRIG11R {
        bits: bool,
    }
    impl SWTRIG11R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _SWTRIG0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG0W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG1W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG2W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG3W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG4W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG5W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG6W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG7W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG8W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG8W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG9W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG9W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG10W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG10W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _SWTRIG11W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SWTRIG11W<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - Channel 0 Software Trigger"]
        #[inline(always)]
        pub fn swtrig0(&self) -> SWTRIG0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG0R { bits }
        }
        #[doc = "Bit 1 - Channel 1 Software Trigger"]
        #[inline(always)]
        pub fn swtrig1(&self) -> SWTRIG1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG1R { bits }
        }
        #[doc = "Bit 2 - Channel 2 Software Trigger"]
        #[inline(always)]
        pub fn swtrig2(&self) -> SWTRIG2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG2R { bits }
        }
        #[doc = "Bit 3 - Channel 3 Software Trigger"]
        #[inline(always)]
        pub fn swtrig3(&self) -> SWTRIG3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG3R { bits }
        }
        #[doc = "Bit 4 - Channel 4 Software Trigger"]
        #[inline(always)]
        pub fn swtrig4(&self) -> SWTRIG4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG4R { bits }
        }
        #[doc = "Bit 5 - Channel 5 Software Trigger"]
        #[inline(always)]
        pub fn swtrig5(&self) -> SWTRIG5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG5R { bits }
        }
        #[doc = "Bit 6 - Channel 6 Software Trigger"]
        #[inline(always)]
        pub fn swtrig6(&self) -> SWTRIG6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG6R { bits }
        }
        #[doc = "Bit 7 - Channel 7 Software Trigger"]
        #[inline(always)]
        pub fn swtrig7(&self) -> SWTRIG7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG7R { bits }
        }
        #[doc = "Bit 8 - Channel 8 Software Trigger"]
        #[inline(always)]
        pub fn swtrig8(&self) -> SWTRIG8R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG8R { bits }
        }
        #[doc = "Bit 9 - Channel 9 Software Trigger"]
        #[inline(always)]
        pub fn swtrig9(&self) -> SWTRIG9R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG9R { bits }
        }
        #[doc = "Bit 10 - Channel 10 Software Trigger"]
        #[inline(always)]
        pub fn swtrig10(&self) -> SWTRIG10R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG10R { bits }
        }
        #[doc = "Bit 11 - Channel 11 Software Trigger"]
        #[inline(always)]
        pub fn swtrig11(&self) -> SWTRIG11R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SWTRIG11R { bits }
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
        #[doc = "Bit 0 - Channel 0 Software Trigger"]
        #[inline(always)]
        pub fn swtrig0(&mut self) -> _SWTRIG0W {
            _SWTRIG0W { w: self }
        }
        #[doc = "Bit 1 - Channel 1 Software Trigger"]
        #[inline(always)]
        pub fn swtrig1(&mut self) -> _SWTRIG1W {
            _SWTRIG1W { w: self }
        }
        #[doc = "Bit 2 - Channel 2 Software Trigger"]
        #[inline(always)]
        pub fn swtrig2(&mut self) -> _SWTRIG2W {
            _SWTRIG2W { w: self }
        }
        #[doc = "Bit 3 - Channel 3 Software Trigger"]
        #[inline(always)]
        pub fn swtrig3(&mut self) -> _SWTRIG3W {
            _SWTRIG3W { w: self }
        }
        #[doc = "Bit 4 - Channel 4 Software Trigger"]
        #[inline(always)]
        pub fn swtrig4(&mut self) -> _SWTRIG4W {
            _SWTRIG4W { w: self }
        }
        #[doc = "Bit 5 - Channel 5 Software Trigger"]
        #[inline(always)]
        pub fn swtrig5(&mut self) -> _SWTRIG5W {
            _SWTRIG5W { w: self }
        }
        #[doc = "Bit 6 - Channel 6 Software Trigger"]
        #[inline(always)]
        pub fn swtrig6(&mut self) -> _SWTRIG6W {
            _SWTRIG6W { w: self }
        }
        #[doc = "Bit 7 - Channel 7 Software Trigger"]
        #[inline(always)]
        pub fn swtrig7(&mut self) -> _SWTRIG7W {
            _SWTRIG7W { w: self }
        }
        #[doc = "Bit 8 - Channel 8 Software Trigger"]
        #[inline(always)]
        pub fn swtrig8(&mut self) -> _SWTRIG8W {
            _SWTRIG8W { w: self }
        }
        #[doc = "Bit 9 - Channel 9 Software Trigger"]
        #[inline(always)]
        pub fn swtrig9(&mut self) -> _SWTRIG9W {
            _SWTRIG9W { w: self }
        }
        #[doc = "Bit 10 - Channel 10 Software Trigger"]
        #[inline(always)]
        pub fn swtrig10(&mut self) -> _SWTRIG10W {
            _SWTRIG10W { w: self }
        }
        #[doc = "Bit 11 - Channel 11 Software Trigger"]
        #[inline(always)]
        pub fn swtrig11(&mut self) -> _SWTRIG11W {
            _SWTRIG11W { w: self }
        }
    }
}
#[doc = "Write-Back Memory Section Base Address"]
pub struct WRBADDR {
    register: VolatileCell<u32>,
}
#[doc = "Write-Back Memory Section Base Address"]
pub mod wrbaddr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::WRBADDR {
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
    pub struct WRBADDRR {
        bits: u32,
    }
    impl WRBADDRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _WRBADDRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WRBADDRW<'a> {
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
        #[doc = "Bits 0:31 - Write-Back Memory Base Address"]
        #[inline(always)]
        pub fn wrbaddr(&self) -> WRBADDRR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            WRBADDRR { bits }
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
        #[doc = "Bits 0:31 - Write-Back Memory Base Address"]
        #[inline(always)]
        pub fn wrbaddr(&mut self) -> _WRBADDRW {
            _WRBADDRW { w: self }
        }
    }
}
