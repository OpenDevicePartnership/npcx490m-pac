#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mdma0_ctl: Mdma0Ctl,
    mdman_srcb0: MdmanSrcb0,
    mdman_dstb0: MdmanDstb0,
    mdma0_tcnt: Mdma0Tcnt,
    _reserved4: [u8; 0x04],
    mdman_cdst0: MdmanCdst0,
    mdma0_ctcnt: Mdma0Ctcnt,
    _reserved6: [u8; 0x04],
    mdma1_ctl: Mdma1Ctl,
    mdman_srcb1: MdmanSrcb1,
    mdman_dstb1: MdmanDstb1,
    mdma1_tcnt: Mdma1Tcnt,
    mdman_csrc1: MdmanCsrc1,
    _reserved11: [u8; 0x04],
    mdma1_ctcnt: Mdma1Ctcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel 0/1 Control Register (MDMAn_CTL0, MDMAn_CTL1)"]
    #[inline(always)]
    pub const fn mdma0_ctl(&self) -> &Mdma0Ctl {
        &self.mdma0_ctl
    }
    #[doc = "0x04 - Channel 0 Source Base Address Register (MDMAn_SRCB0)"]
    #[inline(always)]
    pub const fn mdman_srcb0(&self) -> &MdmanSrcb0 {
        &self.mdman_srcb0
    }
    #[doc = "0x08 - Channel 0 Destination Base Address Register (MDMAn_DSTB0)"]
    #[inline(always)]
    pub const fn mdman_dstb0(&self) -> &MdmanDstb0 {
        &self.mdman_dstb0
    }
    #[doc = "0x0c - Channel 0/1 Transfer Count Register (MDMAn_TCNT0, MDMAn_TCNT1)"]
    #[inline(always)]
    pub const fn mdma0_tcnt(&self) -> &Mdma0Tcnt {
        &self.mdma0_tcnt
    }
    #[doc = "0x14 - Channel 0 Current Destination Register (MDMAn_CDST0)"]
    #[inline(always)]
    pub const fn mdman_cdst0(&self) -> &MdmanCdst0 {
        &self.mdman_cdst0
    }
    #[doc = "0x18 - Channel 0/1 Current Transfer Count Register (MDMAn_CTCNT0, MDMAn_CTCNT1)"]
    #[inline(always)]
    pub const fn mdma0_ctcnt(&self) -> &Mdma0Ctcnt {
        &self.mdma0_ctcnt
    }
    #[doc = "0x20 - Channel 0/1 Control Register (MDMAn_CTL0, MDMAn_CTL1)"]
    #[inline(always)]
    pub const fn mdma1_ctl(&self) -> &Mdma1Ctl {
        &self.mdma1_ctl
    }
    #[doc = "0x24 - Channel 1 Source Base Address Register (MDMAn_SRCB1)"]
    #[inline(always)]
    pub const fn mdman_srcb1(&self) -> &MdmanSrcb1 {
        &self.mdman_srcb1
    }
    #[doc = "0x28 - Channel 1 Destination Base Address Register (MDMAn_DSTB1)"]
    #[inline(always)]
    pub const fn mdman_dstb1(&self) -> &MdmanDstb1 {
        &self.mdman_dstb1
    }
    #[doc = "0x2c - Channel 0/1 Transfer Count Register (MDMAn_TCNT0, MDMAn_TCNT1)"]
    #[inline(always)]
    pub const fn mdma1_tcnt(&self) -> &Mdma1Tcnt {
        &self.mdma1_tcnt
    }
    #[doc = "0x30 - Channel 1 Current Source Register (MDMAn_CSRC1)"]
    #[inline(always)]
    pub const fn mdman_csrc1(&self) -> &MdmanCsrc1 {
        &self.mdman_csrc1
    }
    #[doc = "0x38 - Channel 0/1 Current Transfer Count Register (MDMAn_CTCNT0, MDMAn_CTCNT1)"]
    #[inline(always)]
    pub const fn mdma1_ctcnt(&self) -> &Mdma1Ctcnt {
        &self.mdma1_ctcnt
    }
}
#[doc = "MDMA0_CTL (rw) register accessor: Channel 0/1 Control Register (MDMAn_CTL0, MDMAn_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma0_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma0_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma0_ctl`]
module"]
#[doc(alias = "MDMA0_CTL")]
pub type Mdma0Ctl = crate::Reg<mdma0_ctl::Mdma0CtlSpec>;
#[doc = "Channel 0/1 Control Register (MDMAn_CTL0, MDMAn_CTL1)"]
pub mod mdma0_ctl;
#[doc = "MDMA1_CTL (rw) register accessor: Channel 0/1 Control Register (MDMAn_CTL0, MDMAn_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma1_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma1_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma1_ctl`]
module"]
#[doc(alias = "MDMA1_CTL")]
pub type Mdma1Ctl = crate::Reg<mdma1_ctl::Mdma1CtlSpec>;
#[doc = "Channel 0/1 Control Register (MDMAn_CTL0, MDMAn_CTL1)"]
pub mod mdma1_ctl;
#[doc = "MDMAn_SRCB0 (rw) register accessor: Channel 0 Source Base Address Register (MDMAn_SRCB0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdman_srcb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdman_srcb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdman_srcb0`]
module"]
#[doc(alias = "MDMAn_SRCB0")]
pub type MdmanSrcb0 = crate::Reg<mdman_srcb0::MdmanSrcb0Spec>;
#[doc = "Channel 0 Source Base Address Register (MDMAn_SRCB0)"]
pub mod mdman_srcb0;
#[doc = "MDMAn_DSTB0 (rw) register accessor: Channel 0 Destination Base Address Register (MDMAn_DSTB0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdman_dstb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdman_dstb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdman_dstb0`]
module"]
#[doc(alias = "MDMAn_DSTB0")]
pub type MdmanDstb0 = crate::Reg<mdman_dstb0::MdmanDstb0Spec>;
#[doc = "Channel 0 Destination Base Address Register (MDMAn_DSTB0)"]
pub mod mdman_dstb0;
#[doc = "MDMA0_TCNT (rw) register accessor: Channel 0/1 Transfer Count Register (MDMAn_TCNT0, MDMAn_TCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma0_tcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma0_tcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma0_tcnt`]
module"]
#[doc(alias = "MDMA0_TCNT")]
pub type Mdma0Tcnt = crate::Reg<mdma0_tcnt::Mdma0TcntSpec>;
#[doc = "Channel 0/1 Transfer Count Register (MDMAn_TCNT0, MDMAn_TCNT1)"]
pub mod mdma0_tcnt;
#[doc = "MDMA1_TCNT (rw) register accessor: Channel 0/1 Transfer Count Register (MDMAn_TCNT0, MDMAn_TCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma1_tcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma1_tcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma1_tcnt`]
module"]
#[doc(alias = "MDMA1_TCNT")]
pub type Mdma1Tcnt = crate::Reg<mdma1_tcnt::Mdma1TcntSpec>;
#[doc = "Channel 0/1 Transfer Count Register (MDMAn_TCNT0, MDMAn_TCNT1)"]
pub mod mdma1_tcnt;
#[doc = "MDMAn_CDST0 (rw) register accessor: Channel 0 Current Destination Register (MDMAn_CDST0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdman_cdst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdman_cdst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdman_cdst0`]
module"]
#[doc(alias = "MDMAn_CDST0")]
pub type MdmanCdst0 = crate::Reg<mdman_cdst0::MdmanCdst0Spec>;
#[doc = "Channel 0 Current Destination Register (MDMAn_CDST0)"]
pub mod mdman_cdst0;
#[doc = "MDMA0_CTCNT (rw) register accessor: Channel 0/1 Current Transfer Count Register (MDMAn_CTCNT0, MDMAn_CTCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma0_ctcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma0_ctcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma0_ctcnt`]
module"]
#[doc(alias = "MDMA0_CTCNT")]
pub type Mdma0Ctcnt = crate::Reg<mdma0_ctcnt::Mdma0CtcntSpec>;
#[doc = "Channel 0/1 Current Transfer Count Register (MDMAn_CTCNT0, MDMAn_CTCNT1)"]
pub mod mdma0_ctcnt;
#[doc = "MDMA1_CTCNT (rw) register accessor: Channel 0/1 Current Transfer Count Register (MDMAn_CTCNT0, MDMAn_CTCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma1_ctcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma1_ctcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma1_ctcnt`]
module"]
#[doc(alias = "MDMA1_CTCNT")]
pub type Mdma1Ctcnt = crate::Reg<mdma1_ctcnt::Mdma1CtcntSpec>;
#[doc = "Channel 0/1 Current Transfer Count Register (MDMAn_CTCNT0, MDMAn_CTCNT1)"]
pub mod mdma1_ctcnt;
#[doc = "MDMAn_SRCB1 (rw) register accessor: Channel 1 Source Base Address Register (MDMAn_SRCB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdman_srcb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdman_srcb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdman_srcb1`]
module"]
#[doc(alias = "MDMAn_SRCB1")]
pub type MdmanSrcb1 = crate::Reg<mdman_srcb1::MdmanSrcb1Spec>;
#[doc = "Channel 1 Source Base Address Register (MDMAn_SRCB1)"]
pub mod mdman_srcb1;
#[doc = "MDMAn_DSTB1 (rw) register accessor: Channel 1 Destination Base Address Register (MDMAn_DSTB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdman_dstb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdman_dstb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdman_dstb1`]
module"]
#[doc(alias = "MDMAn_DSTB1")]
pub type MdmanDstb1 = crate::Reg<mdman_dstb1::MdmanDstb1Spec>;
#[doc = "Channel 1 Destination Base Address Register (MDMAn_DSTB1)"]
pub mod mdman_dstb1;
#[doc = "MDMAn_CSRC1 (rw) register accessor: Channel 1 Current Source Register (MDMAn_CSRC1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdman_csrc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdman_csrc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdman_csrc1`]
module"]
#[doc(alias = "MDMAn_CSRC1")]
pub type MdmanCsrc1 = crate::Reg<mdman_csrc1::MdmanCsrc1Spec>;
#[doc = "Channel 1 Current Source Register (MDMAn_CSRC1)"]
pub mod mdman_csrc1;
