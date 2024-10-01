#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    gdma_ctl0: GdmaCtl0,
    gdma_srcb0: GdmaSrcb0,
    gdma_dstb0: GdmaDstb0,
    gdma_tcnt0: GdmaTcnt0,
    gdma_csrc0: GdmaCsrc0,
    gdma_cdst0: GdmaCdst0,
    gdma_ctcnt0: GdmaCtcnt0,
    gdma_req_sl0: GdmaReqSl0,
    gdma_ctl1: GdmaCtl1,
    gdma_srcb1: GdmaSrcb1,
    gdma_dstb1: GdmaDstb1,
    gdma_tcnt1: GdmaTcnt1,
    gdma_csrc1: GdmaCsrc1,
    gdma_cdst1: GdmaCdst1,
    gdma_ctcnt1: GdmaCtcnt1,
    gdma_req_sl1: GdmaReqSl1,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel 0/1 Control Register (GDMAn_CTL0, GDMAn_CTL1)"]
    #[inline(always)]
    pub const fn gdma_ctl0(&self) -> &GdmaCtl0 {
        &self.gdma_ctl0
    }
    #[doc = "0x04 - Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)"]
    #[inline(always)]
    pub const fn gdma_srcb0(&self) -> &GdmaSrcb0 {
        &self.gdma_srcb0
    }
    #[doc = "0x08 - Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)"]
    #[inline(always)]
    pub const fn gdma_dstb0(&self) -> &GdmaDstb0 {
        &self.gdma_dstb0
    }
    #[doc = "0x0c - Channel 0/1 Transfer Count Register (GDMAn_TCNT0, GDMAn_TCNT1)"]
    #[inline(always)]
    pub const fn gdma_tcnt0(&self) -> &GdmaTcnt0 {
        &self.gdma_tcnt0
    }
    #[doc = "0x10 - Channel 0/1 Current Source Register (GDMAn_CSRC0, GDMAn_CSRC1)"]
    #[inline(always)]
    pub const fn gdma_csrc0(&self) -> &GdmaCsrc0 {
        &self.gdma_csrc0
    }
    #[doc = "0x14 - Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)"]
    #[inline(always)]
    pub const fn gdma_cdst0(&self) -> &GdmaCdst0 {
        &self.gdma_cdst0
    }
    #[doc = "0x18 - Channel 0/1 Current Transfer Count Register (GDMAn_CTCNT0, GDMAn_CTCNT1)"]
    #[inline(always)]
    pub const fn gdma_ctcnt0(&self) -> &GdmaCtcnt0 {
        &self.gdma_ctcnt0
    }
    #[doc = "0x1c - Channel 0/1 GDMA Request Select Register (GDMAn_REQ_SL0, GDMAn_REQ_SL1)"]
    #[inline(always)]
    pub const fn gdma_req_sl0(&self) -> &GdmaReqSl0 {
        &self.gdma_req_sl0
    }
    #[doc = "0x20 - Channel 0/1 Control Register (GDMAn_CTL0, GDMAn_CTL1)"]
    #[inline(always)]
    pub const fn gdma_ctl1(&self) -> &GdmaCtl1 {
        &self.gdma_ctl1
    }
    #[doc = "0x24 - Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)"]
    #[inline(always)]
    pub const fn gdma_srcb1(&self) -> &GdmaSrcb1 {
        &self.gdma_srcb1
    }
    #[doc = "0x28 - Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)"]
    #[inline(always)]
    pub const fn gdma_dstb1(&self) -> &GdmaDstb1 {
        &self.gdma_dstb1
    }
    #[doc = "0x2c - Channel 0/1 Transfer Count Register (GDMAn_TCNT0, GDMAn_TCNT1)"]
    #[inline(always)]
    pub const fn gdma_tcnt1(&self) -> &GdmaTcnt1 {
        &self.gdma_tcnt1
    }
    #[doc = "0x30 - Channel 0/1 Current Source Register (GDMAn_CSRC0, GDMAn_CSRC1)"]
    #[inline(always)]
    pub const fn gdma_csrc1(&self) -> &GdmaCsrc1 {
        &self.gdma_csrc1
    }
    #[doc = "0x34 - Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)"]
    #[inline(always)]
    pub const fn gdma_cdst1(&self) -> &GdmaCdst1 {
        &self.gdma_cdst1
    }
    #[doc = "0x38 - Channel 0/1 Current Transfer Count Register (GDMAn_CTCNT0, GDMAn_CTCNT1)"]
    #[inline(always)]
    pub const fn gdma_ctcnt1(&self) -> &GdmaCtcnt1 {
        &self.gdma_ctcnt1
    }
    #[doc = "0x3c - Channel 0/1 GDMA Request Select Register (GDMAn_REQ_SL0, GDMAn_REQ_SL1)"]
    #[inline(always)]
    pub const fn gdma_req_sl1(&self) -> &GdmaReqSl1 {
        &self.gdma_req_sl1
    }
}
#[doc = "GDMA_CTL0 (rw) register accessor: Channel 0/1 Control Register (GDMAn_CTL0, GDMAn_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctl0`]
module"]
#[doc(alias = "GDMA_CTL0")]
pub type GdmaCtl0 = crate::Reg<gdma_ctl0::GdmaCtl0Spec>;
#[doc = "Channel 0/1 Control Register (GDMAn_CTL0, GDMAn_CTL1)"]
pub mod gdma_ctl0;
#[doc = "GDMA_CTL1 (rw) register accessor: Channel 0/1 Control Register (GDMAn_CTL0, GDMAn_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctl1`]
module"]
#[doc(alias = "GDMA_CTL1")]
pub type GdmaCtl1 = crate::Reg<gdma_ctl1::GdmaCtl1Spec>;
#[doc = "Channel 0/1 Control Register (GDMAn_CTL0, GDMAn_CTL1)"]
pub mod gdma_ctl1;
#[doc = "GDMA_SRCB0 (rw) register accessor: Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_srcb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_srcb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_srcb0`]
module"]
#[doc(alias = "GDMA_SRCB0")]
pub type GdmaSrcb0 = crate::Reg<gdma_srcb0::GdmaSrcb0Spec>;
#[doc = "Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)"]
pub mod gdma_srcb0;
#[doc = "GDMA_SRCB1 (rw) register accessor: Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_srcb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_srcb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_srcb1`]
module"]
#[doc(alias = "GDMA_SRCB1")]
pub type GdmaSrcb1 = crate::Reg<gdma_srcb1::GdmaSrcb1Spec>;
#[doc = "Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)"]
pub mod gdma_srcb1;
#[doc = "GDMA_DSTB0 (rw) register accessor: Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_dstb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_dstb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_dstb0`]
module"]
#[doc(alias = "GDMA_DSTB0")]
pub type GdmaDstb0 = crate::Reg<gdma_dstb0::GdmaDstb0Spec>;
#[doc = "Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)"]
pub mod gdma_dstb0;
#[doc = "GDMA_DSTB1 (rw) register accessor: Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_dstb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_dstb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_dstb1`]
module"]
#[doc(alias = "GDMA_DSTB1")]
pub type GdmaDstb1 = crate::Reg<gdma_dstb1::GdmaDstb1Spec>;
#[doc = "Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)"]
pub mod gdma_dstb1;
#[doc = "GDMA_TCNT0 (rw) register accessor: Channel 0/1 Transfer Count Register (GDMAn_TCNT0, GDMAn_TCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_tcnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_tcnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_tcnt0`]
module"]
#[doc(alias = "GDMA_TCNT0")]
pub type GdmaTcnt0 = crate::Reg<gdma_tcnt0::GdmaTcnt0Spec>;
#[doc = "Channel 0/1 Transfer Count Register (GDMAn_TCNT0, GDMAn_TCNT1)"]
pub mod gdma_tcnt0;
#[doc = "GDMA_TCNT1 (rw) register accessor: Channel 0/1 Transfer Count Register (GDMAn_TCNT0, GDMAn_TCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_tcnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_tcnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_tcnt1`]
module"]
#[doc(alias = "GDMA_TCNT1")]
pub type GdmaTcnt1 = crate::Reg<gdma_tcnt1::GdmaTcnt1Spec>;
#[doc = "Channel 0/1 Transfer Count Register (GDMAn_TCNT0, GDMAn_TCNT1)"]
pub mod gdma_tcnt1;
#[doc = "GDMA_CSRC0 (rw) register accessor: Channel 0/1 Current Source Register (GDMAn_CSRC0, GDMAn_CSRC1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_csrc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_csrc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_csrc0`]
module"]
#[doc(alias = "GDMA_CSRC0")]
pub type GdmaCsrc0 = crate::Reg<gdma_csrc0::GdmaCsrc0Spec>;
#[doc = "Channel 0/1 Current Source Register (GDMAn_CSRC0, GDMAn_CSRC1)"]
pub mod gdma_csrc0;
#[doc = "GDMA_CSRC1 (rw) register accessor: Channel 0/1 Current Source Register (GDMAn_CSRC0, GDMAn_CSRC1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_csrc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_csrc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_csrc1`]
module"]
#[doc(alias = "GDMA_CSRC1")]
pub type GdmaCsrc1 = crate::Reg<gdma_csrc1::GdmaCsrc1Spec>;
#[doc = "Channel 0/1 Current Source Register (GDMAn_CSRC0, GDMAn_CSRC1)"]
pub mod gdma_csrc1;
#[doc = "GDMA_CDST0 (rw) register accessor: Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_cdst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_cdst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_cdst0`]
module"]
#[doc(alias = "GDMA_CDST0")]
pub type GdmaCdst0 = crate::Reg<gdma_cdst0::GdmaCdst0Spec>;
#[doc = "Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)"]
pub mod gdma_cdst0;
#[doc = "GDMA_CDST1 (rw) register accessor: Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_cdst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_cdst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_cdst1`]
module"]
#[doc(alias = "GDMA_CDST1")]
pub type GdmaCdst1 = crate::Reg<gdma_cdst1::GdmaCdst1Spec>;
#[doc = "Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)"]
pub mod gdma_cdst1;
#[doc = "GDMA_CTCNT0 (rw) register accessor: Channel 0/1 Current Transfer Count Register (GDMAn_CTCNT0, GDMAn_CTCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctcnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctcnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctcnt0`]
module"]
#[doc(alias = "GDMA_CTCNT0")]
pub type GdmaCtcnt0 = crate::Reg<gdma_ctcnt0::GdmaCtcnt0Spec>;
#[doc = "Channel 0/1 Current Transfer Count Register (GDMAn_CTCNT0, GDMAn_CTCNT1)"]
pub mod gdma_ctcnt0;
#[doc = "GDMA_CTCNT1 (rw) register accessor: Channel 0/1 Current Transfer Count Register (GDMAn_CTCNT0, GDMAn_CTCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctcnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctcnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctcnt1`]
module"]
#[doc(alias = "GDMA_CTCNT1")]
pub type GdmaCtcnt1 = crate::Reg<gdma_ctcnt1::GdmaCtcnt1Spec>;
#[doc = "Channel 0/1 Current Transfer Count Register (GDMAn_CTCNT0, GDMAn_CTCNT1)"]
pub mod gdma_ctcnt1;
#[doc = "GDMA_REQ_SL0 (rw) register accessor: Channel 0/1 GDMA Request Select Register (GDMAn_REQ_SL0, GDMAn_REQ_SL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_req_sl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_req_sl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_req_sl0`]
module"]
#[doc(alias = "GDMA_REQ_SL0")]
pub type GdmaReqSl0 = crate::Reg<gdma_req_sl0::GdmaReqSl0Spec>;
#[doc = "Channel 0/1 GDMA Request Select Register (GDMAn_REQ_SL0, GDMAn_REQ_SL1)"]
pub mod gdma_req_sl0;
#[doc = "GDMA_REQ_SL1 (rw) register accessor: Channel 0/1 GDMA Request Select Register (GDMAn_REQ_SL0, GDMAn_REQ_SL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_req_sl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_req_sl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_req_sl1`]
module"]
#[doc(alias = "GDMA_REQ_SL1")]
pub type GdmaReqSl1 = crate::Reg<gdma_req_sl1::GdmaReqSl1Spec>;
#[doc = "Channel 0/1 GDMA Request Select Register (GDMAn_REQ_SL0, GDMAn_REQ_SL1)"]
pub mod gdma_req_sl1;
