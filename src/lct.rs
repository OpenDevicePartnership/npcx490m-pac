#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x02],
    lctcont: Lctcont,
    _reserved1: [u8; 0x01],
    lctstat: Lctstat,
    lctsecond: Lctsecond,
    lctminut: Lctminut,
    _reserved4: [u8; 0x01],
    lcthour: Lcthour,
    _reserved5: [u8; 0x01],
    lctday: Lctday,
    _reserved6: [u8; 0x01],
    lctweek: Lctweek,
    lctweekm: Lctweekm,
}
impl RegisterBlock {
    #[doc = "0x02 - LCT Control Register (LCTCONT)"]
    #[inline(always)]
    pub const fn lctcont(&self) -> &Lctcont {
        &self.lctcont
    }
    #[doc = "0x04 - LCT Status Register (LCTSTAT)"]
    #[inline(always)]
    pub const fn lctstat(&self) -> &Lctstat {
        &self.lctstat
    }
    #[doc = "0x05 - LCT Seconds Register (LCTSECOND)"]
    #[inline(always)]
    pub const fn lctsecond(&self) -> &Lctsecond {
        &self.lctsecond
    }
    #[doc = "0x06 - LCT Minutes Register (LCTMINUT)"]
    #[inline(always)]
    pub const fn lctminut(&self) -> &Lctminut {
        &self.lctminut
    }
    #[doc = "0x08 - LCT Hours Register (LCTHOUR)"]
    #[inline(always)]
    pub const fn lcthour(&self) -> &Lcthour {
        &self.lcthour
    }
    #[doc = "0x0a - LCT Days Register (LCTDAY)"]
    #[inline(always)]
    pub const fn lctday(&self) -> &Lctday {
        &self.lctday
    }
    #[doc = "0x0c - LCT Weeks LSB Register (LCTWEEK)"]
    #[inline(always)]
    pub const fn lctweek(&self) -> &Lctweek {
        &self.lctweek
    }
    #[doc = "0x0d - LCT Weeks MSB Register (LCTWEEKM)"]
    #[inline(always)]
    pub const fn lctweekm(&self) -> &Lctweekm {
        &self.lctweekm
    }
}
#[doc = "LCTCONT (rw) register accessor: LCT Control Register (LCTCONT)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctcont::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctcont::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lctcont`]
module"]
#[doc(alias = "LCTCONT")]
pub type Lctcont = crate::Reg<lctcont::LctcontSpec>;
#[doc = "LCT Control Register (LCTCONT)"]
pub mod lctcont;
#[doc = "LCTSTAT (rw) register accessor: LCT Status Register (LCTSTAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lctstat`]
module"]
#[doc(alias = "LCTSTAT")]
pub type Lctstat = crate::Reg<lctstat::LctstatSpec>;
#[doc = "LCT Status Register (LCTSTAT)"]
pub mod lctstat;
#[doc = "LCTSECOND (rw) register accessor: LCT Seconds Register (LCTSECOND)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctsecond::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctsecond::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lctsecond`]
module"]
#[doc(alias = "LCTSECOND")]
pub type Lctsecond = crate::Reg<lctsecond::LctsecondSpec>;
#[doc = "LCT Seconds Register (LCTSECOND)"]
pub mod lctsecond;
#[doc = "LCTMINUT (rw) register accessor: LCT Minutes Register (LCTMINUT)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctminut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctminut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lctminut`]
module"]
#[doc(alias = "LCTMINUT")]
pub type Lctminut = crate::Reg<lctminut::LctminutSpec>;
#[doc = "LCT Minutes Register (LCTMINUT)"]
pub mod lctminut;
#[doc = "LCTHOUR (rw) register accessor: LCT Hours Register (LCTHOUR)\n\nYou can [`read`](crate::Reg::read) this register and get [`lcthour::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcthour::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcthour`]
module"]
#[doc(alias = "LCTHOUR")]
pub type Lcthour = crate::Reg<lcthour::LcthourSpec>;
#[doc = "LCT Hours Register (LCTHOUR)"]
pub mod lcthour;
#[doc = "LCTDAY (rw) register accessor: LCT Days Register (LCTDAY)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctday::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctday::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lctday`]
module"]
#[doc(alias = "LCTDAY")]
pub type Lctday = crate::Reg<lctday::LctdaySpec>;
#[doc = "LCT Days Register (LCTDAY)"]
pub mod lctday;
#[doc = "LCTWEEK (rw) register accessor: LCT Weeks LSB Register (LCTWEEK)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctweek::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctweek::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lctweek`]
module"]
#[doc(alias = "LCTWEEK")]
pub type Lctweek = crate::Reg<lctweek::LctweekSpec>;
#[doc = "LCT Weeks LSB Register (LCTWEEK)"]
pub mod lctweek;
#[doc = "LCTWEEKM (rw) register accessor: LCT Weeks MSB Register (LCTWEEKM)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctweekm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctweekm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lctweekm`]
module"]
#[doc(alias = "LCTWEEKM")]
pub type Lctweekm = crate::Reg<lctweekm::LctweekmSpec>;
#[doc = "LCT Weeks MSB Register (LCTWEEKM)"]
pub mod lctweekm;
