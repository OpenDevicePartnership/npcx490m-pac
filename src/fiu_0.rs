#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    burst_cfg: BurstCfg,
    resp_cfg: RespCfg,
    _reserved2: [u8; 0x11],
    spi_fl_cfg: SpiFlCfg,
    _reserved3: [u8; 0x01],
    uma_code: UmaCode,
    uma_ab020: UmaAb020,
    uma_ab021: UmaAb021,
    uma_ab022: UmaAb022,
    uma_db030: UmaDb030,
    uma_db031: UmaDb031,
    uma_db032: UmaDb032,
    uma_db033: UmaDb033,
    uma_cts: UmaCts,
    uma_ects: UmaEcts,
    uma_db0_3: UmaDb0_3,
    _reserved14: [u8; 0x01],
    uma_dmm: UmaDmm,
    _reserved15: [u8; 0x0a],
    dn_fiu_rd_cmd: DnFiuRdCmd,
    dn_fiu_wr_cmd: DnFiuWrCmd,
    dn_fiu_dmm_cyc: DnFiuDmmCyc,
    fiu_ext_cfg: FiuExtCfg,
    uma_ab0_3: UmaAb0_3,
    _reserved20: [u8; 0x04],
    spi_dev: SpiDev,
    _reserved21: [u8; 0x01],
    dn_spi_dev_size: DnSpiDevSize,
}
impl RegisterBlock {
    #[doc = "0x01 - Burst Configuration Register (BURST_CFG)"]
    #[inline(always)]
    pub const fn burst_cfg(&self) -> &BurstCfg {
        &self.burst_cfg
    }
    #[doc = "0x02 - FIU Response Configuration Register (RESP_CFG)"]
    #[inline(always)]
    pub const fn resp_cfg(&self) -> &RespCfg {
        &self.resp_cfg
    }
    #[doc = "0x14 - SPI Flash Configuration Register (SPI_FL_CFG)"]
    #[inline(always)]
    pub const fn spi_fl_cfg(&self) -> &SpiFlCfg {
        &self.spi_fl_cfg
    }
    #[doc = "0x16 - UMA Code Byte Register (UMA_CODE)"]
    #[inline(always)]
    pub const fn uma_code(&self) -> &UmaCode {
        &self.uma_code
    }
    #[doc = "0x17 - UMA Address Byte 0-2 Register (UMA_AB0-2)"]
    #[inline(always)]
    pub const fn uma_ab020(&self) -> &UmaAb020 {
        &self.uma_ab020
    }
    #[doc = "0x18 - UMA Address Byte 0-2 Register (UMA_AB0-2)"]
    #[inline(always)]
    pub const fn uma_ab021(&self) -> &UmaAb021 {
        &self.uma_ab021
    }
    #[doc = "0x19 - UMA Address Byte 0-2 Register (UMA_AB0-2)"]
    #[inline(always)]
    pub const fn uma_ab022(&self) -> &UmaAb022 {
        &self.uma_ab022
    }
    #[doc = "0x1a - UMA Data Byte 0-3 Register (UMA_DB0-3)"]
    #[inline(always)]
    pub const fn uma_db030(&self) -> &UmaDb030 {
        &self.uma_db030
    }
    #[doc = "0x1b - UMA Data Byte 0-3 Register (UMA_DB0-3)"]
    #[inline(always)]
    pub const fn uma_db031(&self) -> &UmaDb031 {
        &self.uma_db031
    }
    #[doc = "0x1c - UMA Data Byte 0-3 Register (UMA_DB0-3)"]
    #[inline(always)]
    pub const fn uma_db032(&self) -> &UmaDb032 {
        &self.uma_db032
    }
    #[doc = "0x1d - UMA Data Byte 0-3 Register (UMA_DB0-3)"]
    #[inline(always)]
    pub const fn uma_db033(&self) -> &UmaDb033 {
        &self.uma_db033
    }
    #[doc = "0x1e - UMA Control and Status Register (UMA_CTS)"]
    #[inline(always)]
    pub const fn uma_cts(&self) -> &UmaCts {
        &self.uma_cts
    }
    #[doc = "0x1f - UMA Extended Control and Status Register (UMA_ECTS)"]
    #[inline(always)]
    pub const fn uma_ects(&self) -> &UmaEcts {
        &self.uma_ects
    }
    #[doc = "0x20 - UMA Data Bytes 0-3 Register (UMA_DB0_3)"]
    #[inline(always)]
    pub const fn uma_db0_3(&self) -> &UmaDb0_3 {
        &self.uma_db0_3
    }
    #[doc = "0x25 - UMA Dummy Register (UMA_DMM)"]
    #[inline(always)]
    pub const fn uma_dmm(&self) -> &UmaDmm {
        &self.uma_dmm
    }
    #[doc = "0x30 - Device 'n' FIU Read Command Register (Dn_FIU_RD_CMD)"]
    #[inline(always)]
    pub const fn dn_fiu_rd_cmd(&self) -> &DnFiuRdCmd {
        &self.dn_fiu_rd_cmd
    }
    #[doc = "0x31 - Device 'n' FIU Write Command Register (Dn_FIU_WR_CMD)"]
    #[inline(always)]
    pub const fn dn_fiu_wr_cmd(&self) -> &DnFiuWrCmd {
        &self.dn_fiu_wr_cmd
    }
    #[doc = "0x32 - Device 'n' FIU Dummy Cycle Register (Dn_FIU_DMM_CYC)"]
    #[inline(always)]
    pub const fn dn_fiu_dmm_cyc(&self) -> &DnFiuDmmCyc {
        &self.dn_fiu_dmm_cyc
    }
    #[doc = "0x33 - FIU Extended Configuration Register (FIU_EXT_CFG)"]
    #[inline(always)]
    pub const fn fiu_ext_cfg(&self) -> &FiuExtCfg {
        &self.fiu_ext_cfg
    }
    #[doc = "0x34 - UMA Address Byte 0_3 Register (UMA_AB0_3)"]
    #[inline(always)]
    pub const fn uma_ab0_3(&self) -> &UmaAb0_3 {
        &self.uma_ab0_3
    }
    #[doc = "0x3c - SPI Device Register (SPI_DEV)"]
    #[inline(always)]
    pub const fn spi_dev(&self) -> &SpiDev {
        &self.spi_dev
    }
    #[doc = "0x3e - Device 'n' SPI Device Size Register (Dn_SPI_DEV_SIZE)"]
    #[inline(always)]
    pub const fn dn_spi_dev_size(&self) -> &DnSpiDevSize {
        &self.dn_spi_dev_size
    }
}
#[doc = "BURST_CFG (rw) register accessor: Burst Configuration Register (BURST_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`burst_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`burst_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@burst_cfg`]
module"]
#[doc(alias = "BURST_CFG")]
pub type BurstCfg = crate::Reg<burst_cfg::BurstCfgSpec>;
#[doc = "Burst Configuration Register (BURST_CFG)"]
pub mod burst_cfg;
#[doc = "RESP_CFG (rw) register accessor: FIU Response Configuration Register (RESP_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`resp_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp_cfg`]
module"]
#[doc(alias = "RESP_CFG")]
pub type RespCfg = crate::Reg<resp_cfg::RespCfgSpec>;
#[doc = "FIU Response Configuration Register (RESP_CFG)"]
pub mod resp_cfg;
#[doc = "SPI_FL_CFG (rw) register accessor: SPI Flash Configuration Register (SPI_FL_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fl_cfg`]
module"]
#[doc(alias = "SPI_FL_CFG")]
pub type SpiFlCfg = crate::Reg<spi_fl_cfg::SpiFlCfgSpec>;
#[doc = "SPI Flash Configuration Register (SPI_FL_CFG)"]
pub mod spi_fl_cfg;
#[doc = "UMA_CODE (rw) register accessor: UMA Code Byte Register (UMA_CODE)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_code::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_code::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_code`]
module"]
#[doc(alias = "UMA_CODE")]
pub type UmaCode = crate::Reg<uma_code::UmaCodeSpec>;
#[doc = "UMA Code Byte Register (UMA_CODE)"]
pub mod uma_code;
#[doc = "UMA_AB0-20 (rw) register accessor: UMA Address Byte 0-2 Register (UMA_AB0-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_ab020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_ab020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_ab020`]
module"]
#[doc(alias = "UMA_AB0-20")]
pub type UmaAb020 = crate::Reg<uma_ab020::UmaAb020Spec>;
#[doc = "UMA Address Byte 0-2 Register (UMA_AB0-2)"]
pub mod uma_ab020;
#[doc = "UMA_AB0-21 (rw) register accessor: UMA Address Byte 0-2 Register (UMA_AB0-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_ab021::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_ab021::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_ab021`]
module"]
#[doc(alias = "UMA_AB0-21")]
pub type UmaAb021 = crate::Reg<uma_ab021::UmaAb021Spec>;
#[doc = "UMA Address Byte 0-2 Register (UMA_AB0-2)"]
pub mod uma_ab021;
#[doc = "UMA_AB0-22 (rw) register accessor: UMA Address Byte 0-2 Register (UMA_AB0-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_ab022::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_ab022::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_ab022`]
module"]
#[doc(alias = "UMA_AB0-22")]
pub type UmaAb022 = crate::Reg<uma_ab022::UmaAb022Spec>;
#[doc = "UMA Address Byte 0-2 Register (UMA_AB0-2)"]
pub mod uma_ab022;
#[doc = "UMA_DB0-30 (rw) register accessor: UMA Data Byte 0-3 Register (UMA_DB0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_db030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_db030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_db030`]
module"]
#[doc(alias = "UMA_DB0-30")]
pub type UmaDb030 = crate::Reg<uma_db030::UmaDb030Spec>;
#[doc = "UMA Data Byte 0-3 Register (UMA_DB0-3)"]
pub mod uma_db030;
#[doc = "UMA_DB0-31 (rw) register accessor: UMA Data Byte 0-3 Register (UMA_DB0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_db031::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_db031::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_db031`]
module"]
#[doc(alias = "UMA_DB0-31")]
pub type UmaDb031 = crate::Reg<uma_db031::UmaDb031Spec>;
#[doc = "UMA Data Byte 0-3 Register (UMA_DB0-3)"]
pub mod uma_db031;
#[doc = "UMA_DB0-32 (rw) register accessor: UMA Data Byte 0-3 Register (UMA_DB0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_db032::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_db032::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_db032`]
module"]
#[doc(alias = "UMA_DB0-32")]
pub type UmaDb032 = crate::Reg<uma_db032::UmaDb032Spec>;
#[doc = "UMA Data Byte 0-3 Register (UMA_DB0-3)"]
pub mod uma_db032;
#[doc = "UMA_DB0-33 (rw) register accessor: UMA Data Byte 0-3 Register (UMA_DB0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_db033::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_db033::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_db033`]
module"]
#[doc(alias = "UMA_DB0-33")]
pub type UmaDb033 = crate::Reg<uma_db033::UmaDb033Spec>;
#[doc = "UMA Data Byte 0-3 Register (UMA_DB0-3)"]
pub mod uma_db033;
#[doc = "UMA_CTS (rw) register accessor: UMA Control and Status Register (UMA_CTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_cts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_cts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_cts`]
module"]
#[doc(alias = "UMA_CTS")]
pub type UmaCts = crate::Reg<uma_cts::UmaCtsSpec>;
#[doc = "UMA Control and Status Register (UMA_CTS)"]
pub mod uma_cts;
#[doc = "UMA_ECTS (rw) register accessor: UMA Extended Control and Status Register (UMA_ECTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_ects::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_ects::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_ects`]
module"]
#[doc(alias = "UMA_ECTS")]
pub type UmaEcts = crate::Reg<uma_ects::UmaEctsSpec>;
#[doc = "UMA Extended Control and Status Register (UMA_ECTS)"]
pub mod uma_ects;
#[doc = "UMA_DB0_3 (rw) register accessor: UMA Data Bytes 0-3 Register (UMA_DB0_3)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_db0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_db0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_db0_3`]
module"]
#[doc(alias = "UMA_DB0_3")]
pub type UmaDb0_3 = crate::Reg<uma_db0_3::UmaDb0_3Spec>;
#[doc = "UMA Data Bytes 0-3 Register (UMA_DB0_3)"]
pub mod uma_db0_3;
#[doc = "UMA_DMM (rw) register accessor: UMA Dummy Register (UMA_DMM)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_dmm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_dmm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_dmm`]
module"]
#[doc(alias = "UMA_DMM")]
pub type UmaDmm = crate::Reg<uma_dmm::UmaDmmSpec>;
#[doc = "UMA Dummy Register (UMA_DMM)"]
pub mod uma_dmm;
#[doc = "Dn_FIU_RD_CMD (rw) register accessor: Device 'n' FIU Read Command Register (Dn_FIU_RD_CMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`dn_fiu_rd_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dn_fiu_rd_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dn_fiu_rd_cmd`]
module"]
#[doc(alias = "Dn_FIU_RD_CMD")]
pub type DnFiuRdCmd = crate::Reg<dn_fiu_rd_cmd::DnFiuRdCmdSpec>;
#[doc = "Device 'n' FIU Read Command Register (Dn_FIU_RD_CMD)"]
pub mod dn_fiu_rd_cmd;
#[doc = "Dn_FIU_WR_CMD (rw) register accessor: Device 'n' FIU Write Command Register (Dn_FIU_WR_CMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`dn_fiu_wr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dn_fiu_wr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dn_fiu_wr_cmd`]
module"]
#[doc(alias = "Dn_FIU_WR_CMD")]
pub type DnFiuWrCmd = crate::Reg<dn_fiu_wr_cmd::DnFiuWrCmdSpec>;
#[doc = "Device 'n' FIU Write Command Register (Dn_FIU_WR_CMD)"]
pub mod dn_fiu_wr_cmd;
#[doc = "Dn_FIU_DMM_CYC (rw) register accessor: Device 'n' FIU Dummy Cycle Register (Dn_FIU_DMM_CYC)\n\nYou can [`read`](crate::Reg::read) this register and get [`dn_fiu_dmm_cyc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dn_fiu_dmm_cyc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dn_fiu_dmm_cyc`]
module"]
#[doc(alias = "Dn_FIU_DMM_CYC")]
pub type DnFiuDmmCyc = crate::Reg<dn_fiu_dmm_cyc::DnFiuDmmCycSpec>;
#[doc = "Device 'n' FIU Dummy Cycle Register (Dn_FIU_DMM_CYC)"]
pub mod dn_fiu_dmm_cyc;
#[doc = "FIU_EXT_CFG (rw) register accessor: FIU Extended Configuration Register (FIU_EXT_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`fiu_ext_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiu_ext_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiu_ext_cfg`]
module"]
#[doc(alias = "FIU_EXT_CFG")]
pub type FiuExtCfg = crate::Reg<fiu_ext_cfg::FiuExtCfgSpec>;
#[doc = "FIU Extended Configuration Register (FIU_EXT_CFG)"]
pub mod fiu_ext_cfg;
#[doc = "UMA_AB0_3 (rw) register accessor: UMA Address Byte 0_3 Register (UMA_AB0_3)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_ab0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_ab0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uma_ab0_3`]
module"]
#[doc(alias = "UMA_AB0_3")]
pub type UmaAb0_3 = crate::Reg<uma_ab0_3::UmaAb0_3Spec>;
#[doc = "UMA Address Byte 0_3 Register (UMA_AB0_3)"]
pub mod uma_ab0_3;
#[doc = "SPI_DEV (rw) register accessor: SPI Device Register (SPI_DEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_dev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dev`]
module"]
#[doc(alias = "SPI_DEV")]
pub type SpiDev = crate::Reg<spi_dev::SpiDevSpec>;
#[doc = "SPI Device Register (SPI_DEV)"]
pub mod spi_dev;
#[doc = "Dn_SPI_DEV_SIZE (rw) register accessor: Device 'n' SPI Device Size Register (Dn_SPI_DEV_SIZE)\n\nYou can [`read`](crate::Reg::read) this register and get [`dn_spi_dev_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dn_spi_dev_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dn_spi_dev_size`]
module"]
#[doc(alias = "Dn_SPI_DEV_SIZE")]
pub type DnSpiDevSize = crate::Reg<dn_spi_dev_size::DnSpiDevSizeSpec>;
#[doc = "Device 'n' SPI Device Size Register (Dn_SPI_DEV_SIZE)"]
pub mod dn_spi_dev_size;
