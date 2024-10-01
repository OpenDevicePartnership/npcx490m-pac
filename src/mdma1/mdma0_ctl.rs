#[doc = "Register `MDMA0_CTL` reader"]
pub type R = crate::R<Mdma0CtlSpec>;
#[doc = "Register `MDMA0_CTL` writer"]
pub type W = crate::W<Mdma0CtlSpec>;
#[doc = "Field `MDMAEN` reader - MDMA Enable"]
pub type MdmaenR = crate::BitReader;
#[doc = "Field `MDMAEN` writer - MDMA Enable"]
pub type MdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPD` reader - MDMA Power-Down"]
pub type MpdR = crate::BitReader;
#[doc = "Field `MPD` writer - MDMA Power-Down"]
pub type MpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIEN` reader - Stop Interrupt Enable"]
pub type SienR = crate::BitReader;
#[doc = "Field `SIEN` writer - Stop Interrupt Enable"]
pub type SienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPS` reader - MDMA Power Save"]
pub type MpsR = crate::BitReader;
#[doc = "Field `MPS` writer - MDMA Power Save"]
pub type MpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Terminal Count"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - Terminal Count"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDMA Enable"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MdmaenR {
        MdmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MDMA Power-Down"]
    #[inline(always)]
    pub fn mpd(&self) -> MpdR {
        MpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop Interrupt Enable"]
    #[inline(always)]
    pub fn sien(&self) -> SienR {
        SienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - MDMA Power Save"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Terminal Count"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDMA0_CTL")
            .field("mdmaen", &self.mdmaen())
            .field("mpd", &self.mpd())
            .field("sien", &self.sien())
            .field("mps", &self.mps())
            .field("tc", &self.tc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MDMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdmaen(&mut self) -> MdmaenW<Mdma0CtlSpec> {
        MdmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - MDMA Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn mpd(&mut self) -> MpdW<Mdma0CtlSpec> {
        MpdW::new(self, 1)
    }
    #[doc = "Bit 8 - Stop Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sien(&mut self) -> SienW<Mdma0CtlSpec> {
        SienW::new(self, 8)
    }
    #[doc = "Bit 14 - MDMA Power Save"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MpsW<Mdma0CtlSpec> {
        MpsW::new(self, 14)
    }
    #[doc = "Bit 18 - Terminal Count"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<Mdma0CtlSpec> {
        TcW::new(self, 18)
    }
}
#[doc = "Channel 0/1 Control Register (MDMAn_CTL0, MDMAn_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma0_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma0_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdma0CtlSpec;
impl crate::RegisterSpec for Mdma0CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma0_ctl::R`](R) reader structure"]
impl crate::Readable for Mdma0CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma0_ctl::W`](W) writer structure"]
impl crate::Writable for Mdma0CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA0_CTL to value 0"]
impl crate::Resettable for Mdma0CtlSpec {
    const RESET_VALUE: u32 = 0;
}
