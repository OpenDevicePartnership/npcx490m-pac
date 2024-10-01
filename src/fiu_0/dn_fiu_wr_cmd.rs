#[doc = "Register `Dn_FIU_WR_CMD` reader"]
pub type R = crate::R<DnFiuWrCmdSpec>;
#[doc = "Register `Dn_FIU_WR_CMD` writer"]
pub type W = crate::W<DnFiuWrCmdSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Device 'n' FIU Write Command Register (Dn_FIU_WR_CMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`dn_fiu_wr_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dn_fiu_wr_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DnFiuWrCmdSpec;
impl crate::RegisterSpec for DnFiuWrCmdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dn_fiu_wr_cmd::R`](R) reader structure"]
impl crate::Readable for DnFiuWrCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`dn_fiu_wr_cmd::W`](W) writer structure"]
impl crate::Writable for DnFiuWrCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets Dn_FIU_WR_CMD to value 0"]
impl crate::Resettable for DnFiuWrCmdSpec {
    const RESET_VALUE: u8 = 0;
}
