#[doc = "Register `HOFS_STS` reader"]
pub type R = crate::R<HofsStsSpec>;
#[doc = "Register `HOFS_STS` writer"]
pub type W = crate::W<HofsStsSpec>;
#[doc = "Field `HOFS1R` reader - Host Read from Offset in Window 1 Status"]
pub type Hofs1rR = crate::BitReader;
#[doc = "Field `HOFS1R` writer - Host Read from Offset in Window 1 Status"]
pub type Hofs1rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS1W` reader - Host Write to Offset in Window 1 Status"]
pub type Hofs1wR = crate::BitReader;
#[doc = "Field `HOFS1W` writer - Host Write to Offset in Window 1 Status"]
pub type Hofs1wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS2R` reader - Host Read from Offset in Window 2 Status"]
pub type Hofs2rR = crate::BitReader;
#[doc = "Field `HOFS2R` writer - Host Read from Offset in Window 2 Status"]
pub type Hofs2rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS2W` reader - Host Write to Offset in Window 2 Status"]
pub type Hofs2wR = crate::BitReader;
#[doc = "Field `HOFS2W` writer - Host Write to Offset in Window 2 Status"]
pub type Hofs2wW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host Read from Offset in Window 1 Status"]
    #[inline(always)]
    pub fn hofs1r(&self) -> Hofs1rR {
        Hofs1rR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Write to Offset in Window 1 Status"]
    #[inline(always)]
    pub fn hofs1w(&self) -> Hofs1wR {
        Hofs1wR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host Read from Offset in Window 2 Status"]
    #[inline(always)]
    pub fn hofs2r(&self) -> Hofs2rR {
        Hofs2rR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Host Write to Offset in Window 2 Status"]
    #[inline(always)]
    pub fn hofs2w(&self) -> Hofs2wR {
        Hofs2wR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOFS_STS")
            .field("hofs1r", &self.hofs1r())
            .field("hofs1w", &self.hofs1w())
            .field("hofs2r", &self.hofs2r())
            .field("hofs2w", &self.hofs2w())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Host Read from Offset in Window 1 Status"]
    #[inline(always)]
    pub fn hofs1r(&mut self) -> Hofs1rW<HofsStsSpec> {
        Hofs1rW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Write to Offset in Window 1 Status"]
    #[inline(always)]
    pub fn hofs1w(&mut self) -> Hofs1wW<HofsStsSpec> {
        Hofs1wW::new(self, 1)
    }
    #[doc = "Bit 2 - Host Read from Offset in Window 2 Status"]
    #[inline(always)]
    pub fn hofs2r(&mut self) -> Hofs2rW<HofsStsSpec> {
        Hofs2rW::new(self, 2)
    }
    #[doc = "Bit 3 - Host Write to Offset in Window 2 Status"]
    #[inline(always)]
    pub fn hofs2w(&mut self) -> Hofs2wW<HofsStsSpec> {
        Hofs2wW::new(self, 3)
    }
}
#[doc = "Host_Offset in Windows 1,2 Status Register (HOFS_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`hofs_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hofs_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HofsStsSpec;
impl crate::RegisterSpec for HofsStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hofs_sts::R`](R) reader structure"]
impl crate::Readable for HofsStsSpec {}
#[doc = "`write(|w| ..)` method takes [`hofs_sts::W`](W) writer structure"]
impl crate::Writable for HofsStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOFS_STS to value 0"]
impl crate::Resettable for HofsStsSpec {
    const RESET_VALUE: u8 = 0;
}
