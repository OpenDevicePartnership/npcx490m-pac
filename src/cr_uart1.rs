#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    utbufn: Utbufn,
    _reserved1: [u8; 0x01],
    urbufn: Urbufn,
    _reserved2: [u8; 0x03],
    ustatn: Ustatn,
    _reserved3: [u8; 0x01],
    ufrsn: Ufrsn,
    _reserved4: [u8; 0x01],
    umdsln: Umdsln,
    _reserved5: [u8; 0x01],
    ubaudn: Ubaudn,
    _reserved6: [u8; 0x01],
    upsrn: Upsrn,
    _reserved7: [u8; 0x11],
    uftstsn: Uftstsn,
    _reserved8: [u8; 0x01],
    ufrstsn: Ufrstsn,
    _reserved9: [u8; 0x01],
    uftctln: Uftctln,
    _reserved10: [u8; 0x01],
    ufrctln: Ufrctln,
    _reserved11: [u8; 0x05],
    ucntln: Ucntln,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit Data Buffer Register (UTBUFn)"]
    #[inline(always)]
    pub const fn utbufn(&self) -> &Utbufn {
        &self.utbufn
    }
    #[doc = "0x02 - Receive Data Buffer Register (URBUFn)"]
    #[inline(always)]
    pub const fn urbufn(&self) -> &Urbufn {
        &self.urbufn
    }
    #[doc = "0x06 - Status Register (USTATn)"]
    #[inline(always)]
    pub const fn ustatn(&self) -> &Ustatn {
        &self.ustatn
    }
    #[doc = "0x08 - Frame Select Register (UFRSn)"]
    #[inline(always)]
    pub const fn ufrsn(&self) -> &Ufrsn {
        &self.ufrsn
    }
    #[doc = "0x0a - Mode Select Register (UMDSLn)"]
    #[inline(always)]
    pub const fn umdsln(&self) -> &Umdsln {
        &self.umdsln
    }
    #[doc = "0x0c - Baud Rate Divisor Register (UBAUDn)"]
    #[inline(always)]
    pub const fn ubaudn(&self) -> &Ubaudn {
        &self.ubaudn
    }
    #[doc = "0x0e - Baud Rate Prescaler Register (UPSRn)"]
    #[inline(always)]
    pub const fn upsrn(&self) -> &Upsrn {
        &self.upsrn
    }
    #[doc = "0x20 - FIFO Mode Transmit Status Register (UFTSTSn)"]
    #[inline(always)]
    pub const fn uftstsn(&self) -> &Uftstsn {
        &self.uftstsn
    }
    #[doc = "0x22 - FIFO Mode Receive Status Register (UFRSTSn)"]
    #[inline(always)]
    pub const fn ufrstsn(&self) -> &Ufrstsn {
        &self.ufrstsn
    }
    #[doc = "0x24 - FIFO Mode Transmit Control Register (UFTCTLn)"]
    #[inline(always)]
    pub const fn uftctln(&self) -> &Uftctln {
        &self.uftctln
    }
    #[doc = "0x26 - FIFO Mode Receive Control Register (UFRCTLn)"]
    #[inline(always)]
    pub const fn ufrctln(&self) -> &Ufrctln {
        &self.ufrctln
    }
    #[doc = "0x2c - Control Register (UCNTLn)"]
    #[inline(always)]
    pub const fn ucntln(&self) -> &Ucntln {
        &self.ucntln
    }
}
#[doc = "UTBUFn (w) register accessor: Transmit Data Buffer Register (UTBUFn)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utbufn::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utbufn`]
module"]
#[doc(alias = "UTBUFn")]
pub type Utbufn = crate::Reg<utbufn::UtbufnSpec>;
#[doc = "Transmit Data Buffer Register (UTBUFn)"]
pub mod utbufn;
#[doc = "URBUFn (r) register accessor: Receive Data Buffer Register (URBUFn)\n\nYou can [`read`](crate::Reg::read) this register and get [`urbufn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urbufn`]
module"]
#[doc(alias = "URBUFn")]
pub type Urbufn = crate::Reg<urbufn::UrbufnSpec>;
#[doc = "Receive Data Buffer Register (URBUFn)"]
pub mod urbufn;
#[doc = "USTATn (r) register accessor: Status Register (USTATn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ustatn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ustatn`]
module"]
#[doc(alias = "USTATn")]
pub type Ustatn = crate::Reg<ustatn::UstatnSpec>;
#[doc = "Status Register (USTATn)"]
pub mod ustatn;
#[doc = "UFRSn (rw) register accessor: Frame Select Register (UFRSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ufrsn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ufrsn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ufrsn`]
module"]
#[doc(alias = "UFRSn")]
pub type Ufrsn = crate::Reg<ufrsn::UfrsnSpec>;
#[doc = "Frame Select Register (UFRSn)"]
pub mod ufrsn;
#[doc = "UMDSLn (rw) register accessor: Mode Select Register (UMDSLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`umdsln::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`umdsln::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@umdsln`]
module"]
#[doc(alias = "UMDSLn")]
pub type Umdsln = crate::Reg<umdsln::UmdslnSpec>;
#[doc = "Mode Select Register (UMDSLn)"]
pub mod umdsln;
#[doc = "UBAUDn (rw) register accessor: Baud Rate Divisor Register (UBAUDn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ubaudn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubaudn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubaudn`]
module"]
#[doc(alias = "UBAUDn")]
pub type Ubaudn = crate::Reg<ubaudn::UbaudnSpec>;
#[doc = "Baud Rate Divisor Register (UBAUDn)"]
pub mod ubaudn;
#[doc = "UPSRn (rw) register accessor: Baud Rate Prescaler Register (UPSRn)\n\nYou can [`read`](crate::Reg::read) this register and get [`upsrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upsrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upsrn`]
module"]
#[doc(alias = "UPSRn")]
pub type Upsrn = crate::Reg<upsrn::UpsrnSpec>;
#[doc = "Baud Rate Prescaler Register (UPSRn)"]
pub mod upsrn;
#[doc = "UFTSTSn (r) register accessor: FIFO Mode Transmit Status Register (UFTSTSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`uftstsn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uftstsn`]
module"]
#[doc(alias = "UFTSTSn")]
pub type Uftstsn = crate::Reg<uftstsn::UftstsnSpec>;
#[doc = "FIFO Mode Transmit Status Register (UFTSTSn)"]
pub mod uftstsn;
#[doc = "UFRSTSn (r) register accessor: FIFO Mode Receive Status Register (UFRSTSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ufrstsn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ufrstsn`]
module"]
#[doc(alias = "UFRSTSn")]
pub type Ufrstsn = crate::Reg<ufrstsn::UfrstsnSpec>;
#[doc = "FIFO Mode Receive Status Register (UFRSTSn)"]
pub mod ufrstsn;
#[doc = "UFTCTLn (rw) register accessor: FIFO Mode Transmit Control Register (UFTCTLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`uftctln::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uftctln::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uftctln`]
module"]
#[doc(alias = "UFTCTLn")]
pub type Uftctln = crate::Reg<uftctln::UftctlnSpec>;
#[doc = "FIFO Mode Transmit Control Register (UFTCTLn)"]
pub mod uftctln;
#[doc = "UFRCTLn (rw) register accessor: FIFO Mode Receive Control Register (UFRCTLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ufrctln::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ufrctln::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ufrctln`]
module"]
#[doc(alias = "UFRCTLn")]
pub type Ufrctln = crate::Reg<ufrctln::UfrctlnSpec>;
#[doc = "FIFO Mode Receive Control Register (UFRCTLn)"]
pub mod ufrctln;
#[doc = "UCNTLn (rw) register accessor: Control Register (UCNTLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ucntln::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucntln::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucntln`]
module"]
#[doc(alias = "UCNTLn")]
pub type Ucntln = crate::Reg<ucntln::UcntlnSpec>;
#[doc = "Control Register (UCNTLn)"]
pub mod ucntln;
