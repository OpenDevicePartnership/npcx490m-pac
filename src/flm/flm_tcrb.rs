#[doc = "Register `FLM_TCRB%s` reader"]
pub type R = crate::R<FlmTcrbSpec>;
#[doc = "Field `TCV` reader - Transaction Counter Value"]
pub type TcvR = crate::FieldReader<u32>;
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
        f.debug_struct("FLM_TCRB")
            .field("tcv", &self.tcv())
            .finish()
    }
}
#[doc = "FLM Transaction Counter Read Register B%s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcrb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmTcrbSpec;
impl crate::RegisterSpec for FlmTcrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_tcrb::R`](R) reader structure"]
impl crate::Readable for FlmTcrbSpec {}
#[doc = "`reset()` method sets FLM_TCRB%s to value 0"]
impl crate::Resettable for FlmTcrbSpec {
    const RESET_VALUE: u32 = 0;
}
