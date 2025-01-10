#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    devcnt: Devcnt,
    strpst: Strpst,
    rstctl: Rstctl,
    _reserved3: [u8; 0x01],
    dev_ctl3: DevCtl3,
    _reserved4: [u8; 0x01],
    dev_ctl4: DevCtl4,
    _reserved5: [u8; 0x03],
    spsts: Spsts,
    _reserved6: [u8; 0x05],
    devalt0: Devalt0,
    devalt1: Devalt1,
    devalt2: Devalt2,
    devalt3: Devalt3,
    devalt4: Devalt4,
    devalt5: Devalt5,
    devalt6: Devalt6,
    devalt7: Devalt7,
    devalt8: Devalt8,
    devalt9: Devalt9,
    devalta: Devalta,
    devaltb: Devaltb,
    devaltc: Devaltc,
    devaltd: Devaltd,
    devalte: Devalte,
    devaltf: Devaltf,
    devaltg: Devaltg,
    devalth: Devalth,
    devaltj: Devaltj,
    devaltk: Devaltk,
    devaltl: Devaltl,
    devaltm: Devaltm,
    devaltn: Devaltn,
    _reserved29: [u8; 0x04],
    pupd_en0: PupdEn0,
    pupd_en1: PupdEn1,
    _reserved31: [u8; 0x01],
    bl_ctl: BlCtl,
    _reserved32: [u8; 0xd1],
    swrst_trg: SwrstTrg,
    _reserved33: [u8; 0x02],
    swrst_ctl1: SwrstCtl1,
    swrst_ctl2: SwrstCtl2,
    swrst_ctl3: SwrstCtl3,
    swrst_ctl4: SwrstCtl4,
    _reserved37: [u8; 0x0c],
    jen_ctl1: JenCtl1,
    jen_ctl2: JenCtl2,
    gp_ctl: GpCtl,
    _reserved40: [u8; 0x2d],
    lv_gpio_ctl0: LvGpioCtl0,
    lv_gpio_ctl1: LvGpioCtl1,
    lv_gpio_ctl2: LvGpioCtl2,
    lv_gpio_ctl3: LvGpioCtl3,
    lv_gpio_ctl4: LvGpioCtl4,
    lv_gpio_ctl5: LvGpioCtl5,
    lv_gpio_ctl6: LvGpioCtl6,
    lv_gpio_ctl7: LvGpioCtl7,
    lv_gpio_ctl8: LvGpioCtl8,
    lv_gpio_ctl9: LvGpioCtl9,
    lv_gpio_ctla: LvGpioCtla,
    lv_gpio_ctlb: LvGpioCtlb,
    _reserved52: [u8; 0x04],
    lv_gpio_ctlp: LvGpioCtlp,
    _reserved53: [u8; 0xaf],
    devalt0_lk: Devalt0Lk,
    _reserved54: [u8; 0x01],
    devalt2_lk: Devalt2Lk,
    devalt3_lk: Devalt3Lk,
    devalt4_lk: Devalt4Lk,
    devalt5_lk: Devalt5Lk,
    devalt6_lk: Devalt6Lk,
    _reserved59: [u8; 0x04],
    devaltb_lk: DevaltbLk,
    _reserved60: [u8; 0x01],
    devaltd_lk: DevaltdLk,
    _reserved61: [u8; 0x01],
    devaltf_lk: DevaltfLk,
    devaltg_lk: DevaltgLk,
    devalth_lk: DevalthLk,
    devaltj_lk: DevaltjLk,
    devaltk_lk: DevaltkLk,
    _reserved66: [u8; 0x02],
    devaltn_lk: DevaltnLk,
    _reserved67: [u8; 0xdd],
    swrst_ctl1_lk: SwrstCtl1Lk,
    swrst_ctl2_lk: SwrstCtl2Lk,
    swrst_ctl3_lk: SwrstCtl3Lk,
    swrst_ctl4_lk: SwrstCtl4Lk,
}
impl RegisterBlock {
    #[doc = "0x00 - Device Control Register (DEVCNT)"]
    #[inline(always)]
    pub const fn devcnt(&self) -> &Devcnt {
        &self.devcnt
    }
    #[doc = "0x01 - Straps Status Register (STRPST)"]
    #[inline(always)]
    pub const fn strpst(&self) -> &Strpst {
        &self.strpst
    }
    #[doc = "0x02 - Reset Control and Status Register (RSTCTL)"]
    #[inline(always)]
    pub const fn rstctl(&self) -> &Rstctl {
        &self.rstctl
    }
    #[doc = "0x04 - Device Control 3 Register (DEV_CTL3)"]
    #[inline(always)]
    pub const fn dev_ctl3(&self) -> &DevCtl3 {
        &self.dev_ctl3
    }
    #[doc = "0x06 - Device Control 4 Register (DEV_CTL4)"]
    #[inline(always)]
    pub const fn dev_ctl4(&self) -> &DevCtl4 {
        &self.dev_ctl4
    }
    #[doc = "0x0a - Supply Status Register (SPSTS)"]
    #[inline(always)]
    pub const fn spsts(&self) -> &Spsts {
        &self.spsts
    }
    #[doc = "0x10 - Device Alternate Function 0 Register (DEVALT0)"]
    #[inline(always)]
    pub const fn devalt0(&self) -> &Devalt0 {
        &self.devalt0
    }
    #[doc = "0x11 - Device Alternate Function 1 Register (DEVALT1)"]
    #[inline(always)]
    pub const fn devalt1(&self) -> &Devalt1 {
        &self.devalt1
    }
    #[doc = "0x12 - Device Alternate Function 2 Register (DEVALT2)"]
    #[inline(always)]
    pub const fn devalt2(&self) -> &Devalt2 {
        &self.devalt2
    }
    #[doc = "0x13 - Device Alternate Function 3 Register (DEVALT3)"]
    #[inline(always)]
    pub const fn devalt3(&self) -> &Devalt3 {
        &self.devalt3
    }
    #[doc = "0x14 - Device Alternate Function 4 Register (DEVALT4)"]
    #[inline(always)]
    pub const fn devalt4(&self) -> &Devalt4 {
        &self.devalt4
    }
    #[doc = "0x15 - Device Alternate Function 5 Register (DEVALT5)"]
    #[inline(always)]
    pub const fn devalt5(&self) -> &Devalt5 {
        &self.devalt5
    }
    #[doc = "0x16 - Device Alternate Function 6 Register (DEVALT6)"]
    #[inline(always)]
    pub const fn devalt6(&self) -> &Devalt6 {
        &self.devalt6
    }
    #[doc = "0x17 - Device Alternate Function 7 Register (DEVALT7)"]
    #[inline(always)]
    pub const fn devalt7(&self) -> &Devalt7 {
        &self.devalt7
    }
    #[doc = "0x18 - Device Alternate Function 8 Register (DEVALT8)"]
    #[inline(always)]
    pub const fn devalt8(&self) -> &Devalt8 {
        &self.devalt8
    }
    #[doc = "0x19 - Device Alternate Function 9 Register (DEVALT9)"]
    #[inline(always)]
    pub const fn devalt9(&self) -> &Devalt9 {
        &self.devalt9
    }
    #[doc = "0x1a - Device Alternate Function A Register (DEVALTA)"]
    #[inline(always)]
    pub const fn devalta(&self) -> &Devalta {
        &self.devalta
    }
    #[doc = "0x1b - Device Alternate Function B Register (DEVALTB)"]
    #[inline(always)]
    pub const fn devaltb(&self) -> &Devaltb {
        &self.devaltb
    }
    #[doc = "0x1c - Device Alternate Function C Register (DEVALTC)"]
    #[inline(always)]
    pub const fn devaltc(&self) -> &Devaltc {
        &self.devaltc
    }
    #[doc = "0x1d - Device Alternate Function D Register (DEVALTD)"]
    #[inline(always)]
    pub const fn devaltd(&self) -> &Devaltd {
        &self.devaltd
    }
    #[doc = "0x1e - Device Alternate Function E Register (DEVALTE)"]
    #[inline(always)]
    pub const fn devalte(&self) -> &Devalte {
        &self.devalte
    }
    #[doc = "0x1f - Device Alternate Function F Register (DEVALTF)"]
    #[inline(always)]
    pub const fn devaltf(&self) -> &Devaltf {
        &self.devaltf
    }
    #[doc = "0x20 - Device Alternate Function G Register (DEVALTG)"]
    #[inline(always)]
    pub const fn devaltg(&self) -> &Devaltg {
        &self.devaltg
    }
    #[doc = "0x21 - Device Alternate Function H Register (DEVALTH)"]
    #[inline(always)]
    pub const fn devalth(&self) -> &Devalth {
        &self.devalth
    }
    #[doc = "0x22 - Device Alternate Function J Register (DEVALTJ)"]
    #[inline(always)]
    pub const fn devaltj(&self) -> &Devaltj {
        &self.devaltj
    }
    #[doc = "0x23 - Device Alternate Function K Register (DEVALTK)"]
    #[inline(always)]
    pub const fn devaltk(&self) -> &Devaltk {
        &self.devaltk
    }
    #[doc = "0x24 - Device Alternate Function L Register (DEVALTL)"]
    #[inline(always)]
    pub const fn devaltl(&self) -> &Devaltl {
        &self.devaltl
    }
    #[doc = "0x25 - Device Alternate Function M Register (DEVALTM)"]
    #[inline(always)]
    pub const fn devaltm(&self) -> &Devaltm {
        &self.devaltm
    }
    #[doc = "0x26 - Device Alternate Function N Register (DEVALTN)"]
    #[inline(always)]
    pub const fn devaltn(&self) -> &Devaltn {
        &self.devaltn
    }
    #[doc = "0x2b - Pull-Up/Pull-Down Enable 0 Register (PUPD_EN0)"]
    #[inline(always)]
    pub const fn pupd_en0(&self) -> &PupdEn0 {
        &self.pupd_en0
    }
    #[doc = "0x2c - Pull-Up/Pull-Down Enable 1 Register (PUPD_EN1)"]
    #[inline(always)]
    pub const fn pupd_en1(&self) -> &PupdEn1 {
        &self.pupd_en1
    }
    #[doc = "0x2e - BootLoader Control Register (BL_CTL)"]
    #[inline(always)]
    pub const fn bl_ctl(&self) -> &BlCtl {
        &self.bl_ctl
    }
    #[doc = "0x100 - Software Reset Trigger Register (SWRST_TRG)"]
    #[inline(always)]
    pub const fn swrst_trg(&self) -> &SwrstTrg {
        &self.swrst_trg
    }
    #[doc = "0x104 - Software Reset Control 1 Register (SWRST_CTL1)"]
    #[inline(always)]
    pub const fn swrst_ctl1(&self) -> &SwrstCtl1 {
        &self.swrst_ctl1
    }
    #[doc = "0x108 - Software Reset Control 2 Register (SWRST_CTL2)"]
    #[inline(always)]
    pub const fn swrst_ctl2(&self) -> &SwrstCtl2 {
        &self.swrst_ctl2
    }
    #[doc = "0x10c - Software Reset Control 3 Register (SWRST_CTL3)"]
    #[inline(always)]
    pub const fn swrst_ctl3(&self) -> &SwrstCtl3 {
        &self.swrst_ctl3
    }
    #[doc = "0x110 - Software Reset Control 4 Register (SWRST_CTL4)"]
    #[inline(always)]
    pub const fn swrst_ctl4(&self) -> &SwrstCtl4 {
        &self.swrst_ctl4
    }
    #[doc = "0x120 - JTAG Enable Control 1 Register (JEN_CTL1)"]
    #[inline(always)]
    pub const fn jen_ctl1(&self) -> &JenCtl1 {
        &self.jen_ctl1
    }
    #[doc = "0x121 - JTAG Enable Control 2 Register (JEN_CTL2)"]
    #[inline(always)]
    pub const fn jen_ctl2(&self) -> &JenCtl2 {
        &self.jen_ctl2
    }
    #[doc = "0x122 - Gang Programmer Control Register (GP_CTL)"]
    #[inline(always)]
    pub const fn gp_ctl(&self) -> &GpCtl {
        &self.gp_ctl
    }
    #[doc = "0x150 - Low-Voltage GPIO Pins Control 0 Register (LV_GPIO_CTL0)"]
    #[inline(always)]
    pub const fn lv_gpio_ctl0(&self) -> &LvGpioCtl0 {
        &self.lv_gpio_ctl0
    }
    #[doc = "0x151 - Low-Voltage GPIO Pins Control 1 Register (LV_GPIO_CTL1)"]
    #[inline(always)]
    pub const fn lv_gpio_ctl1(&self) -> &LvGpioCtl1 {
        &self.lv_gpio_ctl1
    }
    #[doc = "0x152 - Low-Voltage GPIO Pins Control 2 Register (LV_GPIO_CTL2)"]
    #[inline(always)]
    pub const fn lv_gpio_ctl2(&self) -> &LvGpioCtl2 {
        &self.lv_gpio_ctl2
    }
    #[doc = "0x153 - Low-Voltage GPIO Pins Control 3 Register (LV_GPIO_CTL3)"]
    #[inline(always)]
    pub const fn lv_gpio_ctl3(&self) -> &LvGpioCtl3 {
        &self.lv_gpio_ctl3
    }
    #[doc = "0x154 - Low-Voltage GPIO Pins Control 4 Register (LV_GPIO_CTL4)"]
    #[inline(always)]
    pub const fn lv_gpio_ctl4(&self) -> &LvGpioCtl4 {
        &self.lv_gpio_ctl4
    }
    #[doc = "0x155 - Low-Voltage GPIO Pins Control 5 Register (LV_GPIO_CTL5)"]
    #[inline(always)]
    pub const fn lv_gpio_ctl5(&self) -> &LvGpioCtl5 {
        &self.lv_gpio_ctl5
    }
    #[doc = "0x156 - Low-Voltage GPIO Pins Control 6 Register (LV_GPIO_CTL6)"]
    #[inline(always)]
    pub const fn lv_gpio_ctl6(&self) -> &LvGpioCtl6 {
        &self.lv_gpio_ctl6
    }
    #[doc = "0x157 - Low-Voltage GPIO Pins Control 7 Register (LV_GPIO_CTL7)"]
    #[inline(always)]
    pub const fn lv_gpio_ctl7(&self) -> &LvGpioCtl7 {
        &self.lv_gpio_ctl7
    }
    #[doc = "0x158 - Low-Voltage GPIO Pins Control 8 Register (LV_GPIO_CTL8)"]
    #[inline(always)]
    pub const fn lv_gpio_ctl8(&self) -> &LvGpioCtl8 {
        &self.lv_gpio_ctl8
    }
    #[doc = "0x159 - Low-Voltage GPIO Pins Control 9 Register (LV_GPIO_CTL9)"]
    #[inline(always)]
    pub const fn lv_gpio_ctl9(&self) -> &LvGpioCtl9 {
        &self.lv_gpio_ctl9
    }
    #[doc = "0x15a - Low-Voltage GPIO Pins Control A Register (LV_GPIO_CTLA)"]
    #[inline(always)]
    pub const fn lv_gpio_ctla(&self) -> &LvGpioCtla {
        &self.lv_gpio_ctla
    }
    #[doc = "0x15b - Low-Voltage GPIO Pins Control B Register (LV_GPIO_CTLB)"]
    #[inline(always)]
    pub const fn lv_gpio_ctlb(&self) -> &LvGpioCtlb {
        &self.lv_gpio_ctlb
    }
    #[doc = "0x160 - Low-Voltage GPIO Pins Control P Register (LV_GPIO_CTLP)"]
    #[inline(always)]
    pub const fn lv_gpio_ctlp(&self) -> &LvGpioCtlp {
        &self.lv_gpio_ctlp
    }
    #[doc = "0x210 - Device Alternate Function 0 Lock Register (DEVALT0_LK)"]
    #[inline(always)]
    pub const fn devalt0_lk(&self) -> &Devalt0Lk {
        &self.devalt0_lk
    }
    #[doc = "0x212 - Device Alternate Function 2 Lock Register (DEVALT2_LK)"]
    #[inline(always)]
    pub const fn devalt2_lk(&self) -> &Devalt2Lk {
        &self.devalt2_lk
    }
    #[doc = "0x213 - Device Alternate Function 3 Lock Register (DEVALT3_LK)"]
    #[inline(always)]
    pub const fn devalt3_lk(&self) -> &Devalt3Lk {
        &self.devalt3_lk
    }
    #[doc = "0x214 - Device Alternate Function 4 Lock Register (DEVALT4_LK)"]
    #[inline(always)]
    pub const fn devalt4_lk(&self) -> &Devalt4Lk {
        &self.devalt4_lk
    }
    #[doc = "0x215 - Device Alternate Function 5 Lock Register (DEVALT5_LK)"]
    #[inline(always)]
    pub const fn devalt5_lk(&self) -> &Devalt5Lk {
        &self.devalt5_lk
    }
    #[doc = "0x216 - Device Alternate Function 6 Lock Register (DEVALT6_LK)"]
    #[inline(always)]
    pub const fn devalt6_lk(&self) -> &Devalt6Lk {
        &self.devalt6_lk
    }
    #[doc = "0x21b - Device Alternate Function B Lock Register (DEVALTB_LK)"]
    #[inline(always)]
    pub const fn devaltb_lk(&self) -> &DevaltbLk {
        &self.devaltb_lk
    }
    #[doc = "0x21d - Device Alternate Function D Lock Register (DEVALTD_LK)"]
    #[inline(always)]
    pub const fn devaltd_lk(&self) -> &DevaltdLk {
        &self.devaltd_lk
    }
    #[doc = "0x21f - Device Alternate Function F Lock Register (DEVALTF_LK)"]
    #[inline(always)]
    pub const fn devaltf_lk(&self) -> &DevaltfLk {
        &self.devaltf_lk
    }
    #[doc = "0x220 - Device Alternate Function G Lock Register (DEVALTG_LK)"]
    #[inline(always)]
    pub const fn devaltg_lk(&self) -> &DevaltgLk {
        &self.devaltg_lk
    }
    #[doc = "0x221 - Alternate Function H Lock Register (DEVALTH_LK)"]
    #[inline(always)]
    pub const fn devalth_lk(&self) -> &DevalthLk {
        &self.devalth_lk
    }
    #[doc = "0x222 - Device Alternate Function J Lock Register (DEVALTJ_LK)"]
    #[inline(always)]
    pub const fn devaltj_lk(&self) -> &DevaltjLk {
        &self.devaltj_lk
    }
    #[doc = "0x223 - Device Alternate Function K Lock Register (DEVALTK_LK)"]
    #[inline(always)]
    pub const fn devaltk_lk(&self) -> &DevaltkLk {
        &self.devaltk_lk
    }
    #[doc = "0x226 - Device Alternate Function N Lock Register (DEVALTN_LK)"]
    #[inline(always)]
    pub const fn devaltn_lk(&self) -> &DevaltnLk {
        &self.devaltn_lk
    }
    #[doc = "0x304 - Software Reset Control 1 Lock Register (SWRST_CTL1_LK)"]
    #[inline(always)]
    pub const fn swrst_ctl1_lk(&self) -> &SwrstCtl1Lk {
        &self.swrst_ctl1_lk
    }
    #[doc = "0x308 - Software Reset Control 2 Lock Register (SWRST_CTL2_LK)"]
    #[inline(always)]
    pub const fn swrst_ctl2_lk(&self) -> &SwrstCtl2Lk {
        &self.swrst_ctl2_lk
    }
    #[doc = "0x30c - Software Reset Control 3 Lock Register (SWRST_CTL3_LK)"]
    #[inline(always)]
    pub const fn swrst_ctl3_lk(&self) -> &SwrstCtl3Lk {
        &self.swrst_ctl3_lk
    }
    #[doc = "0x310 - Software Reset Control 4 Lock Register (SWRST_CTL4_LK)"]
    #[inline(always)]
    pub const fn swrst_ctl4_lk(&self) -> &SwrstCtl4Lk {
        &self.swrst_ctl4_lk
    }
}
#[doc = "DEVCNT (rw) register accessor: Device Control Register (DEVCNT)\n\nYou can [`read`](crate::Reg::read) this register and get [`devcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devcnt`]
module"]
#[doc(alias = "DEVCNT")]
pub type Devcnt = crate::Reg<devcnt::DevcntSpec>;
#[doc = "Device Control Register (DEVCNT)"]
pub mod devcnt;
#[doc = "STRPST (r) register accessor: Straps Status Register (STRPST)\n\nYou can [`read`](crate::Reg::read) this register and get [`strpst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strpst`]
module"]
#[doc(alias = "STRPST")]
pub type Strpst = crate::Reg<strpst::StrpstSpec>;
#[doc = "Straps Status Register (STRPST)"]
pub mod strpst;
#[doc = "RSTCTL (rw) register accessor: Reset Control and Status Register (RSTCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl`]
module"]
#[doc(alias = "RSTCTL")]
pub type Rstctl = crate::Reg<rstctl::RstctlSpec>;
#[doc = "Reset Control and Status Register (RSTCTL)"]
pub mod rstctl;
#[doc = "DEV_CTL3 (rw) register accessor: Device Control 3 Register (DEV_CTL3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_ctl3`]
module"]
#[doc(alias = "DEV_CTL3")]
pub type DevCtl3 = crate::Reg<dev_ctl3::DevCtl3Spec>;
#[doc = "Device Control 3 Register (DEV_CTL3)"]
pub mod dev_ctl3;
#[doc = "DEV_CTL4 (rw) register accessor: Device Control 4 Register (DEV_CTL4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_ctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_ctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_ctl4`]
module"]
#[doc(alias = "DEV_CTL4")]
pub type DevCtl4 = crate::Reg<dev_ctl4::DevCtl4Spec>;
#[doc = "Device Control 4 Register (DEV_CTL4)"]
pub mod dev_ctl4;
#[doc = "SPSTS (rw) register accessor: Supply Status Register (SPSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`spsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spsts`]
module"]
#[doc(alias = "SPSTS")]
pub type Spsts = crate::Reg<spsts::SpstsSpec>;
#[doc = "Supply Status Register (SPSTS)"]
pub mod spsts;
#[doc = "DEVALT0 (rw) register accessor: Device Alternate Function 0 Register (DEVALT0)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt0`]
module"]
#[doc(alias = "DEVALT0")]
pub type Devalt0 = crate::Reg<devalt0::Devalt0Spec>;
#[doc = "Device Alternate Function 0 Register (DEVALT0)"]
pub mod devalt0;
#[doc = "DEVALT1 (rw) register accessor: Device Alternate Function 1 Register (DEVALT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt1`]
module"]
#[doc(alias = "DEVALT1")]
pub type Devalt1 = crate::Reg<devalt1::Devalt1Spec>;
#[doc = "Device Alternate Function 1 Register (DEVALT1)"]
pub mod devalt1;
#[doc = "DEVALT2 (rw) register accessor: Device Alternate Function 2 Register (DEVALT2)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt2`]
module"]
#[doc(alias = "DEVALT2")]
pub type Devalt2 = crate::Reg<devalt2::Devalt2Spec>;
#[doc = "Device Alternate Function 2 Register (DEVALT2)"]
pub mod devalt2;
#[doc = "DEVALT3 (rw) register accessor: Device Alternate Function 3 Register (DEVALT3)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt3`]
module"]
#[doc(alias = "DEVALT3")]
pub type Devalt3 = crate::Reg<devalt3::Devalt3Spec>;
#[doc = "Device Alternate Function 3 Register (DEVALT3)"]
pub mod devalt3;
#[doc = "DEVALT4 (rw) register accessor: Device Alternate Function 4 Register (DEVALT4)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt4`]
module"]
#[doc(alias = "DEVALT4")]
pub type Devalt4 = crate::Reg<devalt4::Devalt4Spec>;
#[doc = "Device Alternate Function 4 Register (DEVALT4)"]
pub mod devalt4;
#[doc = "DEVALT5 (rw) register accessor: Device Alternate Function 5 Register (DEVALT5)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt5`]
module"]
#[doc(alias = "DEVALT5")]
pub type Devalt5 = crate::Reg<devalt5::Devalt5Spec>;
#[doc = "Device Alternate Function 5 Register (DEVALT5)"]
pub mod devalt5;
#[doc = "DEVALT6 (rw) register accessor: Device Alternate Function 6 Register (DEVALT6)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt6`]
module"]
#[doc(alias = "DEVALT6")]
pub type Devalt6 = crate::Reg<devalt6::Devalt6Spec>;
#[doc = "Device Alternate Function 6 Register (DEVALT6)"]
pub mod devalt6;
#[doc = "DEVALT7 (rw) register accessor: Device Alternate Function 7 Register (DEVALT7)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt7`]
module"]
#[doc(alias = "DEVALT7")]
pub type Devalt7 = crate::Reg<devalt7::Devalt7Spec>;
#[doc = "Device Alternate Function 7 Register (DEVALT7)"]
pub mod devalt7;
#[doc = "DEVALT8 (rw) register accessor: Device Alternate Function 8 Register (DEVALT8)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt8`]
module"]
#[doc(alias = "DEVALT8")]
pub type Devalt8 = crate::Reg<devalt8::Devalt8Spec>;
#[doc = "Device Alternate Function 8 Register (DEVALT8)"]
pub mod devalt8;
#[doc = "DEVALT9 (rw) register accessor: Device Alternate Function 9 Register (DEVALT9)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt9`]
module"]
#[doc(alias = "DEVALT9")]
pub type Devalt9 = crate::Reg<devalt9::Devalt9Spec>;
#[doc = "Device Alternate Function 9 Register (DEVALT9)"]
pub mod devalt9;
#[doc = "DEVALTA (rw) register accessor: Device Alternate Function A Register (DEVALTA)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalta`]
module"]
#[doc(alias = "DEVALTA")]
pub type Devalta = crate::Reg<devalta::DevaltaSpec>;
#[doc = "Device Alternate Function A Register (DEVALTA)"]
pub mod devalta;
#[doc = "DEVALTB (rw) register accessor: Device Alternate Function B Register (DEVALTB)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltb`]
module"]
#[doc(alias = "DEVALTB")]
pub type Devaltb = crate::Reg<devaltb::DevaltbSpec>;
#[doc = "Device Alternate Function B Register (DEVALTB)"]
pub mod devaltb;
#[doc = "DEVALTC (rw) register accessor: Device Alternate Function C Register (DEVALTC)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltc`]
module"]
#[doc(alias = "DEVALTC")]
pub type Devaltc = crate::Reg<devaltc::DevaltcSpec>;
#[doc = "Device Alternate Function C Register (DEVALTC)"]
pub mod devaltc;
#[doc = "DEVALTE (rw) register accessor: Device Alternate Function E Register (DEVALTE)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalte`]
module"]
#[doc(alias = "DEVALTE")]
pub type Devalte = crate::Reg<devalte::DevalteSpec>;
#[doc = "Device Alternate Function E Register (DEVALTE)"]
pub mod devalte;
#[doc = "DEVALTF (rw) register accessor: Device Alternate Function F Register (DEVALTF)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltf`]
module"]
#[doc(alias = "DEVALTF")]
pub type Devaltf = crate::Reg<devaltf::DevaltfSpec>;
#[doc = "Device Alternate Function F Register (DEVALTF)"]
pub mod devaltf;
#[doc = "DEVALTH (rw) register accessor: Device Alternate Function H Register (DEVALTH)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalth`]
module"]
#[doc(alias = "DEVALTH")]
pub type Devalth = crate::Reg<devalth::DevalthSpec>;
#[doc = "Device Alternate Function H Register (DEVALTH)"]
pub mod devalth;
#[doc = "DEVALTJ (rw) register accessor: Device Alternate Function J Register (DEVALTJ)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltj`]
module"]
#[doc(alias = "DEVALTJ")]
pub type Devaltj = crate::Reg<devaltj::DevaltjSpec>;
#[doc = "Device Alternate Function J Register (DEVALTJ)"]
pub mod devaltj;
#[doc = "DEVALTK (rw) register accessor: Device Alternate Function K Register (DEVALTK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltk`]
module"]
#[doc(alias = "DEVALTK")]
pub type Devaltk = crate::Reg<devaltk::DevaltkSpec>;
#[doc = "Device Alternate Function K Register (DEVALTK)"]
pub mod devaltk;
#[doc = "DEVALTL (rw) register accessor: Device Alternate Function L Register (DEVALTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltl`]
module"]
#[doc(alias = "DEVALTL")]
pub type Devaltl = crate::Reg<devaltl::DevaltlSpec>;
#[doc = "Device Alternate Function L Register (DEVALTL)"]
pub mod devaltl;
#[doc = "DEVALTM (rw) register accessor: Device Alternate Function M Register (DEVALTM)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltm`]
module"]
#[doc(alias = "DEVALTM")]
pub type Devaltm = crate::Reg<devaltm::DevaltmSpec>;
#[doc = "Device Alternate Function M Register (DEVALTM)"]
pub mod devaltm;
#[doc = "DEVALTN (rw) register accessor: Device Alternate Function N Register (DEVALTN)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltn`]
module"]
#[doc(alias = "DEVALTN")]
pub type Devaltn = crate::Reg<devaltn::DevaltnSpec>;
#[doc = "Device Alternate Function N Register (DEVALTN)"]
pub mod devaltn;
#[doc = "PUPD_EN0 (rw) register accessor: Pull-Up/Pull-Down Enable 0 Register (PUPD_EN0)\n\nYou can [`read`](crate::Reg::read) this register and get [`pupd_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupd_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupd_en0`]
module"]
#[doc(alias = "PUPD_EN0")]
pub type PupdEn0 = crate::Reg<pupd_en0::PupdEn0Spec>;
#[doc = "Pull-Up/Pull-Down Enable 0 Register (PUPD_EN0)"]
pub mod pupd_en0;
#[doc = "PUPD_EN1 (rw) register accessor: Pull-Up/Pull-Down Enable 1 Register (PUPD_EN1)\n\nYou can [`read`](crate::Reg::read) this register and get [`pupd_en1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupd_en1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupd_en1`]
module"]
#[doc(alias = "PUPD_EN1")]
pub type PupdEn1 = crate::Reg<pupd_en1::PupdEn1Spec>;
#[doc = "Pull-Up/Pull-Down Enable 1 Register (PUPD_EN1)"]
pub mod pupd_en1;
#[doc = "BL_CTL (rw) register accessor: BootLoader Control Register (BL_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`bl_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bl_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bl_ctl`]
module"]
#[doc(alias = "BL_CTL")]
pub type BlCtl = crate::Reg<bl_ctl::BlCtlSpec>;
#[doc = "BootLoader Control Register (BL_CTL)"]
pub mod bl_ctl;
#[doc = "SWRST_TRG (rw) register accessor: Software Reset Trigger Register (SWRST_TRG)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_trg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_trg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_trg`]
module"]
#[doc(alias = "SWRST_TRG")]
pub type SwrstTrg = crate::Reg<swrst_trg::SwrstTrgSpec>;
#[doc = "Software Reset Trigger Register (SWRST_TRG)"]
pub mod swrst_trg;
#[doc = "SWRST_CTL1 (rw) register accessor: Software Reset Control 1 Register (SWRST_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_ctl1`]
module"]
#[doc(alias = "SWRST_CTL1")]
pub type SwrstCtl1 = crate::Reg<swrst_ctl1::SwrstCtl1Spec>;
#[doc = "Software Reset Control 1 Register (SWRST_CTL1)"]
pub mod swrst_ctl1;
#[doc = "SWRST_CTL2 (rw) register accessor: Software Reset Control 2 Register (SWRST_CTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_ctl2`]
module"]
#[doc(alias = "SWRST_CTL2")]
pub type SwrstCtl2 = crate::Reg<swrst_ctl2::SwrstCtl2Spec>;
#[doc = "Software Reset Control 2 Register (SWRST_CTL2)"]
pub mod swrst_ctl2;
#[doc = "SWRST_CTL3 (rw) register accessor: Software Reset Control 3 Register (SWRST_CTL3)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_ctl3`]
module"]
#[doc(alias = "SWRST_CTL3")]
pub type SwrstCtl3 = crate::Reg<swrst_ctl3::SwrstCtl3Spec>;
#[doc = "Software Reset Control 3 Register (SWRST_CTL3)"]
pub mod swrst_ctl3;
#[doc = "SWRST_CTL4 (rw) register accessor: Software Reset Control 4 Register (SWRST_CTL4)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_ctl4`]
module"]
#[doc(alias = "SWRST_CTL4")]
pub type SwrstCtl4 = crate::Reg<swrst_ctl4::SwrstCtl4Spec>;
#[doc = "Software Reset Control 4 Register (SWRST_CTL4)"]
pub mod swrst_ctl4;
#[doc = "JEN_CTL1 (rw) register accessor: JTAG Enable Control 1 Register (JEN_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`jen_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jen_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jen_ctl1`]
module"]
#[doc(alias = "JEN_CTL1")]
pub type JenCtl1 = crate::Reg<jen_ctl1::JenCtl1Spec>;
#[doc = "JTAG Enable Control 1 Register (JEN_CTL1)"]
pub mod jen_ctl1;
#[doc = "JEN_CTL2 (rw) register accessor: JTAG Enable Control 2 Register (JEN_CTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`jen_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jen_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jen_ctl2`]
module"]
#[doc(alias = "JEN_CTL2")]
pub type JenCtl2 = crate::Reg<jen_ctl2::JenCtl2Spec>;
#[doc = "JTAG Enable Control 2 Register (JEN_CTL2)"]
pub mod jen_ctl2;
#[doc = "LV_GPIO_CTL0 (rw) register accessor: Low-Voltage GPIO Pins Control 0 Register (LV_GPIO_CTL0)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctl0`]
module"]
#[doc(alias = "LV_GPIO_CTL0")]
pub type LvGpioCtl0 = crate::Reg<lv_gpio_ctl0::LvGpioCtl0Spec>;
#[doc = "Low-Voltage GPIO Pins Control 0 Register (LV_GPIO_CTL0)"]
pub mod lv_gpio_ctl0;
#[doc = "LV_GPIO_CTL1 (rw) register accessor: Low-Voltage GPIO Pins Control 1 Register (LV_GPIO_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctl1`]
module"]
#[doc(alias = "LV_GPIO_CTL1")]
pub type LvGpioCtl1 = crate::Reg<lv_gpio_ctl1::LvGpioCtl1Spec>;
#[doc = "Low-Voltage GPIO Pins Control 1 Register (LV_GPIO_CTL1)"]
pub mod lv_gpio_ctl1;
#[doc = "LV_GPIO_CTL2 (rw) register accessor: Low-Voltage GPIO Pins Control 2 Register (LV_GPIO_CTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctl2`]
module"]
#[doc(alias = "LV_GPIO_CTL2")]
pub type LvGpioCtl2 = crate::Reg<lv_gpio_ctl2::LvGpioCtl2Spec>;
#[doc = "Low-Voltage GPIO Pins Control 2 Register (LV_GPIO_CTL2)"]
pub mod lv_gpio_ctl2;
#[doc = "LV_GPIO_CTL3 (rw) register accessor: Low-Voltage GPIO Pins Control 3 Register (LV_GPIO_CTL3)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctl3`]
module"]
#[doc(alias = "LV_GPIO_CTL3")]
pub type LvGpioCtl3 = crate::Reg<lv_gpio_ctl3::LvGpioCtl3Spec>;
#[doc = "Low-Voltage GPIO Pins Control 3 Register (LV_GPIO_CTL3)"]
pub mod lv_gpio_ctl3;
#[doc = "LV_GPIO_CTL4 (rw) register accessor: Low-Voltage GPIO Pins Control 4 Register (LV_GPIO_CTL4)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctl4`]
module"]
#[doc(alias = "LV_GPIO_CTL4")]
pub type LvGpioCtl4 = crate::Reg<lv_gpio_ctl4::LvGpioCtl4Spec>;
#[doc = "Low-Voltage GPIO Pins Control 4 Register (LV_GPIO_CTL4)"]
pub mod lv_gpio_ctl4;
#[doc = "LV_GPIO_CTL5 (rw) register accessor: Low-Voltage GPIO Pins Control 5 Register (LV_GPIO_CTL5)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctl5`]
module"]
#[doc(alias = "LV_GPIO_CTL5")]
pub type LvGpioCtl5 = crate::Reg<lv_gpio_ctl5::LvGpioCtl5Spec>;
#[doc = "Low-Voltage GPIO Pins Control 5 Register (LV_GPIO_CTL5)"]
pub mod lv_gpio_ctl5;
#[doc = "LV_GPIO_CTL6 (rw) register accessor: Low-Voltage GPIO Pins Control 6 Register (LV_GPIO_CTL6)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctl6`]
module"]
#[doc(alias = "LV_GPIO_CTL6")]
pub type LvGpioCtl6 = crate::Reg<lv_gpio_ctl6::LvGpioCtl6Spec>;
#[doc = "Low-Voltage GPIO Pins Control 6 Register (LV_GPIO_CTL6)"]
pub mod lv_gpio_ctl6;
#[doc = "LV_GPIO_CTL7 (rw) register accessor: Low-Voltage GPIO Pins Control 7 Register (LV_GPIO_CTL7)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctl7`]
module"]
#[doc(alias = "LV_GPIO_CTL7")]
pub type LvGpioCtl7 = crate::Reg<lv_gpio_ctl7::LvGpioCtl7Spec>;
#[doc = "Low-Voltage GPIO Pins Control 7 Register (LV_GPIO_CTL7)"]
pub mod lv_gpio_ctl7;
#[doc = "LV_GPIO_CTL8 (rw) register accessor: Low-Voltage GPIO Pins Control 8 Register (LV_GPIO_CTL8)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctl8`]
module"]
#[doc(alias = "LV_GPIO_CTL8")]
pub type LvGpioCtl8 = crate::Reg<lv_gpio_ctl8::LvGpioCtl8Spec>;
#[doc = "Low-Voltage GPIO Pins Control 8 Register (LV_GPIO_CTL8)"]
pub mod lv_gpio_ctl8;
#[doc = "LV_GPIO_CTL9 (rw) register accessor: Low-Voltage GPIO Pins Control 9 Register (LV_GPIO_CTL9)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctl9`]
module"]
#[doc(alias = "LV_GPIO_CTL9")]
pub type LvGpioCtl9 = crate::Reg<lv_gpio_ctl9::LvGpioCtl9Spec>;
#[doc = "Low-Voltage GPIO Pins Control 9 Register (LV_GPIO_CTL9)"]
pub mod lv_gpio_ctl9;
#[doc = "LV_GPIO_CTLA (rw) register accessor: Low-Voltage GPIO Pins Control A Register (LV_GPIO_CTLA)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctla`]
module"]
#[doc(alias = "LV_GPIO_CTLA")]
pub type LvGpioCtla = crate::Reg<lv_gpio_ctla::LvGpioCtlaSpec>;
#[doc = "Low-Voltage GPIO Pins Control A Register (LV_GPIO_CTLA)"]
pub mod lv_gpio_ctla;
#[doc = "LV_GPIO_CTLB (rw) register accessor: Low-Voltage GPIO Pins Control B Register (LV_GPIO_CTLB)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctlb`]
module"]
#[doc(alias = "LV_GPIO_CTLB")]
pub type LvGpioCtlb = crate::Reg<lv_gpio_ctlb::LvGpioCtlbSpec>;
#[doc = "Low-Voltage GPIO Pins Control B Register (LV_GPIO_CTLB)"]
pub mod lv_gpio_ctlb;
#[doc = "LV_GPIO_CTLP (rw) register accessor: Low-Voltage GPIO Pins Control P Register (LV_GPIO_CTLP)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctlp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctlp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lv_gpio_ctlp`]
module"]
#[doc(alias = "LV_GPIO_CTLP")]
pub type LvGpioCtlp = crate::Reg<lv_gpio_ctlp::LvGpioCtlpSpec>;
#[doc = "Low-Voltage GPIO Pins Control P Register (LV_GPIO_CTLP)"]
pub mod lv_gpio_ctlp;
#[doc = "DEVALT0_LK (rw) register accessor: Device Alternate Function 0 Lock Register (DEVALT0_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt0_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt0_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt0_lk`]
module"]
#[doc(alias = "DEVALT0_LK")]
pub type Devalt0Lk = crate::Reg<devalt0_lk::Devalt0LkSpec>;
#[doc = "Device Alternate Function 0 Lock Register (DEVALT0_LK)"]
pub mod devalt0_lk;
#[doc = "DEVALT2_LK (rw) register accessor: Device Alternate Function 2 Lock Register (DEVALT2_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt2_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt2_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt2_lk`]
module"]
#[doc(alias = "DEVALT2_LK")]
pub type Devalt2Lk = crate::Reg<devalt2_lk::Devalt2LkSpec>;
#[doc = "Device Alternate Function 2 Lock Register (DEVALT2_LK)"]
pub mod devalt2_lk;
#[doc = "DEVALT3_LK (rw) register accessor: Device Alternate Function 3 Lock Register (DEVALT3_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt3_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt3_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt3_lk`]
module"]
#[doc(alias = "DEVALT3_LK")]
pub type Devalt3Lk = crate::Reg<devalt3_lk::Devalt3LkSpec>;
#[doc = "Device Alternate Function 3 Lock Register (DEVALT3_LK)"]
pub mod devalt3_lk;
#[doc = "DEVALT4_LK (rw) register accessor: Device Alternate Function 4 Lock Register (DEVALT4_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt4_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt4_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt4_lk`]
module"]
#[doc(alias = "DEVALT4_LK")]
pub type Devalt4Lk = crate::Reg<devalt4_lk::Devalt4LkSpec>;
#[doc = "Device Alternate Function 4 Lock Register (DEVALT4_LK)"]
pub mod devalt4_lk;
#[doc = "DEVALT5_LK (rw) register accessor: Device Alternate Function 5 Lock Register (DEVALT5_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt5_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt5_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt5_lk`]
module"]
#[doc(alias = "DEVALT5_LK")]
pub type Devalt5Lk = crate::Reg<devalt5_lk::Devalt5LkSpec>;
#[doc = "Device Alternate Function 5 Lock Register (DEVALT5_LK)"]
pub mod devalt5_lk;
#[doc = "DEVALT6_LK (rw) register accessor: Device Alternate Function 6 Lock Register (DEVALT6_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt6_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt6_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalt6_lk`]
module"]
#[doc(alias = "DEVALT6_LK")]
pub type Devalt6Lk = crate::Reg<devalt6_lk::Devalt6LkSpec>;
#[doc = "Device Alternate Function 6 Lock Register (DEVALT6_LK)"]
pub mod devalt6_lk;
#[doc = "DEVALTB_LK (rw) register accessor: Device Alternate Function B Lock Register (DEVALTB_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltb_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltb_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltb_lk`]
module"]
#[doc(alias = "DEVALTB_LK")]
pub type DevaltbLk = crate::Reg<devaltb_lk::DevaltbLkSpec>;
#[doc = "Device Alternate Function B Lock Register (DEVALTB_LK)"]
pub mod devaltb_lk;
#[doc = "DEVALTF_LK (rw) register accessor: Device Alternate Function F Lock Register (DEVALTF_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltf_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltf_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltf_lk`]
module"]
#[doc(alias = "DEVALTF_LK")]
pub type DevaltfLk = crate::Reg<devaltf_lk::DevaltfLkSpec>;
#[doc = "Device Alternate Function F Lock Register (DEVALTF_LK)"]
pub mod devaltf_lk;
#[doc = "DEVALTH_LK (rw) register accessor: Alternate Function H Lock Register (DEVALTH_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalth_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalth_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devalth_lk`]
module"]
#[doc(alias = "DEVALTH_LK")]
pub type DevalthLk = crate::Reg<devalth_lk::DevalthLkSpec>;
#[doc = "Alternate Function H Lock Register (DEVALTH_LK)"]
pub mod devalth_lk;
#[doc = "DEVALTJ_LK (rw) register accessor: Device Alternate Function J Lock Register (DEVALTJ_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltj_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltj_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltj_lk`]
module"]
#[doc(alias = "DEVALTJ_LK")]
pub type DevaltjLk = crate::Reg<devaltj_lk::DevaltjLkSpec>;
#[doc = "Device Alternate Function J Lock Register (DEVALTJ_LK)"]
pub mod devaltj_lk;
#[doc = "DEVALTK_LK (rw) register accessor: Device Alternate Function K Lock Register (DEVALTK_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltk_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltk_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltk_lk`]
module"]
#[doc(alias = "DEVALTK_LK")]
pub type DevaltkLk = crate::Reg<devaltk_lk::DevaltkLkSpec>;
#[doc = "Device Alternate Function K Lock Register (DEVALTK_LK)"]
pub mod devaltk_lk;
#[doc = "DEVALTN_LK (rw) register accessor: Device Alternate Function N Lock Register (DEVALTN_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltn_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltn_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltn_lk`]
module"]
#[doc(alias = "DEVALTN_LK")]
pub type DevaltnLk = crate::Reg<devaltn_lk::DevaltnLkSpec>;
#[doc = "Device Alternate Function N Lock Register (DEVALTN_LK)"]
pub mod devaltn_lk;
#[doc = "SWRST_CTL1_LK (rw) register accessor: Software Reset Control 1 Lock Register (SWRST_CTL1_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl1_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl1_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_ctl1_lk`]
module"]
#[doc(alias = "SWRST_CTL1_LK")]
pub type SwrstCtl1Lk = crate::Reg<swrst_ctl1_lk::SwrstCtl1LkSpec>;
#[doc = "Software Reset Control 1 Lock Register (SWRST_CTL1_LK)"]
pub mod swrst_ctl1_lk;
#[doc = "SWRST_CTL2_LK (rw) register accessor: Software Reset Control 2 Lock Register (SWRST_CTL2_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl2_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl2_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_ctl2_lk`]
module"]
#[doc(alias = "SWRST_CTL2_LK")]
pub type SwrstCtl2Lk = crate::Reg<swrst_ctl2_lk::SwrstCtl2LkSpec>;
#[doc = "Software Reset Control 2 Lock Register (SWRST_CTL2_LK)"]
pub mod swrst_ctl2_lk;
#[doc = "SWRST_CTL3_LK (rw) register accessor: Software Reset Control 3 Lock Register (SWRST_CTL3_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl3_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl3_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_ctl3_lk`]
module"]
#[doc(alias = "SWRST_CTL3_LK")]
pub type SwrstCtl3Lk = crate::Reg<swrst_ctl3_lk::SwrstCtl3LkSpec>;
#[doc = "Software Reset Control 3 Lock Register (SWRST_CTL3_LK)"]
pub mod swrst_ctl3_lk;
#[doc = "SWRST_CTL4_LK (rw) register accessor: Software Reset Control 4 Lock Register (SWRST_CTL4_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl4_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl4_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_ctl4_lk`]
module"]
#[doc(alias = "SWRST_CTL4_LK")]
pub type SwrstCtl4Lk = crate::Reg<swrst_ctl4_lk::SwrstCtl4LkSpec>;
#[doc = "Software Reset Control 4 Lock Register (SWRST_CTL4_LK)"]
pub mod swrst_ctl4_lk;
#[doc = "DEVALTD (rw) register accessor: Device Alternate Function D Register (DEVALTD)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltd`]
module"]
#[doc(alias = "DEVALTD")]
pub type Devaltd = crate::Reg<devaltd::DevaltdSpec>;
#[doc = "Device Alternate Function D Register (DEVALTD)"]
pub mod devaltd;
#[doc = "DEVALTG (rw) register accessor: Device Alternate Function G Register (DEVALTG)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltg`]
module"]
#[doc(alias = "DEVALTG")]
pub type Devaltg = crate::Reg<devaltg::DevaltgSpec>;
#[doc = "Device Alternate Function G Register (DEVALTG)"]
pub mod devaltg;
#[doc = "DEVALTD_LK (rw) register accessor: Device Alternate Function D Lock Register (DEVALTD_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltd_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltd_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltd_lk`]
module"]
#[doc(alias = "DEVALTD_LK")]
pub type DevaltdLk = crate::Reg<devaltd_lk::DevaltdLkSpec>;
#[doc = "Device Alternate Function D Lock Register (DEVALTD_LK)"]
pub mod devaltd_lk;
#[doc = "DEVALTG_LK (rw) register accessor: Device Alternate Function G Lock Register (DEVALTG_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltg_lk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltg_lk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaltg_lk`]
module"]
#[doc(alias = "DEVALTG_LK")]
pub type DevaltgLk = crate::Reg<devaltg_lk::DevaltgLkSpec>;
#[doc = "Device Alternate Function G Lock Register (DEVALTG_LK)"]
pub mod devaltg_lk;
#[doc = "GP_CTL (rw) register accessor: Gang Programmer Control Register (GP_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`gp_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_ctl`]
module"]
#[doc(alias = "GP_CTL")]
pub type GpCtl = crate::Reg<gp_ctl::GpCtlSpec>;
#[doc = "Gang Programmer Control Register (GP_CTL)"]
pub mod gp_ctl;
