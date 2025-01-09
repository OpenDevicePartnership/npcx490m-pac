#[doc = "Register `HFCGN` reader"]
pub type R = crate::R<HfcgnSpec>;
#[doc = "Register `HFCGN` writer"]
pub type W = crate::W<HfcgnSpec>;
#[doc = "Field `HFCGN5_0` reader - N Value Bits 5-0"]
pub type Hfcgn5_0R = crate::FieldReader;
#[doc = "Field `HFCGN5_0` writer - N Value Bits 5-0"]
pub type Hfcgn5_0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `40M_EN` reader - 40 MHz Enable"]
pub type _40mEnR = crate::BitReader;
#[doc = "Field `40M_EN` writer - 40 MHz Enable"]
pub type _40mEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XF_RANGE` reader - Extended Frequency Range"]
pub type XfRangeR = crate::BitReader;
#[doc = "Field `XF_RANGE` writer - Extended Frequency Range"]
pub type XfRangeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - N Value Bits 5-0"]
    #[inline(always)]
    pub fn hfcgn5_0(&self) -> Hfcgn5_0R {
        Hfcgn5_0R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - 40 MHz Enable"]
    #[inline(always)]
    pub fn _40m_en(&self) -> _40mEnR {
        _40mEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Extended Frequency Range"]
    #[inline(always)]
    pub fn xf_range(&self) -> XfRangeR {
        XfRangeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCGN")
            .field("hfcgn5_0", &self.hfcgn5_0())
            .field("_40m_en", &self._40m_en())
            .field("xf_range", &self.xf_range())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - N Value Bits 5-0"]
    #[inline(always)]
    pub fn hfcgn5_0(&mut self) -> Hfcgn5_0W<HfcgnSpec> {
        Hfcgn5_0W::new(self, 0)
    }
    #[doc = "Bit 6 - 40 MHz Enable"]
    #[inline(always)]
    pub fn _40m_en(&mut self) -> _40mEnW<HfcgnSpec> {
        _40mEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Extended Frequency Range"]
    #[inline(always)]
    pub fn xf_range(&mut self) -> XfRangeW<HfcgnSpec> {
        XfRangeW::new(self, 7)
    }
}
#[doc = "HFCG N Value Register (HFCGN)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfcgnSpec;
impl crate::RegisterSpec for HfcgnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcgn::R`](R) reader structure"]
impl crate::Readable for HfcgnSpec {}
#[doc = "`write(|w| ..)` method takes [`hfcgn::W`](W) writer structure"]
impl crate::Writable for HfcgnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCGN to value 0x02"]
impl crate::Resettable for HfcgnSpec {
    const RESET_VALUE: u8 = 0x02;
}
