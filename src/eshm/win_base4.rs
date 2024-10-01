#[doc = "Register `WIN_BASE4` reader"]
pub type R = crate::R<WinBase4Spec>;
#[doc = "Register `WIN_BASE4` writer"]
pub type W = crate::W<WinBase4Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Shared Access Window 4 Base Register (WIN_BASE4)\n\nYou can [`read`](crate::Reg::read) this register and get [`win_base4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win_base4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinBase4Spec;
impl crate::RegisterSpec for WinBase4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win_base4::R`](R) reader structure"]
impl crate::Readable for WinBase4Spec {}
#[doc = "`write(|w| ..)` method takes [`win_base4::W`](W) writer structure"]
impl crate::Writable for WinBase4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN_BASE4 to value 0"]
impl crate::Resettable for WinBase4Spec {
    const RESET_VALUE: u32 = 0;
}
