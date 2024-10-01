#[doc = "Register `JEN_CTL2` reader"]
pub type R = crate::R<JenCtl2Spec>;
#[doc = "Register `JEN_CTL2` writer"]
pub type W = crate::W<JenCtl2Spec>;
#[doc = "Field `CCDEV_SEL_EN` reader - CCDEV Select Enable"]
pub type CcdevSelEnR = crate::FieldReader;
#[doc = "Field `CCDEV_SEL_EN` writer - CCDEV Select Enable"]
pub type CcdevSelEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `JEN1_SEL` reader - JTAG 1 Select"]
pub type Jen1SelR = crate::BitReader;
#[doc = "Field `JEN1_SEL` writer - JTAG 1 Select"]
pub type Jen1SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JENFULL_SEL` reader - JTAG Full Set Select"]
pub type JenfullSelR = crate::BitReader;
#[doc = "Field `JENFULL_SEL` writer - JTAG Full Set Select"]
pub type JenfullSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBU_SEL` reader - SBU Signals Connection Select"]
pub type SbuSelR = crate::BitReader;
#[doc = "Field `SBU_SEL` writer - SBU Signals Connection Select"]
pub type SbuSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - CCDEV Select Enable"]
    #[inline(always)]
    pub fn ccdev_sel_en(&self) -> CcdevSelEnR {
        CcdevSelEnR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - JTAG 1 Select"]
    #[inline(always)]
    pub fn jen1_sel(&self) -> Jen1SelR {
        Jen1SelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JTAG Full Set Select"]
    #[inline(always)]
    pub fn jenfull_sel(&self) -> JenfullSelR {
        JenfullSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - SBU Signals Connection Select"]
    #[inline(always)]
    pub fn sbu_sel(&self) -> SbuSelR {
        SbuSelR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JEN_CTL2")
            .field("ccdev_sel_en", &self.ccdev_sel_en())
            .field("jen1_sel", &self.jen1_sel())
            .field("jenfull_sel", &self.jenfull_sel())
            .field("sbu_sel", &self.sbu_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - CCDEV Select Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccdev_sel_en(&mut self) -> CcdevSelEnW<JenCtl2Spec> {
        CcdevSelEnW::new(self, 0)
    }
    #[doc = "Bit 4 - JTAG 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn jen1_sel(&mut self) -> Jen1SelW<JenCtl2Spec> {
        Jen1SelW::new(self, 4)
    }
    #[doc = "Bit 5 - JTAG Full Set Select"]
    #[inline(always)]
    #[must_use]
    pub fn jenfull_sel(&mut self) -> JenfullSelW<JenCtl2Spec> {
        JenfullSelW::new(self, 5)
    }
    #[doc = "Bit 7 - SBU Signals Connection Select"]
    #[inline(always)]
    #[must_use]
    pub fn sbu_sel(&mut self) -> SbuSelW<JenCtl2Spec> {
        SbuSelW::new(self, 7)
    }
}
#[doc = "JTAG Enable Control 2 Register (JEN_CTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`jen_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jen_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JenCtl2Spec;
impl crate::RegisterSpec for JenCtl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`jen_ctl2::R`](R) reader structure"]
impl crate::Readable for JenCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`jen_ctl2::W`](W) writer structure"]
impl crate::Writable for JenCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets JEN_CTL2 to value 0"]
impl crate::Resettable for JenCtl2Spec {
    const RESET_VALUE: u8 = 0;
}
