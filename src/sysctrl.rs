use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0c - Power and Clocks Status"]
    pub pclksr: PCLKSR,
    #[doc = "0x10 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xosc: XOSC,
    _reserved0: [u8; 2usize],
    #[doc = "0x14 - 32kHz External Crystal Oscillator (XOSC32K) Control"]
    pub xosc32k: XOSC32K,
    _reserved1: [u8; 2usize],
    #[doc = "0x18 - 32kHz Internal Oscillator (OSC32K) Control"]
    pub osc32k: OSC32K,
    #[doc = "0x1c - 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
    pub osculp32k: OSCULP32K,
    _reserved2: [u8; 3usize],
    #[doc = "0x20 - 8MHz Internal Oscillator (OSC8M) Control"]
    pub osc8m: OSC8M,
    #[doc = "0x24 - DFLL48M Control"]
    pub dfllctrl: DFLLCTRL,
    _reserved3: [u8; 2usize],
    #[doc = "0x28 - DFLL48M Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x2c - DFLL48M Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x30 - DFLL48M Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved4: [u8; 3usize],
    #[doc = "0x34 - 3.3V Brown-Out Detector (BOD33) Control"]
    pub bod33: BOD33,
    _reserved5: [u8; 8usize],
    #[doc = "0x40 - Voltage References System (VREF) Control"]
    pub vref: VREF,
    #[doc = "0x44 - DPLL Control A"]
    pub dpllctrla: DPLLCTRLA,
    _reserved6: [u8; 3usize],
    #[doc = "0x48 - DPLL Ratio Control"]
    pub dpllratio: DPLLRATIO,
    #[doc = "0x4c - DPLL Control B"]
    pub dpllctrlb: DPLLCTRLB,
    #[doc = "0x50 - DPLL Status"]
    pub dpllstatus: DPLLSTATUS,
}
#[doc = "3.3V Brown-Out Detector (BOD33) Control"]
pub struct BOD33 {
    register: VolatileCell<u32>,
}
#[doc = "3.3V Brown-Out Detector (BOD33) Control"]
pub mod bod33 {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::BOD33 {
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
    pub struct HYSTR {
        bits: bool,
    }
    impl HYSTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `ACTION`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ACTIONR {
        #[doc = "No action"]
        NONE,
        #[doc = "The BOD33 generates a reset"]
        RESET,
        #[doc = "The BOD33 generates an interrupt"]
        INTERRUPT,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl ACTIONR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                ACTIONR::NONE => 0,
                ACTIONR::RESET => 1,
                ACTIONR::INTERRUPT => 2,
                ACTIONR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> ACTIONR {
            match value {
                0 => ACTIONR::NONE,
                1 => ACTIONR::RESET,
                2 => ACTIONR::INTERRUPT,
                i => ACTIONR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `NONE`"]
        #[inline(always)]
        pub fn is_none(&self) -> bool {
            *self == ACTIONR::NONE
        }
        #[doc = "Checks if the value of the field is `RESET`"]
        #[inline(always)]
        pub fn is_reset(&self) -> bool {
            *self == ACTIONR::RESET
        }
        #[doc = "Checks if the value of the field is `INTERRUPT`"]
        #[inline(always)]
        pub fn is_interrupt(&self) -> bool {
            *self == ACTIONR::INTERRUPT
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
    #[doc = r" Value of the field"]
    pub struct MODER {
        bits: bool,
    }
    impl MODER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct CENR {
        bits: bool,
    }
    impl CENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `PSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PSELR {
        #[doc = "Divide clock by 2"]
        DIV2,
        #[doc = "Divide clock by 4"]
        DIV4,
        #[doc = "Divide clock by 8"]
        DIV8,
        #[doc = "Divide clock by 16"]
        DIV16,
        #[doc = "Divide clock by 32"]
        DIV32,
        #[doc = "Divide clock by 64"]
        DIV64,
        #[doc = "Divide clock by 128"]
        DIV128,
        #[doc = "Divide clock by 256"]
        DIV256,
        #[doc = "Divide clock by 512"]
        DIV512,
        #[doc = "Divide clock by 1024"]
        DIV1K,
        #[doc = "Divide clock by 2048"]
        DIV2K,
        #[doc = "Divide clock by 4096"]
        DIV4K,
        #[doc = "Divide clock by 8192"]
        DIV8K,
        #[doc = "Divide clock by 16384"]
        DIV16K,
        #[doc = "Divide clock by 32768"]
        DIV32K,
        #[doc = "Divide clock by 65536"]
        DIV64K,
    }
    impl PSELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                PSELR::DIV2 => 0,
                PSELR::DIV4 => 1,
                PSELR::DIV8 => 2,
                PSELR::DIV16 => 3,
                PSELR::DIV32 => 4,
                PSELR::DIV64 => 5,
                PSELR::DIV128 => 6,
                PSELR::DIV256 => 7,
                PSELR::DIV512 => 8,
                PSELR::DIV1K => 9,
                PSELR::DIV2K => 10,
                PSELR::DIV4K => 11,
                PSELR::DIV8K => 12,
                PSELR::DIV16K => 13,
                PSELR::DIV32K => 14,
                PSELR::DIV64K => 15,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> PSELR {
            match value {
                0 => PSELR::DIV2,
                1 => PSELR::DIV4,
                2 => PSELR::DIV8,
                3 => PSELR::DIV16,
                4 => PSELR::DIV32,
                5 => PSELR::DIV64,
                6 => PSELR::DIV128,
                7 => PSELR::DIV256,
                8 => PSELR::DIV512,
                9 => PSELR::DIV1K,
                10 => PSELR::DIV2K,
                11 => PSELR::DIV4K,
                12 => PSELR::DIV8K,
                13 => PSELR::DIV16K,
                14 => PSELR::DIV32K,
                15 => PSELR::DIV64K,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DIV2`"]
        #[inline(always)]
        pub fn is_div2(&self) -> bool {
            *self == PSELR::DIV2
        }
        #[doc = "Checks if the value of the field is `DIV4`"]
        #[inline(always)]
        pub fn is_div4(&self) -> bool {
            *self == PSELR::DIV4
        }
        #[doc = "Checks if the value of the field is `DIV8`"]
        #[inline(always)]
        pub fn is_div8(&self) -> bool {
            *self == PSELR::DIV8
        }
        #[doc = "Checks if the value of the field is `DIV16`"]
        #[inline(always)]
        pub fn is_div16(&self) -> bool {
            *self == PSELR::DIV16
        }
        #[doc = "Checks if the value of the field is `DIV32`"]
        #[inline(always)]
        pub fn is_div32(&self) -> bool {
            *self == PSELR::DIV32
        }
        #[doc = "Checks if the value of the field is `DIV64`"]
        #[inline(always)]
        pub fn is_div64(&self) -> bool {
            *self == PSELR::DIV64
        }
        #[doc = "Checks if the value of the field is `DIV128`"]
        #[inline(always)]
        pub fn is_div128(&self) -> bool {
            *self == PSELR::DIV128
        }
        #[doc = "Checks if the value of the field is `DIV256`"]
        #[inline(always)]
        pub fn is_div256(&self) -> bool {
            *self == PSELR::DIV256
        }
        #[doc = "Checks if the value of the field is `DIV512`"]
        #[inline(always)]
        pub fn is_div512(&self) -> bool {
            *self == PSELR::DIV512
        }
        #[doc = "Checks if the value of the field is `DIV1K`"]
        #[inline(always)]
        pub fn is_div1k(&self) -> bool {
            *self == PSELR::DIV1K
        }
        #[doc = "Checks if the value of the field is `DIV2K`"]
        #[inline(always)]
        pub fn is_div2k(&self) -> bool {
            *self == PSELR::DIV2K
        }
        #[doc = "Checks if the value of the field is `DIV4K`"]
        #[inline(always)]
        pub fn is_div4k(&self) -> bool {
            *self == PSELR::DIV4K
        }
        #[doc = "Checks if the value of the field is `DIV8K`"]
        #[inline(always)]
        pub fn is_div8k(&self) -> bool {
            *self == PSELR::DIV8K
        }
        #[doc = "Checks if the value of the field is `DIV16K`"]
        #[inline(always)]
        pub fn is_div16k(&self) -> bool {
            *self == PSELR::DIV16K
        }
        #[doc = "Checks if the value of the field is `DIV32K`"]
        #[inline(always)]
        pub fn is_div32k(&self) -> bool {
            *self == PSELR::DIV32K
        }
        #[doc = "Checks if the value of the field is `DIV64K`"]
        #[inline(always)]
        pub fn is_div64k(&self) -> bool {
            *self == PSELR::DIV64K
        }
    }
    #[doc = r" Value of the field"]
    pub struct LEVELR {
        bits: u8,
    }
    impl LEVELR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _HYSTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _HYSTW<'a> {
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
    #[doc = "Values that can be written to the field `ACTION`"]
    pub enum ACTIONW {
        #[doc = "No action"]
        NONE,
        #[doc = "The BOD33 generates a reset"]
        RESET,
        #[doc = "The BOD33 generates an interrupt"]
        INTERRUPT,
    }
    impl ACTIONW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                ACTIONW::NONE => 0,
                ACTIONW::RESET => 1,
                ACTIONW::INTERRUPT => 2,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _ACTIONW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ACTIONW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: ACTIONW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "No action"]
        #[inline(always)]
        pub fn none(self) -> &'a mut W {
            self.variant(ACTIONW::NONE)
        }
        #[doc = "The BOD33 generates a reset"]
        #[inline(always)]
        pub fn reset(self) -> &'a mut W {
            self.variant(ACTIONW::RESET)
        }
        #[doc = "The BOD33 generates an interrupt"]
        #[inline(always)]
        pub fn interrupt(self) -> &'a mut W {
            self.variant(ACTIONW::INTERRUPT)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODEW<'a> {
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
    pub struct _CENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CENW<'a> {
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
    #[doc = "Values that can be written to the field `PSEL`"]
    pub enum PSELW {
        #[doc = "Divide clock by 2"]
        DIV2,
        #[doc = "Divide clock by 4"]
        DIV4,
        #[doc = "Divide clock by 8"]
        DIV8,
        #[doc = "Divide clock by 16"]
        DIV16,
        #[doc = "Divide clock by 32"]
        DIV32,
        #[doc = "Divide clock by 64"]
        DIV64,
        #[doc = "Divide clock by 128"]
        DIV128,
        #[doc = "Divide clock by 256"]
        DIV256,
        #[doc = "Divide clock by 512"]
        DIV512,
        #[doc = "Divide clock by 1024"]
        DIV1K,
        #[doc = "Divide clock by 2048"]
        DIV2K,
        #[doc = "Divide clock by 4096"]
        DIV4K,
        #[doc = "Divide clock by 8192"]
        DIV8K,
        #[doc = "Divide clock by 16384"]
        DIV16K,
        #[doc = "Divide clock by 32768"]
        DIV32K,
        #[doc = "Divide clock by 65536"]
        DIV64K,
    }
    impl PSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                PSELW::DIV2 => 0,
                PSELW::DIV4 => 1,
                PSELW::DIV8 => 2,
                PSELW::DIV16 => 3,
                PSELW::DIV32 => 4,
                PSELW::DIV64 => 5,
                PSELW::DIV128 => 6,
                PSELW::DIV256 => 7,
                PSELW::DIV512 => 8,
                PSELW::DIV1K => 9,
                PSELW::DIV2K => 10,
                PSELW::DIV4K => 11,
                PSELW::DIV8K => 12,
                PSELW::DIV16K => 13,
                PSELW::DIV32K => 14,
                PSELW::DIV64K => 15,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _PSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: PSELW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "Divide clock by 2"]
        #[inline(always)]
        pub fn div2(self) -> &'a mut W {
            self.variant(PSELW::DIV2)
        }
        #[doc = "Divide clock by 4"]
        #[inline(always)]
        pub fn div4(self) -> &'a mut W {
            self.variant(PSELW::DIV4)
        }
        #[doc = "Divide clock by 8"]
        #[inline(always)]
        pub fn div8(self) -> &'a mut W {
            self.variant(PSELW::DIV8)
        }
        #[doc = "Divide clock by 16"]
        #[inline(always)]
        pub fn div16(self) -> &'a mut W {
            self.variant(PSELW::DIV16)
        }
        #[doc = "Divide clock by 32"]
        #[inline(always)]
        pub fn div32(self) -> &'a mut W {
            self.variant(PSELW::DIV32)
        }
        #[doc = "Divide clock by 64"]
        #[inline(always)]
        pub fn div64(self) -> &'a mut W {
            self.variant(PSELW::DIV64)
        }
        #[doc = "Divide clock by 128"]
        #[inline(always)]
        pub fn div128(self) -> &'a mut W {
            self.variant(PSELW::DIV128)
        }
        #[doc = "Divide clock by 256"]
        #[inline(always)]
        pub fn div256(self) -> &'a mut W {
            self.variant(PSELW::DIV256)
        }
        #[doc = "Divide clock by 512"]
        #[inline(always)]
        pub fn div512(self) -> &'a mut W {
            self.variant(PSELW::DIV512)
        }
        #[doc = "Divide clock by 1024"]
        #[inline(always)]
        pub fn div1k(self) -> &'a mut W {
            self.variant(PSELW::DIV1K)
        }
        #[doc = "Divide clock by 2048"]
        #[inline(always)]
        pub fn div2k(self) -> &'a mut W {
            self.variant(PSELW::DIV2K)
        }
        #[doc = "Divide clock by 4096"]
        #[inline(always)]
        pub fn div4k(self) -> &'a mut W {
            self.variant(PSELW::DIV4K)
        }
        #[doc = "Divide clock by 8192"]
        #[inline(always)]
        pub fn div8k(self) -> &'a mut W {
            self.variant(PSELW::DIV8K)
        }
        #[doc = "Divide clock by 16384"]
        #[inline(always)]
        pub fn div16k(self) -> &'a mut W {
            self.variant(PSELW::DIV16K)
        }
        #[doc = "Divide clock by 32768"]
        #[inline(always)]
        pub fn div32k(self) -> &'a mut W {
            self.variant(PSELW::DIV32K)
        }
        #[doc = "Divide clock by 65536"]
        #[inline(always)]
        pub fn div64k(self) -> &'a mut W {
            self.variant(PSELW::DIV64K)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _LEVELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LEVELW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
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
        #[doc = "Bit 1 - Enable"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 2 - Hysteresis"]
        #[inline(always)]
        pub fn hyst(&self) -> HYSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            HYSTR { bits }
        }
        #[doc = "Bits 3:4 - BOD33 Action"]
        #[inline(always)]
        pub fn action(&self) -> ACTIONR {
            ACTIONR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&self) -> RUNSTDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RUNSTDBYR { bits }
        }
        #[doc = "Bit 8 - Operation Mode"]
        #[inline(always)]
        pub fn mode(&self) -> MODER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MODER { bits }
        }
        #[doc = "Bit 9 - Clock Enable"]
        #[inline(always)]
        pub fn cen(&self) -> CENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            CENR { bits }
        }
        #[doc = "Bits 12:15 - Prescaler Select"]
        #[inline(always)]
        pub fn psel(&self) -> PSELR {
            PSELR::_from({
                const MASK: u8 = 15;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 16:21 - BOD33 Threshold Level"]
        #[inline(always)]
        pub fn level(&self) -> LEVELR {
            let bits = {
                const MASK: u8 = 63;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            LEVELR { bits }
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
        #[doc = "Bit 1 - Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
        #[doc = "Bit 2 - Hysteresis"]
        #[inline(always)]
        pub fn hyst(&mut self) -> _HYSTW {
            _HYSTW { w: self }
        }
        #[doc = "Bits 3:4 - BOD33 Action"]
        #[inline(always)]
        pub fn action(&mut self) -> _ACTIONW {
            _ACTIONW { w: self }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&mut self) -> _RUNSTDBYW {
            _RUNSTDBYW { w: self }
        }
        #[doc = "Bit 8 - Operation Mode"]
        #[inline(always)]
        pub fn mode(&mut self) -> _MODEW {
            _MODEW { w: self }
        }
        #[doc = "Bit 9 - Clock Enable"]
        #[inline(always)]
        pub fn cen(&mut self) -> _CENW {
            _CENW { w: self }
        }
        #[doc = "Bits 12:15 - Prescaler Select"]
        #[inline(always)]
        pub fn psel(&mut self) -> _PSELW {
            _PSELW { w: self }
        }
        #[doc = "Bits 16:21 - BOD33 Threshold Level"]
        #[inline(always)]
        pub fn level(&mut self) -> _LEVELW {
            _LEVELW { w: self }
        }
    }
}
#[doc = "DFLL48M Control"]
pub struct DFLLCTRL {
    register: VolatileCell<u16>,
}
#[doc = "DFLL48M Control"]
pub mod dfllctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::DFLLCTRL {
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
    pub struct MODER {
        bits: bool,
    }
    impl MODER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct STABLER {
        bits: bool,
    }
    impl STABLER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct LLAWR {
        bits: bool,
    }
    impl LLAWR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct USBCRMR {
        bits: bool,
    }
    impl USBCRMR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    #[doc = r" Value of the field"]
    pub struct ONDEMANDR {
        bits: bool,
    }
    impl ONDEMANDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct CCDISR {
        bits: bool,
    }
    impl CCDISR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct QLDISR {
        bits: bool,
    }
    impl QLDISR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct BPLCKCR {
        bits: bool,
    }
    impl BPLCKCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct WAITLOCKR {
        bits: bool,
    }
    impl WAITLOCKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MODEW<'a> {
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
    pub struct _STABLEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STABLEW<'a> {
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
    pub struct _LLAWW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LLAWW<'a> {
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
    #[doc = r" Proxy"]
    pub struct _USBCRMW<'a> {
        w: &'a mut W,
    }
    impl<'a> _USBCRMW<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ONDEMANDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ONDEMANDW<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CCDISW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CCDISW<'a> {
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
    pub struct _QLDISW<'a> {
        w: &'a mut W,
    }
    impl<'a> _QLDISW<'a> {
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
    pub struct _BPLCKCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BPLCKCW<'a> {
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
    pub struct _WAITLOCKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WAITLOCKW<'a> {
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
        #[doc = "Bit 1 - DFLL Enable"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 2 - Operating Mode Selection"]
        #[inline(always)]
        pub fn mode(&self) -> MODER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            MODER { bits }
        }
        #[doc = "Bit 3 - Stable DFLL Frequency"]
        #[inline(always)]
        pub fn stable(&self) -> STABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            STABLER { bits }
        }
        #[doc = "Bit 4 - Lose Lock After Wake"]
        #[inline(always)]
        pub fn llaw(&self) -> LLAWR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            LLAWR { bits }
        }
        #[doc = "Bit 5 - USB Clock Recovery Mode"]
        #[inline(always)]
        pub fn usbcrm(&self) -> USBCRMR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            USBCRMR { bits }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&self) -> RUNSTDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RUNSTDBYR { bits }
        }
        #[doc = "Bit 7 - On Demand Control"]
        #[inline(always)]
        pub fn ondemand(&self) -> ONDEMANDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            ONDEMANDR { bits }
        }
        #[doc = "Bit 8 - Chill Cycle Disable"]
        #[inline(always)]
        pub fn ccdis(&self) -> CCDISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            CCDISR { bits }
        }
        #[doc = "Bit 9 - Quick Lock Disable"]
        #[inline(always)]
        pub fn qldis(&self) -> QLDISR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            QLDISR { bits }
        }
        #[doc = "Bit 10 - Bypass Coarse Lock"]
        #[inline(always)]
        pub fn bplckc(&self) -> BPLCKCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            BPLCKCR { bits }
        }
        #[doc = "Bit 11 - Wait Lock"]
        #[inline(always)]
        pub fn waitlock(&self) -> WAITLOCKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            WAITLOCKR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 128 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 1 - DFLL Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
        #[doc = "Bit 2 - Operating Mode Selection"]
        #[inline(always)]
        pub fn mode(&mut self) -> _MODEW {
            _MODEW { w: self }
        }
        #[doc = "Bit 3 - Stable DFLL Frequency"]
        #[inline(always)]
        pub fn stable(&mut self) -> _STABLEW {
            _STABLEW { w: self }
        }
        #[doc = "Bit 4 - Lose Lock After Wake"]
        #[inline(always)]
        pub fn llaw(&mut self) -> _LLAWW {
            _LLAWW { w: self }
        }
        #[doc = "Bit 5 - USB Clock Recovery Mode"]
        #[inline(always)]
        pub fn usbcrm(&mut self) -> _USBCRMW {
            _USBCRMW { w: self }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&mut self) -> _RUNSTDBYW {
            _RUNSTDBYW { w: self }
        }
        #[doc = "Bit 7 - On Demand Control"]
        #[inline(always)]
        pub fn ondemand(&mut self) -> _ONDEMANDW {
            _ONDEMANDW { w: self }
        }
        #[doc = "Bit 8 - Chill Cycle Disable"]
        #[inline(always)]
        pub fn ccdis(&mut self) -> _CCDISW {
            _CCDISW { w: self }
        }
        #[doc = "Bit 9 - Quick Lock Disable"]
        #[inline(always)]
        pub fn qldis(&mut self) -> _QLDISW {
            _QLDISW { w: self }
        }
        #[doc = "Bit 10 - Bypass Coarse Lock"]
        #[inline(always)]
        pub fn bplckc(&mut self) -> _BPLCKCW {
            _BPLCKCW { w: self }
        }
        #[doc = "Bit 11 - Wait Lock"]
        #[inline(always)]
        pub fn waitlock(&mut self) -> _WAITLOCKW {
            _WAITLOCKW { w: self }
        }
    }
}
#[doc = "DFLL48M Multiplier"]
pub struct DFLLMUL {
    register: VolatileCell<u32>,
}
#[doc = "DFLL48M Multiplier"]
pub mod dfllmul {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DFLLMUL {
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
    pub struct MULR {
        bits: u16,
    }
    impl MULR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct FSTEPR {
        bits: u16,
    }
    impl FSTEPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct CSTEPR {
        bits: u8,
    }
    impl CSTEPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _MULW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MULW<'a> {
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
    pub struct _FSTEPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FSTEPW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CSTEPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CSTEPW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
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
        #[doc = "Bits 0:15 - DFLL Multiply Factor"]
        #[inline(always)]
        pub fn mul(&self) -> MULR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            MULR { bits }
        }
        #[doc = "Bits 16:25 - Fine Maximum Step"]
        #[inline(always)]
        pub fn fstep(&self) -> FSTEPR {
            let bits = {
                const MASK: u16 = 1023;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            FSTEPR { bits }
        }
        #[doc = "Bits 26:31 - Coarse Maximum Step"]
        #[inline(always)]
        pub fn cstep(&self) -> CSTEPR {
            let bits = {
                const MASK: u8 = 63;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CSTEPR { bits }
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
        #[doc = "Bits 0:15 - DFLL Multiply Factor"]
        #[inline(always)]
        pub fn mul(&mut self) -> _MULW {
            _MULW { w: self }
        }
        #[doc = "Bits 16:25 - Fine Maximum Step"]
        #[inline(always)]
        pub fn fstep(&mut self) -> _FSTEPW {
            _FSTEPW { w: self }
        }
        #[doc = "Bits 26:31 - Coarse Maximum Step"]
        #[inline(always)]
        pub fn cstep(&mut self) -> _CSTEPW {
            _CSTEPW { w: self }
        }
    }
}
#[doc = "DFLL48M Synchronization"]
pub struct DFLLSYNC {
    register: VolatileCell<u8>,
}
#[doc = "DFLL48M Synchronization"]
pub mod dfllsync {
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::DFLLSYNC {
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
    pub struct _READREQW<'a> {
        w: &'a mut W,
    }
    impl<'a> _READREQW<'a> {
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
        #[doc = "Bit 7 - Read Request"]
        #[inline(always)]
        pub fn readreq(&mut self) -> _READREQW {
            _READREQW { w: self }
        }
    }
}
#[doc = "DFLL48M Value"]
pub struct DFLLVAL {
    register: VolatileCell<u32>,
}
#[doc = "DFLL48M Value"]
pub mod dfllval {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DFLLVAL {
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
    pub struct FINER {
        bits: u16,
    }
    impl FINER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct COARSER {
        bits: u8,
    }
    impl COARSER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct DIFFR {
        bits: u16,
    }
    impl DIFFR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _FINEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FINEW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _COARSEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _COARSEW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
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
        #[doc = "Bits 0:9 - Fine Value"]
        #[inline(always)]
        pub fn fine(&self) -> FINER {
            let bits = {
                const MASK: u16 = 1023;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            FINER { bits }
        }
        #[doc = "Bits 10:15 - Coarse Value"]
        #[inline(always)]
        pub fn coarse(&self) -> COARSER {
            let bits = {
                const MASK: u8 = 63;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            COARSER { bits }
        }
        #[doc = "Bits 16:31 - Multiplication Ratio Difference"]
        #[inline(always)]
        pub fn diff(&self) -> DIFFR {
            let bits = {
                const MASK: u16 = 65535;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            DIFFR { bits }
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
        #[doc = "Bits 0:9 - Fine Value"]
        #[inline(always)]
        pub fn fine(&mut self) -> _FINEW {
            _FINEW { w: self }
        }
        #[doc = "Bits 10:15 - Coarse Value"]
        #[inline(always)]
        pub fn coarse(&mut self) -> _COARSEW {
            _COARSEW { w: self }
        }
    }
}
#[doc = "DPLL Control A"]
pub struct DPLLCTRLA {
    register: VolatileCell<u8>,
}
#[doc = "DPLL Control A"]
pub mod dpllctrla {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::DPLLCTRLA {
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
    #[doc = r" Value of the field"]
    pub struct ONDEMANDR {
        bits: bool,
    }
    impl ONDEMANDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ONDEMANDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ONDEMANDW<'a> {
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
        #[doc = "Bit 1 - DPLL Enable"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&self) -> RUNSTDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            RUNSTDBYR { bits }
        }
        #[doc = "Bit 7 - On Demand Clock Activation"]
        #[inline(always)]
        pub fn ondemand(&self) -> ONDEMANDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            ONDEMANDR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 128 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 1 - DPLL Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&mut self) -> _RUNSTDBYW {
            _RUNSTDBYW { w: self }
        }
        #[doc = "Bit 7 - On Demand Clock Activation"]
        #[inline(always)]
        pub fn ondemand(&mut self) -> _ONDEMANDW {
            _ONDEMANDW { w: self }
        }
    }
}
#[doc = "DPLL Control B"]
pub struct DPLLCTRLB {
    register: VolatileCell<u32>,
}
#[doc = "DPLL Control B"]
pub mod dpllctrlb {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DPLLCTRLB {
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
    #[doc = "Possible values of the field `FILTER`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum FILTERR {
        #[doc = "Default filter mode"]
        DEFAULT,
        #[doc = "Low bandwidth filter"]
        LBFILT,
        #[doc = "High bandwidth filter"]
        HBFILT,
        #[doc = "High damping filter"]
        HDFILT,
    }
    impl FILTERR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                FILTERR::DEFAULT => 0,
                FILTERR::LBFILT => 1,
                FILTERR::HBFILT => 2,
                FILTERR::HDFILT => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> FILTERR {
            match value {
                0 => FILTERR::DEFAULT,
                1 => FILTERR::LBFILT,
                2 => FILTERR::HBFILT,
                3 => FILTERR::HDFILT,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `DEFAULT`"]
        #[inline(always)]
        pub fn is_default(&self) -> bool {
            *self == FILTERR::DEFAULT
        }
        #[doc = "Checks if the value of the field is `LBFILT`"]
        #[inline(always)]
        pub fn is_lbfilt(&self) -> bool {
            *self == FILTERR::LBFILT
        }
        #[doc = "Checks if the value of the field is `HBFILT`"]
        #[inline(always)]
        pub fn is_hbfilt(&self) -> bool {
            *self == FILTERR::HBFILT
        }
        #[doc = "Checks if the value of the field is `HDFILT`"]
        #[inline(always)]
        pub fn is_hdfilt(&self) -> bool {
            *self == FILTERR::HDFILT
        }
    }
    #[doc = r" Value of the field"]
    pub struct LPENR {
        bits: bool,
    }
    impl LPENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct WUFR {
        bits: bool,
    }
    impl WUFR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `REFCLK`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum REFCLKR {
        #[doc = "CLK_DPLL_REF0 clock reference"]
        REF0,
        #[doc = "CLK_DPLL_REF1 clock reference"]
        REF1,
        #[doc = "GCLK_DPLL clock reference"]
        GCLK,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl REFCLKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                REFCLKR::REF0 => 0,
                REFCLKR::REF1 => 1,
                REFCLKR::GCLK => 2,
                REFCLKR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> REFCLKR {
            match value {
                0 => REFCLKR::REF0,
                1 => REFCLKR::REF1,
                2 => REFCLKR::GCLK,
                i => REFCLKR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `REF0`"]
        #[inline(always)]
        pub fn is_ref0(&self) -> bool {
            *self == REFCLKR::REF0
        }
        #[doc = "Checks if the value of the field is `REF1`"]
        #[inline(always)]
        pub fn is_ref1(&self) -> bool {
            *self == REFCLKR::REF1
        }
        #[doc = "Checks if the value of the field is `GCLK`"]
        #[inline(always)]
        pub fn is_gclk(&self) -> bool {
            *self == REFCLKR::GCLK
        }
    }
    #[doc = "Possible values of the field `LTIME`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum LTIMER {
        #[doc = "Default\tNo time-out"]
        _0X0,
        #[doc = "8MS\tTime-out if no lock within 8 ms"]
        _0X4,
        #[doc = "9MS\tTime-out if no lock within 9 ms"]
        _0X5,
        #[doc = "10MS\tTime-out if no lock within 10 ms"]
        _0X6,
        #[doc = "11MS\tTime-out if no lock within 11 ms"]
        _0X7,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl LTIMER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                LTIMER::_0X0 => 0,
                LTIMER::_0X4 => 4,
                LTIMER::_0X5 => 5,
                LTIMER::_0X6 => 6,
                LTIMER::_0X7 => 7,
                LTIMER::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> LTIMER {
            match value {
                0 => LTIMER::_0X0,
                4 => LTIMER::_0X4,
                5 => LTIMER::_0X5,
                6 => LTIMER::_0X6,
                7 => LTIMER::_0X7,
                i => LTIMER::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `_0X0`"]
        #[inline(always)]
        pub fn is_0x0(&self) -> bool {
            *self == LTIMER::_0X0
        }
        #[doc = "Checks if the value of the field is `_0X4`"]
        #[inline(always)]
        pub fn is_0x4(&self) -> bool {
            *self == LTIMER::_0X4
        }
        #[doc = "Checks if the value of the field is `_0X5`"]
        #[inline(always)]
        pub fn is_0x5(&self) -> bool {
            *self == LTIMER::_0X5
        }
        #[doc = "Checks if the value of the field is `_0X6`"]
        #[inline(always)]
        pub fn is_0x6(&self) -> bool {
            *self == LTIMER::_0X6
        }
        #[doc = "Checks if the value of the field is `_0X7`"]
        #[inline(always)]
        pub fn is_0x7(&self) -> bool {
            *self == LTIMER::_0X7
        }
    }
    #[doc = r" Value of the field"]
    pub struct LBYPASSR {
        bits: bool,
    }
    impl LBYPASSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    #[doc = "Values that can be written to the field `FILTER`"]
    pub enum FILTERW {
        #[doc = "Default filter mode"]
        DEFAULT,
        #[doc = "Low bandwidth filter"]
        LBFILT,
        #[doc = "High bandwidth filter"]
        HBFILT,
        #[doc = "High damping filter"]
        HDFILT,
    }
    impl FILTERW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                FILTERW::DEFAULT => 0,
                FILTERW::LBFILT => 1,
                FILTERW::HBFILT => 2,
                FILTERW::HDFILT => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _FILTERW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FILTERW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: FILTERW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "Default filter mode"]
        #[inline(always)]
        pub fn default(self) -> &'a mut W {
            self.variant(FILTERW::DEFAULT)
        }
        #[doc = "Low bandwidth filter"]
        #[inline(always)]
        pub fn lbfilt(self) -> &'a mut W {
            self.variant(FILTERW::LBFILT)
        }
        #[doc = "High bandwidth filter"]
        #[inline(always)]
        pub fn hbfilt(self) -> &'a mut W {
            self.variant(FILTERW::HBFILT)
        }
        #[doc = "High damping filter"]
        #[inline(always)]
        pub fn hdfilt(self) -> &'a mut W {
            self.variant(FILTERW::HDFILT)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _LPENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LPENW<'a> {
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
    pub struct _WUFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WUFW<'a> {
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
    #[doc = "Values that can be written to the field `REFCLK`"]
    pub enum REFCLKW {
        #[doc = "CLK_DPLL_REF0 clock reference"]
        REF0,
        #[doc = "CLK_DPLL_REF1 clock reference"]
        REF1,
        #[doc = "GCLK_DPLL clock reference"]
        GCLK,
    }
    impl REFCLKW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                REFCLKW::REF0 => 0,
                REFCLKW::REF1 => 1,
                REFCLKW::GCLK => 2,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _REFCLKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _REFCLKW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: REFCLKW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "CLK_DPLL_REF0 clock reference"]
        #[inline(always)]
        pub fn ref0(self) -> &'a mut W {
            self.variant(REFCLKW::REF0)
        }
        #[doc = "CLK_DPLL_REF1 clock reference"]
        #[inline(always)]
        pub fn ref1(self) -> &'a mut W {
            self.variant(REFCLKW::REF1)
        }
        #[doc = "GCLK_DPLL clock reference"]
        #[inline(always)]
        pub fn gclk(self) -> &'a mut W {
            self.variant(REFCLKW::GCLK)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `LTIME`"]
    pub enum LTIMEW {
        #[doc = "Default\tNo time-out"]
        _0X0,
        #[doc = "8MS\tTime-out if no lock within 8 ms"]
        _0X4,
        #[doc = "9MS\tTime-out if no lock within 9 ms"]
        _0X5,
        #[doc = "10MS\tTime-out if no lock within 10 ms"]
        _0X6,
        #[doc = "11MS\tTime-out if no lock within 11 ms"]
        _0X7,
    }
    impl LTIMEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                LTIMEW::_0X0 => 0,
                LTIMEW::_0X4 => 4,
                LTIMEW::_0X5 => 5,
                LTIMEW::_0X6 => 6,
                LTIMEW::_0X7 => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _LTIMEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LTIMEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: LTIMEW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Default No time-out"]
        #[inline(always)]
        pub fn _0x0(self) -> &'a mut W {
            self.variant(LTIMEW::_0X0)
        }
        #[doc = "8MS Time-out if no lock within 8 ms"]
        #[inline(always)]
        pub fn _0x4(self) -> &'a mut W {
            self.variant(LTIMEW::_0X4)
        }
        #[doc = "9MS Time-out if no lock within 9 ms"]
        #[inline(always)]
        pub fn _0x5(self) -> &'a mut W {
            self.variant(LTIMEW::_0X5)
        }
        #[doc = "10MS Time-out if no lock within 10 ms"]
        #[inline(always)]
        pub fn _0x6(self) -> &'a mut W {
            self.variant(LTIMEW::_0X6)
        }
        #[doc = "11MS Time-out if no lock within 11 ms"]
        #[inline(always)]
        pub fn _0x7(self) -> &'a mut W {
            self.variant(LTIMEW::_0X7)
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
    pub struct _LBYPASSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LBYPASSW<'a> {
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
    pub struct _DIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DIVW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
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
        #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
        #[inline(always)]
        pub fn filter(&self) -> FILTERR {
            FILTERR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 2 - Low-Power Enable"]
        #[inline(always)]
        pub fn lpen(&self) -> LPENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LPENR { bits }
        }
        #[doc = "Bit 3 - Wake Up Fast"]
        #[inline(always)]
        pub fn wuf(&self) -> WUFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WUFR { bits }
        }
        #[doc = "Bits 4:5 - Reference Clock Selection"]
        #[inline(always)]
        pub fn refclk(&self) -> REFCLKR {
            REFCLKR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 8:10 - Lock Time"]
        #[inline(always)]
        pub fn ltime(&self) -> LTIMER {
            LTIMER::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 12 - Lock Bypass"]
        #[inline(always)]
        pub fn lbypass(&self) -> LBYPASSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            LBYPASSR { bits }
        }
        #[doc = "Bits 16:26 - Clock Divider"]
        #[inline(always)]
        pub fn div(&self) -> DIVR {
            let bits = {
                const MASK: u16 = 2047;
                const OFFSET: u8 = 16;
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
        #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
        #[inline(always)]
        pub fn filter(&mut self) -> _FILTERW {
            _FILTERW { w: self }
        }
        #[doc = "Bit 2 - Low-Power Enable"]
        #[inline(always)]
        pub fn lpen(&mut self) -> _LPENW {
            _LPENW { w: self }
        }
        #[doc = "Bit 3 - Wake Up Fast"]
        #[inline(always)]
        pub fn wuf(&mut self) -> _WUFW {
            _WUFW { w: self }
        }
        #[doc = "Bits 4:5 - Reference Clock Selection"]
        #[inline(always)]
        pub fn refclk(&mut self) -> _REFCLKW {
            _REFCLKW { w: self }
        }
        #[doc = "Bits 8:10 - Lock Time"]
        #[inline(always)]
        pub fn ltime(&mut self) -> _LTIMEW {
            _LTIMEW { w: self }
        }
        #[doc = "Bit 12 - Lock Bypass"]
        #[inline(always)]
        pub fn lbypass(&mut self) -> _LBYPASSW {
            _LBYPASSW { w: self }
        }
        #[doc = "Bits 16:26 - Clock Divider"]
        #[inline(always)]
        pub fn div(&mut self) -> _DIVW {
            _DIVW { w: self }
        }
    }
}
#[doc = "DPLL Ratio Control"]
pub struct DPLLRATIO {
    register: VolatileCell<u32>,
}
#[doc = "DPLL Ratio Control"]
pub mod dpllratio {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::DPLLRATIO {
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
    pub struct LDRR {
        bits: u16,
    }
    impl LDRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct LDRFRACR {
        bits: u8,
    }
    impl LDRFRACR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _LDRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LDRW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _LDRFRACW<'a> {
        w: &'a mut W,
    }
    impl<'a> _LDRFRACW<'a> {
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bits 0:11 - Loop Divider Ratio"]
        #[inline(always)]
        pub fn ldr(&self) -> LDRR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            LDRR { bits }
        }
        #[doc = "Bits 16:19 - Loop Divider Ratio Fractional Part"]
        #[inline(always)]
        pub fn ldrfrac(&self) -> LDRFRACR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            LDRFRACR { bits }
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
        #[doc = "Bits 0:11 - Loop Divider Ratio"]
        #[inline(always)]
        pub fn ldr(&mut self) -> _LDRW {
            _LDRW { w: self }
        }
        #[doc = "Bits 16:19 - Loop Divider Ratio Fractional Part"]
        #[inline(always)]
        pub fn ldrfrac(&mut self) -> _LDRFRACW {
            _LDRFRACW { w: self }
        }
    }
}
#[doc = "DPLL Status"]
pub struct DPLLSTATUS {
    register: VolatileCell<u8>,
}
#[doc = "DPLL Status"]
pub mod dpllstatus {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    impl super::DPLLSTATUS {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct LOCKR {
        bits: bool,
    }
    impl LOCKR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct CLKRDYR {
        bits: bool,
    }
    impl CLKRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    #[doc = r" Value of the field"]
    pub struct DIVR {
        bits: bool,
    }
    impl DIVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
        #[doc = "Bit 0 - DPLL Lock Status"]
        #[inline(always)]
        pub fn lock(&self) -> LOCKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            LOCKR { bits }
        }
        #[doc = "Bit 1 - Output Clock Ready"]
        #[inline(always)]
        pub fn clkrdy(&self) -> CLKRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CLKRDYR { bits }
        }
        #[doc = "Bit 2 - DPLL Enable"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 3 - Divider Enable"]
        #[inline(always)]
        pub fn div(&self) -> DIVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            DIVR { bits }
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
    pub struct XOSCRDYR {
        bits: bool,
    }
    impl XOSCRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct XOSC32KRDYR {
        bits: bool,
    }
    impl XOSC32KRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct OSC32KRDYR {
        bits: bool,
    }
    impl OSC32KRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct OSC8MRDYR {
        bits: bool,
    }
    impl OSC8MRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLRDYR {
        bits: bool,
    }
    impl DFLLRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLOOBR {
        bits: bool,
    }
    impl DFLLOOBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLLCKFR {
        bits: bool,
    }
    impl DFLLLCKFR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLLCKCR {
        bits: bool,
    }
    impl DFLLLCKCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLRCSR {
        bits: bool,
    }
    impl DFLLRCSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct BOD33RDYR {
        bits: bool,
    }
    impl BOD33RDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct BOD33DETR {
        bits: bool,
    }
    impl BOD33DETR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct B33SRDYR {
        bits: bool,
    }
    impl B33SRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLCKRR {
        bits: bool,
    }
    impl DPLLLCKRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLCKFR {
        bits: bool,
    }
    impl DPLLLCKFR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLTOR {
        bits: bool,
    }
    impl DPLLLTOR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _XOSCRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _XOSCRDYW<'a> {
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
    pub struct _XOSC32KRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _XOSC32KRDYW<'a> {
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
    pub struct _OSC32KRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSC32KRDYW<'a> {
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
    pub struct _OSC8MRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSC8MRDYW<'a> {
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
    pub struct _DFLLRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLRDYW<'a> {
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
    pub struct _DFLLOOBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLOOBW<'a> {
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
    pub struct _DFLLLCKFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLLCKFW<'a> {
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
    pub struct _DFLLLCKCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLLCKCW<'a> {
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
    pub struct _DFLLRCSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLRCSW<'a> {
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
    pub struct _BOD33RDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BOD33RDYW<'a> {
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
    pub struct _BOD33DETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BOD33DETW<'a> {
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
    pub struct _B33SRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _B33SRDYW<'a> {
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
    pub struct _DPLLLCKRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DPLLLCKRW<'a> {
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
    pub struct _DPLLLCKFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DPLLLCKFW<'a> {
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
    pub struct _DPLLLTOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DPLLLTOW<'a> {
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
        #[inline(always)]
        pub fn xoscrdy(&self) -> XOSCRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            XOSCRDYR { bits }
        }
        #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
        #[inline(always)]
        pub fn xosc32krdy(&self) -> XOSC32KRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            XOSC32KRDYR { bits }
        }
        #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
        #[inline(always)]
        pub fn osc32krdy(&self) -> OSC32KRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OSC32KRDYR { bits }
        }
        #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
        #[inline(always)]
        pub fn osc8mrdy(&self) -> OSC8MRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OSC8MRDYR { bits }
        }
        #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
        #[inline(always)]
        pub fn dfllrdy(&self) -> DFLLRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLRDYR { bits }
        }
        #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
        #[inline(always)]
        pub fn dflloob(&self) -> DFLLOOBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLOOBR { bits }
        }
        #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
        #[inline(always)]
        pub fn dflllckf(&self) -> DFLLLCKFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLLCKFR { bits }
        }
        #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
        #[inline(always)]
        pub fn dflllckc(&self) -> DFLLLCKCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLLCKCR { bits }
        }
        #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
        #[inline(always)]
        pub fn dfllrcs(&self) -> DFLLRCSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLRCSR { bits }
        }
        #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
        #[inline(always)]
        pub fn bod33rdy(&self) -> BOD33RDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOD33RDYR { bits }
        }
        #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
        #[inline(always)]
        pub fn bod33det(&self) -> BOD33DETR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOD33DETR { bits }
        }
        #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
        #[inline(always)]
        pub fn b33srdy(&self) -> B33SRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            B33SRDYR { bits }
        }
        #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
        #[inline(always)]
        pub fn dplllckr(&self) -> DPLLLCKRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLCKRR { bits }
        }
        #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
        #[inline(always)]
        pub fn dplllckf(&self) -> DPLLLCKFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLCKFR { bits }
        }
        #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
        #[inline(always)]
        pub fn dplllto(&self) -> DPLLLTOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLTOR { bits }
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
        #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
        #[inline(always)]
        pub fn xoscrdy(&mut self) -> _XOSCRDYW {
            _XOSCRDYW { w: self }
        }
        #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
        #[inline(always)]
        pub fn xosc32krdy(&mut self) -> _XOSC32KRDYW {
            _XOSC32KRDYW { w: self }
        }
        #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
        #[inline(always)]
        pub fn osc32krdy(&mut self) -> _OSC32KRDYW {
            _OSC32KRDYW { w: self }
        }
        #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
        #[inline(always)]
        pub fn osc8mrdy(&mut self) -> _OSC8MRDYW {
            _OSC8MRDYW { w: self }
        }
        #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
        #[inline(always)]
        pub fn dfllrdy(&mut self) -> _DFLLRDYW {
            _DFLLRDYW { w: self }
        }
        #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
        #[inline(always)]
        pub fn dflloob(&mut self) -> _DFLLOOBW {
            _DFLLOOBW { w: self }
        }
        #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
        #[inline(always)]
        pub fn dflllckf(&mut self) -> _DFLLLCKFW {
            _DFLLLCKFW { w: self }
        }
        #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
        #[inline(always)]
        pub fn dflllckc(&mut self) -> _DFLLLCKCW {
            _DFLLLCKCW { w: self }
        }
        #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
        #[inline(always)]
        pub fn dfllrcs(&mut self) -> _DFLLRCSW {
            _DFLLRCSW { w: self }
        }
        #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
        #[inline(always)]
        pub fn bod33rdy(&mut self) -> _BOD33RDYW {
            _BOD33RDYW { w: self }
        }
        #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
        #[inline(always)]
        pub fn bod33det(&mut self) -> _BOD33DETW {
            _BOD33DETW { w: self }
        }
        #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
        #[inline(always)]
        pub fn b33srdy(&mut self) -> _B33SRDYW {
            _B33SRDYW { w: self }
        }
        #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
        #[inline(always)]
        pub fn dplllckr(&mut self) -> _DPLLLCKRW {
            _DPLLLCKRW { w: self }
        }
        #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
        #[inline(always)]
        pub fn dplllckf(&mut self) -> _DPLLLCKFW {
            _DPLLLCKFW { w: self }
        }
        #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
        #[inline(always)]
        pub fn dplllto(&mut self) -> _DPLLLTOW {
            _DPLLLTOW { w: self }
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
    pub struct XOSCRDYR {
        bits: bool,
    }
    impl XOSCRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct XOSC32KRDYR {
        bits: bool,
    }
    impl XOSC32KRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct OSC32KRDYR {
        bits: bool,
    }
    impl OSC32KRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct OSC8MRDYR {
        bits: bool,
    }
    impl OSC8MRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLRDYR {
        bits: bool,
    }
    impl DFLLRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLOOBR {
        bits: bool,
    }
    impl DFLLOOBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLLCKFR {
        bits: bool,
    }
    impl DFLLLCKFR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLLCKCR {
        bits: bool,
    }
    impl DFLLLCKCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLRCSR {
        bits: bool,
    }
    impl DFLLRCSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct BOD33RDYR {
        bits: bool,
    }
    impl BOD33RDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct BOD33DETR {
        bits: bool,
    }
    impl BOD33DETR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct B33SRDYR {
        bits: bool,
    }
    impl B33SRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLCKRR {
        bits: bool,
    }
    impl DPLLLCKRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLCKFR {
        bits: bool,
    }
    impl DPLLLCKFR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLTOR {
        bits: bool,
    }
    impl DPLLLTOR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _XOSCRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _XOSCRDYW<'a> {
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
    pub struct _XOSC32KRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _XOSC32KRDYW<'a> {
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
    pub struct _OSC32KRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSC32KRDYW<'a> {
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
    pub struct _OSC8MRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSC8MRDYW<'a> {
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
    pub struct _DFLLRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLRDYW<'a> {
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
    pub struct _DFLLOOBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLOOBW<'a> {
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
    pub struct _DFLLLCKFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLLCKFW<'a> {
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
    pub struct _DFLLLCKCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLLCKCW<'a> {
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
    pub struct _DFLLRCSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLRCSW<'a> {
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
    pub struct _BOD33RDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BOD33RDYW<'a> {
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
    pub struct _BOD33DETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BOD33DETW<'a> {
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
    pub struct _B33SRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _B33SRDYW<'a> {
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
    pub struct _DPLLLCKRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DPLLLCKRW<'a> {
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
    pub struct _DPLLLCKFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DPLLLCKFW<'a> {
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
    pub struct _DPLLLTOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DPLLLTOW<'a> {
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
        #[inline(always)]
        pub fn xoscrdy(&self) -> XOSCRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            XOSCRDYR { bits }
        }
        #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
        #[inline(always)]
        pub fn xosc32krdy(&self) -> XOSC32KRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            XOSC32KRDYR { bits }
        }
        #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
        #[inline(always)]
        pub fn osc32krdy(&self) -> OSC32KRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OSC32KRDYR { bits }
        }
        #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
        #[inline(always)]
        pub fn osc8mrdy(&self) -> OSC8MRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OSC8MRDYR { bits }
        }
        #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
        #[inline(always)]
        pub fn dfllrdy(&self) -> DFLLRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLRDYR { bits }
        }
        #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
        #[inline(always)]
        pub fn dflloob(&self) -> DFLLOOBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLOOBR { bits }
        }
        #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
        #[inline(always)]
        pub fn dflllckf(&self) -> DFLLLCKFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLLCKFR { bits }
        }
        #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
        #[inline(always)]
        pub fn dflllckc(&self) -> DFLLLCKCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLLCKCR { bits }
        }
        #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
        #[inline(always)]
        pub fn dfllrcs(&self) -> DFLLRCSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLRCSR { bits }
        }
        #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
        #[inline(always)]
        pub fn bod33rdy(&self) -> BOD33RDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOD33RDYR { bits }
        }
        #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
        #[inline(always)]
        pub fn bod33det(&self) -> BOD33DETR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOD33DETR { bits }
        }
        #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
        #[inline(always)]
        pub fn b33srdy(&self) -> B33SRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            B33SRDYR { bits }
        }
        #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
        #[inline(always)]
        pub fn dplllckr(&self) -> DPLLLCKRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLCKRR { bits }
        }
        #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
        #[inline(always)]
        pub fn dplllckf(&self) -> DPLLLCKFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLCKFR { bits }
        }
        #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
        #[inline(always)]
        pub fn dplllto(&self) -> DPLLLTOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLTOR { bits }
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
        #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
        #[inline(always)]
        pub fn xoscrdy(&mut self) -> _XOSCRDYW {
            _XOSCRDYW { w: self }
        }
        #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
        #[inline(always)]
        pub fn xosc32krdy(&mut self) -> _XOSC32KRDYW {
            _XOSC32KRDYW { w: self }
        }
        #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
        #[inline(always)]
        pub fn osc32krdy(&mut self) -> _OSC32KRDYW {
            _OSC32KRDYW { w: self }
        }
        #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
        #[inline(always)]
        pub fn osc8mrdy(&mut self) -> _OSC8MRDYW {
            _OSC8MRDYW { w: self }
        }
        #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
        #[inline(always)]
        pub fn dfllrdy(&mut self) -> _DFLLRDYW {
            _DFLLRDYW { w: self }
        }
        #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
        #[inline(always)]
        pub fn dflloob(&mut self) -> _DFLLOOBW {
            _DFLLOOBW { w: self }
        }
        #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
        #[inline(always)]
        pub fn dflllckf(&mut self) -> _DFLLLCKFW {
            _DFLLLCKFW { w: self }
        }
        #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
        #[inline(always)]
        pub fn dflllckc(&mut self) -> _DFLLLCKCW {
            _DFLLLCKCW { w: self }
        }
        #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
        #[inline(always)]
        pub fn dfllrcs(&mut self) -> _DFLLRCSW {
            _DFLLRCSW { w: self }
        }
        #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
        #[inline(always)]
        pub fn bod33rdy(&mut self) -> _BOD33RDYW {
            _BOD33RDYW { w: self }
        }
        #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
        #[inline(always)]
        pub fn bod33det(&mut self) -> _BOD33DETW {
            _BOD33DETW { w: self }
        }
        #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
        #[inline(always)]
        pub fn b33srdy(&mut self) -> _B33SRDYW {
            _B33SRDYW { w: self }
        }
        #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
        #[inline(always)]
        pub fn dplllckr(&mut self) -> _DPLLLCKRW {
            _DPLLLCKRW { w: self }
        }
        #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
        #[inline(always)]
        pub fn dplllckf(&mut self) -> _DPLLLCKFW {
            _DPLLLCKFW { w: self }
        }
        #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
        #[inline(always)]
        pub fn dplllto(&mut self) -> _DPLLLTOW {
            _DPLLLTOW { w: self }
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
    pub struct XOSCRDYR {
        bits: bool,
    }
    impl XOSCRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct XOSC32KRDYR {
        bits: bool,
    }
    impl XOSC32KRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct OSC32KRDYR {
        bits: bool,
    }
    impl OSC32KRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct OSC8MRDYR {
        bits: bool,
    }
    impl OSC8MRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLRDYR {
        bits: bool,
    }
    impl DFLLRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLOOBR {
        bits: bool,
    }
    impl DFLLOOBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLLCKFR {
        bits: bool,
    }
    impl DFLLLCKFR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLLCKCR {
        bits: bool,
    }
    impl DFLLLCKCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLRCSR {
        bits: bool,
    }
    impl DFLLRCSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct BOD33RDYR {
        bits: bool,
    }
    impl BOD33RDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct BOD33DETR {
        bits: bool,
    }
    impl BOD33DETR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct B33SRDYR {
        bits: bool,
    }
    impl B33SRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLCKRR {
        bits: bool,
    }
    impl DPLLLCKRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLCKFR {
        bits: bool,
    }
    impl DPLLLCKFR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLTOR {
        bits: bool,
    }
    impl DPLLLTOR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _XOSCRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _XOSCRDYW<'a> {
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
    pub struct _XOSC32KRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _XOSC32KRDYW<'a> {
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
    pub struct _OSC32KRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSC32KRDYW<'a> {
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
    pub struct _OSC8MRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _OSC8MRDYW<'a> {
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
    pub struct _DFLLRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLRDYW<'a> {
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
    pub struct _DFLLOOBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLOOBW<'a> {
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
    pub struct _DFLLLCKFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLLCKFW<'a> {
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
    pub struct _DFLLLCKCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLLCKCW<'a> {
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
    pub struct _DFLLRCSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DFLLRCSW<'a> {
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
    pub struct _BOD33RDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BOD33RDYW<'a> {
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
    pub struct _BOD33DETW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BOD33DETW<'a> {
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
    pub struct _B33SRDYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _B33SRDYW<'a> {
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
    pub struct _DPLLLCKRW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DPLLLCKRW<'a> {
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
    pub struct _DPLLLCKFW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DPLLLCKFW<'a> {
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
    pub struct _DPLLLTOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DPLLLTOW<'a> {
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
    impl R {
        #[doc = r" Value of the register as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        #[doc = "Bit 0 - XOSC Ready"]
        #[inline(always)]
        pub fn xoscrdy(&self) -> XOSCRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            XOSCRDYR { bits }
        }
        #[doc = "Bit 1 - XOSC32K Ready"]
        #[inline(always)]
        pub fn xosc32krdy(&self) -> XOSC32KRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            XOSC32KRDYR { bits }
        }
        #[doc = "Bit 2 - OSC32K Ready"]
        #[inline(always)]
        pub fn osc32krdy(&self) -> OSC32KRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OSC32KRDYR { bits }
        }
        #[doc = "Bit 3 - OSC8M Ready"]
        #[inline(always)]
        pub fn osc8mrdy(&self) -> OSC8MRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OSC8MRDYR { bits }
        }
        #[doc = "Bit 4 - DFLL Ready"]
        #[inline(always)]
        pub fn dfllrdy(&self) -> DFLLRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLRDYR { bits }
        }
        #[doc = "Bit 5 - DFLL Out Of Bounds"]
        #[inline(always)]
        pub fn dflloob(&self) -> DFLLOOBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLOOBR { bits }
        }
        #[doc = "Bit 6 - DFLL Lock Fine"]
        #[inline(always)]
        pub fn dflllckf(&self) -> DFLLLCKFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLLCKFR { bits }
        }
        #[doc = "Bit 7 - DFLL Lock Coarse"]
        #[inline(always)]
        pub fn dflllckc(&self) -> DFLLLCKCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLLCKCR { bits }
        }
        #[doc = "Bit 8 - DFLL Reference Clock Stopped"]
        #[inline(always)]
        pub fn dfllrcs(&self) -> DFLLRCSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLRCSR { bits }
        }
        #[doc = "Bit 9 - BOD33 Ready"]
        #[inline(always)]
        pub fn bod33rdy(&self) -> BOD33RDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOD33RDYR { bits }
        }
        #[doc = "Bit 10 - BOD33 Detection"]
        #[inline(always)]
        pub fn bod33det(&self) -> BOD33DETR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOD33DETR { bits }
        }
        #[doc = "Bit 11 - BOD33 Synchronization Ready"]
        #[inline(always)]
        pub fn b33srdy(&self) -> B33SRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            B33SRDYR { bits }
        }
        #[doc = "Bit 15 - DPLL Lock Rise"]
        #[inline(always)]
        pub fn dplllckr(&self) -> DPLLLCKRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLCKRR { bits }
        }
        #[doc = "Bit 16 - DPLL Lock Fall"]
        #[inline(always)]
        pub fn dplllckf(&self) -> DPLLLCKFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLCKFR { bits }
        }
        #[doc = "Bit 17 - DPLL Lock Timeout"]
        #[inline(always)]
        pub fn dplllto(&self) -> DPLLLTOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLTOR { bits }
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
        #[doc = "Bit 0 - XOSC Ready"]
        #[inline(always)]
        pub fn xoscrdy(&mut self) -> _XOSCRDYW {
            _XOSCRDYW { w: self }
        }
        #[doc = "Bit 1 - XOSC32K Ready"]
        #[inline(always)]
        pub fn xosc32krdy(&mut self) -> _XOSC32KRDYW {
            _XOSC32KRDYW { w: self }
        }
        #[doc = "Bit 2 - OSC32K Ready"]
        #[inline(always)]
        pub fn osc32krdy(&mut self) -> _OSC32KRDYW {
            _OSC32KRDYW { w: self }
        }
        #[doc = "Bit 3 - OSC8M Ready"]
        #[inline(always)]
        pub fn osc8mrdy(&mut self) -> _OSC8MRDYW {
            _OSC8MRDYW { w: self }
        }
        #[doc = "Bit 4 - DFLL Ready"]
        #[inline(always)]
        pub fn dfllrdy(&mut self) -> _DFLLRDYW {
            _DFLLRDYW { w: self }
        }
        #[doc = "Bit 5 - DFLL Out Of Bounds"]
        #[inline(always)]
        pub fn dflloob(&mut self) -> _DFLLOOBW {
            _DFLLOOBW { w: self }
        }
        #[doc = "Bit 6 - DFLL Lock Fine"]
        #[inline(always)]
        pub fn dflllckf(&mut self) -> _DFLLLCKFW {
            _DFLLLCKFW { w: self }
        }
        #[doc = "Bit 7 - DFLL Lock Coarse"]
        #[inline(always)]
        pub fn dflllckc(&mut self) -> _DFLLLCKCW {
            _DFLLLCKCW { w: self }
        }
        #[doc = "Bit 8 - DFLL Reference Clock Stopped"]
        #[inline(always)]
        pub fn dfllrcs(&mut self) -> _DFLLRCSW {
            _DFLLRCSW { w: self }
        }
        #[doc = "Bit 9 - BOD33 Ready"]
        #[inline(always)]
        pub fn bod33rdy(&mut self) -> _BOD33RDYW {
            _BOD33RDYW { w: self }
        }
        #[doc = "Bit 10 - BOD33 Detection"]
        #[inline(always)]
        pub fn bod33det(&mut self) -> _BOD33DETW {
            _BOD33DETW { w: self }
        }
        #[doc = "Bit 11 - BOD33 Synchronization Ready"]
        #[inline(always)]
        pub fn b33srdy(&mut self) -> _B33SRDYW {
            _B33SRDYW { w: self }
        }
        #[doc = "Bit 15 - DPLL Lock Rise"]
        #[inline(always)]
        pub fn dplllckr(&mut self) -> _DPLLLCKRW {
            _DPLLLCKRW { w: self }
        }
        #[doc = "Bit 16 - DPLL Lock Fall"]
        #[inline(always)]
        pub fn dplllckf(&mut self) -> _DPLLLCKFW {
            _DPLLLCKFW { w: self }
        }
        #[doc = "Bit 17 - DPLL Lock Timeout"]
        #[inline(always)]
        pub fn dplllto(&mut self) -> _DPLLLTOW {
            _DPLLLTOW { w: self }
        }
    }
}
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub struct OSCULP32K {
    register: VolatileCell<u8>,
}
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub mod osculp32k {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
    }
    impl super::OSCULP32K {
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
    pub struct CALIBR {
        bits: u8,
    }
    impl CALIBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
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
    pub struct _CALIBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CALIBW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u8) << OFFSET);
            self.w.bits |= ((value & MASK) as u8) << OFFSET;
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
        #[doc = "Bits 0:4 - Oscillator Calibration"]
        #[inline(always)]
        pub fn calib(&self) -> CALIBR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u8) as u8
            };
            CALIBR { bits }
        }
        #[doc = "Bit 7 - Write Lock"]
        #[inline(always)]
        pub fn wrtlock(&self) -> WRTLOCKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            WRTLOCKR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 31 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bits 0:4 - Oscillator Calibration"]
        #[inline(always)]
        pub fn calib(&mut self) -> _CALIBW {
            _CALIBW { w: self }
        }
        #[doc = "Bit 7 - Write Lock"]
        #[inline(always)]
        pub fn wrtlock(&mut self) -> _WRTLOCKW {
            _WRTLOCKW { w: self }
        }
    }
}
#[doc = "8MHz Internal Oscillator (OSC8M) Control"]
pub struct OSC8M {
    register: VolatileCell<u32>,
}
#[doc = "8MHz Internal Oscillator (OSC8M) Control"]
pub mod osc8m {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::OSC8M {
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
    #[doc = r" Value of the field"]
    pub struct ONDEMANDR {
        bits: bool,
    }
    impl ONDEMANDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `PRESC`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PRESCR {
        #[doc = "1"]
        _0X0,
        #[doc = "2"]
        _0X1,
        #[doc = "4"]
        _0X2,
        #[doc = "8"]
        _0X3,
    }
    impl PRESCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                PRESCR::_0X0 => 0,
                PRESCR::_0X1 => 1,
                PRESCR::_0X2 => 2,
                PRESCR::_0X3 => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> PRESCR {
            match value {
                0 => PRESCR::_0X0,
                1 => PRESCR::_0X1,
                2 => PRESCR::_0X2,
                3 => PRESCR::_0X3,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `_0X0`"]
        #[inline(always)]
        pub fn is_0x0(&self) -> bool {
            *self == PRESCR::_0X0
        }
        #[doc = "Checks if the value of the field is `_0X1`"]
        #[inline(always)]
        pub fn is_0x1(&self) -> bool {
            *self == PRESCR::_0X1
        }
        #[doc = "Checks if the value of the field is `_0X2`"]
        #[inline(always)]
        pub fn is_0x2(&self) -> bool {
            *self == PRESCR::_0X2
        }
        #[doc = "Checks if the value of the field is `_0X3`"]
        #[inline(always)]
        pub fn is_0x3(&self) -> bool {
            *self == PRESCR::_0X3
        }
    }
    #[doc = r" Value of the field"]
    pub struct CALIBR {
        bits: u16,
    }
    impl CALIBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = "Possible values of the field `FRANGE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum FRANGER {
        #[doc = "4 to 6MHz"]
        _0X0,
        #[doc = "6 to 8MHz"]
        _0X1,
        #[doc = "8 to 11MHz"]
        _0X2,
        #[doc = "11 to 15MHz"]
        _0X3,
    }
    impl FRANGER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                FRANGER::_0X0 => 0,
                FRANGER::_0X1 => 1,
                FRANGER::_0X2 => 2,
                FRANGER::_0X3 => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> FRANGER {
            match value {
                0 => FRANGER::_0X0,
                1 => FRANGER::_0X1,
                2 => FRANGER::_0X2,
                3 => FRANGER::_0X3,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `_0X0`"]
        #[inline(always)]
        pub fn is_0x0(&self) -> bool {
            *self == FRANGER::_0X0
        }
        #[doc = "Checks if the value of the field is `_0X1`"]
        #[inline(always)]
        pub fn is_0x1(&self) -> bool {
            *self == FRANGER::_0X1
        }
        #[doc = "Checks if the value of the field is `_0X2`"]
        #[inline(always)]
        pub fn is_0x2(&self) -> bool {
            *self == FRANGER::_0X2
        }
        #[doc = "Checks if the value of the field is `_0X3`"]
        #[inline(always)]
        pub fn is_0x3(&self) -> bool {
            *self == FRANGER::_0X3
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ONDEMANDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ONDEMANDW<'a> {
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
    #[doc = "Values that can be written to the field `PRESC`"]
    pub enum PRESCW {
        #[doc = "1"]
        _0X0,
        #[doc = "2"]
        _0X1,
        #[doc = "4"]
        _0X2,
        #[doc = "8"]
        _0X3,
    }
    impl PRESCW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                PRESCW::_0X0 => 0,
                PRESCW::_0X1 => 1,
                PRESCW::_0X2 => 2,
                PRESCW::_0X3 => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _PRESCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _PRESCW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: PRESCW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "1"]
        #[inline(always)]
        pub fn _0x0(self) -> &'a mut W {
            self.variant(PRESCW::_0X0)
        }
        #[doc = "2"]
        #[inline(always)]
        pub fn _0x1(self) -> &'a mut W {
            self.variant(PRESCW::_0X1)
        }
        #[doc = "4"]
        #[inline(always)]
        pub fn _0x2(self) -> &'a mut W {
            self.variant(PRESCW::_0X2)
        }
        #[doc = "8"]
        #[inline(always)]
        pub fn _0x3(self) -> &'a mut W {
            self.variant(PRESCW::_0X3)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CALIBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CALIBW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `FRANGE`"]
    pub enum FRANGEW {
        #[doc = "4 to 6MHz"]
        _0X0,
        #[doc = "6 to 8MHz"]
        _0X1,
        #[doc = "8 to 11MHz"]
        _0X2,
        #[doc = "11 to 15MHz"]
        _0X3,
    }
    impl FRANGEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                FRANGEW::_0X0 => 0,
                FRANGEW::_0X1 => 1,
                FRANGEW::_0X2 => 2,
                FRANGEW::_0X3 => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _FRANGEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FRANGEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: FRANGEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "4 to 6MHz"]
        #[inline(always)]
        pub fn _0x0(self) -> &'a mut W {
            self.variant(FRANGEW::_0X0)
        }
        #[doc = "6 to 8MHz"]
        #[inline(always)]
        pub fn _0x1(self) -> &'a mut W {
            self.variant(FRANGEW::_0X1)
        }
        #[doc = "8 to 11MHz"]
        #[inline(always)]
        pub fn _0x2(self) -> &'a mut W {
            self.variant(FRANGEW::_0X2)
        }
        #[doc = "11 to 15MHz"]
        #[inline(always)]
        pub fn _0x3(self) -> &'a mut W {
            self.variant(FRANGEW::_0X3)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
        #[doc = "Bit 1 - Oscillator Enable"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&self) -> RUNSTDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RUNSTDBYR { bits }
        }
        #[doc = "Bit 7 - On Demand Control"]
        #[inline(always)]
        pub fn ondemand(&self) -> ONDEMANDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ONDEMANDR { bits }
        }
        #[doc = "Bits 8:9 - Oscillator Prescaler"]
        #[inline(always)]
        pub fn presc(&self) -> PRESCR {
            PRESCR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 16:27 - Oscillator Calibration"]
        #[inline(always)]
        pub fn calib(&self) -> CALIBR {
            let bits = {
                const MASK: u16 = 4095;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            CALIBR { bits }
        }
        #[doc = "Bits 30:31 - Oscillator Frequency Range"]
        #[inline(always)]
        pub fn frange(&self) -> FRANGER {
            FRANGER::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 2265383810 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 1 - Oscillator Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&mut self) -> _RUNSTDBYW {
            _RUNSTDBYW { w: self }
        }
        #[doc = "Bit 7 - On Demand Control"]
        #[inline(always)]
        pub fn ondemand(&mut self) -> _ONDEMANDW {
            _ONDEMANDW { w: self }
        }
        #[doc = "Bits 8:9 - Oscillator Prescaler"]
        #[inline(always)]
        pub fn presc(&mut self) -> _PRESCW {
            _PRESCW { w: self }
        }
        #[doc = "Bits 16:27 - Oscillator Calibration"]
        #[inline(always)]
        pub fn calib(&mut self) -> _CALIBW {
            _CALIBW { w: self }
        }
        #[doc = "Bits 30:31 - Oscillator Frequency Range"]
        #[inline(always)]
        pub fn frange(&mut self) -> _FRANGEW {
            _FRANGEW { w: self }
        }
    }
}
#[doc = "32kHz Internal Oscillator (OSC32K) Control"]
pub struct OSC32K {
    register: VolatileCell<u32>,
}
#[doc = "32kHz Internal Oscillator (OSC32K) Control"]
pub mod osc32k {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::OSC32K {
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
    pub struct EN32KR {
        bits: bool,
    }
    impl EN32KR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct EN1KR {
        bits: bool,
    }
    impl EN1KR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    #[doc = r" Value of the field"]
    pub struct ONDEMANDR {
        bits: bool,
    }
    impl ONDEMANDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct STARTUPR {
        bits: u8,
    }
    impl STARTUPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
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
    #[doc = r" Value of the field"]
    pub struct CALIBR {
        bits: u8,
    }
    impl CALIBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
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
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _EN32KW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EN32KW<'a> {
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
    pub struct _EN1KW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EN1KW<'a> {
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ONDEMANDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ONDEMANDW<'a> {
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
    pub struct _STARTUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STARTUPW<'a> {
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
            const OFFSET: u8 = 12;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _CALIBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CALIBW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
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
        #[doc = "Bit 1 - Oscillator Enable"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 2 - 32kHz Output Enable"]
        #[inline(always)]
        pub fn en32k(&self) -> EN32KR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EN32KR { bits }
        }
        #[doc = "Bit 3 - 1kHz Output Enable"]
        #[inline(always)]
        pub fn en1k(&self) -> EN1KR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            EN1KR { bits }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&self) -> RUNSTDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RUNSTDBYR { bits }
        }
        #[doc = "Bit 7 - On Demand Control"]
        #[inline(always)]
        pub fn ondemand(&self) -> ONDEMANDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            ONDEMANDR { bits }
        }
        #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
        #[inline(always)]
        pub fn startup(&self) -> STARTUPR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            STARTUPR { bits }
        }
        #[doc = "Bit 12 - Write Lock"]
        #[inline(always)]
        pub fn wrtlock(&self) -> WRTLOCKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            WRTLOCKR { bits }
        }
        #[doc = "Bits 16:22 - Oscillator Calibration"]
        #[inline(always)]
        pub fn calib(&self) -> CALIBR {
            let bits = {
                const MASK: u8 = 127;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            CALIBR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 4128896 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 1 - Oscillator Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
        #[doc = "Bit 2 - 32kHz Output Enable"]
        #[inline(always)]
        pub fn en32k(&mut self) -> _EN32KW {
            _EN32KW { w: self }
        }
        #[doc = "Bit 3 - 1kHz Output Enable"]
        #[inline(always)]
        pub fn en1k(&mut self) -> _EN1KW {
            _EN1KW { w: self }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&mut self) -> _RUNSTDBYW {
            _RUNSTDBYW { w: self }
        }
        #[doc = "Bit 7 - On Demand Control"]
        #[inline(always)]
        pub fn ondemand(&mut self) -> _ONDEMANDW {
            _ONDEMANDW { w: self }
        }
        #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
        #[inline(always)]
        pub fn startup(&mut self) -> _STARTUPW {
            _STARTUPW { w: self }
        }
        #[doc = "Bit 12 - Write Lock"]
        #[inline(always)]
        pub fn wrtlock(&mut self) -> _WRTLOCKW {
            _WRTLOCKW { w: self }
        }
        #[doc = "Bits 16:22 - Oscillator Calibration"]
        #[inline(always)]
        pub fn calib(&mut self) -> _CALIBW {
            _CALIBW { w: self }
        }
    }
}
#[doc = "Power and Clocks Status"]
pub struct PCLKSR {
    register: VolatileCell<u32>,
}
#[doc = "Power and Clocks Status"]
pub mod pclksr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    impl super::PCLKSR {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
    }
    #[doc = r" Value of the field"]
    pub struct XOSCRDYR {
        bits: bool,
    }
    impl XOSCRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct XOSC32KRDYR {
        bits: bool,
    }
    impl XOSC32KRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct OSC32KRDYR {
        bits: bool,
    }
    impl OSC32KRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct OSC8MRDYR {
        bits: bool,
    }
    impl OSC8MRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLRDYR {
        bits: bool,
    }
    impl DFLLRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLOOBR {
        bits: bool,
    }
    impl DFLLOOBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLLCKFR {
        bits: bool,
    }
    impl DFLLLCKFR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLLCKCR {
        bits: bool,
    }
    impl DFLLLCKCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DFLLRCSR {
        bits: bool,
    }
    impl DFLLRCSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct BOD33RDYR {
        bits: bool,
    }
    impl BOD33RDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct BOD33DETR {
        bits: bool,
    }
    impl BOD33DETR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct B33SRDYR {
        bits: bool,
    }
    impl B33SRDYR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLCKRR {
        bits: bool,
    }
    impl DPLLLCKRR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLCKFR {
        bits: bool,
    }
    impl DPLLLCKFR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DPLLLTOR {
        bits: bool,
    }
    impl DPLLLTOR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
        #[doc = "Bit 0 - XOSC Ready"]
        #[inline(always)]
        pub fn xoscrdy(&self) -> XOSCRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            XOSCRDYR { bits }
        }
        #[doc = "Bit 1 - XOSC32K Ready"]
        #[inline(always)]
        pub fn xosc32krdy(&self) -> XOSC32KRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            XOSC32KRDYR { bits }
        }
        #[doc = "Bit 2 - OSC32K Ready"]
        #[inline(always)]
        pub fn osc32krdy(&self) -> OSC32KRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OSC32KRDYR { bits }
        }
        #[doc = "Bit 3 - OSC8M Ready"]
        #[inline(always)]
        pub fn osc8mrdy(&self) -> OSC8MRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            OSC8MRDYR { bits }
        }
        #[doc = "Bit 4 - DFLL Ready"]
        #[inline(always)]
        pub fn dfllrdy(&self) -> DFLLRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLRDYR { bits }
        }
        #[doc = "Bit 5 - DFLL Out Of Bounds"]
        #[inline(always)]
        pub fn dflloob(&self) -> DFLLOOBR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLOOBR { bits }
        }
        #[doc = "Bit 6 - DFLL Lock Fine"]
        #[inline(always)]
        pub fn dflllckf(&self) -> DFLLLCKFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLLCKFR { bits }
        }
        #[doc = "Bit 7 - DFLL Lock Coarse"]
        #[inline(always)]
        pub fn dflllckc(&self) -> DFLLLCKCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLLCKCR { bits }
        }
        #[doc = "Bit 8 - DFLL Reference Clock Stopped"]
        #[inline(always)]
        pub fn dfllrcs(&self) -> DFLLRCSR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DFLLRCSR { bits }
        }
        #[doc = "Bit 9 - BOD33 Ready"]
        #[inline(always)]
        pub fn bod33rdy(&self) -> BOD33RDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOD33RDYR { bits }
        }
        #[doc = "Bit 10 - BOD33 Detection"]
        #[inline(always)]
        pub fn bod33det(&self) -> BOD33DETR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BOD33DETR { bits }
        }
        #[doc = "Bit 11 - BOD33 Synchronization Ready"]
        #[inline(always)]
        pub fn b33srdy(&self) -> B33SRDYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            B33SRDYR { bits }
        }
        #[doc = "Bit 15 - DPLL Lock Rise"]
        #[inline(always)]
        pub fn dplllckr(&self) -> DPLLLCKRR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLCKRR { bits }
        }
        #[doc = "Bit 16 - DPLL Lock Fall"]
        #[inline(always)]
        pub fn dplllckf(&self) -> DPLLLCKFR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLCKFR { bits }
        }
        #[doc = "Bit 17 - DPLL Lock Timeout"]
        #[inline(always)]
        pub fn dplllto(&self) -> DPLLLTOR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            DPLLLTOR { bits }
        }
    }
}
#[doc = "Voltage References System (VREF) Control"]
pub struct VREF {
    register: VolatileCell<u32>,
}
#[doc = "Voltage References System (VREF) Control"]
pub mod vref {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::VREF {
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
    pub struct TSENR {
        bits: bool,
    }
    impl TSENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct BGOUTENR {
        bits: bool,
    }
    impl BGOUTENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct CALIBR {
        bits: u16,
    }
    impl CALIBR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    #[doc = r" Proxy"]
    pub struct _TSENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TSENW<'a> {
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
    pub struct _BGOUTENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BGOUTENW<'a> {
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
    pub struct _CALIBW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CALIBW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'a mut W {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
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
        #[doc = "Bit 1 - Temperature Sensor Enable"]
        #[inline(always)]
        pub fn tsen(&self) -> TSENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            TSENR { bits }
        }
        #[doc = "Bit 2 - Bandgap Output Enable"]
        #[inline(always)]
        pub fn bgouten(&self) -> BGOUTENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            BGOUTENR { bits }
        }
        #[doc = "Bits 16:26 - Bandgap Voltage Generator Calibration"]
        #[inline(always)]
        pub fn calib(&self) -> CALIBR {
            let bits = {
                const MASK: u16 = 2047;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) as u16
            };
            CALIBR { bits }
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
        #[doc = "Bit 1 - Temperature Sensor Enable"]
        #[inline(always)]
        pub fn tsen(&mut self) -> _TSENW {
            _TSENW { w: self }
        }
        #[doc = "Bit 2 - Bandgap Output Enable"]
        #[inline(always)]
        pub fn bgouten(&mut self) -> _BGOUTENW {
            _BGOUTENW { w: self }
        }
        #[doc = "Bits 16:26 - Bandgap Voltage Generator Calibration"]
        #[inline(always)]
        pub fn calib(&mut self) -> _CALIBW {
            _CALIBW { w: self }
        }
    }
}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub struct XOSC {
    register: VolatileCell<u16>,
}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xosc {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::XOSC {
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
    pub struct XTALENR {
        bits: bool,
    }
    impl XTALENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    #[doc = r" Value of the field"]
    pub struct ONDEMANDR {
        bits: bool,
    }
    impl ONDEMANDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `GAIN`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum GAINR {
        #[doc = "2MHz"]
        _0X0,
        #[doc = "4MHz"]
        _0X1,
        #[doc = "8MHz"]
        _0X2,
        #[doc = "16MHz"]
        _0X3,
        #[doc = "30MHz"]
        _0X4,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl GAINR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                GAINR::_0X0 => 0,
                GAINR::_0X1 => 1,
                GAINR::_0X2 => 2,
                GAINR::_0X3 => 3,
                GAINR::_0X4 => 4,
                GAINR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> GAINR {
            match value {
                0 => GAINR::_0X0,
                1 => GAINR::_0X1,
                2 => GAINR::_0X2,
                3 => GAINR::_0X3,
                4 => GAINR::_0X4,
                i => GAINR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `_0X0`"]
        #[inline(always)]
        pub fn is_0x0(&self) -> bool {
            *self == GAINR::_0X0
        }
        #[doc = "Checks if the value of the field is `_0X1`"]
        #[inline(always)]
        pub fn is_0x1(&self) -> bool {
            *self == GAINR::_0X1
        }
        #[doc = "Checks if the value of the field is `_0X2`"]
        #[inline(always)]
        pub fn is_0x2(&self) -> bool {
            *self == GAINR::_0X2
        }
        #[doc = "Checks if the value of the field is `_0X3`"]
        #[inline(always)]
        pub fn is_0x3(&self) -> bool {
            *self == GAINR::_0X3
        }
        #[doc = "Checks if the value of the field is `_0X4`"]
        #[inline(always)]
        pub fn is_0x4(&self) -> bool {
            *self == GAINR::_0X4
        }
    }
    #[doc = r" Value of the field"]
    pub struct AMPGCR {
        bits: bool,
    }
    impl AMPGCR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct STARTUPR {
        bits: u8,
    }
    impl STARTUPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _XTALENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _XTALENW<'a> {
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ONDEMANDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ONDEMANDW<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `GAIN`"]
    pub enum GAINW {
        #[doc = "2MHz"]
        _0X0,
        #[doc = "4MHz"]
        _0X1,
        #[doc = "8MHz"]
        _0X2,
        #[doc = "16MHz"]
        _0X3,
        #[doc = "30MHz"]
        _0X4,
    }
    impl GAINW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                GAINW::_0X0 => 0,
                GAINW::_0X1 => 1,
                GAINW::_0X2 => 2,
                GAINW::_0X3 => 3,
                GAINW::_0X4 => 4,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _GAINW<'a> {
        w: &'a mut W,
    }
    impl<'a> _GAINW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: GAINW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "2MHz"]
        #[inline(always)]
        pub fn _0x0(self) -> &'a mut W {
            self.variant(GAINW::_0X0)
        }
        #[doc = "4MHz"]
        #[inline(always)]
        pub fn _0x1(self) -> &'a mut W {
            self.variant(GAINW::_0X1)
        }
        #[doc = "8MHz"]
        #[inline(always)]
        pub fn _0x2(self) -> &'a mut W {
            self.variant(GAINW::_0X2)
        }
        #[doc = "16MHz"]
        #[inline(always)]
        pub fn _0x3(self) -> &'a mut W {
            self.variant(GAINW::_0X3)
        }
        #[doc = "30MHz"]
        #[inline(always)]
        pub fn _0x4(self) -> &'a mut W {
            self.variant(GAINW::_0X4)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _AMPGCW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AMPGCW<'a> {
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
    #[doc = r" Proxy"]
    pub struct _STARTUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STARTUPW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
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
        #[doc = "Bit 1 - Oscillator Enable"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 2 - Crystal Oscillator Enable"]
        #[inline(always)]
        pub fn xtalen(&self) -> XTALENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            XTALENR { bits }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&self) -> RUNSTDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RUNSTDBYR { bits }
        }
        #[doc = "Bit 7 - On Demand Control"]
        #[inline(always)]
        pub fn ondemand(&self) -> ONDEMANDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            ONDEMANDR { bits }
        }
        #[doc = "Bits 8:10 - Oscillator Gain"]
        #[inline(always)]
        pub fn gain(&self) -> GAINR {
            GAINR::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            })
        }
        #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
        #[inline(always)]
        pub fn ampgc(&self) -> AMPGCR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            AMPGCR { bits }
        }
        #[doc = "Bits 12:15 - Start-Up Time"]
        #[inline(always)]
        pub fn startup(&self) -> STARTUPR {
            let bits = {
                const MASK: u8 = 15;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            };
            STARTUPR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 128 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 1 - Oscillator Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
        #[doc = "Bit 2 - Crystal Oscillator Enable"]
        #[inline(always)]
        pub fn xtalen(&mut self) -> _XTALENW {
            _XTALENW { w: self }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&mut self) -> _RUNSTDBYW {
            _RUNSTDBYW { w: self }
        }
        #[doc = "Bit 7 - On Demand Control"]
        #[inline(always)]
        pub fn ondemand(&mut self) -> _ONDEMANDW {
            _ONDEMANDW { w: self }
        }
        #[doc = "Bits 8:10 - Oscillator Gain"]
        #[inline(always)]
        pub fn gain(&mut self) -> _GAINW {
            _GAINW { w: self }
        }
        #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
        #[inline(always)]
        pub fn ampgc(&mut self) -> _AMPGCW {
            _AMPGCW { w: self }
        }
        #[doc = "Bits 12:15 - Start-Up Time"]
        #[inline(always)]
        pub fn startup(&mut self) -> _STARTUPW {
            _STARTUPW { w: self }
        }
    }
}
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub struct XOSC32K {
    register: VolatileCell<u16>,
}
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub mod xosc32k {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
    }
    impl super::XOSC32K {
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
    pub struct XTALENR {
        bits: bool,
    }
    impl XTALENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct EN32KR {
        bits: bool,
    }
    impl EN32KR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct EN1KR {
        bits: bool,
    }
    impl EN1KR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct AAMPENR {
        bits: bool,
    }
    impl AAMPENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    #[doc = r" Value of the field"]
    pub struct ONDEMANDR {
        bits: bool,
    }
    impl ONDEMANDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct STARTUPR {
        bits: u8,
    }
    impl STARTUPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _XTALENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _XTALENW<'a> {
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
    pub struct _EN32KW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EN32KW<'a> {
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
    pub struct _EN1KW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EN1KW<'a> {
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
    #[doc = r" Proxy"]
    pub struct _AAMPENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _AAMPENW<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
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
            const OFFSET: u8 = 6;
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _ONDEMANDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _ONDEMANDW<'a> {
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
            self.w.bits &= !((MASK as u16) << OFFSET);
            self.w.bits |= ((value & MASK) as u16) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _STARTUPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _STARTUPW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
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
            const OFFSET: u8 = 12;
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
        #[doc = "Bit 1 - Oscillator Enable"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 2 - Crystal Oscillator Enable"]
        #[inline(always)]
        pub fn xtalen(&self) -> XTALENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            XTALENR { bits }
        }
        #[doc = "Bit 3 - 32kHz Output Enable"]
        #[inline(always)]
        pub fn en32k(&self) -> EN32KR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            EN32KR { bits }
        }
        #[doc = "Bit 4 - 1kHz Output Enable"]
        #[inline(always)]
        pub fn en1k(&self) -> EN1KR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            EN1KR { bits }
        }
        #[doc = "Bit 5 - Automatic Amplitude Control Enable"]
        #[inline(always)]
        pub fn aampen(&self) -> AAMPENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            AAMPENR { bits }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&self) -> RUNSTDBYR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RUNSTDBYR { bits }
        }
        #[doc = "Bit 7 - On Demand Control"]
        #[inline(always)]
        pub fn ondemand(&self) -> ONDEMANDR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            ONDEMANDR { bits }
        }
        #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
        #[inline(always)]
        pub fn startup(&self) -> STARTUPR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) as u8
            };
            STARTUPR { bits }
        }
        #[doc = "Bit 12 - Write Lock"]
        #[inline(always)]
        pub fn wrtlock(&self) -> WRTLOCKR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            WRTLOCKR { bits }
        }
    }
    impl W {
        #[doc = r" Reset value of the register"]
        #[inline(always)]
        pub fn reset_value() -> W {
            W { bits: 128 }
        }
        #[doc = r" Writes raw bits to the register"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
        #[doc = "Bit 1 - Oscillator Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
        #[doc = "Bit 2 - Crystal Oscillator Enable"]
        #[inline(always)]
        pub fn xtalen(&mut self) -> _XTALENW {
            _XTALENW { w: self }
        }
        #[doc = "Bit 3 - 32kHz Output Enable"]
        #[inline(always)]
        pub fn en32k(&mut self) -> _EN32KW {
            _EN32KW { w: self }
        }
        #[doc = "Bit 4 - 1kHz Output Enable"]
        #[inline(always)]
        pub fn en1k(&mut self) -> _EN1KW {
            _EN1KW { w: self }
        }
        #[doc = "Bit 5 - Automatic Amplitude Control Enable"]
        #[inline(always)]
        pub fn aampen(&mut self) -> _AAMPENW {
            _AAMPENW { w: self }
        }
        #[doc = "Bit 6 - Run in Standby"]
        #[inline(always)]
        pub fn runstdby(&mut self) -> _RUNSTDBYW {
            _RUNSTDBYW { w: self }
        }
        #[doc = "Bit 7 - On Demand Control"]
        #[inline(always)]
        pub fn ondemand(&mut self) -> _ONDEMANDW {
            _ONDEMANDW { w: self }
        }
        #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
        #[inline(always)]
        pub fn startup(&mut self) -> _STARTUPW {
            _STARTUPW { w: self }
        }
        #[doc = "Bit 12 - Write Lock"]
        #[inline(always)]
        pub fn wrtlock(&mut self) -> _WRTLOCKW {
            _WRTLOCKW { w: self }
        }
    }
}
