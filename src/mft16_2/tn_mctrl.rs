#[doc = "Register `TnMCTRL` reader"]
pub type R = crate::R<TnMctrlSpec>;
#[doc = "Register `TnMCTRL` writer"]
pub type W = crate::W<TnMctrlSpec>;
#[doc = "Field `MDSEL` reader - Mode Select"]
pub type MdselR = crate::FieldReader;
#[doc = "Field `MDSEL` writer - Mode Select"]
pub type MdselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TAEDG` reader - TAn Edge Polarity"]
pub type TaedgR = crate::BitReader;
#[doc = "Field `TAEDG` writer - TAn Edge Polarity"]
pub type TaedgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBEDG` reader - TBn Edge Polarity"]
pub type TbedgR = crate::BitReader;
#[doc = "Field `TBEDG` writer - TBn Edge Polarity"]
pub type TbedgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEN` reader - TAn Enable"]
pub type TaenR = crate::BitReader;
#[doc = "Field `TAEN` writer - TAn Enable"]
pub type TaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBEN` reader - TBn Enable"]
pub type TbenR = crate::BitReader;
#[doc = "Field `TBEN` writer - TBn Enable"]
pub type TbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAOUT` reader - TAn Output Data"]
pub type TaoutR = crate::BitReader;
#[doc = "Field `TAOUT` writer - TAn Output Data"]
pub type TaoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mdsel(&self) -> MdselR {
        MdselR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - TAn Edge Polarity"]
    #[inline(always)]
    pub fn taedg(&self) -> TaedgR {
        TaedgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TBn Edge Polarity"]
    #[inline(always)]
    pub fn tbedg(&self) -> TbedgR {
        TbedgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TAn Enable"]
    #[inline(always)]
    pub fn taen(&self) -> TaenR {
        TaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TBn Enable"]
    #[inline(always)]
    pub fn tben(&self) -> TbenR {
        TbenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TAn Output Data"]
    #[inline(always)]
    pub fn taout(&self) -> TaoutR {
        TaoutR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TnMCTRL")
            .field("mdsel", &self.mdsel())
            .field("taedg", &self.taedg())
            .field("tbedg", &self.tbedg())
            .field("taen", &self.taen())
            .field("tben", &self.tben())
            .field("taout", &self.taout())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mdsel(&mut self) -> MdselW<TnMctrlSpec> {
        MdselW::new(self, 0)
    }
    #[doc = "Bit 3 - TAn Edge Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn taedg(&mut self) -> TaedgW<TnMctrlSpec> {
        TaedgW::new(self, 3)
    }
    #[doc = "Bit 4 - TBn Edge Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tbedg(&mut self) -> TbedgW<TnMctrlSpec> {
        TbedgW::new(self, 4)
    }
    #[doc = "Bit 5 - TAn Enable"]
    #[inline(always)]
    #[must_use]
    pub fn taen(&mut self) -> TaenW<TnMctrlSpec> {
        TaenW::new(self, 5)
    }
    #[doc = "Bit 6 - TBn Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tben(&mut self) -> TbenW<TnMctrlSpec> {
        TbenW::new(self, 6)
    }
    #[doc = "Bit 7 - TAn Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn taout(&mut self) -> TaoutW<TnMctrlSpec> {
        TaoutW::new(self, 7)
    }
}
#[doc = "Timer Mode Control Register (TnMCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_mctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_mctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnMctrlSpec;
impl crate::RegisterSpec for TnMctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tn_mctrl::R`](R) reader structure"]
impl crate::Readable for TnMctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_mctrl::W`](W) writer structure"]
impl crate::Writable for TnMctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TnMCTRL to value 0"]
impl crate::Resettable for TnMctrlSpec {
    const RESET_VALUE: u8 = 0;
}
