#[doc = "Register `LFCGCTL2` reader"]
pub type R = crate::R<Lfcgctl2Spec>;
#[doc = "Register `LFCGCTL2` writer"]
pub type W = crate::W<Lfcgctl2Spec>;
#[doc = "This bit both indicates that the crystal oscillator option is supported and enables the operation of the crystal oscillator circuit. In addition, it controls the source of the XTCLK signal.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XtOsc {
    #[doc = "0: The crystal oscillator is not supported or disabled"]
    NotSupportedOrDisabled = 0,
    #[doc = "1: The crystal oscillator is supported and enabled"]
    SupportedAndEnabled = 1,
}
impl From<XtOsc> for bool {
    #[inline(always)]
    fn from(variant: XtOsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XT_OSC` reader - This bit both indicates that the crystal oscillator option is supported and enables the operation of the crystal oscillator circuit. In addition, it controls the source of the XTCLK signal."]
pub type XtOscR = crate::BitReader<XtOsc>;
impl XtOscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XtOsc {
        match self.bits {
            false => XtOsc::NotSupportedOrDisabled,
            true => XtOsc::SupportedAndEnabled,
        }
    }
    #[doc = "The crystal oscillator is not supported or disabled"]
    #[inline(always)]
    pub fn is_not_supported_or_disabled(&self) -> bool {
        *self == XtOsc::NotSupportedOrDisabled
    }
    #[doc = "The crystal oscillator is supported and enabled"]
    #[inline(always)]
    pub fn is_supported_and_enabled(&self) -> bool {
        *self == XtOsc::SupportedAndEnabled
    }
}
#[doc = "Field `XT_OSC` writer - This bit both indicates that the crystal oscillator option is supported and enables the operation of the crystal oscillator circuit. In addition, it controls the source of the XTCLK signal."]
pub type XtOscW<'a, REG> = crate::BitWriter<'a, REG, XtOsc>;
impl<'a, REG> XtOscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The crystal oscillator is not supported or disabled"]
    #[inline(always)]
    pub fn not_supported_or_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(XtOsc::NotSupportedOrDisabled)
    }
    #[doc = "The crystal oscillator is supported and enabled"]
    #[inline(always)]
    pub fn supported_and_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(XtOsc::SupportedAndEnabled)
    }
}
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
#[doc = "Crystal Oscillator Selection Enable for LFCLK clock\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XtOscSlEn {
    #[doc = "0: LFCG clock is the source of LFCLK clock (default)"]
    Lfcg = 0,
    #[doc = "1: XTOSC clock (XTCLK either from XTOSC if the crystal circuit is mounted and XT_OSC bit is set to 1 or, bypassing XTOSC, from 32KXIN/32KCLKIN pin if the crystal circuit is not mounted and XT_OSC bit is set to 0) is the source of LFCLK clock"]
    Xtosc = 1,
}
impl From<XtOscSlEn> for bool {
    #[inline(always)]
    fn from(variant: XtOscSlEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XT_OSC_SL_EN` reader - Crystal Oscillator Selection Enable for LFCLK clock"]
pub type XtOscSlEnR = crate::BitReader<XtOscSlEn>;
impl XtOscSlEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XtOscSlEn {
        match self.bits {
            false => XtOscSlEn::Lfcg,
            true => XtOscSlEn::Xtosc,
        }
    }
    #[doc = "LFCG clock is the source of LFCLK clock (default)"]
    #[inline(always)]
    pub fn is_lfcg(&self) -> bool {
        *self == XtOscSlEn::Lfcg
    }
    #[doc = "XTOSC clock (XTCLK either from XTOSC if the crystal circuit is mounted and XT_OSC bit is set to 1 or, bypassing XTOSC, from 32KXIN/32KCLKIN pin if the crystal circuit is not mounted and XT_OSC bit is set to 0) is the source of LFCLK clock"]
    #[inline(always)]
    pub fn is_xtosc(&self) -> bool {
        *self == XtOscSlEn::Xtosc
    }
}
#[doc = "Field `XT_OSC_SL_EN` writer - Crystal Oscillator Selection Enable for LFCLK clock"]
pub type XtOscSlEnW<'a, REG> = crate::BitWriter<'a, REG, XtOscSlEn>;
impl<'a, REG> XtOscSlEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LFCG clock is the source of LFCLK clock (default)"]
    #[inline(always)]
    pub fn lfcg(self) -> &'a mut crate::W<REG> {
        self.variant(XtOscSlEn::Lfcg)
    }
    #[doc = "XTOSC clock (XTCLK either from XTOSC if the crystal circuit is mounted and XT_OSC bit is set to 1 or, bypassing XTOSC, from 32KXIN/32KCLKIN pin if the crystal circuit is not mounted and XT_OSC bit is set to 0) is the source of LFCLK clock"]
    #[inline(always)]
    pub fn xtosc(self) -> &'a mut crate::W<REG> {
        self.variant(XtOscSlEn::Xtosc)
    }
}
impl R {
    #[doc = "Bit 0 - This bit both indicates that the crystal oscillator option is supported and enables the operation of the crystal oscillator circuit. In addition, it controls the source of the XTCLK signal."]
    #[inline(always)]
    pub fn xt_osc(&self) -> XtOscR {
        XtOscR::new((self.bits & 1) != 0)
    }
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
    #[doc = "Bit 6 - Crystal Oscillator Selection Enable for LFCLK clock"]
    #[inline(always)]
    pub fn xt_osc_sl_en(&self) -> XtOscSlEnR {
        XtOscSlEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFCGCTL2")
            .field("audp_en", &self.audp_en())
            .field("stopcal", &self.stopcal())
            .field("fclkrun", &self.fclkrun())
            .field("xt_osc_sl_en", &self.xt_osc_sl_en())
            .field("xt_osc", &self.xt_osc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit both indicates that the crystal oscillator option is supported and enables the operation of the crystal oscillator circuit. In addition, it controls the source of the XTCLK signal."]
    #[inline(always)]
    pub fn xt_osc(&mut self) -> XtOscW<Lfcgctl2Spec> {
        XtOscW::new(self, 0)
    }
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
    #[doc = "Bit 6 - Crystal Oscillator Selection Enable for LFCLK clock"]
    #[inline(always)]
    pub fn xt_osc_sl_en(&mut self) -> XtOscSlEnW<Lfcgctl2Spec> {
        XtOscSlEnW::new(self, 6)
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
#[doc = "`reset()` method sets LFCGCTL2 to value 0x81"]
impl crate::Resettable for Lfcgctl2Spec {
    const RESET_VALUE: u8 = 0x81;
}
