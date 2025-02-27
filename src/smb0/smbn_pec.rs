#[doc = "Register `SMBnPEC` reader"]
pub type R = crate::R<SmbnPecSpec>;
#[doc = "Register `SMBnPEC` writer"]
pub type W = crate::W<SmbnPecSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SMB PEC Register (SMBnPEC)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_pec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_pec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnPecSpec;
impl crate::RegisterSpec for SmbnPecSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_pec::R`](R) reader structure"]
impl crate::Readable for SmbnPecSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_pec::W`](W) writer structure"]
impl crate::Writable for SmbnPecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnPEC to value 0"]
impl crate::Resettable for SmbnPecSpec {
    const RESET_VALUE: u8 = 0;
}
