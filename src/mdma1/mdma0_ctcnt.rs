#[doc = "Register `MDMA0_CTCNT` reader"]
pub type R = crate::R<Mdma0CtcntSpec>;
#[doc = "Register `MDMA0_CTCNT` writer"]
pub type W = crate::W<Mdma0CtcntSpec>;
#[doc = "Field `CURENT_TFR_CNT` reader - 13-bit Current Transfer Count"]
pub type CurentTfrCntR = crate::FieldReader<u16>;
#[doc = "Field `CURENT_TFR_CNT` writer - 13-bit Current Transfer Count"]
pub type CurentTfrCntW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 13-bit Current Transfer Count"]
    #[inline(always)]
    pub fn curent_tfr_cnt(&self) -> CurentTfrCntR {
        CurentTfrCntR::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDMA0_CTCNT")
            .field("curent_tfr_cnt", &self.curent_tfr_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - 13-bit Current Transfer Count"]
    #[inline(always)]
    pub fn curent_tfr_cnt(&mut self) -> CurentTfrCntW<Mdma0CtcntSpec> {
        CurentTfrCntW::new(self, 0)
    }
}
#[doc = "Channel 0/1 Current Transfer Count Register (MDMAn_CTCNT0, MDMAn_CTCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma0_ctcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma0_ctcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdma0CtcntSpec;
impl crate::RegisterSpec for Mdma0CtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma0_ctcnt::R`](R) reader structure"]
impl crate::Readable for Mdma0CtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma0_ctcnt::W`](W) writer structure"]
impl crate::Writable for Mdma0CtcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA0_CTCNT to value 0"]
impl crate::Resettable for Mdma0CtcntSpec {
    const RESET_VALUE: u32 = 0;
}
