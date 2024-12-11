#[doc = "Register `KBS_CFG_INDX` reader"]
pub type R = crate::R<KbsCfgIndxSpec>;
#[doc = "Register `KBS_CFG_INDX` writer"]
pub type W = crate::W<KbsCfgIndxSpec>;
#[doc = "Field `KBS_CFG_INDX` reader - Keyboard Scan Configuration Index"]
pub type KbsCfgIndxR = crate::FieldReader;
#[doc = "Field `KBS_CFG_INDX` writer - Keyboard Scan Configuration Index"]
pub type KbsCfgIndxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Keyboard Scan Configuration Index"]
    #[inline(always)]
    pub fn kbs_cfg_indx(&self) -> KbsCfgIndxR {
        KbsCfgIndxR::new(self.bits & 0x1f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KBS_CFG_INDX")
            .field("kbs_cfg_indx", &self.kbs_cfg_indx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Keyboard Scan Configuration Index"]
    #[inline(always)]
    pub fn kbs_cfg_indx(&mut self) -> KbsCfgIndxW<KbsCfgIndxSpec> {
        KbsCfgIndxW::new(self, 0)
    }
}
#[doc = "Keyboard Scan Configuration Index Register (KBS_CFG_INDX)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_cfg_indx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_cfg_indx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsCfgIndxSpec;
impl crate::RegisterSpec for KbsCfgIndxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbs_cfg_indx::R`](R) reader structure"]
impl crate::Readable for KbsCfgIndxSpec {}
#[doc = "`write(|w| ..)` method takes [`kbs_cfg_indx::W`](W) writer structure"]
impl crate::Writable for KbsCfgIndxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBS_CFG_INDX to value 0"]
impl crate::Resettable for KbsCfgIndxSpec {
    const RESET_VALUE: u8 = 0;
}
