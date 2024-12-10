#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    smbn_sda: SmbnSda,
    _reserved1: [u8; 0x01],
    smbn_st: SmbnSt,
    _reserved2: [u8; 0x01],
    smbn_cst: SmbnCst,
    _reserved3: [u8; 0x01],
    smbn_ctl1: SmbnCtl1,
    _reserved4: [u8; 0x01],
    smbn_addr1: SmbnAddr1,
    _reserved5: [u8; 0x01],
    smbn_ctl2: SmbnCtl2,
    _reserved6: [u8; 0x01],
    smbn_addr2: SmbnAddr2,
    _reserved7: [u8; 0x01],
    smbn_ctl3: SmbnCtl3,
    smbn_t_out: SmbnTOut,
    _reserved_9_smbn: [u8; 0x01],
    smbn_addr7: SmbnAddr7,
    _reserved_11_smbn: [u8; 0x01],
    _reserved_12_smbn: [u8; 0x01],
    smbn_addr5: SmbnAddr5,
    _reserved14: [u8; 0x01],
    _reserved_14_smbn: [u8; 0x01],
    _reserved15: [u8; 0x01],
    smbn_cst2: SmbnCst2,
    smbn_cst3: SmbnCst3,
    _reserved_17_smbn: [u8; 0x01],
    _reserved18: [u8; 0x01],
    _reserved_18_smbn: [u8; 0x01],
    smbn_fif_ctl: SmbnFifCtl,
    _reserved_20_smbn: [u8; 0x01],
}
impl RegisterBlock {
    #[doc = "0x00 - SMB Serial Data Register (SMBnSDA)"]
    #[inline(always)]
    pub const fn smbn_sda(&self) -> &SmbnSda {
        &self.smbn_sda
    }
    #[doc = "0x02 - SMB Status Register (SMBnST)"]
    #[inline(always)]
    pub const fn smbn_st(&self) -> &SmbnSt {
        &self.smbn_st
    }
    #[doc = "0x04 - SMB Control Status Register (SMBnCST)"]
    #[inline(always)]
    pub const fn smbn_cst(&self) -> &SmbnCst {
        &self.smbn_cst
    }
    #[doc = "0x06 - SMB Control 1 Register (SMBnCTL1)"]
    #[inline(always)]
    pub const fn smbn_ctl1(&self) -> &SmbnCtl1 {
        &self.smbn_ctl1
    }
    #[doc = "0x08 - SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
    #[inline(always)]
    pub const fn smbn_addr1(&self) -> &SmbnAddr1 {
        &self.smbn_addr1
    }
    #[doc = "0x0a - SMB Control 2 Register (SMBnCTL2)"]
    #[inline(always)]
    pub const fn smbn_ctl2(&self) -> &SmbnCtl2 {
        &self.smbn_ctl2
    }
    #[doc = "0x0c - SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
    #[inline(always)]
    pub const fn smbn_addr2(&self) -> &SmbnAddr2 {
        &self.smbn_addr2
    }
    #[doc = "0x0e - SMB Control 3 Register (SMBnCTL3)"]
    #[inline(always)]
    pub const fn smbn_ctl3(&self) -> &SmbnCtl3 {
        &self.smbn_ctl3
    }
    #[doc = "0x0f - SMB Bus Timeout Register (SMBnT_OUT)"]
    #[inline(always)]
    pub const fn smbn_t_out(&self) -> &SmbnTOut {
        &self.smbn_t_out
    }
    #[doc = "0x10 - SMB FIFO Control and Status Register (SMBnFIF_CTS)"]
    #[inline(always)]
    pub const fn smbn_fif_cts(&self) -> &SmbnFifCts {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
    #[inline(always)]
    pub const fn smbn_addr3(&self) -> &SmbnAddr3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x11 - SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
    #[inline(always)]
    pub const fn smbn_addr7(&self) -> &SmbnAddr7 {
        &self.smbn_addr7
    }
    #[doc = "0x12 - SMB Tx-FIFO Control Register (SMBnTXF_CTL)"]
    #[inline(always)]
    pub const fn smbn_txf_ctl(&self) -> &SmbnTxfCtl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x12 - SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
    #[inline(always)]
    pub const fn smbn_addr4(&self) -> &SmbnAddr4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x13 - SMB Frame Timeout Register (SMBn_FRTO)"]
    #[inline(always)]
    pub const fn smbn_frto(&self) -> &SmbnFrto {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(19).cast() }
    }
    #[doc = "0x13 - SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
    #[inline(always)]
    pub const fn smbn_addr8(&self) -> &SmbnAddr8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(19).cast() }
    }
    #[doc = "0x14 - SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
    #[inline(always)]
    pub const fn smbn_addr5(&self) -> &SmbnAddr5 {
        &self.smbn_addr5
    }
    #[doc = "0x16 - SMB PEC Register (SMBnPEC)"]
    #[inline(always)]
    pub const fn smbn_pec(&self) -> &SmbnPec {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(22).cast() }
    }
    #[doc = "0x16 - SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
    #[inline(always)]
    pub const fn smbn_addr6(&self) -> &SmbnAddr6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(22).cast() }
    }
    #[doc = "0x18 - SMB Control Status 2 Register (SMBnCST2)"]
    #[inline(always)]
    pub const fn smbn_cst2(&self) -> &SmbnCst2 {
        &self.smbn_cst2
    }
    #[doc = "0x19 - SMB Control Status 3 Register (SMBnCST3)"]
    #[inline(always)]
    pub const fn smbn_cst3(&self) -> &SmbnCst3 {
        &self.smbn_cst3
    }
    #[doc = "0x1a - SMB Control 4 Register (SMBnCTL4)"]
    #[inline(always)]
    pub const fn smbn_ctl4(&self) -> &SmbnCtl4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1a - SMB Tx-FIFO Status Register (SMBnTXF_STS)"]
    #[inline(always)]
    pub const fn smbn_txf_sts(&self) -> &SmbnTxfSts {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1c - SMB Rx-FIFO Status Register (SMBnRXF_STS)"]
    #[inline(always)]
    pub const fn smbn_rxf_sts(&self) -> &SmbnRxfSts {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - SMB SCL Low Time Register (SMBnSCLLT)"]
    #[inline(always)]
    pub const fn smbn_scllt(&self) -> &SmbnScllt {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1d - SMB FIFO Control Register (SMBnFIF_CTL)"]
    #[inline(always)]
    pub const fn smbn_fif_ctl(&self) -> &SmbnFifCtl {
        &self.smbn_fif_ctl
    }
    #[doc = "0x1e - SMB SCL High Time Register (SMBnSCLHT)"]
    #[inline(always)]
    pub const fn smbn_sclht(&self) -> &SmbnSclht {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
    #[doc = "0x1e - SMB Rx-FIFO Control Register (SMBnRXF_CTL)"]
    #[inline(always)]
    pub const fn smbn_rxf_ctl(&self) -> &SmbnRxfCtl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
}
#[doc = "SMBnSDA (rw) register accessor: SMB Serial Data Register (SMBnSDA)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_sda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_sda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_sda`]
module"]
#[doc(alias = "SMBnSDA")]
pub type SmbnSda = crate::Reg<smbn_sda::SmbnSdaSpec>;
#[doc = "SMB Serial Data Register (SMBnSDA)"]
pub mod smbn_sda;
#[doc = "SMBnST (rw) register accessor: SMB Status Register (SMBnST)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_st`]
module"]
#[doc(alias = "SMBnST")]
pub type SmbnSt = crate::Reg<smbn_st::SmbnStSpec>;
#[doc = "SMB Status Register (SMBnST)"]
pub mod smbn_st;
#[doc = "SMBnCST (rw) register accessor: SMB Control Status Register (SMBnCST)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_cst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_cst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_cst`]
module"]
#[doc(alias = "SMBnCST")]
pub type SmbnCst = crate::Reg<smbn_cst::SmbnCstSpec>;
#[doc = "SMB Control Status Register (SMBnCST)"]
pub mod smbn_cst;
#[doc = "SMBnCTL1 (rw) register accessor: SMB Control 1 Register (SMBnCTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_ctl1`]
module"]
#[doc(alias = "SMBnCTL1")]
pub type SmbnCtl1 = crate::Reg<smbn_ctl1::SmbnCtl1Spec>;
#[doc = "SMB Control 1 Register (SMBnCTL1)"]
pub mod smbn_ctl1;
#[doc = "SMBnADDR1 (rw) register accessor: SMB Own Address 1 - 8 Register (SMBnADDR1-8)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_addr1`]
module"]
#[doc(alias = "SMBnADDR1")]
pub type SmbnAddr1 = crate::Reg<smbn_addr1::SmbnAddr1Spec>;
#[doc = "SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
pub mod smbn_addr1;
#[doc = "SMBnADDR2 (rw) register accessor: SMB Own Address 1 - 8 Register (SMBnADDR1-8)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_addr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_addr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_addr2`]
module"]
#[doc(alias = "SMBnADDR2")]
pub type SmbnAddr2 = crate::Reg<smbn_addr2::SmbnAddr2Spec>;
#[doc = "SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
pub mod smbn_addr2;
#[doc = "SMBnADDR3 (rw) register accessor: SMB Own Address 1 - 8 Register (SMBnADDR1-8)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_addr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_addr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_addr3`]
module"]
#[doc(alias = "SMBnADDR3")]
pub type SmbnAddr3 = crate::Reg<smbn_addr3::SmbnAddr3Spec>;
#[doc = "SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
pub mod smbn_addr3;
#[doc = "SMBnADDR4 (rw) register accessor: SMB Own Address 1 - 8 Register (SMBnADDR1-8)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_addr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_addr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_addr4`]
module"]
#[doc(alias = "SMBnADDR4")]
pub type SmbnAddr4 = crate::Reg<smbn_addr4::SmbnAddr4Spec>;
#[doc = "SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
pub mod smbn_addr4;
#[doc = "SMBnADDR5 (rw) register accessor: SMB Own Address 1 - 8 Register (SMBnADDR1-8)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_addr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_addr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_addr5`]
module"]
#[doc(alias = "SMBnADDR5")]
pub type SmbnAddr5 = crate::Reg<smbn_addr5::SmbnAddr5Spec>;
#[doc = "SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
pub mod smbn_addr5;
#[doc = "SMBnADDR6 (rw) register accessor: SMB Own Address 1 - 8 Register (SMBnADDR1-8)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_addr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_addr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_addr6`]
module"]
#[doc(alias = "SMBnADDR6")]
pub type SmbnAddr6 = crate::Reg<smbn_addr6::SmbnAddr6Spec>;
#[doc = "SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
pub mod smbn_addr6;
#[doc = "SMBnADDR7 (rw) register accessor: SMB Own Address 1 - 8 Register (SMBnADDR1-8)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_addr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_addr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_addr7`]
module"]
#[doc(alias = "SMBnADDR7")]
pub type SmbnAddr7 = crate::Reg<smbn_addr7::SmbnAddr7Spec>;
#[doc = "SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
pub mod smbn_addr7;
#[doc = "SMBnADDR8 (rw) register accessor: SMB Own Address 1 - 8 Register (SMBnADDR1-8)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_addr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_addr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_addr8`]
module"]
#[doc(alias = "SMBnADDR8")]
pub type SmbnAddr8 = crate::Reg<smbn_addr8::SmbnAddr8Spec>;
#[doc = "SMB Own Address 1 - 8 Register (SMBnADDR1-8)"]
pub mod smbn_addr8;
#[doc = "SMBnCTL2 (rw) register accessor: SMB Control 2 Register (SMBnCTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_ctl2`]
module"]
#[doc(alias = "SMBnCTL2")]
pub type SmbnCtl2 = crate::Reg<smbn_ctl2::SmbnCtl2Spec>;
#[doc = "SMB Control 2 Register (SMBnCTL2)"]
pub mod smbn_ctl2;
#[doc = "SMBnCTL3 (rw) register accessor: SMB Control 3 Register (SMBnCTL3)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_ctl3`]
module"]
#[doc(alias = "SMBnCTL3")]
pub type SmbnCtl3 = crate::Reg<smbn_ctl3::SmbnCtl3Spec>;
#[doc = "SMB Control 3 Register (SMBnCTL3)"]
pub mod smbn_ctl3;
#[doc = "SMBnT_OUT (rw) register accessor: SMB Bus Timeout Register (SMBnT_OUT)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_t_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_t_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_t_out`]
module"]
#[doc(alias = "SMBnT_OUT")]
pub type SmbnTOut = crate::Reg<smbn_t_out::SmbnTOutSpec>;
#[doc = "SMB Bus Timeout Register (SMBnT_OUT)"]
pub mod smbn_t_out;
#[doc = "SMBnFIF_CTS (rw) register accessor: SMB FIFO Control and Status Register (SMBnFIF_CTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_fif_cts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_fif_cts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_fif_cts`]
module"]
#[doc(alias = "SMBnFIF_CTS")]
pub type SmbnFifCts = crate::Reg<smbn_fif_cts::SmbnFifCtsSpec>;
#[doc = "SMB FIFO Control and Status Register (SMBnFIF_CTS)"]
pub mod smbn_fif_cts;
#[doc = "SMBnTXF_CTL (rw) register accessor: SMB Tx-FIFO Control Register (SMBnTXF_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_txf_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_txf_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_txf_ctl`]
module"]
#[doc(alias = "SMBnTXF_CTL")]
pub type SmbnTxfCtl = crate::Reg<smbn_txf_ctl::SmbnTxfCtlSpec>;
#[doc = "SMB Tx-FIFO Control Register (SMBnTXF_CTL)"]
pub mod smbn_txf_ctl;
#[doc = "SMBn_FRTO (rw) register accessor: SMB Frame Timeout Register (SMBn_FRTO)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_frto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_frto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_frto`]
module"]
#[doc(alias = "SMBn_FRTO")]
pub type SmbnFrto = crate::Reg<smbn_frto::SmbnFrtoSpec>;
#[doc = "SMB Frame Timeout Register (SMBn_FRTO)"]
pub mod smbn_frto;
#[doc = "SMBnPEC (rw) register accessor: SMB PEC Register (SMBnPEC)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_pec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_pec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_pec`]
module"]
#[doc(alias = "SMBnPEC")]
pub type SmbnPec = crate::Reg<smbn_pec::SmbnPecSpec>;
#[doc = "SMB PEC Register (SMBnPEC)"]
pub mod smbn_pec;
#[doc = "SMBnCST2 (rw) register accessor: SMB Control Status 2 Register (SMBnCST2)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_cst2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_cst2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_cst2`]
module"]
#[doc(alias = "SMBnCST2")]
pub type SmbnCst2 = crate::Reg<smbn_cst2::SmbnCst2Spec>;
#[doc = "SMB Control Status 2 Register (SMBnCST2)"]
pub mod smbn_cst2;
#[doc = "SMBnCST3 (rw) register accessor: SMB Control Status 3 Register (SMBnCST3)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_cst3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_cst3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_cst3`]
module"]
#[doc(alias = "SMBnCST3")]
pub type SmbnCst3 = crate::Reg<smbn_cst3::SmbnCst3Spec>;
#[doc = "SMB Control Status 3 Register (SMBnCST3)"]
pub mod smbn_cst3;
#[doc = "SMBnTXF_STS (rw) register accessor: SMB Tx-FIFO Status Register (SMBnTXF_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_txf_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_txf_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_txf_sts`]
module"]
#[doc(alias = "SMBnTXF_STS")]
pub type SmbnTxfSts = crate::Reg<smbn_txf_sts::SmbnTxfStsSpec>;
#[doc = "SMB Tx-FIFO Status Register (SMBnTXF_STS)"]
pub mod smbn_txf_sts;
#[doc = "SMBnCTL4 (rw) register accessor: SMB Control 4 Register (SMBnCTL4)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_ctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_ctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_ctl4`]
module"]
#[doc(alias = "SMBnCTL4")]
pub type SmbnCtl4 = crate::Reg<smbn_ctl4::SmbnCtl4Spec>;
#[doc = "SMB Control 4 Register (SMBnCTL4)"]
pub mod smbn_ctl4;
#[doc = "SMBnSCLLT (rw) register accessor: SMB SCL Low Time Register (SMBnSCLLT)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_scllt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_scllt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_scllt`]
module"]
#[doc(alias = "SMBnSCLLT")]
pub type SmbnScllt = crate::Reg<smbn_scllt::SmbnSclltSpec>;
#[doc = "SMB SCL Low Time Register (SMBnSCLLT)"]
pub mod smbn_scllt;
#[doc = "SMBnRXF_STS (rw) register accessor: SMB Rx-FIFO Status Register (SMBnRXF_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_rxf_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_rxf_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_rxf_sts`]
module"]
#[doc(alias = "SMBnRXF_STS")]
pub type SmbnRxfSts = crate::Reg<smbn_rxf_sts::SmbnRxfStsSpec>;
#[doc = "SMB Rx-FIFO Status Register (SMBnRXF_STS)"]
pub mod smbn_rxf_sts;
#[doc = "SMBnFIF_CTL (rw) register accessor: SMB FIFO Control Register (SMBnFIF_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_fif_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_fif_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_fif_ctl`]
module"]
#[doc(alias = "SMBnFIF_CTL")]
pub type SmbnFifCtl = crate::Reg<smbn_fif_ctl::SmbnFifCtlSpec>;
#[doc = "SMB FIFO Control Register (SMBnFIF_CTL)"]
pub mod smbn_fif_ctl;
#[doc = "SMBnRXF_CTL (rw) register accessor: SMB Rx-FIFO Control Register (SMBnRXF_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_rxf_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_rxf_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_rxf_ctl`]
module"]
#[doc(alias = "SMBnRXF_CTL")]
pub type SmbnRxfCtl = crate::Reg<smbn_rxf_ctl::SmbnRxfCtlSpec>;
#[doc = "SMB Rx-FIFO Control Register (SMBnRXF_CTL)"]
pub mod smbn_rxf_ctl;
#[doc = "SMBnSCLHT (rw) register accessor: SMB SCL High Time Register (SMBnSCLHT)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_sclht::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_sclht::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbn_sclht`]
module"]
#[doc(alias = "SMBnSCLHT")]
pub type SmbnSclht = crate::Reg<smbn_sclht::SmbnSclhtSpec>;
#[doc = "SMB SCL High Time Register (SMBnSCLHT)"]
pub mod smbn_sclht;
