#[doc = "Register `SMBnCST2` reader"]
pub type R = crate::R<SmbnCst2Spec>;
#[doc = "Register `SMBnCST2` writer"]
pub type W = crate::W<SmbnCst2Spec>;
#[doc = "Field `MATCHA1F` reader - Match Address 1 Field"]
pub type Matcha1fR = crate::BitReader;
#[doc = "Field `MATCHA1F` writer - Match Address 1 Field"]
pub type Matcha1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHA2F` reader - Match Address 2 Field"]
pub type Matcha2fR = crate::BitReader;
#[doc = "Field `MATCHA2F` writer - Match Address 2 Field"]
pub type Matcha2fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHA3F` reader - Match Address 3 Field"]
pub type Matcha3fR = crate::BitReader;
#[doc = "Field `MATCHA3F` writer - Match Address 3 Field"]
pub type Matcha3fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHA4F` reader - Match Address 4 Field"]
pub type Matcha4fR = crate::BitReader;
#[doc = "Field `MATCHA4F` writer - Match Address 4 Field"]
pub type Matcha4fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHA5F` reader - Match Address 5 Field"]
pub type Matcha5fR = crate::BitReader;
#[doc = "Field `MATCHA5F` writer - Match Address 5 Field"]
pub type Matcha5fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHA6F` reader - Match Address 6 Field"]
pub type Matcha6fR = crate::BitReader;
#[doc = "Field `MATCHA6F` writer - Match Address 6 Field"]
pub type Matcha6fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHA7F` reader - Match Address 7 Field"]
pub type Matcha7fR = crate::BitReader;
#[doc = "Field `MATCHA7F` writer - Match Address 7 Field"]
pub type Matcha7fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTSTS` reader - Interrupt Status"]
pub type IntstsR = crate::BitReader;
#[doc = "Field `INTSTS` writer - Interrupt Status"]
pub type IntstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Match Address 1 Field"]
    #[inline(always)]
    pub fn matcha1f(&self) -> Matcha1fR {
        Matcha1fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Match Address 2 Field"]
    #[inline(always)]
    pub fn matcha2f(&self) -> Matcha2fR {
        Matcha2fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Match Address 3 Field"]
    #[inline(always)]
    pub fn matcha3f(&self) -> Matcha3fR {
        Matcha3fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Match Address 4 Field"]
    #[inline(always)]
    pub fn matcha4f(&self) -> Matcha4fR {
        Matcha4fR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Match Address 5 Field"]
    #[inline(always)]
    pub fn matcha5f(&self) -> Matcha5fR {
        Matcha5fR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match Address 6 Field"]
    #[inline(always)]
    pub fn matcha6f(&self) -> Matcha6fR {
        Matcha6fR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Match Address 7 Field"]
    #[inline(always)]
    pub fn matcha7f(&self) -> Matcha7fR {
        Matcha7fR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status"]
    #[inline(always)]
    pub fn intsts(&self) -> IntstsR {
        IntstsR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnCST2")
            .field("matcha1f", &self.matcha1f())
            .field("matcha2f", &self.matcha2f())
            .field("matcha3f", &self.matcha3f())
            .field("matcha4f", &self.matcha4f())
            .field("matcha5f", &self.matcha5f())
            .field("matcha6f", &self.matcha6f())
            .field("matcha7f", &self.matcha7f())
            .field("intsts", &self.intsts())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Match Address 1 Field"]
    #[inline(always)]
    #[must_use]
    pub fn matcha1f(&mut self) -> Matcha1fW<SmbnCst2Spec> {
        Matcha1fW::new(self, 0)
    }
    #[doc = "Bit 1 - Match Address 2 Field"]
    #[inline(always)]
    #[must_use]
    pub fn matcha2f(&mut self) -> Matcha2fW<SmbnCst2Spec> {
        Matcha2fW::new(self, 1)
    }
    #[doc = "Bit 2 - Match Address 3 Field"]
    #[inline(always)]
    #[must_use]
    pub fn matcha3f(&mut self) -> Matcha3fW<SmbnCst2Spec> {
        Matcha3fW::new(self, 2)
    }
    #[doc = "Bit 3 - Match Address 4 Field"]
    #[inline(always)]
    #[must_use]
    pub fn matcha4f(&mut self) -> Matcha4fW<SmbnCst2Spec> {
        Matcha4fW::new(self, 3)
    }
    #[doc = "Bit 4 - Match Address 5 Field"]
    #[inline(always)]
    #[must_use]
    pub fn matcha5f(&mut self) -> Matcha5fW<SmbnCst2Spec> {
        Matcha5fW::new(self, 4)
    }
    #[doc = "Bit 5 - Match Address 6 Field"]
    #[inline(always)]
    #[must_use]
    pub fn matcha6f(&mut self) -> Matcha6fW<SmbnCst2Spec> {
        Matcha6fW::new(self, 5)
    }
    #[doc = "Bit 6 - Match Address 7 Field"]
    #[inline(always)]
    #[must_use]
    pub fn matcha7f(&mut self) -> Matcha7fW<SmbnCst2Spec> {
        Matcha7fW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn intsts(&mut self) -> IntstsW<SmbnCst2Spec> {
        IntstsW::new(self, 7)
    }
}
#[doc = "SMB Control Status 2 Register (SMBnCST2)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_cst2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_cst2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnCst2Spec;
impl crate::RegisterSpec for SmbnCst2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_cst2::R`](R) reader structure"]
impl crate::Readable for SmbnCst2Spec {}
#[doc = "`write(|w| ..)` method takes [`smbn_cst2::W`](W) writer structure"]
impl crate::Writable for SmbnCst2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnCST2 to value 0"]
impl crate::Resettable for SmbnCst2Spec {
    const RESET_VALUE: u8 = 0;
}
