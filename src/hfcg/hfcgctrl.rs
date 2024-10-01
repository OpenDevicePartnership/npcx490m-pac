#[doc = "Register `HFCGCTRL` reader"]
pub type R = crate::R<HfcgctrlSpec>;
#[doc = "Register `HFCGCTRL` writer"]
pub type W = crate::W<HfcgctrlSpec>;
#[doc = "Field `LOAD` reader - Load M and N Values"]
pub type LoadR = crate::BitReader;
#[doc = "Field `LOAD` writer - Load M and N Values"]
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Disable Writing to all HFCG registers"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Disable Writing to all HFCG registers"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CHNG` reader - Clock Changing"]
pub type ClkChngR = crate::BitReader;
#[doc = "Field `CLK_CHNG` writer - Clock Changing"]
pub type ClkChngW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Load M and N Values"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Writing to all HFCG registers"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Changing"]
    #[inline(always)]
    pub fn clk_chng(&self) -> ClkChngR {
        ClkChngR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCGCTRL")
            .field("load", &self.load())
            .field("lock", &self.lock())
            .field("clk_chng", &self.clk_chng())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Load M and N Values"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LoadW<HfcgctrlSpec> {
        LoadW::new(self, 0)
    }
    #[doc = "Bit 2 - Disable Writing to all HFCG registers"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<HfcgctrlSpec> {
        LockW::new(self, 2)
    }
    #[doc = "Bit 7 - Clock Changing"]
    #[inline(always)]
    #[must_use]
    pub fn clk_chng(&mut self) -> ClkChngW<HfcgctrlSpec> {
        ClkChngW::new(self, 7)
    }
}
#[doc = "HFCG Control Register (HFCGCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfcgctrlSpec;
impl crate::RegisterSpec for HfcgctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcgctrl::R`](R) reader structure"]
impl crate::Readable for HfcgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfcgctrl::W`](W) writer structure"]
impl crate::Writable for HfcgctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCGCTRL to value 0"]
impl crate::Resettable for HfcgctrlSpec {
    const RESET_VALUE: u8 = 0;
}
