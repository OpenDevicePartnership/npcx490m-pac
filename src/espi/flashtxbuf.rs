#[doc = "Register `FLASHTXBUF%s` reader"]
pub type R = crate::R<FlashtxbufSpec>;
#[doc = "Register `FLASHTXBUF%s` writer"]
pub type W = crate::W<FlashtxbufSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash transmit buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashtxbufSpec;
impl crate::RegisterSpec for FlashtxbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashtxbuf::R`](R) reader structure"]
impl crate::Readable for FlashtxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`flashtxbuf::W`](W) writer structure"]
impl crate::Writable for FlashtxbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHTXBUF%s to value 0"]
impl crate::Resettable for FlashtxbufSpec {
    const RESET_VALUE: u32 = 0;
}
