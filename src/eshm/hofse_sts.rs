#[doc = "Register `HOFSE_STS` reader"]
pub type R = crate::R<HofseStsSpec>;
#[doc = "Register `HOFSE_STS` writer"]
pub type W = crate::W<HofseStsSpec>;
#[doc = "Field `HOFS3R` reader - Host Read from Offset in Window 3 Status"]
pub type Hofs3rR = crate::BitReader;
#[doc = "Field `HOFS3R` writer - Host Read from Offset in Window 3 Status"]
pub type Hofs3rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS3W` reader - Host Write to Offset in Window 3 Status"]
pub type Hofs3wR = crate::BitReader;
#[doc = "Field `HOFS3W` writer - Host Write to Offset in Window 3 Status"]
pub type Hofs3wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS4R` reader - Host Read from Offset in Window 4 Status"]
pub type Hofs4rR = crate::BitReader;
#[doc = "Field `HOFS4R` writer - Host Read from Offset in Window 4 Status"]
pub type Hofs4rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS4W` reader - Host Write to Offset in Window 4 Status"]
pub type Hofs4wR = crate::BitReader;
#[doc = "Field `HOFS4W` writer - Host Write to Offset in Window 4 Status"]
pub type Hofs4wW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host Read from Offset in Window 3 Status"]
    #[inline(always)]
    pub fn hofs3r(&self) -> Hofs3rR {
        Hofs3rR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Write to Offset in Window 3 Status"]
    #[inline(always)]
    pub fn hofs3w(&self) -> Hofs3wR {
        Hofs3wR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host Read from Offset in Window 4 Status"]
    #[inline(always)]
    pub fn hofs4r(&self) -> Hofs4rR {
        Hofs4rR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Host Write to Offset in Window 4 Status"]
    #[inline(always)]
    pub fn hofs4w(&self) -> Hofs4wR {
        Hofs4wR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOFSE_STS")
            .field("hofs3r", &self.hofs3r())
            .field("hofs3w", &self.hofs3w())
            .field("hofs4r", &self.hofs4r())
            .field("hofs4w", &self.hofs4w())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Host Read from Offset in Window 3 Status"]
    #[inline(always)]
    #[must_use]
    pub fn hofs3r(&mut self) -> Hofs3rW<HofseStsSpec> {
        Hofs3rW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Write to Offset in Window 3 Status"]
    #[inline(always)]
    #[must_use]
    pub fn hofs3w(&mut self) -> Hofs3wW<HofseStsSpec> {
        Hofs3wW::new(self, 1)
    }
    #[doc = "Bit 2 - Host Read from Offset in Window 4 Status"]
    #[inline(always)]
    #[must_use]
    pub fn hofs4r(&mut self) -> Hofs4rW<HofseStsSpec> {
        Hofs4rW::new(self, 2)
    }
    #[doc = "Bit 3 - Host Write to Offset in Window 4 Status"]
    #[inline(always)]
    #[must_use]
    pub fn hofs4w(&mut self) -> Hofs4wW<HofseStsSpec> {
        Hofs4wW::new(self, 3)
    }
}
#[doc = "Host_Offset in Windows 3,4 Status Register (HOFSE_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`hofse_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hofse_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HofseStsSpec;
impl crate::RegisterSpec for HofseStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hofse_sts::R`](R) reader structure"]
impl crate::Readable for HofseStsSpec {}
#[doc = "`write(|w| ..)` method takes [`hofse_sts::W`](W) writer structure"]
impl crate::Writable for HofseStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOFSE_STS to value 0"]
impl crate::Resettable for HofseStsSpec {
    const RESET_VALUE: u8 = 0;
}
