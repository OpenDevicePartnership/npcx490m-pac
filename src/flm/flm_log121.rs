#[doc = "Register `FLM_LOG1-21` reader"]
pub type R = crate::R<FlmLog121Spec>;
#[doc = "Register `FLM_LOG1-21` writer"]
pub type W = crate::W<FlmLog121Spec>;
#[doc = "Field `RJ_ADDR` reader - Rejection Address"]
pub type RjAddrR = crate::FieldReader<u32>;
#[doc = "Field `RJ_ADDR` writer - Rejection Address"]
pub type RjAddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RJ_CMD` reader - Rejection Command Byte"]
pub type RjCmdR = crate::FieldReader;
#[doc = "Field `RJ_CMD` writer - Rejection Command Byte"]
pub type RjCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - Rejection Address"]
    #[inline(always)]
    pub fn rj_addr(&self) -> RjAddrR {
        RjAddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Rejection Command Byte"]
    #[inline(always)]
    pub fn rj_cmd(&self) -> RjCmdR {
        RjCmdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_LOG1-21")
            .field("rj_addr", &self.rj_addr())
            .field("rj_cmd", &self.rj_cmd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Rejection Address"]
    #[inline(always)]
    #[must_use]
    pub fn rj_addr(&mut self) -> RjAddrW<FlmLog121Spec> {
        RjAddrW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Rejection Command Byte"]
    #[inline(always)]
    #[must_use]
    pub fn rj_cmd(&mut self) -> RjCmdW<FlmLog121Spec> {
        RjCmdW::new(self, 24)
    }
}
#[doc = "FLM LOG Register 1-2 (FLM_LOG1-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_log121::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_log121::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmLog121Spec;
impl crate::RegisterSpec for FlmLog121Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_log121::R`](R) reader structure"]
impl crate::Readable for FlmLog121Spec {}
#[doc = "`write(|w| ..)` method takes [`flm_log121::W`](W) writer structure"]
impl crate::Writable for FlmLog121Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_LOG1-21 to value 0"]
impl crate::Resettable for FlmLog121Spec {
    const RESET_VALUE: u32 = 0;
}
