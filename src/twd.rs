#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    twcfg: Twcfg,
    _reserved1: [u8; 0x01],
    twcp: Twcp,
    _reserved2: [u8; 0x01],
    twdt0: Twdt0,
    t0csr: T0csr,
    _reserved4: [u8; 0x01],
    wdcnt: Wdcnt,
    _reserved5: [u8; 0x01],
    wdsdm: Wdsdm,
    _reserved6: [u8; 0x01],
    twmt0: Twmt0,
    twmwd: Twmwd,
    _reserved8: [u8; 0x01],
    wdcp: Wdcp,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer and Watchdog Configuration Register (TWCFG)"]
    #[inline(always)]
    pub const fn twcfg(&self) -> &Twcfg {
        &self.twcfg
    }
    #[doc = "0x02 - Timer and Watchdog Clock Prescaler Register (TWCP)"]
    #[inline(always)]
    pub const fn twcp(&self) -> &Twcp {
        &self.twcp
    }
    #[doc = "0x04 - TWD Timer 0 Register (TWDT0)"]
    #[inline(always)]
    pub const fn twdt0(&self) -> &Twdt0 {
        &self.twdt0
    }
    #[doc = "0x06 - TWDT0 Control and Status Register (T0CSR)"]
    #[inline(always)]
    pub const fn t0csr(&self) -> &T0csr {
        &self.t0csr
    }
    #[doc = "0x08 - Watchdog Count Register (WDCNT)"]
    #[inline(always)]
    pub const fn wdcnt(&self) -> &Wdcnt {
        &self.wdcnt
    }
    #[doc = "0x0a - Watchdog Service Data Match Register (WDSDM)"]
    #[inline(always)]
    pub const fn wdsdm(&self) -> &Wdsdm {
        &self.wdsdm
    }
    #[doc = "0x0c - TWD Timer 0 Counter Register (TWMT0)"]
    #[inline(always)]
    pub const fn twmt0(&self) -> &Twmt0 {
        &self.twmt0
    }
    #[doc = "0x0e - Watchdog Counter Register (TWMWD)"]
    #[inline(always)]
    pub const fn twmwd(&self) -> &Twmwd {
        &self.twmwd
    }
    #[doc = "0x10 - Watchdog Clock Prescaler Register (WDCP)"]
    #[inline(always)]
    pub const fn wdcp(&self) -> &Wdcp {
        &self.wdcp
    }
}
#[doc = "TWCFG (rw) register accessor: Timer and Watchdog Configuration Register (TWCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`twcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twcfg`]
module"]
#[doc(alias = "TWCFG")]
pub type Twcfg = crate::Reg<twcfg::TwcfgSpec>;
#[doc = "Timer and Watchdog Configuration Register (TWCFG)"]
pub mod twcfg;
#[doc = "TWCP (rw) register accessor: Timer and Watchdog Clock Prescaler Register (TWCP)\n\nYou can [`read`](crate::Reg::read) this register and get [`twcp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twcp`]
module"]
#[doc(alias = "TWCP")]
pub type Twcp = crate::Reg<twcp::TwcpSpec>;
#[doc = "Timer and Watchdog Clock Prescaler Register (TWCP)"]
pub mod twcp;
#[doc = "TWDT0 (rw) register accessor: TWD Timer 0 Register (TWDT0)\n\nYou can [`read`](crate::Reg::read) this register and get [`twdt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twdt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twdt0`]
module"]
#[doc(alias = "TWDT0")]
pub type Twdt0 = crate::Reg<twdt0::Twdt0Spec>;
#[doc = "TWD Timer 0 Register (TWDT0)"]
pub mod twdt0;
#[doc = "T0CSR (rw) register accessor: TWDT0 Control and Status Register (T0CSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`t0csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0csr`]
module"]
#[doc(alias = "T0CSR")]
pub type T0csr = crate::Reg<t0csr::T0csrSpec>;
#[doc = "TWDT0 Control and Status Register (T0CSR)"]
pub mod t0csr;
#[doc = "WDCNT (rw) register accessor: Watchdog Count Register (WDCNT)\n\nYou can [`read`](crate::Reg::read) this register and get [`wdcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdcnt`]
module"]
#[doc(alias = "WDCNT")]
pub type Wdcnt = crate::Reg<wdcnt::WdcntSpec>;
#[doc = "Watchdog Count Register (WDCNT)"]
pub mod wdcnt;
#[doc = "WDSDM (rw) register accessor: Watchdog Service Data Match Register (WDSDM)\n\nYou can [`read`](crate::Reg::read) this register and get [`wdsdm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdsdm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdsdm`]
module"]
#[doc(alias = "WDSDM")]
pub type Wdsdm = crate::Reg<wdsdm::WdsdmSpec>;
#[doc = "Watchdog Service Data Match Register (WDSDM)"]
pub mod wdsdm;
#[doc = "TWMT0 (rw) register accessor: TWD Timer 0 Counter Register (TWMT0)\n\nYou can [`read`](crate::Reg::read) this register and get [`twmt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twmt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twmt0`]
module"]
#[doc(alias = "TWMT0")]
pub type Twmt0 = crate::Reg<twmt0::Twmt0Spec>;
#[doc = "TWD Timer 0 Counter Register (TWMT0)"]
pub mod twmt0;
#[doc = "TWMWD (rw) register accessor: Watchdog Counter Register (TWMWD)\n\nYou can [`read`](crate::Reg::read) this register and get [`twmwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twmwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twmwd`]
module"]
#[doc(alias = "TWMWD")]
pub type Twmwd = crate::Reg<twmwd::TwmwdSpec>;
#[doc = "Watchdog Counter Register (TWMWD)"]
pub mod twmwd;
#[doc = "WDCP (rw) register accessor: Watchdog Clock Prescaler Register (WDCP)\n\nYou can [`read`](crate::Reg::read) this register and get [`wdcp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdcp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdcp`]
module"]
#[doc(alias = "WDCP")]
pub type Wdcp = crate::Reg<wdcp::WdcpSpec>;
#[doc = "Watchdog Clock Prescaler Register (WDCP)"]
pub mod wdcp;
