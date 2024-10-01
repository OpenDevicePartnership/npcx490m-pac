#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    prscn: Prscn,
    _reserved_1_ctrn: [u8; 0x02],
    pwmctln: Pwmctln,
    n_step_rsn: NStepRsn,
    _reserved_4_dcrn: [u8; 0x02],
    ctr_fln: CtrFln,
    max_dc_fln: MaxDcFln,
    pwmctlexn: Pwmctlexn,
    n_step_fln: NStepFln,
    ext_onn: ExtOnn,
    ext_offn: ExtOffn,
    min_dc_rsfln: MinDcRsfln,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Prescaler Register (PRSCn)"]
    #[inline(always)]
    pub const fn prscn(&self) -> &Prscn {
        &self.prscn
    }
    #[doc = "0x02 - Cycle Time Register (CTRn)"]
    #[inline(always)]
    pub const fn ctrn(&self) -> &Ctrn {
        unsafe { &*(self as *const Self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x02 - Cycle Time for Rise Duty Cycle Register (CTR_RSn)"]
    #[inline(always)]
    pub const fn ctr_rsn(&self) -> &CtrRsn {
        unsafe { &*(self as *const Self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x04 - PWM Control Register (PWMCTLn)"]
    #[inline(always)]
    pub const fn pwmctln(&self) -> &Pwmctln {
        &self.pwmctln
    }
    #[doc = "0x05 - Number of Steps for Rise Duty Cycle Register (N_STEP_RSn)"]
    #[inline(always)]
    pub const fn n_step_rsn(&self) -> &NStepRsn {
        &self.n_step_rsn
    }
    #[doc = "0x06 - Duty Cycle Register (DCRn)"]
    #[inline(always)]
    pub const fn dcrn(&self) -> &Dcrn {
        unsafe { &*(self as *const Self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - Maximum Duty Cycle Rise Register (MAX_DC_RSn)"]
    #[inline(always)]
    pub const fn max_dc_rsn(&self) -> &MaxDcRsn {
        unsafe { &*(self as *const Self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - Cycle Time for Fall Duty Cycle Register (CTR_FLn)"]
    #[inline(always)]
    pub const fn ctr_fln(&self) -> &CtrFln {
        &self.ctr_fln
    }
    #[doc = "0x0a - Maximum Duty Cycle Fall Register (MAX_DC_FLn)"]
    #[inline(always)]
    pub const fn max_dc_fln(&self) -> &MaxDcFln {
        &self.max_dc_fln
    }
    #[doc = "0x0c - PWM Control Extended Register (PWMCTLEXn)"]
    #[inline(always)]
    pub const fn pwmctlexn(&self) -> &Pwmctlexn {
        &self.pwmctlexn
    }
    #[doc = "0x0d - Number of Steps for Fall Duty Cycle Register (N_STEP_FLn)"]
    #[inline(always)]
    pub const fn n_step_fln(&self) -> &NStepFln {
        &self.n_step_fln
    }
    #[doc = "0x0e - Extend ON Time Register (EXT_ONn)"]
    #[inline(always)]
    pub const fn ext_onn(&self) -> &ExtOnn {
        &self.ext_onn
    }
    #[doc = "0x0f - Extend OFF Time Register (EXT_OFFn)"]
    #[inline(always)]
    pub const fn ext_offn(&self) -> &ExtOffn {
        &self.ext_offn
    }
    #[doc = "0x10 - Minimum Duty Cycle Rise/Fall Register (MIN_DC_RSFLn)"]
    #[inline(always)]
    pub const fn min_dc_rsfln(&self) -> &MinDcRsfln {
        &self.min_dc_rsfln
    }
}
#[doc = "PRSCn (rw) register accessor: Clock Prescaler Register (PRSCn)\n\nYou can [`read`](crate::Reg::read) this register and get [`prscn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prscn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prscn`]
module"]
#[doc(alias = "PRSCn")]
pub type Prscn = crate::Reg<prscn::PrscnSpec>;
#[doc = "Clock Prescaler Register (PRSCn)"]
pub mod prscn;
#[doc = "CTR_RSn (rw) register accessor: Cycle Time for Rise Duty Cycle Register (CTR_RSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr_rsn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr_rsn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr_rsn`]
module"]
#[doc(alias = "CTR_RSn")]
pub type CtrRsn = crate::Reg<ctr_rsn::CtrRsnSpec>;
#[doc = "Cycle Time for Rise Duty Cycle Register (CTR_RSn)"]
pub mod ctr_rsn;
#[doc = "CTRn (rw) register accessor: Cycle Time Register (CTRn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrn`]
module"]
#[doc(alias = "CTRn")]
pub type Ctrn = crate::Reg<ctrn::CtrnSpec>;
#[doc = "Cycle Time Register (CTRn)"]
pub mod ctrn;
#[doc = "PWMCTLn (rw) register accessor: PWM Control Register (PWMCTLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmctln::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmctln::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmctln`]
module"]
#[doc(alias = "PWMCTLn")]
pub type Pwmctln = crate::Reg<pwmctln::PwmctlnSpec>;
#[doc = "PWM Control Register (PWMCTLn)"]
pub mod pwmctln;
#[doc = "N_STEP_RSn (rw) register accessor: Number of Steps for Rise Duty Cycle Register (N_STEP_RSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`n_step_rsn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`n_step_rsn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_step_rsn`]
module"]
#[doc(alias = "N_STEP_RSn")]
pub type NStepRsn = crate::Reg<n_step_rsn::NStepRsnSpec>;
#[doc = "Number of Steps for Rise Duty Cycle Register (N_STEP_RSn)"]
pub mod n_step_rsn;
#[doc = "MAX_DC_RSn (rw) register accessor: Maximum Duty Cycle Rise Register (MAX_DC_RSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`max_dc_rsn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_dc_rsn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_dc_rsn`]
module"]
#[doc(alias = "MAX_DC_RSn")]
pub type MaxDcRsn = crate::Reg<max_dc_rsn::MaxDcRsnSpec>;
#[doc = "Maximum Duty Cycle Rise Register (MAX_DC_RSn)"]
pub mod max_dc_rsn;
#[doc = "DCRn (rw) register accessor: Duty Cycle Register (DCRn)\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrn`]
module"]
#[doc(alias = "DCRn")]
pub type Dcrn = crate::Reg<dcrn::DcrnSpec>;
#[doc = "Duty Cycle Register (DCRn)"]
pub mod dcrn;
#[doc = "CTR_FLn (rw) register accessor: Cycle Time for Fall Duty Cycle Register (CTR_FLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr_fln::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr_fln::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr_fln`]
module"]
#[doc(alias = "CTR_FLn")]
pub type CtrFln = crate::Reg<ctr_fln::CtrFlnSpec>;
#[doc = "Cycle Time for Fall Duty Cycle Register (CTR_FLn)"]
pub mod ctr_fln;
#[doc = "MAX_DC_FLn (rw) register accessor: Maximum Duty Cycle Fall Register (MAX_DC_FLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`max_dc_fln::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_dc_fln::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_dc_fln`]
module"]
#[doc(alias = "MAX_DC_FLn")]
pub type MaxDcFln = crate::Reg<max_dc_fln::MaxDcFlnSpec>;
#[doc = "Maximum Duty Cycle Fall Register (MAX_DC_FLn)"]
pub mod max_dc_fln;
#[doc = "PWMCTLEXn (rw) register accessor: PWM Control Extended Register (PWMCTLEXn)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmctlexn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmctlexn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmctlexn`]
module"]
#[doc(alias = "PWMCTLEXn")]
pub type Pwmctlexn = crate::Reg<pwmctlexn::PwmctlexnSpec>;
#[doc = "PWM Control Extended Register (PWMCTLEXn)"]
pub mod pwmctlexn;
#[doc = "N_STEP_FLn (rw) register accessor: Number of Steps for Fall Duty Cycle Register (N_STEP_FLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`n_step_fln::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`n_step_fln::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_step_fln`]
module"]
#[doc(alias = "N_STEP_FLn")]
pub type NStepFln = crate::Reg<n_step_fln::NStepFlnSpec>;
#[doc = "Number of Steps for Fall Duty Cycle Register (N_STEP_FLn)"]
pub mod n_step_fln;
#[doc = "EXT_ONn (rw) register accessor: Extend ON Time Register (EXT_ONn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_onn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_onn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_onn`]
module"]
#[doc(alias = "EXT_ONn")]
pub type ExtOnn = crate::Reg<ext_onn::ExtOnnSpec>;
#[doc = "Extend ON Time Register (EXT_ONn)"]
pub mod ext_onn;
#[doc = "EXT_OFFn (rw) register accessor: Extend OFF Time Register (EXT_OFFn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_offn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_offn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_offn`]
module"]
#[doc(alias = "EXT_OFFn")]
pub type ExtOffn = crate::Reg<ext_offn::ExtOffnSpec>;
#[doc = "Extend OFF Time Register (EXT_OFFn)"]
pub mod ext_offn;
#[doc = "MIN_DC_RSFLn (rw) register accessor: Minimum Duty Cycle Rise/Fall Register (MIN_DC_RSFLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`min_dc_rsfln::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min_dc_rsfln::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@min_dc_rsfln`]
module"]
#[doc(alias = "MIN_DC_RSFLn")]
pub type MinDcRsfln = crate::Reg<min_dc_rsfln::MinDcRsflnSpec>;
#[doc = "Minimum Duty Cycle Rise/Fall Register (MIN_DC_RSFLn)"]
pub mod min_dc_rsfln;
