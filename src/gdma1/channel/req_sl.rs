#[doc = "Register `REQ_SL` reader"]
pub type R = crate::R<ReqSlSpec>;
#[doc = "Register `REQ_SL` writer"]
pub type W = crate::W<ReqSlSpec>;
#[doc = "Field `DREQ_SL` reader - DMA Request Select"]
pub type DreqSlR = crate::FieldReader<u16>;
#[doc = "Field `DREQ_SL` writer - DMA Request Select"]
pub type DreqSlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA Request Select"]
    #[inline(always)]
    pub fn dreq_sl(&self) -> DreqSlR {
        DreqSlR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REQ_SL")
            .field("dreq_sl", &self.dreq_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA Request Select"]
    #[inline(always)]
    pub fn dreq_sl(&mut self) -> DreqSlW<ReqSlSpec> {
        DreqSlW::new(self, 0)
    }
}
#[doc = "Channel 0/1 GDMA Request Select Register (GDMAn_REQ_SL0, GDMAn_REQ_SL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`req_sl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`req_sl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqSlSpec;
impl crate::RegisterSpec for ReqSlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`req_sl::R`](R) reader structure"]
impl crate::Readable for ReqSlSpec {}
#[doc = "`write(|w| ..)` method takes [`req_sl::W`](W) writer structure"]
impl crate::Writable for ReqSlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REQ_SL to value 0"]
impl crate::Resettable for ReqSlSpec {
    const RESET_VALUE: u32 = 0;
}
