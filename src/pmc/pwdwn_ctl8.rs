#[doc = "Register `PWDWN_CTL8` reader"]
pub type R = crate::R<PwdwnCtl8Spec>;
#[doc = "Register `PWDWN_CTL8` writer"]
pub type W = crate::W<PwdwnCtl8Spec>;
#[doc = "Field `I3C1_PD` reader - I3C1 Power-Down"]
pub type I3c1PdR = crate::BitReader;
#[doc = "Field `I3C1_PD` writer - I3C1 Power-Down"]
pub type I3c1PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLM_PD` reader - FLM Power-Down"]
pub type FlmPdR = crate::BitReader;
#[doc = "Field `FLM_PD` writer - FLM Power-Down"]
pub type FlmPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C2_PD` reader - I3C2 Power-Down"]
pub type I3c2PdR = crate::BitReader;
#[doc = "Field `I3C2_PD` writer - I3C2 Power-Down"]
pub type I3c2PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C3_PD` reader - I3C3 Power-Down"]
pub type I3c3PdR = crate::BitReader;
#[doc = "Field `I3C3_PD` writer - I3C3 Power-Down"]
pub type I3c3PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIU0_PD` reader - FIU0 Power-Down"]
pub type Fiu0PdR = crate::BitReader;
#[doc = "Field `FIU0_PD` writer - FIU0 Power-Down"]
pub type Fiu0PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIU1_PD` reader - FIU1 Power-Down"]
pub type Fiu1PdR = crate::BitReader;
#[doc = "Field `FIU1_PD` writer - FIU1 Power-Down"]
pub type Fiu1PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I3C1 Power-Down"]
    #[inline(always)]
    pub fn i3c1_pd(&self) -> I3c1PdR {
        I3c1PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLM Power-Down"]
    #[inline(always)]
    pub fn flm_pd(&self) -> FlmPdR {
        FlmPdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I3C2 Power-Down"]
    #[inline(always)]
    pub fn i3c2_pd(&self) -> I3c2PdR {
        I3c2PdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I3C3 Power-Down"]
    #[inline(always)]
    pub fn i3c3_pd(&self) -> I3c3PdR {
        I3c3PdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - FIU0 Power-Down"]
    #[inline(always)]
    pub fn fiu0_pd(&self) -> Fiu0PdR {
        Fiu0PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIU1 Power-Down"]
    #[inline(always)]
    pub fn fiu1_pd(&self) -> Fiu1PdR {
        Fiu1PdR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDWN_CTL8")
            .field("i3c1_pd", &self.i3c1_pd())
            .field("flm_pd", &self.flm_pd())
            .field("i3c2_pd", &self.i3c2_pd())
            .field("i3c3_pd", &self.i3c3_pd())
            .field("fiu0_pd", &self.fiu0_pd())
            .field("fiu1_pd", &self.fiu1_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I3C1 Power-Down"]
    #[inline(always)]
    pub fn i3c1_pd(&mut self) -> I3c1PdW<PwdwnCtl8Spec> {
        I3c1PdW::new(self, 0)
    }
    #[doc = "Bit 1 - FLM Power-Down"]
    #[inline(always)]
    pub fn flm_pd(&mut self) -> FlmPdW<PwdwnCtl8Spec> {
        FlmPdW::new(self, 1)
    }
    #[doc = "Bit 2 - I3C2 Power-Down"]
    #[inline(always)]
    pub fn i3c2_pd(&mut self) -> I3c2PdW<PwdwnCtl8Spec> {
        I3c2PdW::new(self, 2)
    }
    #[doc = "Bit 3 - I3C3 Power-Down"]
    #[inline(always)]
    pub fn i3c3_pd(&mut self) -> I3c3PdW<PwdwnCtl8Spec> {
        I3c3PdW::new(self, 3)
    }
    #[doc = "Bit 5 - FIU0 Power-Down"]
    #[inline(always)]
    pub fn fiu0_pd(&mut self) -> Fiu0PdW<PwdwnCtl8Spec> {
        Fiu0PdW::new(self, 5)
    }
    #[doc = "Bit 6 - FIU1 Power-Down"]
    #[inline(always)]
    pub fn fiu1_pd(&mut self) -> Fiu1PdW<PwdwnCtl8Spec> {
        Fiu1PdW::new(self, 6)
    }
}
#[doc = "Power-Down Control 8 Register (PWDWN_CTL8)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdwnCtl8Spec;
impl crate::RegisterSpec for PwdwnCtl8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwdwn_ctl8::R`](R) reader structure"]
impl crate::Readable for PwdwnCtl8Spec {}
#[doc = "`write(|w| ..)` method takes [`pwdwn_ctl8::W`](W) writer structure"]
impl crate::Writable for PwdwnCtl8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWDWN_CTL8 to value 0"]
impl crate::Resettable for PwdwnCtl8Spec {
    const RESET_VALUE: u8 = 0;
}
