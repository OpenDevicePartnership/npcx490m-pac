#[doc = "Register `SMBnSCLHT` reader"]
pub type R = crate::R<SmbnSclhtSpec>;
#[doc = "Register `SMBnSCLHT` writer"]
pub type W = crate::W<SmbnSclhtSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SMB SCL High Time Register (SMBnSCLHT)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_sclht::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_sclht::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnSclhtSpec;
impl crate::RegisterSpec for SmbnSclhtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_sclht::R`](R) reader structure"]
impl crate::Readable for SmbnSclhtSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_sclht::W`](W) writer structure"]
impl crate::Writable for SmbnSclhtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnSCLHT to value 0"]
impl crate::Resettable for SmbnSclhtSpec {
    const RESET_VALUE: u8 = 0;
}
