#[doc = "Register `FLASH_RNG_TAG_OVR0-1515` reader"]
pub type R = crate::R<FlashRngTagOvr01515Spec>;
#[doc = "Register `FLASH_RNG_TAG_OVR0-1515` writer"]
pub type W = crate::W<FlashRngTagOvr01515Spec>;
#[doc = "Field `FRNG_WPR_TOVR` reader - Flash Write Protect Tag Override"]
pub type FrngWprTovrR = crate::FieldReader<u16>;
#[doc = "Field `FRNG_WPR_TOVR` writer - Flash Write Protect Tag Override"]
pub type FrngWprTovrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FRNG_RPR_TOVR` reader - Flash Range Read Protect Tag Override"]
pub type FrngRprTovrR = crate::FieldReader<u16>;
#[doc = "Field `FRNG_RPR_TOVR` writer - Flash Range Read Protect Tag Override"]
pub type FrngRprTovrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flash Write Protect Tag Override"]
    #[inline(always)]
    pub fn frng_wpr_tovr(&self) -> FrngWprTovrR {
        FrngWprTovrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Flash Range Read Protect Tag Override"]
    #[inline(always)]
    pub fn frng_rpr_tovr(&self) -> FrngRprTovrR {
        FrngRprTovrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_RNG_TAG_OVR0-1515")
            .field("frng_wpr_tovr", &self.frng_wpr_tovr())
            .field("frng_rpr_tovr", &self.frng_rpr_tovr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Write Protect Tag Override"]
    #[inline(always)]
    #[must_use]
    pub fn frng_wpr_tovr(&mut self) -> FrngWprTovrW<FlashRngTagOvr01515Spec> {
        FrngWprTovrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Flash Range Read Protect Tag Override"]
    #[inline(always)]
    #[must_use]
    pub fn frng_rpr_tovr(&mut self) -> FrngRprTovrW<FlashRngTagOvr01515Spec> {
        FrngRprTovrW::new(self, 16)
    }
}
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr01515::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr01515::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashRngTagOvr01515Spec;
impl crate::RegisterSpec for FlashRngTagOvr01515Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_rng_tag_ovr01515::R`](R) reader structure"]
impl crate::Readable for FlashRngTagOvr01515Spec {}
#[doc = "`write(|w| ..)` method takes [`flash_rng_tag_ovr01515::W`](W) writer structure"]
impl crate::Writable for FlashRngTagOvr01515Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_RNG_TAG_OVR0-1515 to value 0"]
impl crate::Resettable for FlashRngTagOvr01515Spec {
    const RESET_VALUE: u32 = 0;
}
