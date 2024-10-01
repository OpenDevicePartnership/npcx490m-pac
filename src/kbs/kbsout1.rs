#[doc = "Register `KBSOUT1` reader"]
pub type R = crate::R<Kbsout1Spec>;
#[doc = "Register `KBSOUT1` writer"]
pub type W = crate::W<Kbsout1Spec>;
#[doc = "Field `KBSOUT17_KBSOUT16` reader - Keyboard Scan Out Bits 17-16"]
pub type Kbsout17Kbsout16R = crate::FieldReader;
#[doc = "Field `KBSOUT17_KBSOUT16` writer - Keyboard Scan Out Bits 17-16"]
pub type Kbsout17Kbsout16W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Keyboard Scan Out Bits 17-16"]
    #[inline(always)]
    pub fn kbsout17_kbsout16(&self) -> Kbsout17Kbsout16R {
        Kbsout17Kbsout16R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KBSOUT1")
            .field("kbsout17_kbsout16", &self.kbsout17_kbsout16())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Keyboard Scan Out Bits 17-16"]
    #[inline(always)]
    #[must_use]
    pub fn kbsout17_kbsout16(&mut self) -> Kbsout17Kbsout16W<Kbsout1Spec> {
        Kbsout17Kbsout16W::new(self, 0)
    }
}
#[doc = "Keyboard Scan Out 1 Register (KBSOUT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsout1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsout1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Kbsout1Spec;
impl crate::RegisterSpec for Kbsout1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`kbsout1::R`](R) reader structure"]
impl crate::Readable for Kbsout1Spec {}
#[doc = "`write(|w| ..)` method takes [`kbsout1::W`](W) writer structure"]
impl crate::Writable for Kbsout1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets KBSOUT1 to value 0"]
impl crate::Resettable for Kbsout1Spec {
    const RESET_VALUE: u16 = 0;
}
