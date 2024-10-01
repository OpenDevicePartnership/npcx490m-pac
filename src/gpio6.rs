#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    px_dout: PxDout,
    px_din: PxDin,
    px_dir: PxDir,
    px_pull: PxPull,
    px_pud: PxPud,
    px_envdd: PxEnvdd,
    px_otype: PxOtype,
    px_lock_ctl: PxLockCtl,
}
impl RegisterBlock {
    #[doc = "0x00 - Port GPIOx Data Out Register (PxDOUT)"]
    #[inline(always)]
    pub const fn px_dout(&self) -> &PxDout {
        &self.px_dout
    }
    #[doc = "0x01 - Port GPIOx Data In Register (PxDIN)"]
    #[inline(always)]
    pub const fn px_din(&self) -> &PxDin {
        &self.px_din
    }
    #[doc = "0x02 - Port GPIOx Direction Register (PxDIR)"]
    #[inline(always)]
    pub const fn px_dir(&self) -> &PxDir {
        &self.px_dir
    }
    #[doc = "0x03 - Port GPIOx Pull-Up or Pull-Down Enable Register (PxPULL)"]
    #[inline(always)]
    pub const fn px_pull(&self) -> &PxPull {
        &self.px_pull
    }
    #[doc = "0x04 - Port GPIOx Pull-Up/Down Selection Register (PxPUD)"]
    #[inline(always)]
    pub const fn px_pud(&self) -> &PxPud {
        &self.px_pud
    }
    #[doc = "0x05 - Port GPIOx Drive Enable by VDD Present Register (PxENVDD)"]
    #[inline(always)]
    pub const fn px_envdd(&self) -> &PxEnvdd {
        &self.px_envdd
    }
    #[doc = "0x06 - Port GPIOx Output Type Register (PxOTYPE)"]
    #[inline(always)]
    pub const fn px_otype(&self) -> &PxOtype {
        &self.px_otype
    }
    #[doc = "0x07 - Port GPIOx Lock Control Register (PxLOCK_CTL)"]
    #[inline(always)]
    pub const fn px_lock_ctl(&self) -> &PxLockCtl {
        &self.px_lock_ctl
    }
}
#[doc = "PxDOUT (rw) register accessor: Port GPIOx Data Out Register (PxDOUT)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_dout`]
module"]
#[doc(alias = "PxDOUT")]
pub type PxDout = crate::Reg<px_dout::PxDoutSpec>;
#[doc = "Port GPIOx Data Out Register (PxDOUT)"]
pub mod px_dout;
#[doc = "PxDIN (rw) register accessor: Port GPIOx Data In Register (PxDIN)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_din::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_din::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_din`]
module"]
#[doc(alias = "PxDIN")]
pub type PxDin = crate::Reg<px_din::PxDinSpec>;
#[doc = "Port GPIOx Data In Register (PxDIN)"]
pub mod px_din;
#[doc = "PxDIR (rw) register accessor: Port GPIOx Direction Register (PxDIR)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_dir`]
module"]
#[doc(alias = "PxDIR")]
pub type PxDir = crate::Reg<px_dir::PxDirSpec>;
#[doc = "Port GPIOx Direction Register (PxDIR)"]
pub mod px_dir;
#[doc = "PxPULL (rw) register accessor: Port GPIOx Pull-Up or Pull-Down Enable Register (PxPULL)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_pull::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_pull::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_pull`]
module"]
#[doc(alias = "PxPULL")]
pub type PxPull = crate::Reg<px_pull::PxPullSpec>;
#[doc = "Port GPIOx Pull-Up or Pull-Down Enable Register (PxPULL)"]
pub mod px_pull;
#[doc = "PxPUD (rw) register accessor: Port GPIOx Pull-Up/Down Selection Register (PxPUD)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_pud::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_pud::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_pud`]
module"]
#[doc(alias = "PxPUD")]
pub type PxPud = crate::Reg<px_pud::PxPudSpec>;
#[doc = "Port GPIOx Pull-Up/Down Selection Register (PxPUD)"]
pub mod px_pud;
#[doc = "PxENVDD (rw) register accessor: Port GPIOx Drive Enable by VDD Present Register (PxENVDD)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_envdd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_envdd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_envdd`]
module"]
#[doc(alias = "PxENVDD")]
pub type PxEnvdd = crate::Reg<px_envdd::PxEnvddSpec>;
#[doc = "Port GPIOx Drive Enable by VDD Present Register (PxENVDD)"]
pub mod px_envdd;
#[doc = "PxOTYPE (rw) register accessor: Port GPIOx Output Type Register (PxOTYPE)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_otype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_otype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_otype`]
module"]
#[doc(alias = "PxOTYPE")]
pub type PxOtype = crate::Reg<px_otype::PxOtypeSpec>;
#[doc = "Port GPIOx Output Type Register (PxOTYPE)"]
pub mod px_otype;
#[doc = "PxLOCK_CTL (rw) register accessor: Port GPIOx Lock Control Register (PxLOCK_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_lock_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_lock_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_lock_ctl`]
module"]
#[doc(alias = "PxLOCK_CTL")]
pub type PxLockCtl = crate::Reg<px_lock_ctl::PxLockCtlSpec>;
#[doc = "Port GPIOx Lock Control Register (PxLOCK_CTL)"]
pub mod px_lock_ctl;
