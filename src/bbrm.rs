#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    bkup_sts: BkupSts,
    bkup_ctl: BkupCtl,
}
impl RegisterBlock {
    #[doc = "0x00 - BBRM Status Register (BKUP_STS)"]
    #[inline(always)]
    pub const fn bkup_sts(&self) -> &BkupSts {
        &self.bkup_sts
    }
    #[doc = "0x01 - BBRM Control Register (BKUP_CTL)"]
    #[inline(always)]
    pub const fn bkup_ctl(&self) -> &BkupCtl {
        &self.bkup_ctl
    }
}
#[doc = "BKUP_STS (rw) register accessor: BBRM Status Register (BKUP_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`bkup_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkup_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkup_sts`]
module"]
#[doc(alias = "BKUP_STS")]
pub type BkupSts = crate::Reg<bkup_sts::BkupStsSpec>;
#[doc = "BBRM Status Register (BKUP_STS)"]
pub mod bkup_sts;
#[doc = "BKUP_CTL (rw) register accessor: BBRM Control Register (BKUP_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`bkup_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkup_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkup_ctl`]
module"]
#[doc(alias = "BKUP_CTL")]
pub type BkupCtl = crate::Reg<bkup_ctl::BkupCtlSpec>;
#[doc = "BBRM Control Register (BKUP_CTL)"]
pub mod bkup_ctl;
