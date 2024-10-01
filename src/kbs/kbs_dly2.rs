#[doc = "Register `KBS_DLY2` reader"]
pub type R = crate::R<KbsDly2Spec>;
#[doc = "Register `KBS_DLY2` writer"]
pub type W = crate::W<KbsDly2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Keyboard Scan Delay 2 Register (KBS_DLY2)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_dly2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_dly2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsDly2Spec;
impl crate::RegisterSpec for KbsDly2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbs_dly2::R`](R) reader structure"]
impl crate::Readable for KbsDly2Spec {}
#[doc = "`write(|w| ..)` method takes [`kbs_dly2::W`](W) writer structure"]
impl crate::Writable for KbsDly2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBS_DLY2 to value 0"]
impl crate::Resettable for KbsDly2Spec {
    const RESET_VALUE: u8 = 0;
}
