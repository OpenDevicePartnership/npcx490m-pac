#[doc = "Register `EVSTAT2` reader"]
pub type R = crate::R<Evstat2Spec>;
#[doc = "Register `EVSTAT2` writer"]
pub type W = crate::W<Evstat2Spec>;
#[doc = "Field `IBHF2` reader - Input Buffer Half Full 2"]
pub type Ibhf2R = crate::BitReader;
#[doc = "Field `IBHF2` writer - Input Buffer Half Full 2"]
pub type Ibhf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSnRE` reader - SHI_CS Rising Edge"]
pub type CsnReR = crate::BitReader;
#[doc = "Field `CSnRE` writer - SHI_CS Rising Edge"]
pub type CsnReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSnFE` reader - SHI_CS Falling Edge"]
pub type CsnFeR = crate::BitReader;
#[doc = "Field `CSnFE` writer - SHI_CS Falling Edge"]
pub type CsnFeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Input Buffer Half Full 2"]
    #[inline(always)]
    pub fn ibhf2(&self) -> Ibhf2R {
        Ibhf2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SHI_CS Rising Edge"]
    #[inline(always)]
    pub fn csn_re(&self) -> CsnReR {
        CsnReR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SHI_CS Falling Edge"]
    #[inline(always)]
    pub fn csn_fe(&self) -> CsnFeR {
        CsnFeR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVSTAT2")
            .field("ibhf2", &self.ibhf2())
            .field("csn_re", &self.csn_re())
            .field("csn_fe", &self.csn_fe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Input Buffer Half Full 2"]
    #[inline(always)]
    #[must_use]
    pub fn ibhf2(&mut self) -> Ibhf2W<Evstat2Spec> {
        Ibhf2W::new(self, 0)
    }
    #[doc = "Bit 1 - SHI_CS Rising Edge"]
    #[inline(always)]
    #[must_use]
    pub fn csn_re(&mut self) -> CsnReW<Evstat2Spec> {
        CsnReW::new(self, 1)
    }
    #[doc = "Bit 2 - SHI_CS Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn csn_fe(&mut self) -> CsnFeW<Evstat2Spec> {
        CsnFeW::new(self, 2)
    }
}
#[doc = "Event Status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`evstat2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evstat2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Evstat2Spec;
impl crate::RegisterSpec for Evstat2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evstat2::R`](R) reader structure"]
impl crate::Readable for Evstat2Spec {}
#[doc = "`write(|w| ..)` method takes [`evstat2::W`](W) writer structure"]
impl crate::Writable for Evstat2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EVSTAT2 to value 0"]
impl crate::Resettable for Evstat2Spec {
    const RESET_VALUE: u8 = 0;
}
