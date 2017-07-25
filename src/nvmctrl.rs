use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - NVM Parameter"]
    pub param: PARAM,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved1: [u8; 3usize],
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved2: [u8; 3usize],
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved3: [u8; 3usize],
    #[doc = "0x18 - Status"]
    pub status: STATUS,
    _reserved4: [u8; 2usize],
    #[doc = "0x1c - Address"]
    pub addr: ADDR,
    #[doc = "0x20 - Lock Section"]
    pub lock: LOCK,
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
            const MASK: u32 = 4194303;
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
        #[doc = "Bits 0:21 - NVM Address"]
        #[inline(always)]
        pub fn addr(&self) -> ADDRR {
            let bits = {
                const MASK: u32 = 4194303;
                const OFFSET: u8 = 0;
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
        #[doc = "Bits 0:21 - NVM Address"]
        #[inline(always)]
        pub fn addr(&mut self) -> _ADDRW {
            _ADDRW { w: self }
        }
    }
}
#[doc = "Control A"]
pub struct CTRLA {
    register: VolatileCell<u16>,
}
#[doc = "Control A"]
pub mod ctrla {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::CTRLA {
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
    #[doc = "Possible values of the field `CMD`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CMDR {
        #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
        ER,
        #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
        WP,
        #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
        EAR,
        #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
        WAP,
        #[doc = "Security Flow Command"]
        SF,
        #[doc = "Write lockbits"]
        WL,
        #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
        LR,
        #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
        UR,
        #[doc = "Sets the power reduction mode."]
        SPRM,
        #[doc = "Clears the power reduction mode."]
        CPRM,
        #[doc = "Page Buffer Clear - Clears the page buffer."]
        PBC,
        #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
        SSB,
        #[doc = "Invalidates all cache lines."]
        INVALL,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CMDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CMDR::ER => 2,
                CMDR::WP => 4,
                CMDR::EAR => 5,
                CMDR::WAP => 6,
                CMDR::SF => 10,
                CMDR::WL => 15,
                CMDR::LR => 64,
                CMDR::UR => 65,
                CMDR::SPRM => 66,
                CMDR::CPRM => 67,
                CMDR::PBC => 68,
                CMDR::SSB => 69,
                CMDR::INVALL => 70,
                CMDR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CMDR {
            match value {
                2 => CMDR::ER,
                4 => CMDR::WP,
                5 => CMDR::EAR,
                6 => CMDR::WAP,
                10 => CMDR::SF,
                15 => CMDR::WL,
                64 => CMDR::LR,
                65 => CMDR::UR,
                66 => CMDR::SPRM,
                67 => CMDR::CPRM,
                68 => CMDR::PBC,
                69 => CMDR::SSB,
                70 => CMDR::INVALL,
                i => CMDR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `ER`"]
        #[inline(always)]
        pub fn is_er(&self) -> bool {
            *self == CMDR::ER
        }
        #[doc = "Checks if the value of the field is `WP`"]
        #[inline(always)]
        pub fn is_wp(&self) -> bool {
            *self == CMDR::WP
        }
        #[doc = "Checks if the value of the field is `EAR`"]
        #[inline(always)]
        pub fn is_ear(&self) -> bool {
            *self == CMDR::EAR
        }
        #[doc = "Checks if the value of the field is `WAP`"]
        #[inline(always)]
        pub fn is_wap(&self) -> bool {
            *self == CMDR::WAP
        }
        #[doc = "Checks if the value of the field is `SF`"]
        #[inline(always)]
        pub fn is_sf(&self) -> bool {
            *self == CMDR::SF
        }
        #[doc = "Checks if the value of the field is `WL`"]
        #[inline(always)]
        pub fn is_wl(&self) -> bool {
            *self == CMDR::WL
        }
        #[doc = "Checks if the value of the field is `LR`"]
        #[inline(always)]
        pub fn is_lr(&self) -> bool {
            *self == CMDR::LR
        }
        #[doc = "Checks if the value of the field is `UR`"]
        #[inline(always)]
        pub fn is_ur(&self) -> bool {
            *self == CMDR::UR
        }
        #[doc = "Checks if the value of the field is `SPRM`"]
        #[inline(always)]
        pub fn is_sprm(&self) -> bool {
            *self == CMDR::SPRM
        }
        #[doc = "Checks if the value of the field is `CPRM`"]
        #[inline(always)]
        pub fn is_cprm(&self) -> bool {
            *self == CMDR::CPRM
        }
        #[doc = "Checks if the value of the field is `PBC`"]
        #[inline(always)]
        pub fn is_pbc(&self) -> bool {
            *self == CMDR::PBC
        }
        #[doc = "Checks if the value of the field is `SSB`"]
        #[inline(always)]
        pub fn is_ssb(&self) -> bool {
            *self == CMDR::SSB
        }
        #[doc = "Checks if the value of the field is `INVALL`"]
        #[inline(always)]
        pub fn is_invall(&self) -> bool {
            *self == CMDR::INVALL
        }
    }
    #[doc = "Possible values of the field `CMDEX`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CMDEXR {
        #[doc = "Execution Key"]
        KEY,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl CMDEXR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                CMDEXR::KEY => 165,
                CMDEXR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> CMDEXR {
            match value {
                165 => CMDEXR::KEY,
                i => CMDEXR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `KEY`"]
        #[inline(always)]
        pub fn is_key(&self) -> bool {
            *self == CMDEXR::KEY
        }
    }
    #[doc = "Values that can be written to the field `CMD`"]
    pub enum CMDW {
        #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
        ER,
        #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
        WP,
        #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
        EAR,
        #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
        WAP,
        #[doc = "Security Flow Command"]
        SF,
        #[doc = "Write lockbits"]
        WL,
        #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
        LR,
        #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
        UR,
        #[doc = "Sets the power reduction mode."]
        SPRM,
        #[doc = "Clears the power reduction mode."]
        CPRM,
        #[doc = "Page Buffer Clear - Clears the page buffer."]
        PBC,
        #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
        SSB,
        #[doc = "Invalidates all cache lines."]
        INVALL,
    }
    impl CMDW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CMDW::ER => 2,
                CMDW::WP => 4,
                CMDW::EAR => 5,
                CMDW::WAP => 6,
                CMDW::SF => 10,
                CMDW::WL => 15,
                CMDW::LR => 64,
                CMDW::UR => 65,
                CMDW::SPRM => 66,
                CMDW::CPRM => 67,
                CMDW::PBC => 68,
                CMDW::SSB => 69,
                CMDW::INVALL => 70,
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
        #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
        #[inline(always)]
        pub fn er(self) -> &'a mut W {
            self.variant(CMDW::ER)
        }
        #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
        #[inline(always)]
        pub fn wp(self) -> &'a mut W {
            self.variant(CMDW::WP)
        }
        #[doc = "Erase Auxiliary Row - Erases the auxiliary row addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
        #[inline(always)]
        pub fn ear(self) -> &'a mut W {
            self.variant(CMDW::EAR)
        }
        #[doc = "Write Auxiliary Page - Writes the contents of the page buffer to the page addressed by the ADDR register. This command can be given only when the security bit is not set and only to the user configuration row."]
        #[inline(always)]
        pub fn wap(self) -> &'a mut W {
            self.variant(CMDW::WAP)
        }
        #[doc = "Security Flow Command"]
        #[inline(always)]
        pub fn sf(self) -> &'a mut W {
            self.variant(CMDW::SF)
        }
        #[doc = "Write lockbits"]
        #[inline(always)]
        pub fn wl(self) -> &'a mut W {
            self.variant(CMDW::WL)
        }
        #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
        #[inline(always)]
        pub fn lr(self) -> &'a mut W {
            self.variant(CMDW::LR)
        }
        #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
        #[inline(always)]
        pub fn ur(self) -> &'a mut W {
            self.variant(CMDW::UR)
        }
        #[doc = "Sets the power reduction mode."]
        #[inline(always)]
        pub fn sprm(self) -> &'a mut W {
            self.variant(CMDW::SPRM)
        }
        #[doc = "Clears the power reduction mode."]
        #[inline(always)]
        pub fn cprm(self) -> &'a mut W {
            self.variant(CMDW::CPRM)
        }
        #[doc = "Page Buffer Clear - Clears the page buffer."]
        #[inline(always)]
        pub fn pbc(self) -> &'a mut W {
            self.variant(CMDW::PBC)
        }
        #[doc = "Set Security Bit - Sets the security bit by writing 0x00 to the first byte in the lockbit row."]
        #[inline(always)]
        pub fn ssb(self) -> &'a mut W {
            self.variant(CMDW::SSB)
        }
        #[doc = "Invalidates all cache lines."]
        #[inline(always)]
        pub fn invall(self) -> &'a mut W {
            self.variant(CMDW::INVALL)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `CMDEX`"]
    pub enum CMDEXW {
        #[doc = "Execution Key"]
        KEY,
    }
    impl CMDEXW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                CMDEXW::KEY => 165,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CMDEXW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CMDEXW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CMDEXW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Execution Key"]
        #[inline(always)]
        pub fn key(self) -> &'a mut W {
            self.variant(CMDEXW::KEY)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 255;
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
        #[doc = "Bits 0:6 - Command"]
        #[inline(always)]
        pub fn cmd(&self) -> CMDR {
            CMDR::_from({
                const MASK: u8 = 127;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            })
        }
        #[doc = "Bits 8:15 - Command Execution"]
        #[inline(always)]
        pub fn cmdex(&self) -> CMDEXR {
            CMDEXR::_from({
                const MASK: u8 = 255;
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
        #[doc = "Bits 0:6 - Command"]
        #[inline(always)]
        pub fn cmd(&mut self) -> _CMDW {
            _CMDW { w: self }
        }
        #[doc = "Bits 8:15 - Command Execution"]
        #[inline(always)]
        pub fn cmdex(&mut self) -> _CMDEXW {
            _CMDEXW { w: self }
        }
    }
}
#[doc = "Control B"]
pub struct CTRLB {
    register: VolatileCell<u32>,
}
#[doc = "Control B"]
pub mod ctrlb {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::CTRLB {
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
    #[doc = "Possible values of the field `RWS`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum RWSR {
        #[doc = "Single Auto Wait State"]
        SINGLE,
        #[doc = "Half Auto Wait State"]
        HALF,
        #[doc = "Dual Auto Wait State"]
        DUAL,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl RWSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                RWSR::SINGLE => 0,
                RWSR::HALF => 1,
                RWSR::DUAL => 2,
                RWSR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> RWSR {
            match value {
                0 => RWSR::SINGLE,
                1 => RWSR::HALF,
                2 => RWSR::DUAL,
                i => RWSR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `SINGLE`"]
        #[inline(always)]
        pub fn is_single(&self) -> bool {
            *self == RWSR::SINGLE
        }
        #[doc = "Checks if the value of the field is `HALF`"]
        #[inline(always)]
        pub fn is_half(&self) -> bool {
            *self == RWSR::HALF
        }
        #[doc = "Checks if the value of the field is `DUAL`"]
        #[inline(always)]
        pub fn is_dual(&self) -> bool {
            *self == RWSR::DUAL
        }
    }
    #[doc = r" Value of the field"]
    pub struct MANWR {
        bits: bool,
    }
    impl MANWR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `SLEEPPRM`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SLEEPPRMR {
        #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
        WAKEONACCESS,
        #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
        WAKEUPINSTANT,
        #[doc = "Auto power reduction disabled."]
        DISABLED,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SLEEPPRMR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SLEEPPRMR::WAKEONACCESS => 0,
                SLEEPPRMR::WAKEUPINSTANT => 1,
                SLEEPPRMR::DISABLED => 3,
                SLEEPPRMR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SLEEPPRMR {
            match value {
                0 => SLEEPPRMR::WAKEONACCESS,
                1 => SLEEPPRMR::WAKEUPINSTANT,
                3 => SLEEPPRMR::DISABLED,
                i => SLEEPPRMR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `WAKEONACCESS`"]
        #[inline(always)]
        pub fn is_wakeonaccess(&self) -> bool {
            *self == SLEEPPRMR::WAKEONACCESS
        }
        #[doc = "Checks if the value of the field is `WAKEUPINSTANT`"]
        #[inline(always)]
        pub fn is_wakeupinstant(&self) -> bool {
            *self == SLEEPPRMR::WAKEUPINSTANT
        }
        #[doc = "Checks if the value of the field is `DISABLED`"]
        #[inline(always)]
        pub fn is_disabled(&self) -> bool {
            *self == SLEEPPRMR::DISABLED
        }
    }
    #[doc = "Possible values of the field `READMODE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum READMODER {
        #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
        NO_MISS_PENALTY,
        #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
        LOW_POWER,
        #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
        DETERMINISTIC,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl READMODER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                READMODER::NO_MISS_PENALTY => 0,
                READMODER::LOW_POWER => 1,
                READMODER::DETERMINISTIC => 2,
                READMODER::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> READMODER {
            match value {
                0 => READMODER::NO_MISS_PENALTY,
                1 => READMODER::LOW_POWER,
                2 => READMODER::DETERMINISTIC,
                i => READMODER::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NO_MISS_PENALTY`"]
        #[inline(always)]
        pub fn is_no_miss_penalty(&self) -> bool {
            *self == READMODER::NO_MISS_PENALTY
        }
        #[doc = "Checks if the value of the field is `LOW_POWER`"]
        #[inline(always)]
        pub fn is_low_power(&self) -> bool {
            *self == READMODER::LOW_POWER
        }
        #[doc = "Checks if the value of the field is `DETERMINISTIC`"]
        #[inline(always)]
        pub fn is_deterministic(&self) -> bool {
            *self == READMODER::DETERMINISTIC
        }
    }
    #[doc = r" Value of the field"]
    pub struct CACHEDISR {
        bits: bool,
    }
    impl CACHEDISR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Values that can be written to the field `RWS`"]
    pub enum RWSW {
        #[doc = "Single Auto Wait State"]
        SINGLE,
        #[doc = "Half Auto Wait State"]
        HALF,
        #[doc = "Dual Auto Wait State"]
        DUAL,
    }
    impl RWSW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                RWSW::SINGLE => 0,
                RWSW::HALF => 1,
                RWSW::DUAL => 2,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _RWSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RWSW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: RWSW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Single Auto Wait State"]
        #[inline(always)]
        pub fn single(self) -> &'a mut W {
            self.variant(RWSW::SINGLE)
        }
        #[doc = "Half Auto Wait State"]
        #[inline(always)]
        pub fn half(self) -> &'a mut W {
            self.variant(RWSW::HALF)
        }
        #[doc = "Dual Auto Wait State"]
        #[inline(always)]
        pub fn dual(self) -> &'a mut W {
            self.variant(RWSW::DUAL)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MANWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MANWW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    #[doc = "Values that can be written to the field `SLEEPPRM`"]
    pub enum SLEEPPRMW {
        #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
        WAKEONACCESS,
        #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
        WAKEUPINSTANT,
        #[doc = "Auto power reduction disabled."]
        DISABLED,
    }
    impl SLEEPPRMW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SLEEPPRMW::WAKEONACCESS => 0,
                SLEEPPRMW::WAKEUPINSTANT => 1,
                SLEEPPRMW::DISABLED => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SLEEPPRMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLEEPPRMW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SLEEPPRMW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
        #[inline(always)]
        pub fn wakeonaccess(self) -> &'a mut W {
            self.variant(SLEEPPRMW::WAKEONACCESS)
        }
        #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
        #[inline(always)]
        pub fn wakeupinstant(self) -> &'a mut W {
            self.variant(SLEEPPRMW::WAKEUPINSTANT)
        }
        #[doc = "Auto power reduction disabled."]
        #[inline(always)]
        pub fn disabled(self) -> &'a mut W {
            self.variant(SLEEPPRMW::DISABLED)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `READMODE`"]
    pub enum READMODEW {
        #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
        NO_MISS_PENALTY,
        #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
        LOW_POWER,
        #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
        DETERMINISTIC,
    }
    impl READMODEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                READMODEW::NO_MISS_PENALTY => 0,
                READMODEW::LOW_POWER => 1,
                READMODEW::DETERMINISTIC => 2,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _READMODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _READMODEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: READMODEW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
        #[inline(always)]
        pub fn no_miss_penalty(self) -> &'a mut W {
            self.variant(READMODEW::NO_MISS_PENALTY)
        }
        #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
        #[inline(always)]
        pub fn low_power(self) -> &'a mut W {
            self.variant(READMODEW::LOW_POWER)
        }
        #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
        #[inline(always)]
        pub fn deterministic(self) -> &'a mut W {
            self.variant(READMODEW::DETERMINISTIC)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CACHEDISW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CACHEDISW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 1:4 - NVM Read Wait States"]
        #[inline(always)]
        pub fn rws(&self) -> RWSR {
            RWSR::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 7 - Manual Write"]
        #[inline(always)]
        pub fn manw(&self) -> MANWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MANWR { bits }
        }
        #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
        #[inline(always)]
        pub fn sleepprm(&self) -> SLEEPPRMR {
            SLEEPPRMR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
        #[inline(always)]
        pub fn readmode(&self) -> READMODER {
            READMODER::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 18 - Cache Disable"]
        #[inline(always)]
        pub fn cachedis(&self) -> CACHEDISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CACHEDISR { bits }
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
        #[doc = "Bits 1:4 - NVM Read Wait States"]
        #[inline(always)]
        pub fn rws(&mut self) -> _RWSW {
            _RWSW { w: self }
        }
        #[doc = "Bit 7 - Manual Write"]
        #[inline(always)]
        pub fn manw(&mut self) -> _MANWW {
            _MANWW { w: self }
        }
        #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
        #[inline(always)]
        pub fn sleepprm(&mut self) -> _SLEEPPRMW {
            _SLEEPPRMW { w: self }
        }
        #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
        #[inline(always)]
        pub fn readmode(&mut self) -> _READMODEW {
            _READMODEW { w: self }
        }
        #[doc = "Bit 18 - Cache Disable"]
        #[inline(always)]
        pub fn cachedis(&mut self) -> _CACHEDISW {
            _CACHEDISW { w: self }
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
    pub struct READYR {
        bits: bool,
    }
    impl READYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct ERRORR {
        bits: bool,
    }
    impl ERRORR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _READYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _READYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ERRORW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ERRORW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - NVM Ready Interrupt Enable"]
        #[inline(always)]
        pub fn ready(&self) -> READYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            READYR { bits }
        }
        #[doc = "Bit 1 - Error Interrupt Enable"]
        #[inline(always)]
        pub fn error(&self) -> ERRORR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            ERRORR { bits }
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
        #[doc = "Bit 0 - NVM Ready Interrupt Enable"]
        #[inline(always)]
        pub fn ready(&mut self) -> _READYW {
            _READYW { w: self }
        }
        #[doc = "Bit 1 - Error Interrupt Enable"]
        #[inline(always)]
        pub fn error(&mut self) -> _ERRORW {
            _ERRORW { w: self }
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
    pub struct READYR {
        bits: bool,
    }
    impl READYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct ERRORR {
        bits: bool,
    }
    impl ERRORR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _READYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _READYW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _ERRORW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ERRORW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - NVM Ready Interrupt Enable"]
        #[inline(always)]
        pub fn ready(&self) -> READYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            READYR { bits }
        }
        #[doc = "Bit 1 - Error Interrupt Enable"]
        #[inline(always)]
        pub fn error(&self) -> ERRORR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            ERRORR { bits }
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
        #[doc = "Bit 0 - NVM Ready Interrupt Enable"]
        #[inline(always)]
        pub fn ready(&mut self) -> _READYW {
            _READYW { w: self }
        }
        #[doc = "Bit 1 - Error Interrupt Enable"]
        #[inline(always)]
        pub fn error(&mut self) -> _ERRORW {
            _ERRORW { w: self }
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
    pub struct READYR {
        bits: bool,
    }
    impl READYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct ERRORR {
        bits: bool,
    }
    impl ERRORR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _ERRORW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ERRORW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - NVM Ready"]
        #[inline(always)]
        pub fn ready(&self) -> READYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            READYR { bits }
        }
        #[doc = "Bit 1 - Error"]
        #[inline(always)]
        pub fn error(&self) -> ERRORR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            ERRORR { bits }
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
        #[doc = "Bit 1 - Error"]
        #[inline(always)]
        pub fn error(&mut self) -> _ERRORW {
            _ERRORW { w: self }
        }
    }
}
#[doc = "Lock Section"]
pub struct LOCK {
    register: VolatileCell<u16>,
}
#[doc = "Lock Section"]
pub mod lock {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    impl super::LOCK {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct LOCKR {
        bits: u16,
    }
    impl LOCKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bits 0:15 - Region Lock Bits"]
        #[inline(always)]
        pub fn lock(&self) -> LOCKR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) as u16
            };
            LOCKR { bits }
        }
    }
}
#[doc = "NVM Parameter"]
pub struct PARAM {
    register: VolatileCell<u32>,
}
#[doc = "NVM Parameter"]
pub mod param {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PARAM {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct NVMPR {
        bits: u16,
    }
    impl NVMPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = "Possible values of the field `PSZ`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PSZR {
        #[doc = "8 bytes"]
        _8,
        #[doc = "16 bytes"]
        _16,
        #[doc = "32 bytes"]
        _32,
        #[doc = "64 bytes"]
        _64,
        #[doc = "128 bytes"]
        _128,
        #[doc = "256 bytes"]
        _256,
        #[doc = "512 bytes"]
        _512,
        #[doc = "1024 bytes"]
        _1024,
    }
    impl PSZR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                PSZR::_8 => 0,
                PSZR::_16 => 1,
                PSZR::_32 => 2,
                PSZR::_64 => 3,
                PSZR::_128 => 4,
                PSZR::_256 => 5,
                PSZR::_512 => 6,
                PSZR::_1024 => 7,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> PSZR {
            match value {
                0 => PSZR::_8,
                1 => PSZR::_16,
                2 => PSZR::_32,
                3 => PSZR::_64,
                4 => PSZR::_128,
                5 => PSZR::_256,
                6 => PSZR::_512,
                7 => PSZR::_1024,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `_8`"]
        #[inline(always)]
        pub fn is_8(&self) -> bool {
            *self == PSZR::_8
        }
        #[doc = "Checks if the value of the field is `_16`"]
        #[inline(always)]
        pub fn is_16(&self) -> bool {
            *self == PSZR::_16
        }
        #[doc = "Checks if the value of the field is `_32`"]
        #[inline(always)]
        pub fn is_32(&self) -> bool {
            *self == PSZR::_32
        }
        #[doc = "Checks if the value of the field is `_64`"]
        #[inline(always)]
        pub fn is_64(&self) -> bool {
            *self == PSZR::_64
        }
        #[doc = "Checks if the value of the field is `_128`"]
        #[inline(always)]
        pub fn is_128(&self) -> bool {
            *self == PSZR::_128
        }
        #[doc = "Checks if the value of the field is `_256`"]
        #[inline(always)]
        pub fn is_256(&self) -> bool {
            *self == PSZR::_256
        }
        #[doc = "Checks if the value of the field is `_512`"]
        #[inline(always)]
        pub fn is_512(&self) -> bool {
            *self == PSZR::_512
        }
        #[doc = "Checks if the value of the field is `_1024`"]
        #[inline(always)]
        pub fn is_1024(&self) -> bool {
            *self == PSZR::_1024
        }
    }
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:15 - NVM Pages"]
        #[inline(always)]
        pub fn nvmp(&self) -> NVMPR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            NVMPR { bits }
        }
        #[doc = "Bits 16:18 - Page Size"]
        #[inline(always)]
        pub fn psz(&self) -> PSZR {
            PSZR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
    }
}
#[doc = "Status"]
pub struct STATUS {
    register: VolatileCell<u16>,
}
#[doc = "Status"]
pub mod status {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::STATUS {
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
    pub struct PRMR {
        bits: bool,
    }
    impl PRMR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LOADR {
        bits: bool,
    }
    impl LOADR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct PROGER {
        bits: bool,
    }
    impl PROGER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct LOCKER {
        bits: bool,
    }
    impl LOCKER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct NVMER {
        bits: bool,
    }
    impl NVMER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = r" Value of the field"]
    pub struct SBR {
        bits: bool,
    }
    impl SBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _LOADW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LOADW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _PROGEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PROGEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
    pub struct _LOCKEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LOCKEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _NVMEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _NVMEW<'a> {
        #[doc = r" Sets the field bit"]
        pub fn set_bit(self) -> &'a mut W {
            self.bit(true)
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
        #[doc = "Bit 0 - Power Reduction Mode"]
        #[inline(always)]
        pub fn prm(&self) -> PRMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PRMR { bits }
        }
        #[doc = "Bit 1 - NVM Page Buffer Active Loading"]
        #[inline(always)]
        pub fn load(&self) -> LOADR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            LOADR { bits }
        }
        #[doc = "Bit 2 - Programming Error Status"]
        #[inline(always)]
        pub fn proge(&self) -> PROGER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            PROGER { bits }
        }
        #[doc = "Bit 3 - Lock Error Status"]
        #[inline(always)]
        pub fn locke(&self) -> LOCKER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            LOCKER { bits }
        }
        #[doc = "Bit 4 - NVM Error"]
        #[inline(always)]
        pub fn nvme(&self) -> NVMER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            NVMER { bits }
        }
        #[doc = "Bit 8 - Security Bit Status"]
        #[inline(always)]
        pub fn sb(&self) -> SBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            SBR { bits }
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
        #[doc = "Bit 1 - NVM Page Buffer Active Loading"]
        #[inline(always)]
        pub fn load(&mut self) -> _LOADW {
            _LOADW { w: self }
        }
        #[doc = "Bit 2 - Programming Error Status"]
        #[inline(always)]
        pub fn proge(&mut self) -> _PROGEW {
            _PROGEW { w: self }
        }
        #[doc = "Bit 3 - Lock Error Status"]
        #[inline(always)]
        pub fn locke(&mut self) -> _LOCKEW {
            _LOCKEW { w: self }
        }
        #[doc = "Bit 4 - NVM Error"]
        #[inline(always)]
        pub fn nvme(&mut self) -> _NVMEW {
            _NVMEW { w: self }
        }
    }
}
