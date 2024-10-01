#[doc = "Register `SWRST_TRG` reader"]
pub type R = crate::R<SwrstTrgSpec>;
#[doc = "Register `SWRST_TRG` writer"]
pub type W = crate::W<SwrstTrgSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Software Reset Trigger Register (SWRST_TRG)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_trg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_trg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstTrgSpec;
impl crate::RegisterSpec for SwrstTrgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`swrst_trg::R`](R) reader structure"]
impl crate::Readable for SwrstTrgSpec {}
#[doc = "`write(|w| ..)` method takes [`swrst_trg::W`](W) writer structure"]
impl crate::Writable for SwrstTrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SWRST_TRG to value 0"]
impl crate::Resettable for SwrstTrgSpec {
    const RESET_VALUE: u16 = 0;
}
