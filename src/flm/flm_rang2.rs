#[doc = "Register `FLM_RANG2` reader"]
pub type R = crate::R<FlmRang2Spec>;
#[doc = "Register `FLM_RANG2` writer"]
pub type W = crate::W<FlmRang2Spec>;
#[doc = "Field `STRTRANGM` reader - Range Start Address"]
pub type StrtrangmR = crate::FieldReader<u16>;
#[doc = "Field `STRTRANGM` writer - Range Start Address"]
pub type StrtrangmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LASTRANG` reader - Range Last Address"]
pub type LastrangR = crate::FieldReader<u16>;
#[doc = "Field `LASTRANG` writer - Range Last Address"]
pub type LastrangW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Range Start Address"]
    #[inline(always)]
    pub fn strtrangm(&self) -> StrtrangmR {
        StrtrangmR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Range Last Address"]
    #[inline(always)]
    pub fn lastrang(&self) -> LastrangR {
        LastrangR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_RANG2")
            .field("strtrangm", &self.strtrangm())
            .field("lastrang", &self.lastrang())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Range Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn strtrangm(&mut self) -> StrtrangmW<FlmRang2Spec> {
        StrtrangmW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Range Last Address"]
    #[inline(always)]
    #[must_use]
    pub fn lastrang(&mut self) -> LastrangW<FlmRang2Spec> {
        LastrangW::new(self, 16)
    }
}
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmRang2Spec;
impl crate::RegisterSpec for FlmRang2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_rang2::R`](R) reader structure"]
impl crate::Readable for FlmRang2Spec {}
#[doc = "`write(|w| ..)` method takes [`flm_rang2::W`](W) writer structure"]
impl crate::Writable for FlmRang2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_RANG2 to value 0"]
impl crate::Resettable for FlmRang2Spec {
    const RESET_VALUE: u32 = 0;
}
