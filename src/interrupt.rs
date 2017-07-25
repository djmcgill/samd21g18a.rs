
use bare_metal::Nr;
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn PM() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn SYSCTRL() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn WDT() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn RTC() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn EIC() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn NVMCTRL() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn DMAC() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn USB() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn EVSYS() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn SERCOM0() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn SERCOM1() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn SERCOM2() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn SERCOM3() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn SERCOM4() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn SERCOM5() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn TCC0() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn TCC1() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn TCC2() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn TC3() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn TC4() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn TC5() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn ADC() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn AC() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn DAC() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(non_snake_case)]
#[allow(private_no_mangle_fns)]
#[cfg(feature = "rt")]
#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn I2S() {
    unsafe {
        asm! ( "b DEFAULT_HANDLER" :: :: "volatile" );
        ::core::intrinsics::unreachable()
    }
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<extern "C" fn()>; 28] = [
    Some(PM),
    Some(SYSCTRL),
    Some(WDT),
    Some(RTC),
    Some(EIC),
    Some(NVMCTRL),
    Some(DMAC),
    Some(USB),
    Some(EVSYS),
    Some(SERCOM0),
    Some(SERCOM1),
    Some(SERCOM2),
    Some(SERCOM3),
    Some(SERCOM4),
    Some(SERCOM5),
    Some(TCC0),
    Some(TCC1),
    Some(TCC2),
    Some(TC3),
    Some(TC4),
    Some(TC5),
    None,
    None,
    Some(ADC),
    Some(AC),
    Some(DAC),
    None,
    Some(I2S),
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - PM"]
    PM,
    #[doc = "1 - SYSCTRL"]
    SYSCTRL,
    #[doc = "2 - WDT"]
    WDT,
    #[doc = "3 - RTC"]
    RTC,
    #[doc = "4 - EIC"]
    EIC,
    #[doc = "5 - NVMCTRL"]
    NVMCTRL,
    #[doc = "6 - DMAC"]
    DMAC,
    #[doc = "7 - USB"]
    USB,
    #[doc = "8 - EVSYS"]
    EVSYS,
    #[doc = "9 - SERCOM0"]
    SERCOM0,
    #[doc = "10 - SERCOM1"]
    SERCOM1,
    #[doc = "11 - SERCOM2"]
    SERCOM2,
    #[doc = "12 - SERCOM3"]
    SERCOM3,
    #[doc = "13 - SERCOM4"]
    SERCOM4,
    #[doc = "14 - SERCOM5"]
    SERCOM5,
    #[doc = "15 - TCC0"]
    TCC0,
    #[doc = "16 - TCC1"]
    TCC1,
    #[doc = "17 - TCC2"]
    TCC2,
    #[doc = "18 - TC3"]
    TC3,
    #[doc = "19 - TC4"]
    TC4,
    #[doc = "20 - TC5"]
    TC5,
    #[doc = "23 - ADC"]
    ADC,
    #[doc = "24 - AC"]
    AC,
    #[doc = "25 - DAC"]
    DAC,
    #[doc = "27 - I2S"]
    I2S,
}
unsafe impl Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PM => 0,
            Interrupt::SYSCTRL => 1,
            Interrupt::WDT => 2,
            Interrupt::RTC => 3,
            Interrupt::EIC => 4,
            Interrupt::NVMCTRL => 5,
            Interrupt::DMAC => 6,
            Interrupt::USB => 7,
            Interrupt::EVSYS => 8,
            Interrupt::SERCOM0 => 9,
            Interrupt::SERCOM1 => 10,
            Interrupt::SERCOM2 => 11,
            Interrupt::SERCOM3 => 12,
            Interrupt::SERCOM4 => 13,
            Interrupt::SERCOM5 => 14,
            Interrupt::TCC0 => 15,
            Interrupt::TCC1 => 16,
            Interrupt::TCC2 => 17,
            Interrupt::TC3 => 18,
            Interrupt::TC4 => 19,
            Interrupt::TC5 => 20,
            Interrupt::ADC => 23,
            Interrupt::AC => 24,
            Interrupt::DAC => 25,
            Interrupt::I2S => 27,
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ident = $ lval : expr; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * }; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path; f ( unsafe { & mut LOCALS } ); } }; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME; let f : fn ( ) = $ path; f ( ); } } }

