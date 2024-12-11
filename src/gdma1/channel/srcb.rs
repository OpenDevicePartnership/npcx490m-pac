#[doc = "Register `SRCB` reader"]
pub type R = crate::R<SrcbSpec>;
#[doc = "Register `SRCB` writer"]
pub type W = crate::W<SrcbSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`srcb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcbSpec;
impl crate::RegisterSpec for SrcbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcb::R`](R) reader structure"]
impl crate::Readable for SrcbSpec {}
#[doc = "`write(|w| ..)` method takes [`srcb::W`](W) writer structure"]
impl crate::Writable for SrcbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCB to value 0"]
impl crate::Resettable for SrcbSpec {
    const RESET_VALUE: u32 = 0;
}
