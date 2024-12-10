#[doc = "Register `PWDWN_CTL1` reader"]
pub type R = crate::R<PwdwnCtl1Spec>;
#[doc = "Register `PWDWN_CTL1` writer"]
pub type W = crate::W<PwdwnCtl1Spec>;
#[doc = "Field `KBS_PD` reader - Keyboard Scan Power-Down"]
pub type KbsPdR = crate::BitReader;
#[doc = "Field `KBS_PD` writer - Keyboard Scan Power-Down"]
pub type KbsPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDP_PD` reader - SDP Power-Down"]
pub type SdpPdR = crate::BitReader;
#[doc = "Field `SDP_PD` writer - SDP Power-Down"]
pub type SdpPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_PD` reader - PS/2 Power-Down"]
pub type Ps2PdR = crate::BitReader;
#[doc = "Field `PS2_PD` writer - PS/2 Power-Down"]
pub type Ps2PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_PD` reader - CR_UART1 Power-Down"]
pub type Uart1PdR = crate::BitReader;
#[doc = "Field `UART1_PD` writer - CR_UART1 Power-Down"]
pub type Uart1PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFT1_PD` reader - MFT16-1 Power-Down"]
pub type Mft1PdR = crate::BitReader;
#[doc = "Field `MFT1_PD` writer - MFT16-1 Power-Down"]
pub type Mft1PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFT2_PD` reader - MFT16-2 Power-Down"]
pub type Mft2PdR = crate::BitReader;
#[doc = "Field `MFT2_PD` writer - MFT16-2 Power-Down"]
pub type Mft2PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFT3_PD` reader - MFT16-3 Power-Down"]
pub type Mft3PdR = crate::BitReader;
#[doc = "Field `MFT3_PD` writer - MFT16-3 Power-Down"]
pub type Mft3PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Keyboard Scan Power-Down"]
    #[inline(always)]
    pub fn kbs_pd(&self) -> KbsPdR {
        KbsPdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDP Power-Down"]
    #[inline(always)]
    pub fn sdp_pd(&self) -> SdpPdR {
        SdpPdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - PS/2 Power-Down"]
    #[inline(always)]
    pub fn ps2_pd(&self) -> Ps2PdR {
        Ps2PdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CR_UART1 Power-Down"]
    #[inline(always)]
    pub fn uart1_pd(&self) -> Uart1PdR {
        Uart1PdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MFT16-1 Power-Down"]
    #[inline(always)]
    pub fn mft1_pd(&self) -> Mft1PdR {
        Mft1PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MFT16-2 Power-Down"]
    #[inline(always)]
    pub fn mft2_pd(&self) -> Mft2PdR {
        Mft2PdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MFT16-3 Power-Down"]
    #[inline(always)]
    pub fn mft3_pd(&self) -> Mft3PdR {
        Mft3PdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDWN_CTL1")
            .field("kbs_pd", &self.kbs_pd())
            .field("sdp_pd", &self.sdp_pd())
            .field("ps2_pd", &self.ps2_pd())
            .field("uart1_pd", &self.uart1_pd())
            .field("mft1_pd", &self.mft1_pd())
            .field("mft2_pd", &self.mft2_pd())
            .field("mft3_pd", &self.mft3_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Keyboard Scan Power-Down"]
    #[inline(always)]
    pub fn kbs_pd(&mut self) -> KbsPdW<PwdwnCtl1Spec> {
        KbsPdW::new(self, 0)
    }
    #[doc = "Bit 1 - SDP Power-Down"]
    #[inline(always)]
    pub fn sdp_pd(&mut self) -> SdpPdW<PwdwnCtl1Spec> {
        SdpPdW::new(self, 1)
    }
    #[doc = "Bit 3 - PS/2 Power-Down"]
    #[inline(always)]
    pub fn ps2_pd(&mut self) -> Ps2PdW<PwdwnCtl1Spec> {
        Ps2PdW::new(self, 3)
    }
    #[doc = "Bit 4 - CR_UART1 Power-Down"]
    #[inline(always)]
    pub fn uart1_pd(&mut self) -> Uart1PdW<PwdwnCtl1Spec> {
        Uart1PdW::new(self, 4)
    }
    #[doc = "Bit 5 - MFT16-1 Power-Down"]
    #[inline(always)]
    pub fn mft1_pd(&mut self) -> Mft1PdW<PwdwnCtl1Spec> {
        Mft1PdW::new(self, 5)
    }
    #[doc = "Bit 6 - MFT16-2 Power-Down"]
    #[inline(always)]
    pub fn mft2_pd(&mut self) -> Mft2PdW<PwdwnCtl1Spec> {
        Mft2PdW::new(self, 6)
    }
    #[doc = "Bit 7 - MFT16-3 Power-Down"]
    #[inline(always)]
    pub fn mft3_pd(&mut self) -> Mft3PdW<PwdwnCtl1Spec> {
        Mft3PdW::new(self, 7)
    }
}
#[doc = "Power-Down Control 1 Register (PWDWN_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdwnCtl1Spec;
impl crate::RegisterSpec for PwdwnCtl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwdwn_ctl1::R`](R) reader structure"]
impl crate::Readable for PwdwnCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwdwn_ctl1::W`](W) writer structure"]
impl crate::Writable for PwdwnCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWDWN_CTL1 to value 0"]
impl crate::Resettable for PwdwnCtl1Spec {
    const RESET_VALUE: u8 = 0;
}
