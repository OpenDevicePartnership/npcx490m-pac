#[doc = "Register `KBS_CFG_DATA` reader"]
pub type R = crate::R<KbsCfgDataSpec>;
#[doc = "Register `KBS_CFG_DATA` writer"]
pub type W = crate::W<KbsCfgDataSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Keyboard Scan Configuration Data Register (KBS_CFG_DATA)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_cfg_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_cfg_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsCfgDataSpec;
impl crate::RegisterSpec for KbsCfgDataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbs_cfg_data::R`](R) reader structure"]
impl crate::Readable for KbsCfgDataSpec {}
#[doc = "`write(|w| ..)` method takes [`kbs_cfg_data::W`](W) writer structure"]
impl crate::Writable for KbsCfgDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBS_CFG_DATA to value 0"]
impl crate::Resettable for KbsCfgDataSpec {
    const RESET_VALUE: u8 = 0;
}
