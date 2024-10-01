#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    itpre64: Itpre64,
    _reserved1: [u8; 0x02],
    itcts64: Itcts64,
    _reserved2: [u8; 0x03],
    itcnt64l: Itcnt64l,
    itcnt64h: Itcnt64h,
}
impl RegisterBlock {
    #[doc = "0x01 - Internal Timer Prescaler Register (ITPRE64)"]
    #[inline(always)]
    pub const fn itpre64(&self) -> &Itpre64 {
        &self.itpre64
    }
    #[doc = "0x04 - Internal Timer Control and Status Register (ITCTS64)"]
    #[inline(always)]
    pub const fn itcts64(&self) -> &Itcts64 {
        &self.itcts64
    }
    #[doc = "0x08 - Internal 64-Bit Timer Counter Register, Low DWord (ITCNT64L)"]
    #[inline(always)]
    pub const fn itcnt64l(&self) -> &Itcnt64l {
        &self.itcnt64l
    }
    #[doc = "0x0c - Internal 64-Bit Timer Counter Register, High DWord (ITCNT64H)"]
    #[inline(always)]
    pub const fn itcnt64h(&self) -> &Itcnt64h {
        &self.itcnt64h
    }
}
#[doc = "ITPRE64 (rw) register accessor: Internal Timer Prescaler Register (ITPRE64)\n\nYou can [`read`](crate::Reg::read) this register and get [`itpre64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itpre64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itpre64`]
module"]
#[doc(alias = "ITPRE64")]
pub type Itpre64 = crate::Reg<itpre64::Itpre64Spec>;
#[doc = "Internal Timer Prescaler Register (ITPRE64)"]
pub mod itpre64;
#[doc = "ITCTS64 (rw) register accessor: Internal Timer Control and Status Register (ITCTS64)\n\nYou can [`read`](crate::Reg::read) this register and get [`itcts64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcts64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itcts64`]
module"]
#[doc(alias = "ITCTS64")]
pub type Itcts64 = crate::Reg<itcts64::Itcts64Spec>;
#[doc = "Internal Timer Control and Status Register (ITCTS64)"]
pub mod itcts64;
#[doc = "ITCNT64L (rw) register accessor: Internal 64-Bit Timer Counter Register, Low DWord (ITCNT64L)\n\nYou can [`read`](crate::Reg::read) this register and get [`itcnt64l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcnt64l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itcnt64l`]
module"]
#[doc(alias = "ITCNT64L")]
pub type Itcnt64l = crate::Reg<itcnt64l::Itcnt64lSpec>;
#[doc = "Internal 64-Bit Timer Counter Register, Low DWord (ITCNT64L)"]
pub mod itcnt64l;
#[doc = "ITCNT64H (rw) register accessor: Internal 64-Bit Timer Counter Register, High DWord (ITCNT64H)\n\nYou can [`read`](crate::Reg::read) this register and get [`itcnt64h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcnt64h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itcnt64h`]
module"]
#[doc(alias = "ITCNT64H")]
pub type Itcnt64h = crate::Reg<itcnt64h::Itcnt64hSpec>;
#[doc = "Internal 64-Bit Timer Counter Register, High DWord (ITCNT64H)"]
pub mod itcnt64h;
