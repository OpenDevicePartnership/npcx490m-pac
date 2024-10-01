#[doc = "Register `KBS_CDIV` reader"]
pub type R = crate::R<KbsCdivSpec>;
#[doc = "Register `KBS_CDIV` writer"]
pub type W = crate::W<KbsCdivSpec>;
#[doc = "Field `KBS_CLK_DIV` reader - Keyboard Scan Clock Divisor"]
pub type KbsClkDivR = crate::FieldReader;
#[doc = "Field `KBS_CLK_DIV` writer - Keyboard Scan Clock Divisor"]
pub type KbsClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Keyboard Scan Clock Divisor"]
    #[inline(always)]
    pub fn kbs_clk_div(&self) -> KbsClkDivR {
        KbsClkDivR::new(self.bits & 0x3f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KBS_CDIV")
            .field("kbs_clk_div", &self.kbs_clk_div())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Keyboard Scan Clock Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn kbs_clk_div(&mut self) -> KbsClkDivW<KbsCdivSpec> {
        KbsClkDivW::new(self, 0)
    }
}
#[doc = "Keyboard Scan Clock Divisor Register (KBS_CDIV)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_cdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_cdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsCdivSpec;
impl crate::RegisterSpec for KbsCdivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbs_cdiv::R`](R) reader structure"]
impl crate::Readable for KbsCdivSpec {}
#[doc = "`write(|w| ..)` method takes [`kbs_cdiv::W`](W) writer structure"]
impl crate::Writable for KbsCdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBS_CDIV to value 0"]
impl crate::Resettable for KbsCdivSpec {
    const RESET_VALUE: u8 = 0;
}
