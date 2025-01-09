#[doc = "Register `HFCBCD3` reader"]
pub type R = crate::R<Hfcbcd3Spec>;
#[doc = "Register `HFCBCD3` writer"]
pub type W = crate::W<Hfcbcd3Spec>;
#[doc = "Selects the division of MCLK to generate the MCLKD clock for all I3CI modules.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MclkdSl {
    #[doc = "0: The I3CI modules MCLKD clock is MCLK divided by 1 − i.e. without division (default)."]
    Div1 = 0,
    #[doc = "1: The I3CI modules MCLKD clock is MCLK divided by 2."]
    Div2 = 1,
    #[doc = "2: The I3CI modules MCLKD clock is MCLK divided by 3."]
    Div3 = 2,
}
impl From<MclkdSl> for u8 {
    #[inline(always)]
    fn from(variant: MclkdSl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MclkdSl {
    type Ux = u8;
}
impl crate::IsEnum for MclkdSl {}
#[doc = "Field `MCLKD_SL` reader - Selects the division of MCLK to generate the MCLKD clock for all I3CI modules."]
pub type MclkdSlR = crate::FieldReader<MclkdSl>;
impl MclkdSlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MclkdSl> {
        match self.bits {
            0 => Some(MclkdSl::Div1),
            1 => Some(MclkdSl::Div2),
            2 => Some(MclkdSl::Div3),
            _ => None,
        }
    }
    #[doc = "The I3CI modules MCLKD clock is MCLK divided by 1 − i.e. without division (default)."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MclkdSl::Div1
    }
    #[doc = "The I3CI modules MCLKD clock is MCLK divided by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MclkdSl::Div2
    }
    #[doc = "The I3CI modules MCLKD clock is MCLK divided by 3."]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == MclkdSl::Div3
    }
}
#[doc = "Field `MCLKD_SL` writer - Selects the division of MCLK to generate the MCLKD clock for all I3CI modules."]
pub type MclkdSlW<'a, REG> = crate::FieldWriter<'a, REG, 2, MclkdSl>;
impl<'a, REG> MclkdSlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The I3CI modules MCLKD clock is MCLK divided by 1 − i.e. without division (default)."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(MclkdSl::Div1)
    }
    #[doc = "The I3CI modules MCLKD clock is MCLK divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(MclkdSl::Div2)
    }
    #[doc = "The I3CI modules MCLKD clock is MCLK divided by 3."]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(MclkdSl::Div3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects the division of MCLK to generate the MCLKD clock for all I3CI modules."]
    #[inline(always)]
    pub fn mclkd_sl(&self) -> MclkdSlR {
        MclkdSlR::new(self.bits & 3)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCBCD3")
            .field("mclkd_sl", &self.mclkd_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the division of MCLK to generate the MCLKD clock for all I3CI modules."]
    #[inline(always)]
    pub fn mclkd_sl(&mut self) -> MclkdSlW<Hfcbcd3Spec> {
        MclkdSlW::new(self, 0)
    }
}
#[doc = "HFCG Bus Clock Dividers 3 Register (HFCBCD3)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcbcd3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcbcd3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfcbcd3Spec;
impl crate::RegisterSpec for Hfcbcd3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcbcd3::R`](R) reader structure"]
impl crate::Readable for Hfcbcd3Spec {}
#[doc = "`write(|w| ..)` method takes [`hfcbcd3::W`](W) writer structure"]
impl crate::Writable for Hfcbcd3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCBCD3 to value 0"]
impl crate::Resettable for Hfcbcd3Spec {
    const RESET_VALUE: u8 = 0;
}
