#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ttc: Ttc,
    wtc: Wtc,
}
impl RegisterBlock {
    #[doc = "0x00 - Timing Ticks Count Register (TTC)"]
    #[inline(always)]
    pub const fn ttc(&self) -> &Ttc {
        &self.ttc
    }
    #[doc = "0x04 - Wake-Up Ticks Count Register (WTC)"]
    #[inline(always)]
    pub const fn wtc(&self) -> &Wtc {
        &self.wtc
    }
}
#[doc = "TTC (rw) register accessor: Timing Ticks Count Register (TTC)\n\nYou can [`read`](crate::Reg::read) this register and get [`ttc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttc`]
module"]
#[doc(alias = "TTC")]
pub type Ttc = crate::Reg<ttc::TtcSpec>;
#[doc = "Timing Ticks Count Register (TTC)"]
pub mod ttc;
#[doc = "WTC (rw) register accessor: Wake-Up Ticks Count Register (WTC)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtc`]
module"]
#[doc(alias = "WTC")]
pub type Wtc = crate::Reg<wtc::WtcSpec>;
#[doc = "Wake-Up Ticks Count Register (WTC)"]
pub mod wtc;
