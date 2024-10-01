#[doc = "Register `MDMA1_TCNT` reader"]
pub type R = crate::R<Mdma1TcntSpec>;
#[doc = "Register `MDMA1_TCNT` writer"]
pub type W = crate::W<Mdma1TcntSpec>;
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
        f.debug_struct("MDMA1_TCNT")
            .field("tfr_cnt", &self.tfr_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - 13-bit Transfer Count"]
    #[inline(always)]
    #[must_use]
    pub fn tfr_cnt(&mut self) -> TfrCntW<Mdma1TcntSpec> {
        TfrCntW::new(self, 0)
    }
}
#[doc = "Channel 0/1 Transfer Count Register (MDMAn_TCNT0, MDMAn_TCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma1_tcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma1_tcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdma1TcntSpec;
impl crate::RegisterSpec for Mdma1TcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma1_tcnt::R`](R) reader structure"]
impl crate::Readable for Mdma1TcntSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma1_tcnt::W`](W) writer structure"]
impl crate::Writable for Mdma1TcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA1_TCNT to value 0"]
impl crate::Resettable for Mdma1TcntSpec {
    const RESET_VALUE: u32 = 0;
}
