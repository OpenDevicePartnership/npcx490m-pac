#[doc = "Register `SHAW3-4_SEM0` reader"]
pub type R = crate::R<Shaw34Sem0Spec>;
#[doc = "Register `SHAW3-4_SEM0` writer"]
pub type W = crate::W<Shaw34Sem0Spec>;
#[doc = "Field `HSEM3_0` reader - Host Semaphore Bits 3-0"]
pub type Hsem3_0R = crate::FieldReader;
#[doc = "Field `HSEM3_0` writer - Host Semaphore Bits 3-0"]
pub type Hsem3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSEM3_0` reader - Core Semaphore Bits 3-0"]
pub type Csem3_0R = crate::FieldReader;
#[doc = "Field `CSEM3_0` writer - Core Semaphore Bits 3-0"]
pub type Csem3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Host Semaphore Bits 3-0"]
    #[inline(always)]
    pub fn hsem3_0(&self) -> Hsem3_0R {
        Hsem3_0R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Core Semaphore Bits 3-0"]
    #[inline(always)]
    pub fn csem3_0(&self) -> Csem3_0R {
        Csem3_0R::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHAW3-4_SEM0")
            .field("hsem3_0", &self.hsem3_0())
            .field("csem3_0", &self.csem3_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Host Semaphore Bits 3-0"]
    #[inline(always)]
    #[must_use]
    pub fn hsem3_0(&mut self) -> Hsem3_0W<Shaw34Sem0Spec> {
        Hsem3_0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Core Semaphore Bits 3-0"]
    #[inline(always)]
    #[must_use]
    pub fn csem3_0(&mut self) -> Csem3_0W<Shaw34Sem0Spec> {
        Csem3_0W::new(self, 4)
    }
}
#[doc = "Shared Access Window 3-4, Semaphore Register (SHAW3-4_SEM)\n\nYou can [`read`](crate::Reg::read) this register and get [`shaw34_sem0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shaw34_sem0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shaw34Sem0Spec;
impl crate::RegisterSpec for Shaw34Sem0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shaw34_sem0::R`](R) reader structure"]
impl crate::Readable for Shaw34Sem0Spec {}
#[doc = "`write(|w| ..)` method takes [`shaw34_sem0::W`](W) writer structure"]
impl crate::Writable for Shaw34Sem0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHAW3-4_SEM0 to value 0"]
impl crate::Resettable for Shaw34Sem0Spec {
    const RESET_VALUE: u8 = 0;
}
