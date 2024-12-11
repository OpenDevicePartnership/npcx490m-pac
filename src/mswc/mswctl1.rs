#[doc = "Register `MSWCTL1` reader"]
pub type R = crate::R<Mswctl1Spec>;
#[doc = "Register `MSWCTL1` writer"]
pub type W = crate::W<Mswctl1Spec>;
#[doc = "Field `HRSTOB` reader - Host Reset Control Bit"]
pub type HrstobR = crate::BitReader;
#[doc = "Field `HRSTOB` writer - Host Reset Control Bit"]
pub type HrstobW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPWRON` reader - Host Power-On"]
pub type HpwronR = crate::BitReader;
#[doc = "Field `HPWRON` writer - Host Power-On"]
pub type HpwronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LRESET_PLTRST_ACT` reader - LRESET or PLTRST Active"]
pub type LresetPltrstActR = crate::BitReader;
#[doc = "Field `LRESET_PLTRST_ACT` writer - LRESET or PLTRST Active"]
pub type LresetPltrstActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VHCFGA` reader - Valid Host Configuration Address"]
pub type VhcfgaR = crate::BitReader;
#[doc = "Field `VHCFGA` writer - Valid Host Configuration Address"]
pub type VhcfgaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCFGLK` reader - Host Configuration Address Lock"]
pub type HcfglkR = crate::BitReader;
#[doc = "Field `HCFGLK` writer - Host Configuration Address Lock"]
pub type HcfglkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_OUTB` reader - RESET_OUT Control Bit"]
pub type ResetOutbR = crate::BitReader;
#[doc = "Field `RESET_OUTB` writer - RESET_OUT Control Bit"]
pub type ResetOutbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host Reset Control Bit"]
    #[inline(always)]
    pub fn hrstob(&self) -> HrstobR {
        HrstobR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Power-On"]
    #[inline(always)]
    pub fn hpwron(&self) -> HpwronR {
        HpwronR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LRESET or PLTRST Active"]
    #[inline(always)]
    pub fn lreset_pltrst_act(&self) -> LresetPltrstActR {
        LresetPltrstActR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Valid Host Configuration Address"]
    #[inline(always)]
    pub fn vhcfga(&self) -> VhcfgaR {
        VhcfgaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host Configuration Address Lock"]
    #[inline(always)]
    pub fn hcfglk(&self) -> HcfglkR {
        HcfglkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - RESET_OUT Control Bit"]
    #[inline(always)]
    pub fn reset_outb(&self) -> ResetOutbR {
        ResetOutbR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSWCTL1")
            .field("hrstob", &self.hrstob())
            .field("hpwron", &self.hpwron())
            .field("lreset_pltrst_act", &self.lreset_pltrst_act())
            .field("vhcfga", &self.vhcfga())
            .field("hcfglk", &self.hcfglk())
            .field("reset_outb", &self.reset_outb())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Host Reset Control Bit"]
    #[inline(always)]
    pub fn hrstob(&mut self) -> HrstobW<Mswctl1Spec> {
        HrstobW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Power-On"]
    #[inline(always)]
    pub fn hpwron(&mut self) -> HpwronW<Mswctl1Spec> {
        HpwronW::new(self, 1)
    }
    #[doc = "Bit 2 - LRESET or PLTRST Active"]
    #[inline(always)]
    pub fn lreset_pltrst_act(&mut self) -> LresetPltrstActW<Mswctl1Spec> {
        LresetPltrstActW::new(self, 2)
    }
    #[doc = "Bit 3 - Valid Host Configuration Address"]
    #[inline(always)]
    pub fn vhcfga(&mut self) -> VhcfgaW<Mswctl1Spec> {
        VhcfgaW::new(self, 3)
    }
    #[doc = "Bit 4 - Host Configuration Address Lock"]
    #[inline(always)]
    pub fn hcfglk(&mut self) -> HcfglkW<Mswctl1Spec> {
        HcfglkW::new(self, 4)
    }
    #[doc = "Bit 6 - RESET_OUT Control Bit"]
    #[inline(always)]
    pub fn reset_outb(&mut self) -> ResetOutbW<Mswctl1Spec> {
        ResetOutbW::new(self, 6)
    }
}
#[doc = "MSWC Control Status 1 Register (MSWCTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mswctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mswctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mswctl1Spec;
impl crate::RegisterSpec for Mswctl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mswctl1::R`](R) reader structure"]
impl crate::Readable for Mswctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mswctl1::W`](W) writer structure"]
impl crate::Writable for Mswctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MSWCTL1 to value 0"]
impl crate::Resettable for Mswctl1Spec {
    const RESET_VALUE: u8 = 0;
}
