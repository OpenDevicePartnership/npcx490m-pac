#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x74],
    dbgctrl: Dbgctrl,
    dbgctrl2: Dbgctrl2,
    dbgfrzen1: Dbgfrzen1,
    dbgfrzen2: Dbgfrzen2,
    dbgfrzen3: Dbgfrzen3,
    dbgfrzen4: Dbgfrzen4,
    dbgfrzen5: Dbgfrzen5,
}
impl RegisterBlock {
    #[doc = "0x74 - Debug Control Register (DBGCTRL)"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x75 - Debug Control 2 Register (DBGCTRL2)"]
    #[inline(always)]
    pub const fn dbgctrl2(&self) -> &Dbgctrl2 {
        &self.dbgctrl2
    }
    #[doc = "0x76 - Debug Freeze Enable 1 Register (DBGFRZEN1)"]
    #[inline(always)]
    pub const fn dbgfrzen1(&self) -> &Dbgfrzen1 {
        &self.dbgfrzen1
    }
    #[doc = "0x77 - Debug Freeze Enable 2 Register (DBGFRZEN2)"]
    #[inline(always)]
    pub const fn dbgfrzen2(&self) -> &Dbgfrzen2 {
        &self.dbgfrzen2
    }
    #[doc = "0x78 - Debug Freeze Enable 3 Register (DBGFRZEN3)"]
    #[inline(always)]
    pub const fn dbgfrzen3(&self) -> &Dbgfrzen3 {
        &self.dbgfrzen3
    }
    #[doc = "0x79 - Debug Freeze Enable 4 Register (DBGFRZEN4)"]
    #[inline(always)]
    pub const fn dbgfrzen4(&self) -> &Dbgfrzen4 {
        &self.dbgfrzen4
    }
    #[doc = "0x7a - Debug Freeze Enable 5 Register (DBGFRZEN5)"]
    #[inline(always)]
    pub const fn dbgfrzen5(&self) -> &Dbgfrzen5 {
        &self.dbgfrzen5
    }
}
#[doc = "DBGCTRL (rw) register accessor: Debug Control Register (DBGCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control Register (DBGCTRL)"]
pub mod dbgctrl;
#[doc = "DBGCTRL2 (rw) register accessor: Debug Control 2 Register (DBGCTRL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl2`]
module"]
#[doc(alias = "DBGCTRL2")]
pub type Dbgctrl2 = crate::Reg<dbgctrl2::Dbgctrl2Spec>;
#[doc = "Debug Control 2 Register (DBGCTRL2)"]
pub mod dbgctrl2;
#[doc = "DBGFRZEN1 (rw) register accessor: Debug Freeze Enable 1 Register (DBGFRZEN1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgfrzen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgfrzen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgfrzen1`]
module"]
#[doc(alias = "DBGFRZEN1")]
pub type Dbgfrzen1 = crate::Reg<dbgfrzen1::Dbgfrzen1Spec>;
#[doc = "Debug Freeze Enable 1 Register (DBGFRZEN1)"]
pub mod dbgfrzen1;
#[doc = "DBGFRZEN2 (rw) register accessor: Debug Freeze Enable 2 Register (DBGFRZEN2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgfrzen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgfrzen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgfrzen2`]
module"]
#[doc(alias = "DBGFRZEN2")]
pub type Dbgfrzen2 = crate::Reg<dbgfrzen2::Dbgfrzen2Spec>;
#[doc = "Debug Freeze Enable 2 Register (DBGFRZEN2)"]
pub mod dbgfrzen2;
#[doc = "DBGFRZEN3 (rw) register accessor: Debug Freeze Enable 3 Register (DBGFRZEN3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgfrzen3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgfrzen3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgfrzen3`]
module"]
#[doc(alias = "DBGFRZEN3")]
pub type Dbgfrzen3 = crate::Reg<dbgfrzen3::Dbgfrzen3Spec>;
#[doc = "Debug Freeze Enable 3 Register (DBGFRZEN3)"]
pub mod dbgfrzen3;
#[doc = "DBGFRZEN4 (rw) register accessor: Debug Freeze Enable 4 Register (DBGFRZEN4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgfrzen4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgfrzen4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgfrzen4`]
module"]
#[doc(alias = "DBGFRZEN4")]
pub type Dbgfrzen4 = crate::Reg<dbgfrzen4::Dbgfrzen4Spec>;
#[doc = "Debug Freeze Enable 4 Register (DBGFRZEN4)"]
pub mod dbgfrzen4;
#[doc = "DBGFRZEN5 (rw) register accessor: Debug Freeze Enable 5 Register (DBGFRZEN5)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgfrzen5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgfrzen5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgfrzen5`]
module"]
#[doc(alias = "DBGFRZEN5")]
pub type Dbgfrzen5 = crate::Reg<dbgfrzen5::Dbgfrzen5Spec>;
#[doc = "Debug Freeze Enable 5 Register (DBGFRZEN5)"]
pub mod dbgfrzen5;
