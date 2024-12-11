#[doc = "Register `HIPMnCTL2` reader"]
pub type R = crate::R<HipmnCtl2Spec>;
#[doc = "Register `HIPMnCTL2` writer"]
pub type W = crate::W<HipmnCtl2Spec>;
#[doc = "Field `FW_OBF` reader - Firmware Control Over OBF"]
pub type FwObfR = crate::BitReader;
#[doc = "Field `FW_OBF` writer - Firmware Control Over OBF"]
pub type FwObfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Firmware Control Over OBF"]
    #[inline(always)]
    pub fn fw_obf(&self) -> FwObfR {
        FwObfR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIPMnCTL2")
            .field("fw_obf", &self.fw_obf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Firmware Control Over OBF"]
    #[inline(always)]
    pub fn fw_obf(&mut self) -> FwObfW<HipmnCtl2Spec> {
        FwObfW::new(self, 1)
    }
}
#[doc = "Host Interface PM n Control 2 Register (HIPMnCTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HipmnCtl2Spec;
impl crate::RegisterSpec for HipmnCtl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hipmn_ctl2::R`](R) reader structure"]
impl crate::Readable for HipmnCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`hipmn_ctl2::W`](W) writer structure"]
impl crate::Writable for HipmnCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIPMnCTL2 to value 0"]
impl crate::Resettable for HipmnCtl2Spec {
    const RESET_VALUE: u8 = 0;
}
