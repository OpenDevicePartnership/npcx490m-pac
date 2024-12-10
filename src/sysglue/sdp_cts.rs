#[doc = "Register `SDP_CTS` reader"]
pub type R = crate::R<SdpCtsSpec>;
#[doc = "Register `SDP_CTS` writer"]
pub type W = crate::W<SdpCtsSpec>;
#[doc = "Field `SDP_EN` reader - SDP Enable"]
pub type SdpEnR = crate::BitReader;
#[doc = "Field `SDP_EN` writer - SDP Enable"]
pub type SdpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDP_MOD` reader - SDP Mode Select"]
pub type SdpModR = crate::BitReader;
#[doc = "Field `SDP_MOD` writer - SDP Mode Select"]
pub type SdpModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL_P80` reader - Dual Port 80 Display"]
pub type DualP80R = crate::BitReader;
#[doc = "Field `DUAL_P80` writer - Dual Port 80 Display"]
pub type DualP80W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SDP Enable"]
    #[inline(always)]
    pub fn sdp_en(&self) -> SdpEnR {
        SdpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDP Mode Select"]
    #[inline(always)]
    pub fn sdp_mod(&self) -> SdpModR {
        SdpModR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Dual Port 80 Display"]
    #[inline(always)]
    pub fn dual_p80(&self) -> DualP80R {
        DualP80R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDP_CTS")
            .field("sdp_en", &self.sdp_en())
            .field("sdp_mod", &self.sdp_mod())
            .field("dual_p80", &self.dual_p80())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SDP Enable"]
    #[inline(always)]
    pub fn sdp_en(&mut self) -> SdpEnW<SdpCtsSpec> {
        SdpEnW::new(self, 0)
    }
    #[doc = "Bit 1 - SDP Mode Select"]
    #[inline(always)]
    pub fn sdp_mod(&mut self) -> SdpModW<SdpCtsSpec> {
        SdpModW::new(self, 1)
    }
    #[doc = "Bit 3 - Dual Port 80 Display"]
    #[inline(always)]
    pub fn dual_p80(&mut self) -> DualP80W<SdpCtsSpec> {
        DualP80W::new(self, 3)
    }
}
#[doc = "Simple Debug Port Control and Status Register (SDP_CTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`sdp_cts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdp_cts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdpCtsSpec;
impl crate::RegisterSpec for SdpCtsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdp_cts::R`](R) reader structure"]
impl crate::Readable for SdpCtsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdp_cts::W`](W) writer structure"]
impl crate::Writable for SdpCtsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDP_CTS to value 0"]
impl crate::Resettable for SdpCtsSpec {
    const RESET_VALUE: u8 = 0;
}
