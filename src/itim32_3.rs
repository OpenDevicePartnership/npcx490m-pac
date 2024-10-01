#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    itpre32n: Itpre32n,
    _reserved1: [u8; 0x02],
    itcts32n: Itcts32n,
    _reserved2: [u8; 0x03],
    itcnt32n: Itcnt32n,
}
impl RegisterBlock {
    #[doc = "0x01 - Internal Timer Prescaler Register (ITPRE32n)"]
    #[inline(always)]
    pub const fn itpre32n(&self) -> &Itpre32n {
        &self.itpre32n
    }
    #[doc = "0x04 - Internal Timer Control and Status Register (ITCTS32n)"]
    #[inline(always)]
    pub const fn itcts32n(&self) -> &Itcts32n {
        &self.itcts32n
    }
    #[doc = "0x08 - Internal 32-Bit Timer Counter Register (ITCNT32n)"]
    #[inline(always)]
    pub const fn itcnt32n(&self) -> &Itcnt32n {
        &self.itcnt32n
    }
}
#[doc = "ITPRE32n (rw) register accessor: Internal Timer Prescaler Register (ITPRE32n)\n\nYou can [`read`](crate::Reg::read) this register and get [`itpre32n::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itpre32n::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itpre32n`]
module"]
#[doc(alias = "ITPRE32n")]
pub type Itpre32n = crate::Reg<itpre32n::Itpre32nSpec>;
#[doc = "Internal Timer Prescaler Register (ITPRE32n)"]
pub mod itpre32n;
#[doc = "ITCTS32n (rw) register accessor: Internal Timer Control and Status Register (ITCTS32n)\n\nYou can [`read`](crate::Reg::read) this register and get [`itcts32n::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcts32n::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itcts32n`]
module"]
#[doc(alias = "ITCTS32n")]
pub type Itcts32n = crate::Reg<itcts32n::Itcts32nSpec>;
#[doc = "Internal Timer Control and Status Register (ITCTS32n)"]
pub mod itcts32n;
#[doc = "ITCNT32n (rw) register accessor: Internal 32-Bit Timer Counter Register (ITCNT32n)\n\nYou can [`read`](crate::Reg::read) this register and get [`itcnt32n::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcnt32n::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itcnt32n`]
module"]
#[doc(alias = "ITCNT32n")]
pub type Itcnt32n = crate::Reg<itcnt32n::Itcnt32nSpec>;
#[doc = "Internal 32-Bit Timer Counter Register (ITCNT32n)"]
pub mod itcnt32n;
