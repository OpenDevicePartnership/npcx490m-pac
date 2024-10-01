#[doc = "Register `PWDWN_CTL9` reader"]
pub type R = crate::R<PwdwnCtl9Spec>;
#[doc = "Register `PWDWN_CTL9` writer"]
pub type W = crate::W<PwdwnCtl9Spec>;
#[doc = "Field `MDMA1_PD` reader - MDMA1 Power-Down"]
pub type Mdma1PdR = crate::BitReader;
#[doc = "Field `MDMA1_PD` writer - MDMA1 Power-Down"]
pub type Mdma1PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA2_PD` reader - MDMA2 Power-Down"]
pub type Mdma2PdR = crate::BitReader;
#[doc = "Field `MDMA2_PD` writer - MDMA2 Power-Down"]
pub type Mdma2PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA3_PD` reader - MDMA3 Power-Down"]
pub type Mdma3PdR = crate::BitReader;
#[doc = "Field `MDMA3_PD` writer - MDMA3 Power-Down"]
pub type Mdma3PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA4_PD` reader - MDMA4 Power-Down"]
pub type Mdma4PdR = crate::BitReader;
#[doc = "Field `MDMA4_PD` writer - MDMA4 Power-Down"]
pub type Mdma4PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA5_PD` reader - MDMA5 Power-Down"]
pub type Mdma5PdR = crate::BitReader;
#[doc = "Field `MDMA5_PD` writer - MDMA5 Power-Down"]
pub type Mdma5PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA6_PD` reader - MDMA6 Power-Down"]
pub type Mdma6PdR = crate::BitReader;
#[doc = "Field `MDMA6_PD` writer - MDMA6 Power-Down"]
pub type Mdma6PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA7_PD` reader - MDMA7 Power-Down"]
pub type Mdma7PdR = crate::BitReader;
#[doc = "Field `MDMA7_PD` writer - MDMA7 Power-Down"]
pub type Mdma7PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDMA1 Power-Down"]
    #[inline(always)]
    pub fn mdma1_pd(&self) -> Mdma1PdR {
        Mdma1PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MDMA2 Power-Down"]
    #[inline(always)]
    pub fn mdma2_pd(&self) -> Mdma2PdR {
        Mdma2PdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MDMA3 Power-Down"]
    #[inline(always)]
    pub fn mdma3_pd(&self) -> Mdma3PdR {
        Mdma3PdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MDMA4 Power-Down"]
    #[inline(always)]
    pub fn mdma4_pd(&self) -> Mdma4PdR {
        Mdma4PdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MDMA5 Power-Down"]
    #[inline(always)]
    pub fn mdma5_pd(&self) -> Mdma5PdR {
        Mdma5PdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDMA6 Power-Down"]
    #[inline(always)]
    pub fn mdma6_pd(&self) -> Mdma6PdR {
        Mdma6PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MDMA7 Power-Down"]
    #[inline(always)]
    pub fn mdma7_pd(&self) -> Mdma7PdR {
        Mdma7PdR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDWN_CTL9")
            .field("mdma1_pd", &self.mdma1_pd())
            .field("mdma2_pd", &self.mdma2_pd())
            .field("mdma3_pd", &self.mdma3_pd())
            .field("mdma4_pd", &self.mdma4_pd())
            .field("mdma5_pd", &self.mdma5_pd())
            .field("mdma6_pd", &self.mdma6_pd())
            .field("mdma7_pd", &self.mdma7_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MDMA1 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn mdma1_pd(&mut self) -> Mdma1PdW<PwdwnCtl9Spec> {
        Mdma1PdW::new(self, 0)
    }
    #[doc = "Bit 1 - MDMA2 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn mdma2_pd(&mut self) -> Mdma2PdW<PwdwnCtl9Spec> {
        Mdma2PdW::new(self, 1)
    }
    #[doc = "Bit 2 - MDMA3 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn mdma3_pd(&mut self) -> Mdma3PdW<PwdwnCtl9Spec> {
        Mdma3PdW::new(self, 2)
    }
    #[doc = "Bit 3 - MDMA4 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn mdma4_pd(&mut self) -> Mdma4PdW<PwdwnCtl9Spec> {
        Mdma4PdW::new(self, 3)
    }
    #[doc = "Bit 4 - MDMA5 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn mdma5_pd(&mut self) -> Mdma5PdW<PwdwnCtl9Spec> {
        Mdma5PdW::new(self, 4)
    }
    #[doc = "Bit 5 - MDMA6 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn mdma6_pd(&mut self) -> Mdma6PdW<PwdwnCtl9Spec> {
        Mdma6PdW::new(self, 5)
    }
    #[doc = "Bit 6 - MDMA7 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn mdma7_pd(&mut self) -> Mdma7PdW<PwdwnCtl9Spec> {
        Mdma7PdW::new(self, 6)
    }
}
#[doc = "Power-Down Control 9 Register (PWDWN_CTL9)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdwnCtl9Spec;
impl crate::RegisterSpec for PwdwnCtl9Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwdwn_ctl9::R`](R) reader structure"]
impl crate::Readable for PwdwnCtl9Spec {}
#[doc = "`write(|w| ..)` method takes [`pwdwn_ctl9::W`](W) writer structure"]
impl crate::Writable for PwdwnCtl9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWDWN_CTL9 to value 0"]
impl crate::Resettable for PwdwnCtl9Spec {
    const RESET_VALUE: u8 = 0;
}
