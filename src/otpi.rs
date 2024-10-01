#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    otp_ctl: OtpCtl,
    otp_addr_l: OtpAddrL,
    otp_addr_h: OtpAddrH,
    otp_dout: OtpDout,
    _reserved4: [u8; 0x01],
    otp_rd_prot_0: OtpRdProt0,
    otp_rd_prot_1: OtpRdProt1,
    otp_rd_prot_2: OtpRdProt2,
    otp_rd_prot_3: OtpRdProt3,
    otp_rd_prot_4: OtpRdProt4,
    otp_rd_prot_5: OtpRdProt5,
    otp_rd_prot_6: OtpRdProt6,
    otp_rd_prot_7: OtpRdProt7,
    otp_wr_prot_0: OtpWrProt0,
    otp_wr_prot_1: OtpWrProt1,
    otp_wr_prot_2: OtpWrProt2,
    otp_wr_prot_3: OtpWrProt3,
    otp_wr_prot_4: OtpWrProt4,
    otp_wr_prot_5: OtpWrProt5,
    otp_wr_prot_6: OtpWrProt6,
    otp_wr_prot_7: OtpWrProt7,
    otp_presc: OtpPresc,
    otp_write_en0: OtpWriteEn0,
    otp_write_en1: OtpWriteEn1,
    otp_write_en2: OtpWriteEn2,
    otp_write_en3: OtpWriteEn3,
    _reserved25: [u8; 0x26],
    otp_rd_prot_8: OtpRdProt8,
    otp_rd_prot_9: OtpRdProt9,
    otp_rd_prot_10: OtpRdProt10,
    otp_rd_prot_11: OtpRdProt11,
    otp_rd_prot_12: OtpRdProt12,
    otp_rd_prot_13: OtpRdProt13,
    otp_rd_prot_14: OtpRdProt14,
    otp_rd_prot_15: OtpRdProt15,
    _reserved33: [u8; 0x18],
    otp_wr_prot_8: OtpWrProt8,
    otp_wr_prot_9: OtpWrProt9,
    otp_wr_prot_10: OtpWrProt10,
    otp_wr_prot_11: OtpWrProt11,
    otp_wr_prot_12: OtpWrProt12,
    otp_wr_prot_13: OtpWrProt13,
    otp_wr_prot_14: OtpWrProt14,
    otp_wr_prot_15: OtpWrProt15,
}
impl RegisterBlock {
    #[doc = "0x00 - OTP Control Register (OTP_CTL)"]
    #[inline(always)]
    pub const fn otp_ctl(&self) -> &OtpCtl {
        &self.otp_ctl
    }
    #[doc = "0x01 - OTP Address Low Register (OTP_ADDR_L)"]
    #[inline(always)]
    pub const fn otp_addr_l(&self) -> &OtpAddrL {
        &self.otp_addr_l
    }
    #[doc = "0x02 - OTP Address High Register (OTP_ADDR_H)"]
    #[inline(always)]
    pub const fn otp_addr_h(&self) -> &OtpAddrH {
        &self.otp_addr_h
    }
    #[doc = "0x03 - OTP Read Data Register (OTP_DOUT)"]
    #[inline(always)]
    pub const fn otp_dout(&self) -> &OtpDout {
        &self.otp_dout
    }
    #[doc = "0x05 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_0(&self) -> &OtpRdProt0 {
        &self.otp_rd_prot_0
    }
    #[doc = "0x06 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_1(&self) -> &OtpRdProt1 {
        &self.otp_rd_prot_1
    }
    #[doc = "0x07 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_2(&self) -> &OtpRdProt2 {
        &self.otp_rd_prot_2
    }
    #[doc = "0x08 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_3(&self) -> &OtpRdProt3 {
        &self.otp_rd_prot_3
    }
    #[doc = "0x09 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_4(&self) -> &OtpRdProt4 {
        &self.otp_rd_prot_4
    }
    #[doc = "0x0a - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_5(&self) -> &OtpRdProt5 {
        &self.otp_rd_prot_5
    }
    #[doc = "0x0b - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_6(&self) -> &OtpRdProt6 {
        &self.otp_rd_prot_6
    }
    #[doc = "0x0c - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_7(&self) -> &OtpRdProt7 {
        &self.otp_rd_prot_7
    }
    #[doc = "0x0d - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_0(&self) -> &OtpWrProt0 {
        &self.otp_wr_prot_0
    }
    #[doc = "0x0e - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_1(&self) -> &OtpWrProt1 {
        &self.otp_wr_prot_1
    }
    #[doc = "0x0f - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_2(&self) -> &OtpWrProt2 {
        &self.otp_wr_prot_2
    }
    #[doc = "0x10 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_3(&self) -> &OtpWrProt3 {
        &self.otp_wr_prot_3
    }
    #[doc = "0x11 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_4(&self) -> &OtpWrProt4 {
        &self.otp_wr_prot_4
    }
    #[doc = "0x12 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_5(&self) -> &OtpWrProt5 {
        &self.otp_wr_prot_5
    }
    #[doc = "0x13 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_6(&self) -> &OtpWrProt6 {
        &self.otp_wr_prot_6
    }
    #[doc = "0x14 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_7(&self) -> &OtpWrProt7 {
        &self.otp_wr_prot_7
    }
    #[doc = "0x15 - OTP Prescaler Register (OTP_PRESC)"]
    #[inline(always)]
    pub const fn otp_presc(&self) -> &OtpPresc {
        &self.otp_presc
    }
    #[doc = "0x16 - OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)"]
    #[inline(always)]
    pub const fn otp_write_en0(&self) -> &OtpWriteEn0 {
        &self.otp_write_en0
    }
    #[doc = "0x17 - OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)"]
    #[inline(always)]
    pub const fn otp_write_en1(&self) -> &OtpWriteEn1 {
        &self.otp_write_en1
    }
    #[doc = "0x18 - OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)"]
    #[inline(always)]
    pub const fn otp_write_en2(&self) -> &OtpWriteEn2 {
        &self.otp_write_en2
    }
    #[doc = "0x19 - OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)"]
    #[inline(always)]
    pub const fn otp_write_en3(&self) -> &OtpWriteEn3 {
        &self.otp_write_en3
    }
    #[doc = "0x40 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_8(&self) -> &OtpRdProt8 {
        &self.otp_rd_prot_8
    }
    #[doc = "0x41 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_9(&self) -> &OtpRdProt9 {
        &self.otp_rd_prot_9
    }
    #[doc = "0x42 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_10(&self) -> &OtpRdProt10 {
        &self.otp_rd_prot_10
    }
    #[doc = "0x43 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_11(&self) -> &OtpRdProt11 {
        &self.otp_rd_prot_11
    }
    #[doc = "0x44 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_12(&self) -> &OtpRdProt12 {
        &self.otp_rd_prot_12
    }
    #[doc = "0x45 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_13(&self) -> &OtpRdProt13 {
        &self.otp_rd_prot_13
    }
    #[doc = "0x46 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_14(&self) -> &OtpRdProt14 {
        &self.otp_rd_prot_14
    }
    #[doc = "0x47 - OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_rd_prot_15(&self) -> &OtpRdProt15 {
        &self.otp_rd_prot_15
    }
    #[doc = "0x60 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_8(&self) -> &OtpWrProt8 {
        &self.otp_wr_prot_8
    }
    #[doc = "0x61 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_9(&self) -> &OtpWrProt9 {
        &self.otp_wr_prot_9
    }
    #[doc = "0x62 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_10(&self) -> &OtpWrProt10 {
        &self.otp_wr_prot_10
    }
    #[doc = "0x63 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_11(&self) -> &OtpWrProt11 {
        &self.otp_wr_prot_11
    }
    #[doc = "0x64 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_12(&self) -> &OtpWrProt12 {
        &self.otp_wr_prot_12
    }
    #[doc = "0x65 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_13(&self) -> &OtpWrProt13 {
        &self.otp_wr_prot_13
    }
    #[doc = "0x66 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_14(&self) -> &OtpWrProt14 {
        &self.otp_wr_prot_14
    }
    #[doc = "0x67 - OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
    #[inline(always)]
    pub const fn otp_wr_prot_15(&self) -> &OtpWrProt15 {
        &self.otp_wr_prot_15
    }
}
#[doc = "OTP_CTL (rw) register accessor: OTP Control Register (OTP_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_ctl`]
module"]
#[doc(alias = "OTP_CTL")]
pub type OtpCtl = crate::Reg<otp_ctl::OtpCtlSpec>;
#[doc = "OTP Control Register (OTP_CTL)"]
pub mod otp_ctl;
#[doc = "OTP_ADDR_L (rw) register accessor: OTP Address Low Register (OTP_ADDR_L)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_addr_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_addr_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_addr_l`]
module"]
#[doc(alias = "OTP_ADDR_L")]
pub type OtpAddrL = crate::Reg<otp_addr_l::OtpAddrLSpec>;
#[doc = "OTP Address Low Register (OTP_ADDR_L)"]
pub mod otp_addr_l;
#[doc = "OTP_ADDR_H (rw) register accessor: OTP Address High Register (OTP_ADDR_H)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_addr_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_addr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_addr_h`]
module"]
#[doc(alias = "OTP_ADDR_H")]
pub type OtpAddrH = crate::Reg<otp_addr_h::OtpAddrHSpec>;
#[doc = "OTP Address High Register (OTP_ADDR_H)"]
pub mod otp_addr_h;
#[doc = "OTP_DOUT (rw) register accessor: OTP Read Data Register (OTP_DOUT)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_dout`]
module"]
#[doc(alias = "OTP_DOUT")]
pub type OtpDout = crate::Reg<otp_dout::OtpDoutSpec>;
#[doc = "OTP Read Data Register (OTP_DOUT)"]
pub mod otp_dout;
#[doc = "OTP_RD_PROT_0 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_0`]
module"]
#[doc(alias = "OTP_RD_PROT_0")]
pub type OtpRdProt0 = crate::Reg<otp_rd_prot_0::OtpRdProt0Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_0;
#[doc = "OTP_RD_PROT_1 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_1`]
module"]
#[doc(alias = "OTP_RD_PROT_1")]
pub type OtpRdProt1 = crate::Reg<otp_rd_prot_1::OtpRdProt1Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_1;
#[doc = "OTP_RD_PROT_2 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_2`]
module"]
#[doc(alias = "OTP_RD_PROT_2")]
pub type OtpRdProt2 = crate::Reg<otp_rd_prot_2::OtpRdProt2Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_2;
#[doc = "OTP_RD_PROT_3 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_3`]
module"]
#[doc(alias = "OTP_RD_PROT_3")]
pub type OtpRdProt3 = crate::Reg<otp_rd_prot_3::OtpRdProt3Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_3;
#[doc = "OTP_RD_PROT_4 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_4`]
module"]
#[doc(alias = "OTP_RD_PROT_4")]
pub type OtpRdProt4 = crate::Reg<otp_rd_prot_4::OtpRdProt4Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_4;
#[doc = "OTP_RD_PROT_5 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_5`]
module"]
#[doc(alias = "OTP_RD_PROT_5")]
pub type OtpRdProt5 = crate::Reg<otp_rd_prot_5::OtpRdProt5Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_5;
#[doc = "OTP_RD_PROT_6 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_6`]
module"]
#[doc(alias = "OTP_RD_PROT_6")]
pub type OtpRdProt6 = crate::Reg<otp_rd_prot_6::OtpRdProt6Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_6;
#[doc = "OTP_RD_PROT_7 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_7`]
module"]
#[doc(alias = "OTP_RD_PROT_7")]
pub type OtpRdProt7 = crate::Reg<otp_rd_prot_7::OtpRdProt7Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_7;
#[doc = "OTP_RD_PROT_8 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_8`]
module"]
#[doc(alias = "OTP_RD_PROT_8")]
pub type OtpRdProt8 = crate::Reg<otp_rd_prot_8::OtpRdProt8Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_8;
#[doc = "OTP_RD_PROT_9 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_9`]
module"]
#[doc(alias = "OTP_RD_PROT_9")]
pub type OtpRdProt9 = crate::Reg<otp_rd_prot_9::OtpRdProt9Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_9;
#[doc = "OTP_RD_PROT_10 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_10`]
module"]
#[doc(alias = "OTP_RD_PROT_10")]
pub type OtpRdProt10 = crate::Reg<otp_rd_prot_10::OtpRdProt10Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_10;
#[doc = "OTP_RD_PROT_11 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_11`]
module"]
#[doc(alias = "OTP_RD_PROT_11")]
pub type OtpRdProt11 = crate::Reg<otp_rd_prot_11::OtpRdProt11Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_11;
#[doc = "OTP_RD_PROT_12 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_12`]
module"]
#[doc(alias = "OTP_RD_PROT_12")]
pub type OtpRdProt12 = crate::Reg<otp_rd_prot_12::OtpRdProt12Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_12;
#[doc = "OTP_RD_PROT_13 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_13`]
module"]
#[doc(alias = "OTP_RD_PROT_13")]
pub type OtpRdProt13 = crate::Reg<otp_rd_prot_13::OtpRdProt13Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_13;
#[doc = "OTP_RD_PROT_14 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_14`]
module"]
#[doc(alias = "OTP_RD_PROT_14")]
pub type OtpRdProt14 = crate::Reg<otp_rd_prot_14::OtpRdProt14Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_14;
#[doc = "OTP_RD_PROT_15 (rw) register accessor: OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_rd_prot_15`]
module"]
#[doc(alias = "OTP_RD_PROT_15")]
pub type OtpRdProt15 = crate::Reg<otp_rd_prot_15::OtpRdProt15Spec>;
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)"]
pub mod otp_rd_prot_15;
#[doc = "OTP_WR_PROT_0 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_0`]
module"]
#[doc(alias = "OTP_WR_PROT_0")]
pub type OtpWrProt0 = crate::Reg<otp_wr_prot_0::OtpWrProt0Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_0;
#[doc = "OTP_WR_PROT_1 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_1`]
module"]
#[doc(alias = "OTP_WR_PROT_1")]
pub type OtpWrProt1 = crate::Reg<otp_wr_prot_1::OtpWrProt1Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_1;
#[doc = "OTP_WR_PROT_2 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_2`]
module"]
#[doc(alias = "OTP_WR_PROT_2")]
pub type OtpWrProt2 = crate::Reg<otp_wr_prot_2::OtpWrProt2Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_2;
#[doc = "OTP_WR_PROT_3 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_3`]
module"]
#[doc(alias = "OTP_WR_PROT_3")]
pub type OtpWrProt3 = crate::Reg<otp_wr_prot_3::OtpWrProt3Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_3;
#[doc = "OTP_WR_PROT_4 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_4`]
module"]
#[doc(alias = "OTP_WR_PROT_4")]
pub type OtpWrProt4 = crate::Reg<otp_wr_prot_4::OtpWrProt4Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_4;
#[doc = "OTP_WR_PROT_5 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_5`]
module"]
#[doc(alias = "OTP_WR_PROT_5")]
pub type OtpWrProt5 = crate::Reg<otp_wr_prot_5::OtpWrProt5Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_5;
#[doc = "OTP_WR_PROT_6 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_6`]
module"]
#[doc(alias = "OTP_WR_PROT_6")]
pub type OtpWrProt6 = crate::Reg<otp_wr_prot_6::OtpWrProt6Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_6;
#[doc = "OTP_WR_PROT_7 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_7`]
module"]
#[doc(alias = "OTP_WR_PROT_7")]
pub type OtpWrProt7 = crate::Reg<otp_wr_prot_7::OtpWrProt7Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_7;
#[doc = "OTP_WR_PROT_8 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_8`]
module"]
#[doc(alias = "OTP_WR_PROT_8")]
pub type OtpWrProt8 = crate::Reg<otp_wr_prot_8::OtpWrProt8Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_8;
#[doc = "OTP_WR_PROT_9 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_9`]
module"]
#[doc(alias = "OTP_WR_PROT_9")]
pub type OtpWrProt9 = crate::Reg<otp_wr_prot_9::OtpWrProt9Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_9;
#[doc = "OTP_WR_PROT_10 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_10`]
module"]
#[doc(alias = "OTP_WR_PROT_10")]
pub type OtpWrProt10 = crate::Reg<otp_wr_prot_10::OtpWrProt10Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_10;
#[doc = "OTP_WR_PROT_11 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_11`]
module"]
#[doc(alias = "OTP_WR_PROT_11")]
pub type OtpWrProt11 = crate::Reg<otp_wr_prot_11::OtpWrProt11Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_11;
#[doc = "OTP_WR_PROT_12 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_12`]
module"]
#[doc(alias = "OTP_WR_PROT_12")]
pub type OtpWrProt12 = crate::Reg<otp_wr_prot_12::OtpWrProt12Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_12;
#[doc = "OTP_WR_PROT_13 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_13`]
module"]
#[doc(alias = "OTP_WR_PROT_13")]
pub type OtpWrProt13 = crate::Reg<otp_wr_prot_13::OtpWrProt13Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_13;
#[doc = "OTP_WR_PROT_14 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_14`]
module"]
#[doc(alias = "OTP_WR_PROT_14")]
pub type OtpWrProt14 = crate::Reg<otp_wr_prot_14::OtpWrProt14Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_14;
#[doc = "OTP_WR_PROT_15 (rw) register accessor: OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_wr_prot_15`]
module"]
#[doc(alias = "OTP_WR_PROT_15")]
pub type OtpWrProt15 = crate::Reg<otp_wr_prot_15::OtpWrProt15Spec>;
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)"]
pub mod otp_wr_prot_15;
#[doc = "OTP_PRESC (rw) register accessor: OTP Prescaler Register (OTP_PRESC)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_presc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_presc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_presc`]
module"]
#[doc(alias = "OTP_PRESC")]
pub type OtpPresc = crate::Reg<otp_presc::OtpPrescSpec>;
#[doc = "OTP Prescaler Register (OTP_PRESC)"]
pub mod otp_presc;
#[doc = "OTP_WRITE_EN0 (rw) register accessor: OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_write_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_write_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_write_en0`]
module"]
#[doc(alias = "OTP_WRITE_EN0")]
pub type OtpWriteEn0 = crate::Reg<otp_write_en0::OtpWriteEn0Spec>;
#[doc = "OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)"]
pub mod otp_write_en0;
#[doc = "OTP_WRITE_EN1 (rw) register accessor: OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_write_en1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_write_en1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_write_en1`]
module"]
#[doc(alias = "OTP_WRITE_EN1")]
pub type OtpWriteEn1 = crate::Reg<otp_write_en1::OtpWriteEn1Spec>;
#[doc = "OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)"]
pub mod otp_write_en1;
#[doc = "OTP_WRITE_EN2 (rw) register accessor: OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_write_en2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_write_en2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_write_en2`]
module"]
#[doc(alias = "OTP_WRITE_EN2")]
pub type OtpWriteEn2 = crate::Reg<otp_write_en2::OtpWriteEn2Spec>;
#[doc = "OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)"]
pub mod otp_write_en2;
#[doc = "OTP_WRITE_EN3 (rw) register accessor: OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_write_en3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_write_en3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_write_en3`]
module"]
#[doc(alias = "OTP_WRITE_EN3")]
pub type OtpWriteEn3 = crate::Reg<otp_write_en3::OtpWriteEn3Spec>;
#[doc = "OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)"]
pub mod otp_write_en3;
