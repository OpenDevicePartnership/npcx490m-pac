#[doc = "Register `MDMA0_TCNT` reader"]
pub type R = crate::R<Mdma0TcntSpec>;
#[doc = "Register `MDMA0_TCNT` writer"]
pub type W = crate::W<Mdma0TcntSpec>;
#[doc = "Field `TFR_CNT` reader - 13-bit Transfer Count"]
pub type TfrCntR = crate::FieldReader<u16>;
#[doc = "Field `TFR_CNT` writer - 13-bit Transfer Count"]
pub type TfrCntW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 13-bit Transfer Count"]
    #[inline(always)]
    pub fn tfr_cnt(&self) -> TfrCntR {
        TfrCntR::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDMA0_TCNT")
            .field("tfr_cnt", &self.tfr_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - 13-bit Transfer Count"]
    #[inline(always)]
    pub fn tfr_cnt(&mut self) -> TfrCntW<Mdma0TcntSpec> {
        TfrCntW::new(self, 0)
    }
}
#[doc = "Channel 0/1 Transfer Count Register (MDMAn_TCNT0, MDMAn_TCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma0_tcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma0_tcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdma0TcntSpec;
impl crate::RegisterSpec for Mdma0TcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma0_tcnt::R`](R) reader structure"]
impl crate::Readable for Mdma0TcntSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma0_tcnt::W`](W) writer structure"]
impl crate::Writable for Mdma0TcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA0_TCNT to value 0"]
impl crate::Resettable for Mdma0TcntSpec {
    const RESET_VALUE: u32 = 0;
}
