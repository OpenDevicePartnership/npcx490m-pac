#[doc = "Register `JEN_CTL1` reader"]
pub type R = crate::R<JenCtl1Spec>;
#[doc = "Register `JEN_CTL1` writer"]
pub type W = crate::W<JenCtl1Spec>;
#[doc = "Field `JEN_EN` reader - JTAG Enable"]
pub type JenEnR = crate::FieldReader;
#[doc = "Field `JEN_EN` writer - JTAG Enable"]
pub type JenEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - JTAG Enable"]
    #[inline(always)]
    pub fn jen_en(&self) -> JenEnR {
        JenEnR::new(self.bits & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JEN_CTL1")
            .field("jen_en", &self.jen_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - JTAG Enable"]
    #[inline(always)]
    #[must_use]
    pub fn jen_en(&mut self) -> JenEnW<JenCtl1Spec> {
        JenEnW::new(self, 0)
    }
}
#[doc = "JTAG Enable Control 1 Register (JEN_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`jen_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jen_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JenCtl1Spec;
impl crate::RegisterSpec for JenCtl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`jen_ctl1::R`](R) reader structure"]
impl crate::Readable for JenCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`jen_ctl1::W`](W) writer structure"]
impl crate::Writable for JenCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets JEN_CTL1 to value 0"]
impl crate::Resettable for JenCtl1Spec {
    const RESET_VALUE: u8 = 0;
}
