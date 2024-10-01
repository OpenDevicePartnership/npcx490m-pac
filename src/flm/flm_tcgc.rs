#[doc = "Register `FLM_TCGC` reader"]
pub type R = crate::R<FlmTcgcSpec>;
#[doc = "Register `FLM_TCGC` writer"]
pub type W = crate::W<FlmTcgcSpec>;
#[doc = "Field `TCCLR` reader - Transaction Counters Clear"]
pub type TcclrR = crate::BitReader;
#[doc = "Field `TCCLR` writer - Transaction Counters Clear"]
pub type TcclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transaction Counters Clear"]
    #[inline(always)]
    pub fn tcclr(&self) -> TcclrR {
        TcclrR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_TCGC")
            .field("tcclr", &self.tcclr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transaction Counters Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcclr(&mut self) -> TcclrW<FlmTcgcSpec> {
        TcclrW::new(self, 0)
    }
}
#[doc = "FLM Transaction Counter Global Control Register (FLM_TCGC)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcgc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcgc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmTcgcSpec;
impl crate::RegisterSpec for FlmTcgcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_tcgc::R`](R) reader structure"]
impl crate::Readable for FlmTcgcSpec {}
#[doc = "`write(|w| ..)` method takes [`flm_tcgc::W`](W) writer structure"]
impl crate::Writable for FlmTcgcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_TCGC to value 0"]
impl crate::Resettable for FlmTcgcSpec {
    const RESET_VALUE: u32 = 0;
}
