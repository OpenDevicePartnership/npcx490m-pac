#[doc = "Register `FLM_LOG1` reader"]
pub type R = crate::R<FlmLog1Spec>;
#[doc = "Field `RJ_ADDR` reader - Rejection Address"]
pub type RjAddrR = crate::FieldReader<u32>;
#[doc = "Field `RJ_CMD` reader - Rejection Command Byte"]
pub type RjCmdR = crate::FieldReader;
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
        f.debug_struct("FLM_LOG1")
            .field("rj_addr", &self.rj_addr())
            .field("rj_cmd", &self.rj_cmd())
            .finish()
    }
}
#[doc = "FLM LOG Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_log1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmLog1Spec;
impl crate::RegisterSpec for FlmLog1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_log1::R`](R) reader structure"]
impl crate::Readable for FlmLog1Spec {}
#[doc = "`reset()` method sets FLM_LOG1 to value 0"]
impl crate::Resettable for FlmLog1Spec {
    const RESET_VALUE: u32 = 0;
}
