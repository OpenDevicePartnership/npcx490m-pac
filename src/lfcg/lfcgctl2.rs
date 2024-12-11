#[doc = "Register `LFCGCTL2` reader"]
pub type R = crate::R<Lfcgctl2Spec>;
#[doc = "Register `LFCGCTL2` writer"]
pub type W = crate::W<Lfcgctl2Spec>;
#[doc = "Field `AUDP_EN` reader - Automatic Update Enable"]
pub type AudpEnR = crate::BitReader;
#[doc = "Field `AUDP_EN` writer - Automatic Update Enable"]
pub type AudpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPCAL` reader - Stop Calibration to External Reference"]
pub type StopcalR = crate::BitReader;
#[doc = "Field `STOPCAL` writer - Stop Calibration to External Reference"]
pub type StopcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCLKRUN` reader - Force PCI_CLK Running"]
pub type FclkrunR = crate::BitReader;
#[doc = "Field `FCLKRUN` writer - Force PCI_CLK Running"]
pub type FclkrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Automatic Update Enable"]
    #[inline(always)]
    pub fn audp_en(&self) -> AudpEnR {
        AudpEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop Calibration to External Reference"]
    #[inline(always)]
    pub fn stopcal(&self) -> StopcalR {
        StopcalR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force PCI_CLK Running"]
    #[inline(always)]
    pub fn fclkrun(&self) -> FclkrunR {
        FclkrunR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFCGCTL2")
            .field("audp_en", &self.audp_en())
            .field("stopcal", &self.stopcal())
            .field("fclkrun", &self.fclkrun())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Automatic Update Enable"]
    #[inline(always)]
    pub fn audp_en(&mut self) -> AudpEnW<Lfcgctl2Spec> {
        AudpEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Stop Calibration to External Reference"]
    #[inline(always)]
    pub fn stopcal(&mut self) -> StopcalW<Lfcgctl2Spec> {
        StopcalW::new(self, 4)
    }
    #[doc = "Bit 5 - Force PCI_CLK Running"]
    #[inline(always)]
    pub fn fclkrun(&mut self) -> FclkrunW<Lfcgctl2Spec> {
        FclkrunW::new(self, 5)
    }
}
#[doc = "LFCG Control 2 Register (LFCGCTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfcgctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfcgctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfcgctl2Spec;
impl crate::RegisterSpec for Lfcgctl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lfcgctl2::R`](R) reader structure"]
impl crate::Readable for Lfcgctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`lfcgctl2::W`](W) writer structure"]
impl crate::Writable for Lfcgctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LFCGCTL2 to value 0"]
impl crate::Resettable for Lfcgctl2Spec {
    const RESET_VALUE: u8 = 0;
}
