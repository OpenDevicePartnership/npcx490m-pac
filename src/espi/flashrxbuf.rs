#[doc = "Register `FLASHRXBUF%s` reader"]
pub type R = crate::R<FlashrxbufSpec>;
#[doc = "Register `FLASHRXBUF%s` writer"]
pub type W = crate::W<FlashrxbufSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashrxbufSpec;
impl crate::RegisterSpec for FlashrxbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashrxbuf::R`](R) reader structure"]
impl crate::Readable for FlashrxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`flashrxbuf::W`](W) writer structure"]
impl crate::Writable for FlashrxbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHRXBUF%s to value 0"]
impl crate::Resettable for FlashrxbufSpec {
    const RESET_VALUE: u32 = 0;
}
