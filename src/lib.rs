#![cfg_attr ( feature = "rt" , feature ( asm ) ) ]
#![cfg_attr ( feature = "rt" , feature ( core_intrinsics ) ) ]
#![cfg_attr ( feature = "rt" , feature ( linkage ) ) ]
#![cfg_attr ( feature = "rt" , feature ( macro_reexport ) ) ]
#![cfg_attr ( feature = "rt" , feature ( naked_functions ) ) ]
#![cfg_attr ( feature = "rt" , feature ( used ) ) ]
#![doc = "Peripheral access API for ATSAMD21G18A microcontrollers (generated using svd2rust v0.11.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.11.1/svd2rust/#peripheral-api" ]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
#[macro_reexport(default_handler, exception)]
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate bare_metal;
extern crate vcell;
use core::ops::Deref;
use bare_metal::Peripheral;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[doc(hidden)]
pub mod interrupt;
pub use interrupt::Interrupt;
pub use cortex_m::peripheral::CPUID;
pub use cortex_m::peripheral::DCB;
pub use cortex_m::peripheral::DWT;
pub use cortex_m::peripheral::FPB;
pub use cortex_m::peripheral::FPU;
pub use cortex_m::peripheral::ITM;
pub use cortex_m::peripheral::MPU;
pub use cortex_m::peripheral::NVIC;
pub use cortex_m::peripheral::SCB;
pub use cortex_m::peripheral::SYST;
pub use cortex_m::peripheral::TPIU;
#[doc = "Analog Comparators"]
pub const AC: Peripheral<AC> = unsafe { Peripheral::new(1107313664) };
#[doc = "Analog Comparators"]
pub mod ac;
#[doc = "Analog Comparators"]
pub struct AC {
    register_block: ac::RegisterBlock,
}
impl Deref for AC {
    type Target = ac::RegisterBlock;
    fn deref(&self) -> &ac::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Analog Digital Converter"]
pub const ADC: Peripheral<ADC> = unsafe { Peripheral::new(1107312640) };
#[doc = "Analog Digital Converter"]
pub mod adc;
#[doc = "Analog Digital Converter"]
pub struct ADC {
    register_block: adc::RegisterBlock,
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Digital Analog Converter"]
pub const DAC: Peripheral<DAC> = unsafe { Peripheral::new(1107314688) };
#[doc = "Digital Analog Converter"]
pub mod dac;
#[doc = "Digital Analog Converter"]
pub struct DAC {
    register_block: dac::RegisterBlock,
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Direct Memory Access Controller"]
pub const DMAC: Peripheral<DMAC> = unsafe { Peripheral::new(1090537472) };
#[doc = "Direct Memory Access Controller"]
pub mod dmac;
#[doc = "Direct Memory Access Controller"]
pub struct DMAC {
    register_block: dmac::RegisterBlock,
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    fn deref(&self) -> &dmac::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Device Service Unit"]
pub const DSU: Peripheral<DSU> = unsafe { Peripheral::new(1090527232) };
#[doc = "Device Service Unit"]
pub mod dsu;
#[doc = "Device Service Unit"]
pub struct DSU {
    register_block: dsu::RegisterBlock,
}
impl Deref for DSU {
    type Target = dsu::RegisterBlock;
    fn deref(&self) -> &dsu::RegisterBlock {
        &self.register_block
    }
}
#[doc = "External Interrupt Controller"]
pub const EIC: Peripheral<EIC> = unsafe { Peripheral::new(1073747968) };
#[doc = "External Interrupt Controller"]
pub mod eic;
#[doc = "External Interrupt Controller"]
pub struct EIC {
    register_block: eic::RegisterBlock,
}
impl Deref for EIC {
    type Target = eic::RegisterBlock;
    fn deref(&self) -> &eic::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Event System Interface"]
pub const EVSYS: Peripheral<EVSYS> = unsafe { Peripheral::new(1107297280) };
#[doc = "Event System Interface"]
pub mod evsys;
#[doc = "Event System Interface"]
pub struct EVSYS {
    register_block: evsys::RegisterBlock,
}
impl Deref for EVSYS {
    type Target = evsys::RegisterBlock;
    fn deref(&self) -> &evsys::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Generic Clock Generator"]
pub const GCLK: Peripheral<GCLK> = unsafe { Peripheral::new(1073744896) };
#[doc = "Generic Clock Generator"]
pub mod gclk;
#[doc = "Generic Clock Generator"]
pub struct GCLK {
    register_block: gclk::RegisterBlock,
}
impl Deref for GCLK {
    type Target = gclk::RegisterBlock;
    fn deref(&self) -> &gclk::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Inter-IC Sound Interface"]
pub const I2S: Peripheral<I2S> = unsafe { Peripheral::new(1107316736) };
#[doc = "Inter-IC Sound Interface"]
pub mod i2s;
#[doc = "Inter-IC Sound Interface"]
pub struct I2S {
    register_block: i2s::RegisterBlock,
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    fn deref(&self) -> &i2s::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub const MTB: Peripheral<MTB> = unsafe { Peripheral::new(1090543616) };
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub mod mtb;
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub struct MTB {
    register_block: mtb::RegisterBlock,
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    fn deref(&self) -> &mtb::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Non-Volatile Memory Controller"]
pub const NVMCTRL: Peripheral<NVMCTRL> = unsafe { Peripheral::new(1090535424) };
#[doc = "Non-Volatile Memory Controller"]
pub mod nvmctrl;
#[doc = "Non-Volatile Memory Controller"]
pub struct NVMCTRL {
    register_block: nvmctrl::RegisterBlock,
}
impl Deref for NVMCTRL {
    type Target = nvmctrl::RegisterBlock;
    fn deref(&self) -> &nvmctrl::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Peripheral Access Controller 0"]
pub const PAC0: Peripheral<PAC0> = unsafe { Peripheral::new(1073741824) };
#[doc = "Peripheral Access Controller 0"]
pub mod pac0;
#[doc = "Peripheral Access Controller 0"]
pub struct PAC0 {
    register_block: pac0::RegisterBlock,
}
impl Deref for PAC0 {
    type Target = pac0::RegisterBlock;
    fn deref(&self) -> &pac0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Peripheral Access Controller 1"]
pub const PAC1: Peripheral<PAC1> = unsafe { Peripheral::new(1090519040) };
#[doc = r" Register block"]
pub struct PAC1 {
    register_block: pac0::RegisterBlock,
}
impl Deref for PAC1 {
    type Target = pac0::RegisterBlock;
    fn deref(&self) -> &pac0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Peripheral Access Controller 2"]
pub const PAC2: Peripheral<PAC2> = unsafe { Peripheral::new(1107296256) };
#[doc = r" Register block"]
pub struct PAC2 {
    register_block: pac0::RegisterBlock,
}
impl Deref for PAC2 {
    type Target = pac0::RegisterBlock;
    fn deref(&self) -> &pac0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Power Manager"]
pub const PM: Peripheral<PM> = unsafe { Peripheral::new(1073742848) };
#[doc = "Power Manager"]
pub mod pm;
#[doc = "Power Manager"]
pub struct PM {
    register_block: pm::RegisterBlock,
}
impl Deref for PM {
    type Target = pm::RegisterBlock;
    fn deref(&self) -> &pm::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port Module"]
pub const PORTA: Peripheral<PORTA> = unsafe { Peripheral::new(1090536448) }; // 0x41004400
#[doc = "Port Module"]
pub const PORTB: Peripheral<PORTB> = unsafe { Peripheral::new(1090536576) }; // 0x41004480
#[doc = "Port Module"]
pub mod port;
#[doc = "Port Module"]
pub struct PORTA {
    register_block: port::RegisterBlock,
}
impl Deref for PORTA {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &port::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port Module"]
pub struct PORTB {
    register_block: port::RegisterBlock,
}
impl Deref for PORTB {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &port::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control"]
pub const SYSCTRL: Peripheral<SYSCTRL> = unsafe { Peripheral::new(1073743872) };
#[doc = "System Control"]
pub mod sysctrl;
#[doc = "System Control"]
pub struct SYSCTRL {
    register_block: sysctrl::RegisterBlock,
}
impl Deref for SYSCTRL {
    type Target = sysctrl::RegisterBlock;
    fn deref(&self) -> &sysctrl::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Timer Counter Control 0"]
pub const TCC0: Peripheral<TCC0> = unsafe { Peripheral::new(1107304448) };
#[doc = "Timer Counter Control 0"]
pub mod tcc0;
#[doc = "Timer Counter Control 0"]
pub struct TCC0 {
    register_block: tcc0::RegisterBlock,
}
impl Deref for TCC0 {
    type Target = tcc0::RegisterBlock;
    fn deref(&self) -> &tcc0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Timer Counter Control 1"]
pub const TCC1: Peripheral<TCC1> = unsafe { Peripheral::new(1107305472) };
#[doc = r" Register block"]
pub struct TCC1 {
    register_block: tcc0::RegisterBlock,
}
impl Deref for TCC1 {
    type Target = tcc0::RegisterBlock;
    fn deref(&self) -> &tcc0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Timer Counter Control 2"]
pub const TCC2: Peripheral<TCC2> = unsafe { Peripheral::new(1107306496) };
#[doc = r" Register block"]
pub struct TCC2 {
    register_block: tcc0::RegisterBlock,
}
impl Deref for TCC2 {
    type Target = tcc0::RegisterBlock;
    fn deref(&self) -> &tcc0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Watchdog Timer"]
pub const WDT: Peripheral<WDT> = unsafe { Peripheral::new(1073745920) };
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Watchdog Timer"]
pub struct WDT {
    register_block: wdt::RegisterBlock,
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        &self.register_block
    }
}
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals<'a> {
    #[doc = "CPUID"]
    pub CPUID: &'a CPUID,
    #[doc = "DCB"]
    pub DCB: &'a DCB,
    #[doc = "DWT"]
    pub DWT: &'a DWT,
    #[doc = "FPB"]
    pub FPB: &'a FPB,
    #[doc = "FPU"]
    pub FPU: &'a FPU,
    #[doc = "ITM"]
    pub ITM: &'a ITM,
    #[doc = "MPU"]
    pub MPU: &'a MPU,
    #[doc = "NVIC"]
    pub NVIC: &'a NVIC,
    #[doc = "SCB"]
    pub SCB: &'a SCB,
    #[doc = "SYST"]
    pub SYST: &'a SYST,
    #[doc = "TPIU"]
    pub TPIU: &'a TPIU,
    #[doc = "AC"]
    pub AC: &'a AC,
    #[doc = "ADC"]
    pub ADC: &'a ADC,
    #[doc = "DAC"]
    pub DAC: &'a DAC,
    #[doc = "DMAC"]
    pub DMAC: &'a DMAC,
    #[doc = "DSU"]
    pub DSU: &'a DSU,
    #[doc = "EIC"]
    pub EIC: &'a EIC,
    #[doc = "EVSYS"]
    pub EVSYS: &'a EVSYS,
    #[doc = "GCLK"]
    pub GCLK: &'a GCLK,
    #[doc = "I2S"]
    pub I2S: &'a I2S,
    #[doc = "MTB"]
    pub MTB: &'a MTB,
    #[doc = "NVMCTRL"]
    pub NVMCTRL: &'a NVMCTRL,
    #[doc = "PAC0"]
    pub PAC0: &'a PAC0,
    #[doc = "PAC1"]
    pub PAC1: &'a PAC1,
    #[doc = "PAC2"]
    pub PAC2: &'a PAC2,
    #[doc = "PM"]
    pub PM: &'a PM,
    #[doc = "PORT A"]
    pub PORTA: &'a PORTA,
    #[doc = "PORTB"]
    pub PORTB: &'a PORTB,
    #[doc = "SYSCTRL"]
    pub SYSCTRL: &'a SYSCTRL,
    #[doc = "TCC1"]
    pub TCC1: &'a TCC1,
    #[doc = "TCC2"]
    pub TCC2: &'a TCC2,
    #[doc = "WDT"]
    pub WDT: &'a WDT,
}
impl<'a> Peripherals<'a> {
    #[doc = r" Grants access to all the peripherals"]
    pub unsafe fn all() -> Self {
        Peripherals {
            CPUID: &*CPUID.get(),
            DCB: &*DCB.get(),
            DWT: &*DWT.get(),
            FPB: &*FPB.get(),
            FPU: &*FPU.get(),
            ITM: &*ITM.get(),
            MPU: &*MPU.get(),
            NVIC: &*NVIC.get(),
            SCB: &*SCB.get(),
            SYST: &*SYST.get(),
            TPIU: &*TPIU.get(),
            AC: &*AC.get(),
            ADC: &*ADC.get(),
            DAC: &*DAC.get(),
            DMAC: &*DMAC.get(),
            DSU: &*DSU.get(),
            EIC: &*EIC.get(),
            EVSYS: &*EVSYS.get(),
            GCLK: &*GCLK.get(),
            I2S: &*I2S.get(),
            MTB: &*MTB.get(),
            NVMCTRL: &*NVMCTRL.get(),
            PAC0: &*PAC0.get(),
            PAC1: &*PAC1.get(),
            PAC2: &*PAC2.get(),
            PM: &*PM.get(),
            PORTA: &*PORTA.get(),
            PORTB: &*PORTB.get(),
            SYSCTRL: &*SYSCTRL.get(),
            TCC1: &*TCC1.get(),
            TCC2: &*TCC2.get(),
            WDT: &*WDT.get(),
        }
    }
}
