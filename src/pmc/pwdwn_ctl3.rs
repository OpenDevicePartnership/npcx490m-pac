#[doc = "Register `PWDWN_CTL3` reader"]
pub type R = crate::R<PwdwnCtl3Spec>;
#[doc = "Register `PWDWN_CTL3` writer"]
pub type W = crate::W<PwdwnCtl3Spec>;
#[doc = "Field `SMB0_PD` reader - SMB0 Power-Down"]
pub type Smb0PdR = crate::BitReader;
#[doc = "Field `SMB0_PD` writer - SMB0 Power-Down"]
pub type Smb0PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB1_PD` reader - SMB1 Power-Down"]
pub type Smb1PdR = crate::BitReader;
#[doc = "Field `SMB1_PD` writer - SMB1 Power-Down"]
pub type Smb1PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB2_PD` reader - SMB2 Power-Down"]
pub type Smb2PdR = crate::BitReader;
#[doc = "Field `SMB2_PD` writer - SMB2 Power-Down"]
pub type Smb2PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB3_PD` reader - SMB3 Power-Down"]
pub type Smb3PdR = crate::BitReader;
#[doc = "Field `SMB3_PD` writer - SMB3 Power-Down"]
pub type Smb3PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB4_PD` reader - SMB4 Power-Down"]
pub type Smb4PdR = crate::BitReader;
#[doc = "Field `SMB4_PD` writer - SMB4 Power-Down"]
pub type Smb4PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA2_PD` reader - GDMA2 Power-Down"]
pub type Gdma2PdR = crate::BitReader;
#[doc = "Field `GDMA2_PD` writer - GDMA2 Power-Down"]
pub type Gdma2PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA1_PD` reader - GDMA1 Power-Down"]
pub type Gdma1PdR = crate::BitReader;
#[doc = "Field `GDMA1_PD` writer - GDMA1 Power-Down"]
pub type Gdma1PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SMB0 Power-Down"]
    #[inline(always)]
    pub fn smb0_pd(&self) -> Smb0PdR {
        Smb0PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMB1 Power-Down"]
    #[inline(always)]
    pub fn smb1_pd(&self) -> Smb1PdR {
        Smb1PdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMB2 Power-Down"]
    #[inline(always)]
    pub fn smb2_pd(&self) -> Smb2PdR {
        Smb2PdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMB3 Power-Down"]
    #[inline(always)]
    pub fn smb3_pd(&self) -> Smb3PdR {
        Smb3PdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMB4 Power-Down"]
    #[inline(always)]
    pub fn smb4_pd(&self) -> Smb4PdR {
        Smb4PdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GDMA2 Power-Down"]
    #[inline(always)]
    pub fn gdma2_pd(&self) -> Gdma2PdR {
        Gdma2PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - GDMA1 Power-Down"]
    #[inline(always)]
    pub fn gdma1_pd(&self) -> Gdma1PdR {
        Gdma1PdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDWN_CTL3")
            .field("smb0_pd", &self.smb0_pd())
            .field("smb1_pd", &self.smb1_pd())
            .field("smb2_pd", &self.smb2_pd())
            .field("smb3_pd", &self.smb3_pd())
            .field("smb4_pd", &self.smb4_pd())
            .field("gdma2_pd", &self.gdma2_pd())
            .field("gdma1_pd", &self.gdma1_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SMB0 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn smb0_pd(&mut self) -> Smb0PdW<PwdwnCtl3Spec> {
        Smb0PdW::new(self, 0)
    }
    #[doc = "Bit 1 - SMB1 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn smb1_pd(&mut self) -> Smb1PdW<PwdwnCtl3Spec> {
        Smb1PdW::new(self, 1)
    }
    #[doc = "Bit 2 - SMB2 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn smb2_pd(&mut self) -> Smb2PdW<PwdwnCtl3Spec> {
        Smb2PdW::new(self, 2)
    }
    #[doc = "Bit 3 - SMB3 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn smb3_pd(&mut self) -> Smb3PdW<PwdwnCtl3Spec> {
        Smb3PdW::new(self, 3)
    }
    #[doc = "Bit 4 - SMB4 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn smb4_pd(&mut self) -> Smb4PdW<PwdwnCtl3Spec> {
        Smb4PdW::new(self, 4)
    }
    #[doc = "Bit 5 - GDMA2 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn gdma2_pd(&mut self) -> Gdma2PdW<PwdwnCtl3Spec> {
        Gdma2PdW::new(self, 5)
    }
    #[doc = "Bit 7 - GDMA1 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn gdma1_pd(&mut self) -> Gdma1PdW<PwdwnCtl3Spec> {
        Gdma1PdW::new(self, 7)
    }
}
#[doc = "Power-Down Control 3 Register (PWDWN_CTL3)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdwnCtl3Spec;
impl crate::RegisterSpec for PwdwnCtl3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwdwn_ctl3::R`](R) reader structure"]
impl crate::Readable for PwdwnCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`pwdwn_ctl3::W`](W) writer structure"]
impl crate::Writable for PwdwnCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWDWN_CTL3 to value 0"]
impl crate::Resettable for PwdwnCtl3Spec {
    const RESET_VALUE: u8 = 0;
}
