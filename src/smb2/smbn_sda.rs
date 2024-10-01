#[doc = "Register `SMBnSDA` reader"]
pub type R = crate::R<SmbnSdaSpec>;
#[doc = "Register `SMBnSDA` writer"]
pub type W = crate::W<SmbnSdaSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SMB Serial Data Register (SMBnSDA)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_sda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_sda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnSdaSpec;
impl crate::RegisterSpec for SmbnSdaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_sda::R`](R) reader structure"]
impl crate::Readable for SmbnSdaSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_sda::W`](W) writer structure"]
impl crate::Writable for SmbnSdaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnSDA to value 0"]
impl crate::Resettable for SmbnSdaSpec {
    const RESET_VALUE: u8 = 0;
}
