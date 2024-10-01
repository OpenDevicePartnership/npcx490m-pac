#[doc = "Register `FLM_CMB0-3123` reader"]
pub type R = crate::R<FlmCmb03123Spec>;
#[doc = "Register `FLM_CMB0-3123` writer"]
pub type W = crate::W<FlmCmb03123Spec>;
#[doc = "Field `CMD` reader - Command Byte"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - Command Byte"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Command Byte"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_CMB0-3123")
            .field("cmd", &self.cmd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Command Byte"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<FlmCmb03123Spec> {
        CmdW::new(self, 0)
    }
}
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03123::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03123::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCmb03123Spec;
impl crate::RegisterSpec for FlmCmb03123Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cmb03123::R`](R) reader structure"]
impl crate::Readable for FlmCmb03123Spec {}
#[doc = "`write(|w| ..)` method takes [`flm_cmb03123::W`](W) writer structure"]
impl crate::Writable for FlmCmb03123Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CMB0-3123 to value 0"]
impl crate::Resettable for FlmCmb03123Spec {
    const RESET_VALUE: u32 = 0;
}
