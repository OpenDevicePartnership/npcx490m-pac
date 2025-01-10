#[doc = "Register `BL_CTL` reader"]
pub type R = crate::R<BlCtlSpec>;
#[doc = "Register `BL_CTL` writer"]
pub type W = crate::W<BlCtlSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "BootLoader Control Register (BL_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`bl_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bl_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlCtlSpec;
impl crate::RegisterSpec for BlCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bl_ctl::R`](R) reader structure"]
impl crate::Readable for BlCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`bl_ctl::W`](W) writer structure"]
impl crate::Writable for BlCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BL_CTL to value 0"]
impl crate::Resettable for BlCtlSpec {
    const RESET_VALUE: u8 = 0;
}
