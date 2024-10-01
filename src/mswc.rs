#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mswctl1: Mswctl1,
    _reserved1: [u8; 0x07],
    hcbal: Hcbal,
    _reserved2: [u8; 0x01],
    hcbah: Hcbah,
    _reserved3: [u8; 0x07],
    host_ctl: HostCtl,
    _reserved4: [u8; 0x09],
    srid_cr: SridCr,
    _reserved5: [u8; 0x03],
    sid_cr: SidCr,
    _reserved6: [u8; 0x01],
    device_id_cr: DeviceIdCr,
    _reserved7: [u8; 0x05],
    chprev_cr: ChprevCr,
}
impl RegisterBlock {
    #[doc = "0x00 - MSWC Control Status 1 Register (MSWCTL1)"]
    #[inline(always)]
    pub const fn mswctl1(&self) -> &Mswctl1 {
        &self.mswctl1
    }
    #[doc = "0x08 - Host Configuration Base Address Low Register (HCBAL)"]
    #[inline(always)]
    pub const fn hcbal(&self) -> &Hcbal {
        &self.hcbal
    }
    #[doc = "0x0a - Host Configuration Base Address High Register (HCBAH)"]
    #[inline(always)]
    pub const fn hcbah(&self) -> &Hcbah {
        &self.hcbah
    }
    #[doc = "0x12 - Host Control Register (HOST_CTL)"]
    #[inline(always)]
    pub const fn host_ctl(&self) -> &HostCtl {
        &self.host_ctl
    }
    #[doc = "0x1c - SRID Core Access Register (SRID_CR)"]
    #[inline(always)]
    pub const fn srid_cr(&self) -> &SridCr {
        &self.srid_cr
    }
    #[doc = "0x20 - SID Core Access Register (SID_CR)"]
    #[inline(always)]
    pub const fn sid_cr(&self) -> &SidCr {
        &self.sid_cr
    }
    #[doc = "0x22 - DEVICE_ID Core Access Register (DEVICE_ID_CR)"]
    #[inline(always)]
    pub const fn device_id_cr(&self) -> &DeviceIdCr {
        &self.device_id_cr
    }
    #[doc = "0x28 - Chip Revision Core Access Register (CHPREV_CR)"]
    #[inline(always)]
    pub const fn chprev_cr(&self) -> &ChprevCr {
        &self.chprev_cr
    }
}
#[doc = "MSWCTL1 (rw) register accessor: MSWC Control Status 1 Register (MSWCTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mswctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mswctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mswctl1`]
module"]
#[doc(alias = "MSWCTL1")]
pub type Mswctl1 = crate::Reg<mswctl1::Mswctl1Spec>;
#[doc = "MSWC Control Status 1 Register (MSWCTL1)"]
pub mod mswctl1;
#[doc = "HCBAL (rw) register accessor: Host Configuration Base Address Low Register (HCBAL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcbal`]
module"]
#[doc(alias = "HCBAL")]
pub type Hcbal = crate::Reg<hcbal::HcbalSpec>;
#[doc = "Host Configuration Base Address Low Register (HCBAL)"]
pub mod hcbal;
#[doc = "HCBAH (rw) register accessor: Host Configuration Base Address High Register (HCBAH)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbah::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbah::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcbah`]
module"]
#[doc(alias = "HCBAH")]
pub type Hcbah = crate::Reg<hcbah::HcbahSpec>;
#[doc = "Host Configuration Base Address High Register (HCBAH)"]
pub mod hcbah;
#[doc = "HOST_CTL (rw) register accessor: Host Control Register (HOST_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctl`]
module"]
#[doc(alias = "HOST_CTL")]
pub type HostCtl = crate::Reg<host_ctl::HostCtlSpec>;
#[doc = "Host Control Register (HOST_CTL)"]
pub mod host_ctl;
#[doc = "SRID_CR (rw) register accessor: SRID Core Access Register (SRID_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`srid_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srid_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srid_cr`]
module"]
#[doc(alias = "SRID_CR")]
pub type SridCr = crate::Reg<srid_cr::SridCrSpec>;
#[doc = "SRID Core Access Register (SRID_CR)"]
pub mod srid_cr;
#[doc = "SID_CR (rw) register accessor: SID Core Access Register (SID_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`sid_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sid_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sid_cr`]
module"]
#[doc(alias = "SID_CR")]
pub type SidCr = crate::Reg<sid_cr::SidCrSpec>;
#[doc = "SID Core Access Register (SID_CR)"]
pub mod sid_cr;
#[doc = "DEVICE_ID_CR (rw) register accessor: DEVICE_ID Core Access Register (DEVICE_ID_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`device_id_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`device_id_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_id_cr`]
module"]
#[doc(alias = "DEVICE_ID_CR")]
pub type DeviceIdCr = crate::Reg<device_id_cr::DeviceIdCrSpec>;
#[doc = "DEVICE_ID Core Access Register (DEVICE_ID_CR)"]
pub mod device_id_cr;
#[doc = "CHPREV_CR (rw) register accessor: Chip Revision Core Access Register (CHPREV_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`chprev_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chprev_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chprev_cr`]
module"]
#[doc(alias = "CHPREV_CR")]
pub type ChprevCr = crate::Reg<chprev_cr::ChprevCrSpec>;
#[doc = "Chip Revision Core Access Register (CHPREV_CR)"]
pub mod chprev_cr;
