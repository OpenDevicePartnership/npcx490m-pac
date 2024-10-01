#[doc = "Register `SMCE_STS` reader"]
pub type R = crate::R<SmceStsSpec>;
#[doc = "Register `SMCE_STS` writer"]
pub type W = crate::W<SmceStsSpec>;
#[doc = "Field `HSEM3W` reader - Host Semaphore 3 Written"]
pub type Hsem3wR = crate::BitReader;
#[doc = "Field `HSEM3W` writer - Host Semaphore 3 Written"]
pub type Hsem3wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEM4W` reader - Host Semaphore 4 Written"]
pub type Hsem4wR = crate::BitReader;
#[doc = "Field `HSEM4W` writer - Host Semaphore 4 Written"]
pub type Hsem4wW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Host Semaphore 3 Written"]
    #[inline(always)]
    pub fn hsem3w(&self) -> Hsem3wR {
        Hsem3wR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host Semaphore 4 Written"]
    #[inline(always)]
    pub fn hsem4w(&self) -> Hsem4wR {
        Hsem4wR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCE_STS")
            .field("hsem3w", &self.hsem3w())
            .field("hsem4w", &self.hsem4w())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Host Semaphore 3 Written"]
    #[inline(always)]
    #[must_use]
    pub fn hsem3w(&mut self) -> Hsem3wW<SmceStsSpec> {
        Hsem3wW::new(self, 4)
    }
    #[doc = "Bit 5 - Host Semaphore 4 Written"]
    #[inline(always)]
    #[must_use]
    pub fn hsem4w(&mut self) -> Hsem4wW<SmceStsSpec> {
        Hsem4wW::new(self, 5)
    }
}
#[doc = "Extended Shared Memory Core Status Register (SMCE_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`smce_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smce_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmceStsSpec;
impl crate::RegisterSpec for SmceStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smce_sts::R`](R) reader structure"]
impl crate::Readable for SmceStsSpec {}
#[doc = "`write(|w| ..)` method takes [`smce_sts::W`](W) writer structure"]
impl crate::Writable for SmceStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMCE_STS to value 0"]
impl crate::Resettable for SmceStsSpec {
    const RESET_VALUE: u8 = 0;
}
