#[doc = "Register `RST_CFG` reader"]
pub type R = crate::R<RstCfgSpec>;
#[doc = "Register `RST_CFG` writer"]
pub type W = crate::W<RstCfgSpec>;
#[doc = "Field `LRESET_PLTRST_DIS` reader - LRESET or PLTRST Disable"]
pub type LresetPltrstDisR = crate::BitReader;
#[doc = "Field `LRESET_PLTRST_DIS` writer - LRESET or PLTRST Disable"]
pub type LresetPltrstDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - LRESET or PLTRST Disable"]
    #[inline(always)]
    pub fn lreset_pltrst_dis(&self) -> LresetPltrstDisR {
        LresetPltrstDisR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RST_CFG")
            .field("lreset_pltrst_dis", &self.lreset_pltrst_dis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - LRESET or PLTRST Disable"]
    #[inline(always)]
    pub fn lreset_pltrst_dis(&mut self) -> LresetPltrstDisW<RstCfgSpec> {
        LresetPltrstDisW::new(self, 7)
    }
}
#[doc = "Reset Configuration Register (RST_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstCfgSpec;
impl crate::RegisterSpec for RstCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rst_cfg::R`](R) reader structure"]
impl crate::Readable for RstCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_cfg::W`](W) writer structure"]
impl crate::Writable for RstCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RST_CFG to value 0"]
impl crate::Resettable for RstCfgSpec {
    const RESET_VALUE: u8 = 0;
}
