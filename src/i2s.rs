use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - Clock Unit n Control"]
    pub clkctrl0: CLKCTRL,
    #[doc = "0x08 - Clock Unit n Control"]
    pub clkctrl1: CLKCTRL,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved1: [u8; 2usize],
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved2: [u8; 2usize],
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved3: [u8; 2usize],
    #[doc = "0x18 - Synchronization Status"]
    pub syncbusy: SYNCBUSY,
    _reserved4: [u8; 6usize],
    #[doc = "0x20 - Serializer n Control"]
    pub serctrl0: SERCTRL,
    #[doc = "0x24 - Serializer n Control"]
    pub serctrl1: SERCTRL,
    _reserved5: [u8; 8usize],
    #[doc = "0x30 - Data n"]
    pub data0: DATA,
    #[doc = "0x34 - Data n"]
    pub data1: DATA,
}
#[doc = "Clock Unit n Control"]
pub struct CLKCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Clock Unit n Control"]
pub mod clkctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
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
    #[doc = "Possible values of the field `SLOTSIZE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SLOTSIZER {
        #[doc = "8-bit Slot for Clock Unit n"]
        _8,
        #[doc = "16-bit Slot for Clock Unit n"]
        _16,
        #[doc = "24-bit Slot for Clock Unit n"]
        _24,
        #[doc = "32-bit Slot for Clock Unit n"]
        _32,
    }
    impl SLOTSIZER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SLOTSIZER::_8 => 0,
                SLOTSIZER::_16 => 1,
                SLOTSIZER::_24 => 2,
                SLOTSIZER::_32 => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SLOTSIZER {
            match value {
                0 => SLOTSIZER::_8,
                1 => SLOTSIZER::_16,
                2 => SLOTSIZER::_24,
                3 => SLOTSIZER::_32,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `_8`"]
        #[inline(always)]
        pub fn is_8(&self) -> bool {
            *self == SLOTSIZER::_8
        }
        #[doc = "Checks if the value of the field is `_16`"]
        #[inline(always)]
        pub fn is_16(&self) -> bool {
            *self == SLOTSIZER::_16
        }
        #[doc = "Checks if the value of the field is `_24`"]
        #[inline(always)]
        pub fn is_24(&self) -> bool {
            *self == SLOTSIZER::_24
        }
        #[doc = "Checks if the value of the field is `_32`"]
        #[inline(always)]
        pub fn is_32(&self) -> bool {
            *self == SLOTSIZER::_32
        }
    }
    #[doc = r" Value of the field"]
    pub struct NBSLOTSR {
        bits: u8,
    }
    impl NBSLOTSR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = "Possible values of the field `FSWIDTH`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum FSWIDTHR {
        #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
        SLOT,
        #[doc = "Frame Sync Pulse is half a Frame wide"]
        HALF,
        #[doc = "Frame Sync Pulse is 1 Bit wide"]
        BIT,
        #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
        BURST,
    }
    impl FSWIDTHR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                FSWIDTHR::SLOT => 0,
                FSWIDTHR::HALF => 1,
                FSWIDTHR::BIT => 2,
                FSWIDTHR::BURST => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> FSWIDTHR {
            match value {
                0 => FSWIDTHR::SLOT,
                1 => FSWIDTHR::HALF,
                2 => FSWIDTHR::BIT,
                3 => FSWIDTHR::BURST,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `SLOT`"]
        #[inline(always)]
        pub fn is_slot(&self) -> bool {
            *self == FSWIDTHR::SLOT
        }
        #[doc = "Checks if the value of the field is `HALF`"]
        #[inline(always)]
        pub fn is_half(&self) -> bool {
            *self == FSWIDTHR::HALF
        }
        #[doc = "Checks if the value of the field is `BIT`"]
        #[inline(always)]
        pub fn is_bit_(&self) -> bool {
            *self == FSWIDTHR::BIT
        }
        #[doc = "Checks if the value of the field is `BURST`"]
        #[inline(always)]
        pub fn is_burst(&self) -> bool {
            *self == FSWIDTHR::BURST
        }
    }
    #[doc = "Possible values of the field `BITDELAY`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum BITDELAYR {
        #[doc = "Left Justified (0 Bit Delay)"]
        LJ,
        #[doc = "I2S (1 Bit Delay)"]
        I2S,
    }
    impl BITDELAYR {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                BITDELAYR::LJ => false,
                BITDELAYR::I2S => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> BITDELAYR {
            if value { BITDELAYR::I2S } else { BITDELAYR::LJ }
        }
        #[doc = "Checks if the value of the field is `LJ`"]
        #[inline(always)]
        pub fn is_lj(&self) -> bool {
            *self == BITDELAYR::LJ
        }
        #[doc = "Checks if the value of the field is `I2S`"]
        #[inline(always)]
        pub fn is_i2s(&self) -> bool {
            *self == BITDELAYR::I2S
        }
    }
    #[doc = "Possible values of the field `FSSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum FSSELR {
        #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
        SCKDIV,
        #[doc = "FSn input pin is used as Frame Sync n source"]
        FSPIN,
    }
    impl FSSELR {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                FSSELR::SCKDIV => false,
                FSSELR::FSPIN => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> FSSELR {
            if value { FSSELR::FSPIN } else { FSSELR::SCKDIV }
        }
        #[doc = "Checks if the value of the field is `SCKDIV`"]
        #[inline(always)]
        pub fn is_sckdiv(&self) -> bool {
            *self == FSSELR::SCKDIV
        }
        #[doc = "Checks if the value of the field is `FSPIN`"]
        #[inline(always)]
        pub fn is_fspin(&self) -> bool {
            *self == FSSELR::FSPIN
        }
    }
    #[doc = r" Value of the field"]
    pub struct FSINVR {
        bits: bool,
    }
    impl FSINVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `SCKSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SCKSELR {
        #[doc = "Divided Master Clock n is used as Serial Clock n source"]
        MCKDIV,
        #[doc = "SCKn input pin is used as Serial Clock n source"]
        SCKPIN,
    }
    impl SCKSELR {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                SCKSELR::MCKDIV => false,
                SCKSELR::SCKPIN => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> SCKSELR {
            if value { SCKSELR::SCKPIN } else { SCKSELR::MCKDIV }
        }
        #[doc = "Checks if the value of the field is `MCKDIV`"]
        #[inline(always)]
        pub fn is_mckdiv(&self) -> bool {
            *self == SCKSELR::MCKDIV
        }
        #[doc = "Checks if the value of the field is `SCKPIN`"]
        #[inline(always)]
        pub fn is_sckpin(&self) -> bool {
            *self == SCKSELR::SCKPIN
        }
    }
    #[doc = "Possible values of the field `MCKSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum MCKSELR {
        #[doc = "clk_gen_n is used as Master Clock n source"]
        GCLK,
        #[doc = "MCKn input pin is used as Master Clock n source"]
        MCKPIN,
    }
    impl MCKSELR {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                MCKSELR::GCLK => false,
                MCKSELR::MCKPIN => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> MCKSELR {
            if value { MCKSELR::MCKPIN } else { MCKSELR::GCLK }
        }
        #[doc = "Checks if the value of the field is `GCLK`"]
        #[inline(always)]
        pub fn is_gclk(&self) -> bool {
            *self == MCKSELR::GCLK
        }
        #[doc = "Checks if the value of the field is `MCKPIN`"]
        #[inline(always)]
        pub fn is_mckpin(&self) -> bool {
            *self == MCKSELR::MCKPIN
        }
    }
    #[doc = r" Value of the field"]
    pub struct MCKENR {
        bits: bool,
    }
    impl MCKENR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct MCKDIVR {
        bits: u8,
    }
    impl MCKDIVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct MCKOUTDIVR {
        bits: u8,
    }
    impl MCKOUTDIVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    #[doc = r" Value of the field"]
    pub struct FSOUTINVR {
        bits: bool,
    }
    impl FSOUTINVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SCKOUTINVR {
        bits: bool,
    }
    impl SCKOUTINVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct MCKOUTINVR {
        bits: bool,
    }
    impl MCKOUTINVR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Values that can be written to the field `SLOTSIZE`"]
    pub enum SLOTSIZEW {
        #[doc = "8-bit Slot for Clock Unit n"]
        _8,
        #[doc = "16-bit Slot for Clock Unit n"]
        _16,
        #[doc = "24-bit Slot for Clock Unit n"]
        _24,
        #[doc = "32-bit Slot for Clock Unit n"]
        _32,
    }
    impl SLOTSIZEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SLOTSIZEW::_8 => 0,
                SLOTSIZEW::_16 => 1,
                SLOTSIZEW::_24 => 2,
                SLOTSIZEW::_32 => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SLOTSIZEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLOTSIZEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SLOTSIZEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "8-bit Slot for Clock Unit n"]
        #[inline(always)]
        pub fn _8(self) -> &'a mut W {
            self.variant(SLOTSIZEW::_8)
        }
        #[doc = "16-bit Slot for Clock Unit n"]
        #[inline(always)]
        pub fn _16(self) -> &'a mut W {
            self.variant(SLOTSIZEW::_16)
        }
        #[doc = "24-bit Slot for Clock Unit n"]
        #[inline(always)]
        pub fn _24(self) -> &'a mut W {
            self.variant(SLOTSIZEW::_24)
        }
        #[doc = "32-bit Slot for Clock Unit n"]
        #[inline(always)]
        pub fn _32(self) -> &'a mut W {
            self.variant(SLOTSIZEW::_32)
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
    pub struct _NBSLOTSW<'a> {
        w: &'a mut W,
    }
    impl<'a> _NBSLOTSW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `FSWIDTH`"]
    pub enum FSWIDTHW {
        #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
        SLOT,
        #[doc = "Frame Sync Pulse is half a Frame wide"]
        HALF,
        #[doc = "Frame Sync Pulse is 1 Bit wide"]
        BIT,
        #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
        BURST,
    }
    impl FSWIDTHW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                FSWIDTHW::SLOT => 0,
                FSWIDTHW::HALF => 1,
                FSWIDTHW::BIT => 2,
                FSWIDTHW::BURST => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _FSWIDTHW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FSWIDTHW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: FSWIDTHW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
        #[inline(always)]
        pub fn slot(self) -> &'a mut W {
            self.variant(FSWIDTHW::SLOT)
        }
        #[doc = "Frame Sync Pulse is half a Frame wide"]
        #[inline(always)]
        pub fn half(self) -> &'a mut W {
            self.variant(FSWIDTHW::HALF)
        }
        #[doc = "Frame Sync Pulse is 1 Bit wide"]
        #[inline(always)]
        pub fn bit_(self) -> &'a mut W {
            self.variant(FSWIDTHW::BIT)
        }
        #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
        #[inline(always)]
        pub fn burst(self) -> &'a mut W {
            self.variant(FSWIDTHW::BURST)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `BITDELAY`"]
    pub enum BITDELAYW {
        #[doc = "Left Justified (0 Bit Delay)"]
        LJ,
        #[doc = "I2S (1 Bit Delay)"]
        I2S,
    }
    impl BITDELAYW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                BITDELAYW::LJ => false,
                BITDELAYW::I2S => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _BITDELAYW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BITDELAYW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: BITDELAYW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "Left Justified (0 Bit Delay)"]
        #[inline(always)]
        pub fn lj(self) -> &'a mut W {
            self.variant(BITDELAYW::LJ)
        }
        #[doc = "I2S (1 Bit Delay)"]
        #[inline(always)]
        pub fn i2s(self) -> &'a mut W {
            self.variant(BITDELAYW::I2S)
        }
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
    #[doc = "Values that can be written to the field `FSSEL`"]
    pub enum FSSELW {
        #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
        SCKDIV,
        #[doc = "FSn input pin is used as Frame Sync n source"]
        FSPIN,
    }
    impl FSSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                FSSELW::SCKDIV => false,
                FSSELW::FSPIN => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _FSSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FSSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: FSSELW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
        #[inline(always)]
        pub fn sckdiv(self) -> &'a mut W {
            self.variant(FSSELW::SCKDIV)
        }
        #[doc = "FSn input pin is used as Frame Sync n source"]
        #[inline(always)]
        pub fn fspin(self) -> &'a mut W {
            self.variant(FSSELW::FSPIN)
        }
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
    pub struct _FSINVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FSINVW<'a> {
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
    #[doc = "Values that can be written to the field `SCKSEL`"]
    pub enum SCKSELW {
        #[doc = "Divided Master Clock n is used as Serial Clock n source"]
        MCKDIV,
        #[doc = "SCKn input pin is used as Serial Clock n source"]
        SCKPIN,
    }
    impl SCKSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                SCKSELW::MCKDIV => false,
                SCKSELW::SCKPIN => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SCKSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SCKSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SCKSELW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "Divided Master Clock n is used as Serial Clock n source"]
        #[inline(always)]
        pub fn mckdiv(self) -> &'a mut W {
            self.variant(SCKSELW::MCKDIV)
        }
        #[doc = "SCKn input pin is used as Serial Clock n source"]
        #[inline(always)]
        pub fn sckpin(self) -> &'a mut W {
            self.variant(SCKSELW::SCKPIN)
        }
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
    #[doc = "Values that can be written to the field `MCKSEL`"]
    pub enum MCKSELW {
        #[doc = "clk_gen_n is used as Master Clock n source"]
        GCLK,
        #[doc = "MCKn input pin is used as Master Clock n source"]
        MCKPIN,
    }
    impl MCKSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                MCKSELW::GCLK => false,
                MCKSELW::MCKPIN => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _MCKSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCKSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: MCKSELW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "clk_gen_n is used as Master Clock n source"]
        #[inline(always)]
        pub fn gclk(self) -> &'a mut W {
            self.variant(MCKSELW::GCLK)
        }
        #[doc = "MCKn input pin is used as Master Clock n source"]
        #[inline(always)]
        pub fn mckpin(self) -> &'a mut W {
            self.variant(MCKSELW::MCKPIN)
        }
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
    pub struct _MCKENW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCKENW<'a> {
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
    pub struct _MCKDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCKDIVW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _MCKOUTDIVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCKOUTDIVW<'a> {
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _FSOUTINVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _FSOUTINVW<'a> {
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
            const OFFSET: u8 = 29;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _SCKOUTINVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SCKOUTINVW<'a> {
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
    pub struct _MCKOUTINVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MCKOUTINVW<'a> {
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
        #[doc = "Bits 0:1 - Slot Size"]
        #[inline(always)]
        pub fn slotsize(&self) -> SLOTSIZER {
            SLOTSIZER::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 2:4 - Number of Slots in Frame"]
        #[inline(always)]
        pub fn nbslots(&self) -> NBSLOTSR {
            let bits = {
                const MASK: u8 = 7;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            NBSLOTSR { bits }
        }
        #[doc = "Bits 5:6 - Frame Sync Width"]
        #[inline(always)]
        pub fn fswidth(&self) -> FSWIDTHR {
            FSWIDTHR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 7 - Data Delay from Frame Sync"]
        #[inline(always)]
        pub fn bitdelay(&self) -> BITDELAYR {
            BITDELAYR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bit 8 - Frame Sync Select"]
        #[inline(always)]
        pub fn fssel(&self) -> FSSELR {
            FSSELR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bit 11 - Frame Sync Invert"]
        #[inline(always)]
        pub fn fsinv(&self) -> FSINVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FSINVR { bits }
        }
        #[doc = "Bit 12 - Serial Clock Select"]
        #[inline(always)]
        pub fn scksel(&self) -> SCKSELR {
            SCKSELR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bit 16 - Master Clock Select"]
        #[inline(always)]
        pub fn mcksel(&self) -> MCKSELR {
            MCKSELR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bit 18 - Master Clock Enable"]
        #[inline(always)]
        pub fn mcken(&self) -> MCKENR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MCKENR { bits }
        }
        #[doc = "Bits 19:23 - Master Clock Division Factor"]
        #[inline(always)]
        pub fn mckdiv(&self) -> MCKDIVR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MCKDIVR { bits }
        }
        #[doc = "Bits 24:28 - Master Clock Output Division Factor"]
        #[inline(always)]
        pub fn mckoutdiv(&self) -> MCKOUTDIVR {
            let bits = {
                const MASK: u8 = 31;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            };
            MCKOUTDIVR { bits }
        }
        #[doc = "Bit 29 - Frame Sync Output Invert"]
        #[inline(always)]
        pub fn fsoutinv(&self) -> FSOUTINVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 29;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            FSOUTINVR { bits }
        }
        #[doc = "Bit 30 - Serial Clock Output Invert"]
        #[inline(always)]
        pub fn sckoutinv(&self) -> SCKOUTINVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 30;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SCKOUTINVR { bits }
        }
        #[doc = "Bit 31 - Master Clock Output Invert"]
        #[inline(always)]
        pub fn mckoutinv(&self) -> MCKOUTINVR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 31;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            MCKOUTINVR { bits }
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
        #[doc = "Bits 0:1 - Slot Size"]
        #[inline(always)]
        pub fn slotsize(&mut self) -> _SLOTSIZEW {
            _SLOTSIZEW { w: self }
        }
        #[doc = "Bits 2:4 - Number of Slots in Frame"]
        #[inline(always)]
        pub fn nbslots(&mut self) -> _NBSLOTSW {
            _NBSLOTSW { w: self }
        }
        #[doc = "Bits 5:6 - Frame Sync Width"]
        #[inline(always)]
        pub fn fswidth(&mut self) -> _FSWIDTHW {
            _FSWIDTHW { w: self }
        }
        #[doc = "Bit 7 - Data Delay from Frame Sync"]
        #[inline(always)]
        pub fn bitdelay(&mut self) -> _BITDELAYW {
            _BITDELAYW { w: self }
        }
        #[doc = "Bit 8 - Frame Sync Select"]
        #[inline(always)]
        pub fn fssel(&mut self) -> _FSSELW {
            _FSSELW { w: self }
        }
        #[doc = "Bit 11 - Frame Sync Invert"]
        #[inline(always)]
        pub fn fsinv(&mut self) -> _FSINVW {
            _FSINVW { w: self }
        }
        #[doc = "Bit 12 - Serial Clock Select"]
        #[inline(always)]
        pub fn scksel(&mut self) -> _SCKSELW {
            _SCKSELW { w: self }
        }
        #[doc = "Bit 16 - Master Clock Select"]
        #[inline(always)]
        pub fn mcksel(&mut self) -> _MCKSELW {
            _MCKSELW { w: self }
        }
        #[doc = "Bit 18 - Master Clock Enable"]
        #[inline(always)]
        pub fn mcken(&mut self) -> _MCKENW {
            _MCKENW { w: self }
        }
        #[doc = "Bits 19:23 - Master Clock Division Factor"]
        #[inline(always)]
        pub fn mckdiv(&mut self) -> _MCKDIVW {
            _MCKDIVW { w: self }
        }
        #[doc = "Bits 24:28 - Master Clock Output Division Factor"]
        #[inline(always)]
        pub fn mckoutdiv(&mut self) -> _MCKOUTDIVW {
            _MCKOUTDIVW { w: self }
        }
        #[doc = "Bit 29 - Frame Sync Output Invert"]
        #[inline(always)]
        pub fn fsoutinv(&mut self) -> _FSOUTINVW {
            _FSOUTINVW { w: self }
        }
        #[doc = "Bit 30 - Serial Clock Output Invert"]
        #[inline(always)]
        pub fn sckoutinv(&mut self) -> _SCKOUTINVW {
            _SCKOUTINVW { w: self }
        }
        #[doc = "Bit 31 - Master Clock Output Invert"]
        #[inline(always)]
        pub fn mckoutinv(&mut self) -> _MCKOUTINVW {
            _MCKOUTINVW { w: self }
        }
    }
}
#[doc = "Control A"]
pub struct CTRLA {
    register: VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u8,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u8,
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
    #[doc = r" Value of the field"]
    pub struct CKEN0R {
        bits: bool,
    }
    impl CKEN0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct CKEN1R {
        bits: bool,
    }
    impl CKEN1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SEREN0R {
        bits: bool,
    }
    impl SEREN0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SEREN1R {
        bits: bool,
    }
    impl SEREN1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    #[doc = r" Proxy"]
    pub struct _CKEN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CKEN0W<'a> {
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
    pub struct _CKEN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _CKEN1W<'a> {
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
    pub struct _SEREN0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SEREN0W<'a> {
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
    #[doc = r" Proxy"]
    pub struct _SEREN1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SEREN1W<'a> {
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
        #[doc = "Bit 2 - Clock Unit 0 Enable"]
        #[inline(always)]
        pub fn cken0(&self) -> CKEN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CKEN0R { bits }
        }
        #[doc = "Bit 3 - Clock Unit 1 Enable"]
        #[inline(always)]
        pub fn cken1(&self) -> CKEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            CKEN1R { bits }
        }
        #[doc = "Bit 4 - Serializer 0 Enable"]
        #[inline(always)]
        pub fn seren0(&self) -> SEREN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SEREN0R { bits }
        }
        #[doc = "Bit 5 - Serializer 1 Enable"]
        #[inline(always)]
        pub fn seren1(&self) -> SEREN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u8) != 0
            };
            SEREN1R { bits }
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
        #[doc = "Bit 1 - Enable"]
        #[inline(always)]
        pub fn enable(&mut self) -> _ENABLEW {
            _ENABLEW { w: self }
        }
        #[doc = "Bit 2 - Clock Unit 0 Enable"]
        #[inline(always)]
        pub fn cken0(&mut self) -> _CKEN0W {
            _CKEN0W { w: self }
        }
        #[doc = "Bit 3 - Clock Unit 1 Enable"]
        #[inline(always)]
        pub fn cken1(&mut self) -> _CKEN1W {
            _CKEN1W { w: self }
        }
        #[doc = "Bit 4 - Serializer 0 Enable"]
        #[inline(always)]
        pub fn seren0(&mut self) -> _SEREN0W {
            _SEREN0W { w: self }
        }
        #[doc = "Bit 5 - Serializer 1 Enable"]
        #[inline(always)]
        pub fn seren1(&mut self) -> _SEREN1W {
            _SEREN1W { w: self }
        }
    }
}
#[doc = "Data n"]
pub struct DATA {
    register: VolatileCell<u32>,
}
#[doc = "Data n"]
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
        #[doc = "Bits 0:31 - Sample Data"]
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
        #[doc = "Bits 0:31 - Sample Data"]
        #[inline(always)]
        pub fn data(&mut self) -> _DATAW {
            _DATAW { w: self }
        }
    }
}
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: VolatileCell<u16>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
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
    pub struct RXRDY0R {
        bits: bool,
    }
    impl RXRDY0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct RXRDY1R {
        bits: bool,
    }
    impl RXRDY1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct RXOR0R {
        bits: bool,
    }
    impl RXOR0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct RXOR1R {
        bits: bool,
    }
    impl RXOR1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXRDY0R {
        bits: bool,
    }
    impl TXRDY0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXRDY1R {
        bits: bool,
    }
    impl TXRDY1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXUR0R {
        bits: bool,
    }
    impl TXUR0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXUR1R {
        bits: bool,
    }
    impl TXUR1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _RXRDY0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXRDY0W<'a> {
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
    pub struct _RXRDY1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXRDY1W<'a> {
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
    pub struct _RXOR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXOR0W<'a> {
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
    pub struct _RXOR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXOR1W<'a> {
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
    pub struct _TXRDY0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXRDY0W<'a> {
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
    pub struct _TXRDY1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXRDY1W<'a> {
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
    pub struct _TXUR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXUR0W<'a> {
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
    #[doc = r" Proxy"]
    pub struct _TXUR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXUR1W<'a> {
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
        #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
        #[inline(always)]
        pub fn rxrdy0(&self) -> RXRDY0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXRDY0R { bits }
        }
        #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
        #[inline(always)]
        pub fn rxrdy1(&self) -> RXRDY1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXRDY1R { bits }
        }
        #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
        #[inline(always)]
        pub fn rxor0(&self) -> RXOR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXOR0R { bits }
        }
        #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
        #[inline(always)]
        pub fn rxor1(&self) -> RXOR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXOR1R { bits }
        }
        #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
        #[inline(always)]
        pub fn txrdy0(&self) -> TXRDY0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXRDY0R { bits }
        }
        #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
        #[inline(always)]
        pub fn txrdy1(&self) -> TXRDY1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXRDY1R { bits }
        }
        #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
        #[inline(always)]
        pub fn txur0(&self) -> TXUR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXUR0R { bits }
        }
        #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
        #[inline(always)]
        pub fn txur1(&self) -> TXUR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXUR1R { bits }
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
        #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
        #[inline(always)]
        pub fn rxrdy0(&mut self) -> _RXRDY0W {
            _RXRDY0W { w: self }
        }
        #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
        #[inline(always)]
        pub fn rxrdy1(&mut self) -> _RXRDY1W {
            _RXRDY1W { w: self }
        }
        #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
        #[inline(always)]
        pub fn rxor0(&mut self) -> _RXOR0W {
            _RXOR0W { w: self }
        }
        #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
        #[inline(always)]
        pub fn rxor1(&mut self) -> _RXOR1W {
            _RXOR1W { w: self }
        }
        #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
        #[inline(always)]
        pub fn txrdy0(&mut self) -> _TXRDY0W {
            _TXRDY0W { w: self }
        }
        #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
        #[inline(always)]
        pub fn txrdy1(&mut self) -> _TXRDY1W {
            _TXRDY1W { w: self }
        }
        #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
        #[inline(always)]
        pub fn txur0(&mut self) -> _TXUR0W {
            _TXUR0W { w: self }
        }
        #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
        #[inline(always)]
        pub fn txur1(&mut self) -> _TXUR1W {
            _TXUR1W { w: self }
        }
    }
}
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: VolatileCell<u16>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
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
    pub struct RXRDY0R {
        bits: bool,
    }
    impl RXRDY0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct RXRDY1R {
        bits: bool,
    }
    impl RXRDY1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct RXOR0R {
        bits: bool,
    }
    impl RXOR0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct RXOR1R {
        bits: bool,
    }
    impl RXOR1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXRDY0R {
        bits: bool,
    }
    impl TXRDY0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXRDY1R {
        bits: bool,
    }
    impl TXRDY1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXUR0R {
        bits: bool,
    }
    impl TXUR0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXUR1R {
        bits: bool,
    }
    impl TXUR1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _RXRDY0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXRDY0W<'a> {
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
    pub struct _RXRDY1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXRDY1W<'a> {
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
    pub struct _RXOR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXOR0W<'a> {
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
    pub struct _RXOR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXOR1W<'a> {
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
    pub struct _TXRDY0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXRDY0W<'a> {
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
    pub struct _TXRDY1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXRDY1W<'a> {
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
    pub struct _TXUR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXUR0W<'a> {
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
    #[doc = r" Proxy"]
    pub struct _TXUR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXUR1W<'a> {
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
        #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
        #[inline(always)]
        pub fn rxrdy0(&self) -> RXRDY0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXRDY0R { bits }
        }
        #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
        #[inline(always)]
        pub fn rxrdy1(&self) -> RXRDY1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXRDY1R { bits }
        }
        #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
        #[inline(always)]
        pub fn rxor0(&self) -> RXOR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXOR0R { bits }
        }
        #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
        #[inline(always)]
        pub fn rxor1(&self) -> RXOR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXOR1R { bits }
        }
        #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
        #[inline(always)]
        pub fn txrdy0(&self) -> TXRDY0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXRDY0R { bits }
        }
        #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
        #[inline(always)]
        pub fn txrdy1(&self) -> TXRDY1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXRDY1R { bits }
        }
        #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
        #[inline(always)]
        pub fn txur0(&self) -> TXUR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXUR0R { bits }
        }
        #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
        #[inline(always)]
        pub fn txur1(&self) -> TXUR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXUR1R { bits }
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
        #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
        #[inline(always)]
        pub fn rxrdy0(&mut self) -> _RXRDY0W {
            _RXRDY0W { w: self }
        }
        #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
        #[inline(always)]
        pub fn rxrdy1(&mut self) -> _RXRDY1W {
            _RXRDY1W { w: self }
        }
        #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
        #[inline(always)]
        pub fn rxor0(&mut self) -> _RXOR0W {
            _RXOR0W { w: self }
        }
        #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
        #[inline(always)]
        pub fn rxor1(&mut self) -> _RXOR1W {
            _RXOR1W { w: self }
        }
        #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
        #[inline(always)]
        pub fn txrdy0(&mut self) -> _TXRDY0W {
            _TXRDY0W { w: self }
        }
        #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
        #[inline(always)]
        pub fn txrdy1(&mut self) -> _TXRDY1W {
            _TXRDY1W { w: self }
        }
        #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
        #[inline(always)]
        pub fn txur0(&mut self) -> _TXUR0W {
            _TXUR0W { w: self }
        }
        #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
        #[inline(always)]
        pub fn txur1(&mut self) -> _TXUR1W {
            _TXUR1W { w: self }
        }
    }
}
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: VolatileCell<u16>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u16,
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
    pub struct RXRDY0R {
        bits: bool,
    }
    impl RXRDY0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct RXRDY1R {
        bits: bool,
    }
    impl RXRDY1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct RXOR0R {
        bits: bool,
    }
    impl RXOR0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct RXOR1R {
        bits: bool,
    }
    impl RXOR1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXRDY0R {
        bits: bool,
    }
    impl TXRDY0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXRDY1R {
        bits: bool,
    }
    impl TXRDY1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXUR0R {
        bits: bool,
    }
    impl TXUR0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct TXUR1R {
        bits: bool,
    }
    impl TXUR1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct _RXRDY0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXRDY0W<'a> {
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
    pub struct _RXRDY1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXRDY1W<'a> {
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
    pub struct _RXOR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXOR0W<'a> {
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
    pub struct _RXOR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXOR1W<'a> {
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
    pub struct _TXRDY0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXRDY0W<'a> {
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
    pub struct _TXRDY1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXRDY1W<'a> {
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
    pub struct _TXUR0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXUR0W<'a> {
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
    #[doc = r" Proxy"]
    pub struct _TXUR1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXUR1W<'a> {
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
        #[doc = "Bit 0 - Receive Ready 0"]
        #[inline(always)]
        pub fn rxrdy0(&self) -> RXRDY0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXRDY0R { bits }
        }
        #[doc = "Bit 1 - Receive Ready 1"]
        #[inline(always)]
        pub fn rxrdy1(&self) -> RXRDY1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXRDY1R { bits }
        }
        #[doc = "Bit 4 - Receive Overrun 0"]
        #[inline(always)]
        pub fn rxor0(&self) -> RXOR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXOR0R { bits }
        }
        #[doc = "Bit 5 - Receive Overrun 1"]
        #[inline(always)]
        pub fn rxor1(&self) -> RXOR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            RXOR1R { bits }
        }
        #[doc = "Bit 8 - Transmit Ready 0"]
        #[inline(always)]
        pub fn txrdy0(&self) -> TXRDY0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXRDY0R { bits }
        }
        #[doc = "Bit 9 - Transmit Ready 1"]
        #[inline(always)]
        pub fn txrdy1(&self) -> TXRDY1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXRDY1R { bits }
        }
        #[doc = "Bit 12 - Transmit Underrun 0"]
        #[inline(always)]
        pub fn txur0(&self) -> TXUR0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXUR0R { bits }
        }
        #[doc = "Bit 13 - Transmit Underrun 1"]
        #[inline(always)]
        pub fn txur1(&self) -> TXUR1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            TXUR1R { bits }
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
        #[doc = "Bit 0 - Receive Ready 0"]
        #[inline(always)]
        pub fn rxrdy0(&mut self) -> _RXRDY0W {
            _RXRDY0W { w: self }
        }
        #[doc = "Bit 1 - Receive Ready 1"]
        #[inline(always)]
        pub fn rxrdy1(&mut self) -> _RXRDY1W {
            _RXRDY1W { w: self }
        }
        #[doc = "Bit 4 - Receive Overrun 0"]
        #[inline(always)]
        pub fn rxor0(&mut self) -> _RXOR0W {
            _RXOR0W { w: self }
        }
        #[doc = "Bit 5 - Receive Overrun 1"]
        #[inline(always)]
        pub fn rxor1(&mut self) -> _RXOR1W {
            _RXOR1W { w: self }
        }
        #[doc = "Bit 8 - Transmit Ready 0"]
        #[inline(always)]
        pub fn txrdy0(&mut self) -> _TXRDY0W {
            _TXRDY0W { w: self }
        }
        #[doc = "Bit 9 - Transmit Ready 1"]
        #[inline(always)]
        pub fn txrdy1(&mut self) -> _TXRDY1W {
            _TXRDY1W { w: self }
        }
        #[doc = "Bit 12 - Transmit Underrun 0"]
        #[inline(always)]
        pub fn txur0(&mut self) -> _TXUR0W {
            _TXUR0W { w: self }
        }
        #[doc = "Bit 13 - Transmit Underrun 1"]
        #[inline(always)]
        pub fn txur1(&mut self) -> _TXUR1W {
            _TXUR1W { w: self }
        }
    }
}
#[doc = "Serializer n Control"]
pub struct SERCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Serializer n Control"]
pub mod serctrl {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u32,
    }
    #[doc = r" Value to write to the register"]
    pub struct W {
        bits: u32,
    }
    impl super::SERCTRL {
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
    #[doc = "Possible values of the field `SERMODE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SERMODER {
        #[doc = "Receive"]
        RX,
        #[doc = "Transmit"]
        TX,
        #[doc = "Receive 1 PDM data on each clock edge"]
        PDM2,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl SERMODER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                SERMODER::RX => 0,
                SERMODER::TX => 1,
                SERMODER::PDM2 => 2,
                SERMODER::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> SERMODER {
            match value {
                0 => SERMODER::RX,
                1 => SERMODER::TX,
                2 => SERMODER::PDM2,
                i => SERMODER::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `RX`"]
        #[inline(always)]
        pub fn is_rx(&self) -> bool {
            *self == SERMODER::RX
        }
        #[doc = "Checks if the value of the field is `TX`"]
        #[inline(always)]
        pub fn is_tx(&self) -> bool {
            *self == SERMODER::TX
        }
        #[doc = "Checks if the value of the field is `PDM2`"]
        #[inline(always)]
        pub fn is_pdm2(&self) -> bool {
            *self == SERMODER::PDM2
        }
    }
    #[doc = "Possible values of the field `TXDEFAULT`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum TXDEFAULTR {
        #[doc = "Output Default Value is 0"]
        ZERO,
        #[doc = "Output Default Value is 1"]
        ONE,
        #[doc = "Output Default Value is high impedance"]
        HIZ,
        #[doc = r" Reserved"]
        _Reserved(u8),
    }
    impl TXDEFAULTR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                TXDEFAULTR::ZERO => 0,
                TXDEFAULTR::ONE => 1,
                TXDEFAULTR::HIZ => 3,
                TXDEFAULTR::_Reserved(bits) => bits,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> TXDEFAULTR {
            match value {
                0 => TXDEFAULTR::ZERO,
                1 => TXDEFAULTR::ONE,
                3 => TXDEFAULTR::HIZ,
                i => TXDEFAULTR::_Reserved(i),
            }
        }
        #[doc = "Checks if the value of the field is `ZERO`"]
        #[inline(always)]
        pub fn is_zero(&self) -> bool {
            *self == TXDEFAULTR::ZERO
        }
        #[doc = "Checks if the value of the field is `ONE`"]
        #[inline(always)]
        pub fn is_one(&self) -> bool {
            *self == TXDEFAULTR::ONE
        }
        #[doc = "Checks if the value of the field is `HIZ`"]
        #[inline(always)]
        pub fn is_hiz(&self) -> bool {
            *self == TXDEFAULTR::HIZ
        }
    }
    #[doc = "Possible values of the field `TXSAME`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum TXSAMER {
        #[doc = "Zero data transmitted in case of underrun"]
        ZERO,
        #[doc = "Last data transmitted in case of underrun"]
        SAME,
    }
    impl TXSAMER {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                TXSAMER::ZERO => false,
                TXSAMER::SAME => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> TXSAMER {
            if value { TXSAMER::SAME } else { TXSAMER::ZERO }
        }
        #[doc = "Checks if the value of the field is `ZERO`"]
        #[inline(always)]
        pub fn is_zero(&self) -> bool {
            *self == TXSAMER::ZERO
        }
        #[doc = "Checks if the value of the field is `SAME`"]
        #[inline(always)]
        pub fn is_same(&self) -> bool {
            *self == TXSAMER::SAME
        }
    }
    #[doc = "Possible values of the field `CLKSEL`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CLKSELR {
        #[doc = "Use Clock Unit 0"]
        CLK0,
        #[doc = "Use Clock Unit 1"]
        CLK1,
    }
    impl CLKSELR {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                CLKSELR::CLK0 => false,
                CLKSELR::CLK1 => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> CLKSELR {
            if value { CLKSELR::CLK1 } else { CLKSELR::CLK0 }
        }
        #[doc = "Checks if the value of the field is `CLK0`"]
        #[inline(always)]
        pub fn is_clk0(&self) -> bool {
            *self == CLKSELR::CLK0
        }
        #[doc = "Checks if the value of the field is `CLK1`"]
        #[inline(always)]
        pub fn is_clk1(&self) -> bool {
            *self == CLKSELR::CLK1
        }
    }
    #[doc = "Possible values of the field `SLOTADJ`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SLOTADJR {
        #[doc = "Data is right adjusted in slot"]
        RIGHT,
        #[doc = "Data is left adjusted in slot"]
        LEFT,
    }
    impl SLOTADJR {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                SLOTADJR::RIGHT => false,
                SLOTADJR::LEFT => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> SLOTADJR {
            if value { SLOTADJR::LEFT } else { SLOTADJR::RIGHT }
        }
        #[doc = "Checks if the value of the field is `RIGHT`"]
        #[inline(always)]
        pub fn is_right(&self) -> bool {
            *self == SLOTADJR::RIGHT
        }
        #[doc = "Checks if the value of the field is `LEFT`"]
        #[inline(always)]
        pub fn is_left(&self) -> bool {
            *self == SLOTADJR::LEFT
        }
    }
    #[doc = "Possible values of the field `DATASIZE`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum DATASIZER {
        #[doc = "32 bits"]
        _32,
        #[doc = "24 bits"]
        _24,
        #[doc = "20 bits"]
        _20,
        #[doc = "18 bits"]
        _18,
        #[doc = "16 bits"]
        _16,
        #[doc = "16 bits compact stereo"]
        _16C,
        #[doc = "8 bits"]
        _8,
        #[doc = "8 bits compact stereo"]
        _8C,
    }
    impl DATASIZER {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                DATASIZER::_32 => 0,
                DATASIZER::_24 => 1,
                DATASIZER::_20 => 2,
                DATASIZER::_18 => 3,
                DATASIZER::_16 => 4,
                DATASIZER::_16C => 5,
                DATASIZER::_8 => 6,
                DATASIZER::_8C => 7,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> DATASIZER {
            match value {
                0 => DATASIZER::_32,
                1 => DATASIZER::_24,
                2 => DATASIZER::_20,
                3 => DATASIZER::_18,
                4 => DATASIZER::_16,
                5 => DATASIZER::_16C,
                6 => DATASIZER::_8,
                7 => DATASIZER::_8C,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `_32`"]
        #[inline(always)]
        pub fn is_32(&self) -> bool {
            *self == DATASIZER::_32
        }
        #[doc = "Checks if the value of the field is `_24`"]
        #[inline(always)]
        pub fn is_24(&self) -> bool {
            *self == DATASIZER::_24
        }
        #[doc = "Checks if the value of the field is `_20`"]
        #[inline(always)]
        pub fn is_20(&self) -> bool {
            *self == DATASIZER::_20
        }
        #[doc = "Checks if the value of the field is `_18`"]
        #[inline(always)]
        pub fn is_18(&self) -> bool {
            *self == DATASIZER::_18
        }
        #[doc = "Checks if the value of the field is `_16`"]
        #[inline(always)]
        pub fn is_16(&self) -> bool {
            *self == DATASIZER::_16
        }
        #[doc = "Checks if the value of the field is `_16C`"]
        #[inline(always)]
        pub fn is_16c(&self) -> bool {
            *self == DATASIZER::_16C
        }
        #[doc = "Checks if the value of the field is `_8`"]
        #[inline(always)]
        pub fn is_8(&self) -> bool {
            *self == DATASIZER::_8
        }
        #[doc = "Checks if the value of the field is `_8C`"]
        #[inline(always)]
        pub fn is_8c(&self) -> bool {
            *self == DATASIZER::_8C
        }
    }
    #[doc = "Possible values of the field `WORDADJ`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum WORDADJR {
        #[doc = "Data is right adjusted in word"]
        RIGHT,
        #[doc = "Data is left adjusted in word"]
        LEFT,
    }
    impl WORDADJR {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                WORDADJR::RIGHT => false,
                WORDADJR::LEFT => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> WORDADJR {
            if value { WORDADJR::LEFT } else { WORDADJR::RIGHT }
        }
        #[doc = "Checks if the value of the field is `RIGHT`"]
        #[inline(always)]
        pub fn is_right(&self) -> bool {
            *self == WORDADJR::RIGHT
        }
        #[doc = "Checks if the value of the field is `LEFT`"]
        #[inline(always)]
        pub fn is_left(&self) -> bool {
            *self == WORDADJR::LEFT
        }
    }
    #[doc = "Possible values of the field `EXTEND`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EXTENDR {
        #[doc = "Extend with zeroes"]
        ZERO,
        #[doc = "Extend with ones"]
        ONE,
        #[doc = "Extend with Most Significant Bit"]
        MSBIT,
        #[doc = "Extend with Least Significant Bit"]
        LSBIT,
    }
    impl EXTENDR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bits(&self) -> u8 {
            match *self {
                EXTENDR::ZERO => 0,
                EXTENDR::ONE => 1,
                EXTENDR::MSBIT => 2,
                EXTENDR::LSBIT => 3,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: u8) -> EXTENDR {
            match value {
                0 => EXTENDR::ZERO,
                1 => EXTENDR::ONE,
                2 => EXTENDR::MSBIT,
                3 => EXTENDR::LSBIT,
                _ => unreachable!(),
            }
        }
        #[doc = "Checks if the value of the field is `ZERO`"]
        #[inline(always)]
        pub fn is_zero(&self) -> bool {
            *self == EXTENDR::ZERO
        }
        #[doc = "Checks if the value of the field is `ONE`"]
        #[inline(always)]
        pub fn is_one(&self) -> bool {
            *self == EXTENDR::ONE
        }
        #[doc = "Checks if the value of the field is `MSBIT`"]
        #[inline(always)]
        pub fn is_msbit(&self) -> bool {
            *self == EXTENDR::MSBIT
        }
        #[doc = "Checks if the value of the field is `LSBIT`"]
        #[inline(always)]
        pub fn is_lsbit(&self) -> bool {
            *self == EXTENDR::LSBIT
        }
    }
    #[doc = "Possible values of the field `BITREV`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum BITREVR {
        #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
        MSBIT,
        #[doc = "Transfer Data Least Significant Bit (LSB) first"]
        LSBIT,
    }
    impl BITREVR {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                BITREVR::MSBIT => false,
                BITREVR::LSBIT => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> BITREVR {
            if value { BITREVR::LSBIT } else { BITREVR::MSBIT }
        }
        #[doc = "Checks if the value of the field is `MSBIT`"]
        #[inline(always)]
        pub fn is_msbit(&self) -> bool {
            *self == BITREVR::MSBIT
        }
        #[doc = "Checks if the value of the field is `LSBIT`"]
        #[inline(always)]
        pub fn is_lsbit(&self) -> bool {
            *self == BITREVR::LSBIT
        }
    }
    #[doc = r" Value of the field"]
    pub struct SLOTDIS0R {
        bits: bool,
    }
    impl SLOTDIS0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SLOTDIS1R {
        bits: bool,
    }
    impl SLOTDIS1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SLOTDIS2R {
        bits: bool,
    }
    impl SLOTDIS2R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SLOTDIS3R {
        bits: bool,
    }
    impl SLOTDIS3R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SLOTDIS4R {
        bits: bool,
    }
    impl SLOTDIS4R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SLOTDIS5R {
        bits: bool,
    }
    impl SLOTDIS5R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SLOTDIS6R {
        bits: bool,
    }
    impl SLOTDIS6R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SLOTDIS7R {
        bits: bool,
    }
    impl SLOTDIS7R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Possible values of the field `MONO`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum MONOR {
        #[doc = "Normal mode"]
        STEREO,
        #[doc = "Left channel data is duplicated to right channel"]
        MONO,
    }
    impl MONOR {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                MONOR::STEREO => false,
                MONOR::MONO => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> MONOR {
            if value { MONOR::MONO } else { MONOR::STEREO }
        }
        #[doc = "Checks if the value of the field is `STEREO`"]
        #[inline(always)]
        pub fn is_stereo(&self) -> bool {
            *self == MONOR::STEREO
        }
        #[doc = "Checks if the value of the field is `MONO`"]
        #[inline(always)]
        pub fn is_mono(&self) -> bool {
            *self == MONOR::MONO
        }
    }
    #[doc = "Possible values of the field `DMA`"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum DMAR {
        #[doc = "Single DMA channel"]
        SINGLE,
        #[doc = "One DMA channel per data channel"]
        MULTIPLE,
    }
    impl DMAR {
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            match *self {
                DMAR::SINGLE => false,
                DMAR::MULTIPLE => true,
            }
        }
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _from(value: bool) -> DMAR {
            if value { DMAR::MULTIPLE } else { DMAR::SINGLE }
        }
        #[doc = "Checks if the value of the field is `SINGLE`"]
        #[inline(always)]
        pub fn is_single(&self) -> bool {
            *self == DMAR::SINGLE
        }
        #[doc = "Checks if the value of the field is `MULTIPLE`"]
        #[inline(always)]
        pub fn is_multiple(&self) -> bool {
            *self == DMAR::MULTIPLE
        }
    }
    #[doc = r" Value of the field"]
    pub struct RXLOOPR {
        bits: bool,
    }
    impl RXLOOPR {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
        #[doc = r" Returns `true` if the bit is set (1)"]
        #[inline(always)]
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }
    }
    #[doc = "Values that can be written to the field `SERMODE`"]
    pub enum SERMODEW {
        #[doc = "Receive"]
        RX,
        #[doc = "Transmit"]
        TX,
        #[doc = "Receive 1 PDM data on each clock edge"]
        PDM2,
    }
    impl SERMODEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                SERMODEW::RX => 0,
                SERMODEW::TX => 1,
                SERMODEW::PDM2 => 2,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SERMODEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SERMODEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SERMODEW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Receive"]
        #[inline(always)]
        pub fn rx(self) -> &'a mut W {
            self.variant(SERMODEW::RX)
        }
        #[doc = "Transmit"]
        #[inline(always)]
        pub fn tx(self) -> &'a mut W {
            self.variant(SERMODEW::TX)
        }
        #[doc = "Receive 1 PDM data on each clock edge"]
        #[inline(always)]
        pub fn pdm2(self) -> &'a mut W {
            self.variant(SERMODEW::PDM2)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `TXDEFAULT`"]
    pub enum TXDEFAULTW {
        #[doc = "Output Default Value is 0"]
        ZERO,
        #[doc = "Output Default Value is 1"]
        ONE,
        #[doc = "Output Default Value is high impedance"]
        HIZ,
    }
    impl TXDEFAULTW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                TXDEFAULTW::ZERO => 0,
                TXDEFAULTW::ONE => 1,
                TXDEFAULTW::HIZ => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _TXDEFAULTW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXDEFAULTW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: TXDEFAULTW) -> &'a mut W {
            unsafe { self.bits(variant._bits()) }
        }
        #[doc = "Output Default Value is 0"]
        #[inline(always)]
        pub fn zero(self) -> &'a mut W {
            self.variant(TXDEFAULTW::ZERO)
        }
        #[doc = "Output Default Value is 1"]
        #[inline(always)]
        pub fn one(self) -> &'a mut W {
            self.variant(TXDEFAULTW::ONE)
        }
        #[doc = "Output Default Value is high impedance"]
        #[inline(always)]
        pub fn hiz(self) -> &'a mut W {
            self.variant(TXDEFAULTW::HIZ)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub unsafe fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `TXSAME`"]
    pub enum TXSAMEW {
        #[doc = "Zero data transmitted in case of underrun"]
        ZERO,
        #[doc = "Last data transmitted in case of underrun"]
        SAME,
    }
    impl TXSAMEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                TXSAMEW::ZERO => false,
                TXSAMEW::SAME => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _TXSAMEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _TXSAMEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: TXSAMEW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "Zero data transmitted in case of underrun"]
        #[inline(always)]
        pub fn zero(self) -> &'a mut W {
            self.variant(TXSAMEW::ZERO)
        }
        #[doc = "Last data transmitted in case of underrun"]
        #[inline(always)]
        pub fn same(self) -> &'a mut W {
            self.variant(TXSAMEW::SAME)
        }
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
    #[doc = "Values that can be written to the field `CLKSEL`"]
    pub enum CLKSELW {
        #[doc = "Use Clock Unit 0"]
        CLK0,
        #[doc = "Use Clock Unit 1"]
        CLK1,
    }
    impl CLKSELW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                CLKSELW::CLK0 => false,
                CLKSELW::CLK1 => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _CLKSELW<'a> {
        w: &'a mut W,
    }
    impl<'a> _CLKSELW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: CLKSELW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "Use Clock Unit 0"]
        #[inline(always)]
        pub fn clk0(self) -> &'a mut W {
            self.variant(CLKSELW::CLK0)
        }
        #[doc = "Use Clock Unit 1"]
        #[inline(always)]
        pub fn clk1(self) -> &'a mut W {
            self.variant(CLKSELW::CLK1)
        }
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
    #[doc = "Values that can be written to the field `SLOTADJ`"]
    pub enum SLOTADJW {
        #[doc = "Data is right adjusted in slot"]
        RIGHT,
        #[doc = "Data is left adjusted in slot"]
        LEFT,
    }
    impl SLOTADJW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                SLOTADJW::RIGHT => false,
                SLOTADJW::LEFT => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _SLOTADJW<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLOTADJW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: SLOTADJW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "Data is right adjusted in slot"]
        #[inline(always)]
        pub fn right(self) -> &'a mut W {
            self.variant(SLOTADJW::RIGHT)
        }
        #[doc = "Data is left adjusted in slot"]
        #[inline(always)]
        pub fn left(self) -> &'a mut W {
            self.variant(SLOTADJW::LEFT)
        }
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
    #[doc = "Values that can be written to the field `DATASIZE`"]
    pub enum DATASIZEW {
        #[doc = "32 bits"]
        _32,
        #[doc = "24 bits"]
        _24,
        #[doc = "20 bits"]
        _20,
        #[doc = "18 bits"]
        _18,
        #[doc = "16 bits"]
        _16,
        #[doc = "16 bits compact stereo"]
        _16C,
        #[doc = "8 bits"]
        _8,
        #[doc = "8 bits compact stereo"]
        _8C,
    }
    impl DATASIZEW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                DATASIZEW::_32 => 0,
                DATASIZEW::_24 => 1,
                DATASIZEW::_20 => 2,
                DATASIZEW::_18 => 3,
                DATASIZEW::_16 => 4,
                DATASIZEW::_16C => 5,
                DATASIZEW::_8 => 6,
                DATASIZEW::_8C => 7,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _DATASIZEW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DATASIZEW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: DATASIZEW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "32 bits"]
        #[inline(always)]
        pub fn _32(self) -> &'a mut W {
            self.variant(DATASIZEW::_32)
        }
        #[doc = "24 bits"]
        #[inline(always)]
        pub fn _24(self) -> &'a mut W {
            self.variant(DATASIZEW::_24)
        }
        #[doc = "20 bits"]
        #[inline(always)]
        pub fn _20(self) -> &'a mut W {
            self.variant(DATASIZEW::_20)
        }
        #[doc = "18 bits"]
        #[inline(always)]
        pub fn _18(self) -> &'a mut W {
            self.variant(DATASIZEW::_18)
        }
        #[doc = "16 bits"]
        #[inline(always)]
        pub fn _16(self) -> &'a mut W {
            self.variant(DATASIZEW::_16)
        }
        #[doc = "16 bits compact stereo"]
        #[inline(always)]
        pub fn _16c(self) -> &'a mut W {
            self.variant(DATASIZEW::_16C)
        }
        #[doc = "8 bits"]
        #[inline(always)]
        pub fn _8(self) -> &'a mut W {
            self.variant(DATASIZEW::_8)
        }
        #[doc = "8 bits compact stereo"]
        #[inline(always)]
        pub fn _8c(self) -> &'a mut W {
            self.variant(DATASIZEW::_8C)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `WORDADJ`"]
    pub enum WORDADJW {
        #[doc = "Data is right adjusted in word"]
        RIGHT,
        #[doc = "Data is left adjusted in word"]
        LEFT,
    }
    impl WORDADJW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                WORDADJW::RIGHT => false,
                WORDADJW::LEFT => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _WORDADJW<'a> {
        w: &'a mut W,
    }
    impl<'a> _WORDADJW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: WORDADJW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "Data is right adjusted in word"]
        #[inline(always)]
        pub fn right(self) -> &'a mut W {
            self.variant(WORDADJW::RIGHT)
        }
        #[doc = "Data is left adjusted in word"]
        #[inline(always)]
        pub fn left(self) -> &'a mut W {
            self.variant(WORDADJW::LEFT)
        }
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
    #[doc = "Values that can be written to the field `EXTEND`"]
    pub enum EXTENDW {
        #[doc = "Extend with zeroes"]
        ZERO,
        #[doc = "Extend with ones"]
        ONE,
        #[doc = "Extend with Most Significant Bit"]
        MSBIT,
        #[doc = "Extend with Least Significant Bit"]
        LSBIT,
    }
    impl EXTENDW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> u8 {
            match *self {
                EXTENDW::ZERO => 0,
                EXTENDW::ONE => 1,
                EXTENDW::MSBIT => 2,
                EXTENDW::LSBIT => 3,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _EXTENDW<'a> {
        w: &'a mut W,
    }
    impl<'a> _EXTENDW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: EXTENDW) -> &'a mut W {
            {
                self.bits(variant._bits())
            }
        }
        #[doc = "Extend with zeroes"]
        #[inline(always)]
        pub fn zero(self) -> &'a mut W {
            self.variant(EXTENDW::ZERO)
        }
        #[doc = "Extend with ones"]
        #[inline(always)]
        pub fn one(self) -> &'a mut W {
            self.variant(EXTENDW::ONE)
        }
        #[doc = "Extend with Most Significant Bit"]
        #[inline(always)]
        pub fn msbit(self) -> &'a mut W {
            self.variant(EXTENDW::MSBIT)
        }
        #[doc = "Extend with Least Significant Bit"]
        #[inline(always)]
        pub fn lsbit(self) -> &'a mut W {
            self.variant(EXTENDW::LSBIT)
        }
        #[doc = r" Writes raw bits to the field"]
        #[inline(always)]
        pub fn bits(self, value: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `BITREV`"]
    pub enum BITREVW {
        #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
        MSBIT,
        #[doc = "Transfer Data Least Significant Bit (LSB) first"]
        LSBIT,
    }
    impl BITREVW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                BITREVW::MSBIT => false,
                BITREVW::LSBIT => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _BITREVW<'a> {
        w: &'a mut W,
    }
    impl<'a> _BITREVW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: BITREVW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
        #[inline(always)]
        pub fn msbit(self) -> &'a mut W {
            self.variant(BITREVW::MSBIT)
        }
        #[doc = "Transfer Data Least Significant Bit (LSB) first"]
        #[inline(always)]
        pub fn lsbit(self) -> &'a mut W {
            self.variant(BITREVW::LSBIT)
        }
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
    pub struct _SLOTDIS0W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLOTDIS0W<'a> {
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
    pub struct _SLOTDIS1W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLOTDIS1W<'a> {
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
    pub struct _SLOTDIS2W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLOTDIS2W<'a> {
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
    pub struct _SLOTDIS3W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLOTDIS3W<'a> {
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
    pub struct _SLOTDIS4W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLOTDIS4W<'a> {
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
    pub struct _SLOTDIS5W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLOTDIS5W<'a> {
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
    #[doc = r" Proxy"]
    pub struct _SLOTDIS6W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLOTDIS6W<'a> {
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
    pub struct _SLOTDIS7W<'a> {
        w: &'a mut W,
    }
    impl<'a> _SLOTDIS7W<'a> {
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
    #[doc = "Values that can be written to the field `MONO`"]
    pub enum MONOW {
        #[doc = "Normal mode"]
        STEREO,
        #[doc = "Left channel data is duplicated to right channel"]
        MONO,
    }
    impl MONOW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                MONOW::STEREO => false,
                MONOW::MONO => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _MONOW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MONOW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: MONOW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "Normal mode"]
        #[inline(always)]
        pub fn stereo(self) -> &'a mut W {
            self.variant(MONOW::STEREO)
        }
        #[doc = "Left channel data is duplicated to right channel"]
        #[inline(always)]
        pub fn mono(self) -> &'a mut W {
            self.variant(MONOW::MONO)
        }
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
            const OFFSET: u8 = 24;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = "Values that can be written to the field `DMA`"]
    pub enum DMAW {
        #[doc = "Single DMA channel"]
        SINGLE,
        #[doc = "One DMA channel per data channel"]
        MULTIPLE,
    }
    impl DMAW {
        #[allow(missing_docs)]
        #[doc(hidden)]
        #[inline(always)]
        pub fn _bits(&self) -> bool {
            match *self {
                DMAW::SINGLE => false,
                DMAW::MULTIPLE => true,
            }
        }
    }
    #[doc = r" Proxy"]
    pub struct _DMAW<'a> {
        w: &'a mut W,
    }
    impl<'a> _DMAW<'a> {
        #[doc = r" Writes `variant` to the field"]
        #[inline(always)]
        pub fn variant(self, variant: DMAW) -> &'a mut W {
            {
                self.bit(variant._bits())
            }
        }
        #[doc = "Single DMA channel"]
        #[inline(always)]
        pub fn single(self) -> &'a mut W {
            self.variant(DMAW::SINGLE)
        }
        #[doc = "One DMA channel per data channel"]
        #[inline(always)]
        pub fn multiple(self) -> &'a mut W {
            self.variant(DMAW::MULTIPLE)
        }
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
            const OFFSET: u8 = 25;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    #[doc = r" Proxy"]
    pub struct _RXLOOPW<'a> {
        w: &'a mut W,
    }
    impl<'a> _RXLOOPW<'a> {
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
        #[doc = "Bits 0:1 - Serializer Mode"]
        #[inline(always)]
        pub fn sermode(&self) -> SERMODER {
            SERMODER::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bits 2:3 - Line Default Line when Slot Disabled"]
        #[inline(always)]
        pub fn txdefault(&self) -> TXDEFAULTR {
            TXDEFAULTR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 4 - Transmit Data when Underrun"]
        #[inline(always)]
        pub fn txsame(&self) -> TXSAMER {
            TXSAMER::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bit 5 - Clock Unit Selection"]
        #[inline(always)]
        pub fn clksel(&self) -> CLKSELR {
            CLKSELR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bit 7 - Data Slot Formatting Adjust"]
        #[inline(always)]
        pub fn slotadj(&self) -> SLOTADJR {
            SLOTADJR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bits 8:10 - Data Word Size"]
        #[inline(always)]
        pub fn datasize(&self) -> DATASIZER {
            DATASIZER::_from({
                const MASK: u8 = 7;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 12 - Data Word Formatting Adjust"]
        #[inline(always)]
        pub fn wordadj(&self) -> WORDADJR {
            WORDADJR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 12;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bits 13:14 - Data Formatting Bit Extension"]
        #[inline(always)]
        pub fn extend(&self) -> EXTENDR {
            EXTENDR::_from({
                const MASK: u8 = 3;
                const OFFSET: u8 = 13;
                ((self.bits >> OFFSET) & MASK as u32) as u8
            })
        }
        #[doc = "Bit 15 - Data Formatting Bit Reverse"]
        #[inline(always)]
        pub fn bitrev(&self) -> BITREVR {
            BITREVR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 15;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bit 16 - Slot 0 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis0(&self) -> SLOTDIS0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 16;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SLOTDIS0R { bits }
        }
        #[doc = "Bit 17 - Slot 1 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis1(&self) -> SLOTDIS1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 17;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SLOTDIS1R { bits }
        }
        #[doc = "Bit 18 - Slot 2 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis2(&self) -> SLOTDIS2R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 18;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SLOTDIS2R { bits }
        }
        #[doc = "Bit 19 - Slot 3 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis3(&self) -> SLOTDIS3R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 19;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SLOTDIS3R { bits }
        }
        #[doc = "Bit 20 - Slot 4 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis4(&self) -> SLOTDIS4R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 20;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SLOTDIS4R { bits }
        }
        #[doc = "Bit 21 - Slot 5 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis5(&self) -> SLOTDIS5R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 21;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SLOTDIS5R { bits }
        }
        #[doc = "Bit 22 - Slot 6 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis6(&self) -> SLOTDIS6R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 22;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SLOTDIS6R { bits }
        }
        #[doc = "Bit 23 - Slot 7 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis7(&self) -> SLOTDIS7R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 23;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            SLOTDIS7R { bits }
        }
        #[doc = "Bit 24 - Mono Mode"]
        #[inline(always)]
        pub fn mono(&self) -> MONOR {
            MONOR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 24;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bit 25 - Single or Multiple DMA Channels"]
        #[inline(always)]
        pub fn dma(&self) -> DMAR {
            DMAR::_from({
                const MASK: bool = true;
                const OFFSET: u8 = 25;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            })
        }
        #[doc = "Bit 26 - Loop-back Test Mode"]
        #[inline(always)]
        pub fn rxloop(&self) -> RXLOOPR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 26;
                ((self.bits >> OFFSET) & MASK as u32) != 0
            };
            RXLOOPR { bits }
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
        #[doc = "Bits 0:1 - Serializer Mode"]
        #[inline(always)]
        pub fn sermode(&mut self) -> _SERMODEW {
            _SERMODEW { w: self }
        }
        #[doc = "Bits 2:3 - Line Default Line when Slot Disabled"]
        #[inline(always)]
        pub fn txdefault(&mut self) -> _TXDEFAULTW {
            _TXDEFAULTW { w: self }
        }
        #[doc = "Bit 4 - Transmit Data when Underrun"]
        #[inline(always)]
        pub fn txsame(&mut self) -> _TXSAMEW {
            _TXSAMEW { w: self }
        }
        #[doc = "Bit 5 - Clock Unit Selection"]
        #[inline(always)]
        pub fn clksel(&mut self) -> _CLKSELW {
            _CLKSELW { w: self }
        }
        #[doc = "Bit 7 - Data Slot Formatting Adjust"]
        #[inline(always)]
        pub fn slotadj(&mut self) -> _SLOTADJW {
            _SLOTADJW { w: self }
        }
        #[doc = "Bits 8:10 - Data Word Size"]
        #[inline(always)]
        pub fn datasize(&mut self) -> _DATASIZEW {
            _DATASIZEW { w: self }
        }
        #[doc = "Bit 12 - Data Word Formatting Adjust"]
        #[inline(always)]
        pub fn wordadj(&mut self) -> _WORDADJW {
            _WORDADJW { w: self }
        }
        #[doc = "Bits 13:14 - Data Formatting Bit Extension"]
        #[inline(always)]
        pub fn extend(&mut self) -> _EXTENDW {
            _EXTENDW { w: self }
        }
        #[doc = "Bit 15 - Data Formatting Bit Reverse"]
        #[inline(always)]
        pub fn bitrev(&mut self) -> _BITREVW {
            _BITREVW { w: self }
        }
        #[doc = "Bit 16 - Slot 0 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis0(&mut self) -> _SLOTDIS0W {
            _SLOTDIS0W { w: self }
        }
        #[doc = "Bit 17 - Slot 1 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis1(&mut self) -> _SLOTDIS1W {
            _SLOTDIS1W { w: self }
        }
        #[doc = "Bit 18 - Slot 2 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis2(&mut self) -> _SLOTDIS2W {
            _SLOTDIS2W { w: self }
        }
        #[doc = "Bit 19 - Slot 3 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis3(&mut self) -> _SLOTDIS3W {
            _SLOTDIS3W { w: self }
        }
        #[doc = "Bit 20 - Slot 4 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis4(&mut self) -> _SLOTDIS4W {
            _SLOTDIS4W { w: self }
        }
        #[doc = "Bit 21 - Slot 5 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis5(&mut self) -> _SLOTDIS5W {
            _SLOTDIS5W { w: self }
        }
        #[doc = "Bit 22 - Slot 6 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis6(&mut self) -> _SLOTDIS6W {
            _SLOTDIS6W { w: self }
        }
        #[doc = "Bit 23 - Slot 7 Disabled for this Serializer"]
        #[inline(always)]
        pub fn slotdis7(&mut self) -> _SLOTDIS7W {
            _SLOTDIS7W { w: self }
        }
        #[doc = "Bit 24 - Mono Mode"]
        #[inline(always)]
        pub fn mono(&mut self) -> _MONOW {
            _MONOW { w: self }
        }
        #[doc = "Bit 25 - Single or Multiple DMA Channels"]
        #[inline(always)]
        pub fn dma(&mut self) -> _DMAW {
            _DMAW { w: self }
        }
        #[doc = "Bit 26 - Loop-back Test Mode"]
        #[inline(always)]
        pub fn rxloop(&mut self) -> _RXLOOPW {
            _RXLOOPW { w: self }
        }
    }
}
#[doc = "Synchronization Status"]
pub struct SYNCBUSY {
    register: VolatileCell<u16>,
}
#[doc = "Synchronization Status"]
pub mod syncbusy {
    #[doc = r" Value read from the register"]
    pub struct R {
        bits: u16,
    }
    impl super::SYNCBUSY {
        #[doc = r" Reads the contents of the register"]
        #[inline(always)]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
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
    #[doc = r" Value of the field"]
    pub struct CKEN0R {
        bits: bool,
    }
    impl CKEN0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct CKEN1R {
        bits: bool,
    }
    impl CKEN1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SEREN0R {
        bits: bool,
    }
    impl SEREN0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct SEREN1R {
        bits: bool,
    }
    impl SEREN1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DATA0R {
        bits: bool,
    }
    impl DATA0R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
    pub struct DATA1R {
        bits: bool,
    }
    impl DATA1R {
        #[doc = r" Value of the field as raw bits"]
        #[inline(always)]
        pub fn bit(&self) -> bool {
            self.bits
        }
        #[doc = r" Returns `true` if the bit is clear (0)"]
        #[inline(always)]
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
        pub fn bits(&self) -> u16 {
            self.bits
        }
        #[doc = "Bit 0 - Software Reset Synchronization Status"]
        #[inline(always)]
        pub fn swrst(&self) -> SWRSTR {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            SWRSTR { bits }
        }
        #[doc = "Bit 1 - Enable Synchronization Status"]
        #[inline(always)]
        pub fn enable(&self) -> ENABLER {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            ENABLER { bits }
        }
        #[doc = "Bit 2 - Clock Unit 0 Enable Synchronization Status"]
        #[inline(always)]
        pub fn cken0(&self) -> CKEN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            CKEN0R { bits }
        }
        #[doc = "Bit 3 - Clock Unit 1 Enable Synchronization Status"]
        #[inline(always)]
        pub fn cken1(&self) -> CKEN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            CKEN1R { bits }
        }
        #[doc = "Bit 4 - Serializer 0 Enable Synchronization Status"]
        #[inline(always)]
        pub fn seren0(&self) -> SEREN0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            SEREN0R { bits }
        }
        #[doc = "Bit 5 - Serializer 1 Enable Synchronization Status"]
        #[inline(always)]
        pub fn seren1(&self) -> SEREN1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            SEREN1R { bits }
        }
        #[doc = "Bit 8 - Data 0 Synchronization Status"]
        #[inline(always)]
        pub fn data0(&self) -> DATA0R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            DATA0R { bits }
        }
        #[doc = "Bit 9 - Data 1 Synchronization Status"]
        #[inline(always)]
        pub fn data1(&self) -> DATA1R {
            let bits = {
                const MASK: bool = true;
                const OFFSET: u8 = 9;
                ((self.bits >> OFFSET) & MASK as u16) != 0
            };
            DATA1R { bits }
        }
    }
}
