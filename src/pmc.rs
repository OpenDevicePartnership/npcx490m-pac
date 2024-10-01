#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmcsr: Pmcsr,
    _reserved1: [u8; 0x02],
    enslp_ctl: EnslpCtl,
    disidl_ctl: DisidlCtl,
    disidl_ctl1: DisidlCtl1,
    _reserved4: [u8; 0x02],
    pwdwn_ctl1: PwdwnCtl1,
    pwdwn_ctl2: PwdwnCtl2,
    pwdwn_ctl3: PwdwnCtl3,
    pwdwn_ctl4: PwdwnCtl4,
    pwdwn_ctl5: PwdwnCtl5,
    pwdwn_ctl6: PwdwnCtl6,
    _reserved10: [u8; 0x12],
    ram_pd1: RamPd1,
    _reserved11: [u8; 0x01],
    ram_pd3: RamPd3,
    _reserved12: [u8; 0x01],
    pwdwn_ctl7: PwdwnCtl7,
    pwdwn_ctl8: PwdwnCtl8,
    pwdwn_ctl9: PwdwnCtl9,
}
impl RegisterBlock {
    #[doc = "0x00 - Power Management Controller Status Register (PMCSR)"]
    #[inline(always)]
    pub const fn pmcsr(&self) -> &Pmcsr {
        &self.pmcsr
    }
    #[doc = "0x03 - Enable in Sleep Control Register (ENSLP_CTL)"]
    #[inline(always)]
    pub const fn enslp_ctl(&self) -> &EnslpCtl {
        &self.enslp_ctl
    }
    #[doc = "0x04 - Disable in Idle Control Register (DISIDL_CTL)"]
    #[inline(always)]
    pub const fn disidl_ctl(&self) -> &DisidlCtl {
        &self.disidl_ctl
    }
    #[doc = "0x05 - Disable in Idle Control 1 Register (DISIDL_CTL1)"]
    #[inline(always)]
    pub const fn disidl_ctl1(&self) -> &DisidlCtl1 {
        &self.disidl_ctl1
    }
    #[doc = "0x08 - Power-Down Control 1 Register (PWDWN_CTL1)"]
    #[inline(always)]
    pub const fn pwdwn_ctl1(&self) -> &PwdwnCtl1 {
        &self.pwdwn_ctl1
    }
    #[doc = "0x09 - Power-Down Control 2 Register (PWDWN_CTL2)"]
    #[inline(always)]
    pub const fn pwdwn_ctl2(&self) -> &PwdwnCtl2 {
        &self.pwdwn_ctl2
    }
    #[doc = "0x0a - Power-Down Control 3 Register (PWDWN_CTL3)"]
    #[inline(always)]
    pub const fn pwdwn_ctl3(&self) -> &PwdwnCtl3 {
        &self.pwdwn_ctl3
    }
    #[doc = "0x0b - Power-Down Control 4 Register (PWDWN_CTL4)"]
    #[inline(always)]
    pub const fn pwdwn_ctl4(&self) -> &PwdwnCtl4 {
        &self.pwdwn_ctl4
    }
    #[doc = "0x0c - Power-Down Control 5 Register (PWDWN_CTL5)"]
    #[inline(always)]
    pub const fn pwdwn_ctl5(&self) -> &PwdwnCtl5 {
        &self.pwdwn_ctl5
    }
    #[doc = "0x0d - Power-Down Control 6 Register (PWDWN_CTL6)"]
    #[inline(always)]
    pub const fn pwdwn_ctl6(&self) -> &PwdwnCtl6 {
        &self.pwdwn_ctl6
    }
    #[doc = "0x20 - RAM Power-Down Control 1 Register (RAM_PD1)"]
    #[inline(always)]
    pub const fn ram_pd1(&self) -> &RamPd1 {
        &self.ram_pd1
    }
    #[doc = "0x22 - RAM Power-Down Control 3 Register (RAM_PD3)"]
    #[inline(always)]
    pub const fn ram_pd3(&self) -> &RamPd3 {
        &self.ram_pd3
    }
    #[doc = "0x24 - Power-Down Control 7 Register (PWDWN_CTL7)"]
    #[inline(always)]
    pub const fn pwdwn_ctl7(&self) -> &PwdwnCtl7 {
        &self.pwdwn_ctl7
    }
    #[doc = "0x25 - Power-Down Control 8 Register (PWDWN_CTL8)"]
    #[inline(always)]
    pub const fn pwdwn_ctl8(&self) -> &PwdwnCtl8 {
        &self.pwdwn_ctl8
    }
    #[doc = "0x26 - Power-Down Control 9 Register (PWDWN_CTL9)"]
    #[inline(always)]
    pub const fn pwdwn_ctl9(&self) -> &PwdwnCtl9 {
        &self.pwdwn_ctl9
    }
}
#[doc = "PMCSR (rw) register accessor: Power Management Controller Status Register (PMCSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`pmcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmcsr`]
module"]
#[doc(alias = "PMCSR")]
pub type Pmcsr = crate::Reg<pmcsr::PmcsrSpec>;
#[doc = "Power Management Controller Status Register (PMCSR)"]
pub mod pmcsr;
#[doc = "ENSLP_CTL (rw) register accessor: Enable in Sleep Control Register (ENSLP_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`enslp_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enslp_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enslp_ctl`]
module"]
#[doc(alias = "ENSLP_CTL")]
pub type EnslpCtl = crate::Reg<enslp_ctl::EnslpCtlSpec>;
#[doc = "Enable in Sleep Control Register (ENSLP_CTL)"]
pub mod enslp_ctl;
#[doc = "DISIDL_CTL (rw) register accessor: Disable in Idle Control Register (DISIDL_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`disidl_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disidl_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disidl_ctl`]
module"]
#[doc(alias = "DISIDL_CTL")]
pub type DisidlCtl = crate::Reg<disidl_ctl::DisidlCtlSpec>;
#[doc = "Disable in Idle Control Register (DISIDL_CTL)"]
pub mod disidl_ctl;
#[doc = "DISIDL_CTL1 (rw) register accessor: Disable in Idle Control 1 Register (DISIDL_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`disidl_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disidl_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disidl_ctl1`]
module"]
#[doc(alias = "DISIDL_CTL1")]
pub type DisidlCtl1 = crate::Reg<disidl_ctl1::DisidlCtl1Spec>;
#[doc = "Disable in Idle Control 1 Register (DISIDL_CTL1)"]
pub mod disidl_ctl1;
#[doc = "PWDWN_CTL1 (rw) register accessor: Power-Down Control 1 Register (PWDWN_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdwn_ctl1`]
module"]
#[doc(alias = "PWDWN_CTL1")]
pub type PwdwnCtl1 = crate::Reg<pwdwn_ctl1::PwdwnCtl1Spec>;
#[doc = "Power-Down Control 1 Register (PWDWN_CTL1)"]
pub mod pwdwn_ctl1;
#[doc = "PWDWN_CTL2 (rw) register accessor: Power-Down Control 2 Register (PWDWN_CTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdwn_ctl2`]
module"]
#[doc(alias = "PWDWN_CTL2")]
pub type PwdwnCtl2 = crate::Reg<pwdwn_ctl2::PwdwnCtl2Spec>;
#[doc = "Power-Down Control 2 Register (PWDWN_CTL2)"]
pub mod pwdwn_ctl2;
#[doc = "PWDWN_CTL3 (rw) register accessor: Power-Down Control 3 Register (PWDWN_CTL3)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdwn_ctl3`]
module"]
#[doc(alias = "PWDWN_CTL3")]
pub type PwdwnCtl3 = crate::Reg<pwdwn_ctl3::PwdwnCtl3Spec>;
#[doc = "Power-Down Control 3 Register (PWDWN_CTL3)"]
pub mod pwdwn_ctl3;
#[doc = "PWDWN_CTL4 (rw) register accessor: Power-Down Control 4 Register (PWDWN_CTL4)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdwn_ctl4`]
module"]
#[doc(alias = "PWDWN_CTL4")]
pub type PwdwnCtl4 = crate::Reg<pwdwn_ctl4::PwdwnCtl4Spec>;
#[doc = "Power-Down Control 4 Register (PWDWN_CTL4)"]
pub mod pwdwn_ctl4;
#[doc = "PWDWN_CTL5 (rw) register accessor: Power-Down Control 5 Register (PWDWN_CTL5)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdwn_ctl5`]
module"]
#[doc(alias = "PWDWN_CTL5")]
pub type PwdwnCtl5 = crate::Reg<pwdwn_ctl5::PwdwnCtl5Spec>;
#[doc = "Power-Down Control 5 Register (PWDWN_CTL5)"]
pub mod pwdwn_ctl5;
#[doc = "PWDWN_CTL6 (rw) register accessor: Power-Down Control 6 Register (PWDWN_CTL6)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdwn_ctl6`]
module"]
#[doc(alias = "PWDWN_CTL6")]
pub type PwdwnCtl6 = crate::Reg<pwdwn_ctl6::PwdwnCtl6Spec>;
#[doc = "Power-Down Control 6 Register (PWDWN_CTL6)"]
pub mod pwdwn_ctl6;
#[doc = "RAM_PD1 (rw) register accessor: RAM Power-Down Control 1 Register (RAM_PD1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_pd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_pd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_pd1`]
module"]
#[doc(alias = "RAM_PD1")]
pub type RamPd1 = crate::Reg<ram_pd1::RamPd1Spec>;
#[doc = "RAM Power-Down Control 1 Register (RAM_PD1)"]
pub mod ram_pd1;
#[doc = "RAM_PD3 (rw) register accessor: RAM Power-Down Control 3 Register (RAM_PD3)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_pd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_pd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_pd3`]
module"]
#[doc(alias = "RAM_PD3")]
pub type RamPd3 = crate::Reg<ram_pd3::RamPd3Spec>;
#[doc = "RAM Power-Down Control 3 Register (RAM_PD3)"]
pub mod ram_pd3;
#[doc = "PWDWN_CTL7 (rw) register accessor: Power-Down Control 7 Register (PWDWN_CTL7)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdwn_ctl7`]
module"]
#[doc(alias = "PWDWN_CTL7")]
pub type PwdwnCtl7 = crate::Reg<pwdwn_ctl7::PwdwnCtl7Spec>;
#[doc = "Power-Down Control 7 Register (PWDWN_CTL7)"]
pub mod pwdwn_ctl7;
#[doc = "PWDWN_CTL8 (rw) register accessor: Power-Down Control 8 Register (PWDWN_CTL8)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdwn_ctl8`]
module"]
#[doc(alias = "PWDWN_CTL8")]
pub type PwdwnCtl8 = crate::Reg<pwdwn_ctl8::PwdwnCtl8Spec>;
#[doc = "Power-Down Control 8 Register (PWDWN_CTL8)"]
pub mod pwdwn_ctl8;
#[doc = "PWDWN_CTL9 (rw) register accessor: Power-Down Control 9 Register (PWDWN_CTL9)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdwn_ctl9`]
module"]
#[doc(alias = "PWDWN_CTL9")]
pub type PwdwnCtl9 = crate::Reg<pwdwn_ctl9::PwdwnCtl9Spec>;
#[doc = "Power-Down Control 9 Register (PWDWN_CTL9)"]
pub mod pwdwn_ctl9;
