#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    psdat: Psdat,
    _reserved1: [u8; 0x01],
    pstat: Pstat,
    _reserved2: [u8; 0x01],
    pscon: Pscon,
    _reserved3: [u8; 0x01],
    psosig: Psosig,
    _reserved4: [u8; 0x01],
    psisig: Psisig,
    _reserved5: [u8; 0x01],
    psien: Psien,
}
impl RegisterBlock {
    #[doc = "0x00 - PS/2 Data Register (PSDAT)"]
    #[inline(always)]
    pub const fn psdat(&self) -> &Psdat {
        &self.psdat
    }
    #[doc = "0x02 - PS/2 Status Register (PSTAT)"]
    #[inline(always)]
    pub const fn pstat(&self) -> &Pstat {
        &self.pstat
    }
    #[doc = "0x04 - PS/2 Control Register (PSCON)"]
    #[inline(always)]
    pub const fn pscon(&self) -> &Pscon {
        &self.pscon
    }
    #[doc = "0x06 - PS/2 Output Signal Register (PSOSIG)"]
    #[inline(always)]
    pub const fn psosig(&self) -> &Psosig {
        &self.psosig
    }
    #[doc = "0x08 - PS/2 Input Signal Register (PSISIG)"]
    #[inline(always)]
    pub const fn psisig(&self) -> &Psisig {
        &self.psisig
    }
    #[doc = "0x0a - PS/2 Interrupt Enable Register (PSIEN)"]
    #[inline(always)]
    pub const fn psien(&self) -> &Psien {
        &self.psien
    }
}
#[doc = "PSDAT (rw) register accessor: PS/2 Data Register (PSDAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`psdat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psdat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psdat`]
module"]
#[doc(alias = "PSDAT")]
pub type Psdat = crate::Reg<psdat::PsdatSpec>;
#[doc = "PS/2 Data Register (PSDAT)"]
pub mod psdat;
#[doc = "PSTAT (rw) register accessor: PS/2 Status Register (PSTAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`pstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstat`]
module"]
#[doc(alias = "PSTAT")]
pub type Pstat = crate::Reg<pstat::PstatSpec>;
#[doc = "PS/2 Status Register (PSTAT)"]
pub mod pstat;
#[doc = "PSCON (rw) register accessor: PS/2 Control Register (PSCON)\n\nYou can [`read`](crate::Reg::read) this register and get [`pscon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscon`]
module"]
#[doc(alias = "PSCON")]
pub type Pscon = crate::Reg<pscon::PsconSpec>;
#[doc = "PS/2 Control Register (PSCON)"]
pub mod pscon;
#[doc = "PSOSIG (rw) register accessor: PS/2 Output Signal Register (PSOSIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`psosig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psosig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psosig`]
module"]
#[doc(alias = "PSOSIG")]
pub type Psosig = crate::Reg<psosig::PsosigSpec>;
#[doc = "PS/2 Output Signal Register (PSOSIG)"]
pub mod psosig;
#[doc = "PSISIG (rw) register accessor: PS/2 Input Signal Register (PSISIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`psisig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psisig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psisig`]
module"]
#[doc(alias = "PSISIG")]
pub type Psisig = crate::Reg<psisig::PsisigSpec>;
#[doc = "PS/2 Input Signal Register (PSISIG)"]
pub mod psisig;
#[doc = "PSIEN (rw) register accessor: PS/2 Interrupt Enable Register (PSIEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`psien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psien`]
module"]
#[doc(alias = "PSIEN")]
pub type Psien = crate::Reg<psien::PsienSpec>;
#[doc = "PS/2 Interrupt Enable Register (PSIEN)"]
pub mod psien;
