#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    spip_data: SpipData,
    spip_ctl1: SpipCtl1,
    spip_stat: SpipStat,
}
impl RegisterBlock {
    #[doc = "0x00 - SPIP Data In Out Register"]
    #[inline(always)]
    pub const fn spip_data(&self) -> &SpipData {
        &self.spip_data
    }
    #[doc = "0x02 - SPIP Control 1 Register (SPIP_CTL1)"]
    #[inline(always)]
    pub const fn spip_ctl1(&self) -> &SpipCtl1 {
        &self.spip_ctl1
    }
    #[doc = "0x04 - SPIP Status Register (SPIP_STAT)"]
    #[inline(always)]
    pub const fn spip_stat(&self) -> &SpipStat {
        &self.spip_stat
    }
}
#[doc = "SPIP_DATA (rw) register accessor: SPIP Data In Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spip_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spip_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spip_data`]
module"]
#[doc(alias = "SPIP_DATA")]
pub type SpipData = crate::Reg<spip_data::SpipDataSpec>;
#[doc = "SPIP Data In Out Register"]
pub mod spip_data;
#[doc = "SPIP_CTL1 (rw) register accessor: SPIP Control 1 Register (SPIP_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`spip_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spip_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spip_ctl1`]
module"]
#[doc(alias = "SPIP_CTL1")]
pub type SpipCtl1 = crate::Reg<spip_ctl1::SpipCtl1Spec>;
#[doc = "SPIP Control 1 Register (SPIP_CTL1)"]
pub mod spip_ctl1;
#[doc = "SPIP_STAT (rw) register accessor: SPIP Status Register (SPIP_STAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`spip_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spip_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spip_stat`]
module"]
#[doc(alias = "SPIP_STAT")]
pub type SpipStat = crate::Reg<spip_stat::SpipStatSpec>;
#[doc = "SPIP Status Register (SPIP_STAT)"]
pub mod spip_stat;
