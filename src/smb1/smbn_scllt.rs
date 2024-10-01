#[doc = "Register `SMBnSCLLT` reader"]
pub type R = crate::R<SmbnSclltSpec>;
#[doc = "Register `SMBnSCLLT` writer"]
pub type W = crate::W<SmbnSclltSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SMB SCL Low Time Register (SMBnSCLLT)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_scllt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_scllt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnSclltSpec;
impl crate::RegisterSpec for SmbnSclltSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_scllt::R`](R) reader structure"]
impl crate::Readable for SmbnSclltSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_scllt::W`](W) writer structure"]
impl crate::Writable for SmbnSclltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnSCLLT to value 0"]
impl crate::Resettable for SmbnSclltSpec {
    const RESET_VALUE: u8 = 0;
}
