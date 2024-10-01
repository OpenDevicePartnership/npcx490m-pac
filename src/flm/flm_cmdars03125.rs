#[doc = "Register `FLM_CMDARS0-3125` reader"]
pub type R = crate::R<FlmCmdars03125Spec>;
#[doc = "Register `FLM_CMDARS0-3125` writer"]
pub type W = crate::W<FlmCmdars03125Spec>;
#[doc = "Field `CARSEL` reader - Command Address Range Select"]
pub type CarselR = crate::FieldReader<u16>;
#[doc = "Field `CARSEL` writer - Command Address Range Select"]
pub type CarselW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Command Address Range Select"]
    #[inline(always)]
    pub fn carsel(&self) -> CarselR {
        CarselR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_CMDARS0-3125")
            .field("carsel", &self.carsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Command Address Range Select"]
    #[inline(always)]
    #[must_use]
    pub fn carsel(&mut self) -> CarselW<FlmCmdars03125Spec> {
        CarselW::new(self, 0)
    }
}
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03125::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03125::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCmdars03125Spec;
impl crate::RegisterSpec for FlmCmdars03125Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cmdars03125::R`](R) reader structure"]
impl crate::Readable for FlmCmdars03125Spec {}
#[doc = "`write(|w| ..)` method takes [`flm_cmdars03125::W`](W) writer structure"]
impl crate::Writable for FlmCmdars03125Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CMDARS0-3125 to value 0"]
impl crate::Resettable for FlmCmdars03125Spec {
    const RESET_VALUE: u32 = 0;
}
