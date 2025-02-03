#[doc = "Register `PSMRXBUF%s` reader"]
pub type R = crate::R<PsmrxbufSpec>;
#[doc = "Register `PSMRXBUF%s` writer"]
pub type W = crate::W<PsmrxbufSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsmrxbufSpec;
impl crate::RegisterSpec for PsmrxbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psmrxbuf::R`](R) reader structure"]
impl crate::Readable for PsmrxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`psmrxbuf::W`](W) writer structure"]
impl crate::Writable for PsmrxbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSMRXBUF%s to value 0"]
impl crate::Resettable for PsmrxbufSpec {
    const RESET_VALUE: u32 = 0;
}
