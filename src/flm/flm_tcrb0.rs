#[doc = "Register `FLM_TCRB0` reader"]
pub type R = crate::R<FlmTcrb0Spec>;
#[doc = "Register `FLM_TCRB0` writer"]
pub type W = crate::W<FlmTcrb0Spec>;
#[doc = "Field `TCV` reader - Transaction Counter Value"]
pub type TcvR = crate::FieldReader<u32>;
#[doc = "Field `TCV` writer - Transaction Counter Value"]
pub type TcvW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Transaction Counter Value"]
    #[inline(always)]
    pub fn tcv(&self) -> TcvR {
        TcvR::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_TCRB0")
            .field("tcv", &self.tcv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Transaction Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn tcv(&mut self) -> TcvW<FlmTcrb0Spec> {
        TcvW::new(self, 0)
    }
}
#[doc = "FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcrb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcrb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmTcrb0Spec;
impl crate::RegisterSpec for FlmTcrb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_tcrb0::R`](R) reader structure"]
impl crate::Readable for FlmTcrb0Spec {}
#[doc = "`write(|w| ..)` method takes [`flm_tcrb0::W`](W) writer structure"]
impl crate::Writable for FlmTcrb0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_TCRB0 to value 0"]
impl crate::Resettable for FlmTcrb0Spec {
    const RESET_VALUE: u32 = 0;
}
