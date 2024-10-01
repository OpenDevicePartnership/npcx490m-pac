#[doc = "Register `EVENABLE2` reader"]
pub type R = crate::R<Evenable2Spec>;
#[doc = "Register `EVENABLE2` writer"]
pub type W = crate::W<Evenable2Spec>;
#[doc = "Field `IBHF2EN` reader - Input Buffer Half Full 2 Interrupt Enable"]
pub type Ibhf2enR = crate::BitReader;
#[doc = "Field `IBHF2EN` writer - Input Buffer Half Full 2 Interrupt Enable"]
pub type Ibhf2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSnREEN` reader - SHI_CS Rising Edge Interrupt Enable"]
pub type CsnReenR = crate::BitReader;
#[doc = "Field `CSnREEN` writer - SHI_CS Rising Edge Interrupt Enable"]
pub type CsnReenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSnFEEN` reader - SHI_CS Falling Edge Interrupt Enable"]
pub type CsnFeenR = crate::BitReader;
#[doc = "Field `CSnFEEN` writer - SHI_CS Falling Edge Interrupt Enable"]
pub type CsnFeenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Input Buffer Half Full 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ibhf2en(&self) -> Ibhf2enR {
        Ibhf2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SHI_CS Rising Edge Interrupt Enable"]
    #[inline(always)]
    pub fn csn_reen(&self) -> CsnReenR {
        CsnReenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SHI_CS Falling Edge Interrupt Enable"]
    #[inline(always)]
    pub fn csn_feen(&self) -> CsnFeenR {
        CsnFeenR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVENABLE2")
            .field("ibhf2en", &self.ibhf2en())
            .field("csn_reen", &self.csn_reen())
            .field("csn_feen", &self.csn_feen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Input Buffer Half Full 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ibhf2en(&mut self) -> Ibhf2enW<Evenable2Spec> {
        Ibhf2enW::new(self, 0)
    }
    #[doc = "Bit 1 - SHI_CS Rising Edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csn_reen(&mut self) -> CsnReenW<Evenable2Spec> {
        CsnReenW::new(self, 1)
    }
    #[doc = "Bit 2 - SHI_CS Falling Edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csn_feen(&mut self) -> CsnFeenW<Evenable2Spec> {
        CsnFeenW::new(self, 2)
    }
}
#[doc = "Event Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`evenable2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evenable2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Evenable2Spec;
impl crate::RegisterSpec for Evenable2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evenable2::R`](R) reader structure"]
impl crate::Readable for Evenable2Spec {}
#[doc = "`write(|w| ..)` method takes [`evenable2::W`](W) writer structure"]
impl crate::Writable for Evenable2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EVENABLE2 to value 0"]
impl crate::Resettable for Evenable2Spec {
    const RESET_VALUE: u8 = 0;
}
