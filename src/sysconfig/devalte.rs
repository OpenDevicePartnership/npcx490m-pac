#[doc = "Register `DEVALTE` reader"]
pub type R = crate::R<DevalteSpec>;
#[doc = "Register `DEVALTE` writer"]
pub type W = crate::W<DevalteSpec>;
#[doc = "Field `CR_SIN4_SL` reader - CR_SIN4 Select"]
pub type CrSin4SlR = crate::BitReader;
#[doc = "Field `CR_SIN4_SL` writer - CR_SIN4 Select"]
pub type CrSin4SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SOUT4_SL` reader - CR_SOUT4 Select"]
pub type CrSout4SlR = crate::BitReader;
#[doc = "Field `CR_SOUT4_SL` writer - CR_SOUT4 Select"]
pub type CrSout4SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - CR_SIN4 Select"]
    #[inline(always)]
    pub fn cr_sin4_sl(&self) -> CrSin4SlR {
        CrSin4SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CR_SOUT4 Select"]
    #[inline(always)]
    pub fn cr_sout4_sl(&self) -> CrSout4SlR {
        CrSout4SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTE")
            .field("cr_sin4_sl", &self.cr_sin4_sl())
            .field("cr_sout4_sl", &self.cr_sout4_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 6 - CR_SIN4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cr_sin4_sl(&mut self) -> CrSin4SlW<DevalteSpec> {
        CrSin4SlW::new(self, 6)
    }
    #[doc = "Bit 7 - CR_SOUT4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cr_sout4_sl(&mut self) -> CrSout4SlW<DevalteSpec> {
        CrSout4SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function E Register (DEVALTE)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevalteSpec;
impl crate::RegisterSpec for DevalteSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalte::R`](R) reader structure"]
impl crate::Readable for DevalteSpec {}
#[doc = "`write(|w| ..)` method takes [`devalte::W`](W) writer structure"]
impl crate::Writable for DevalteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTE to value 0"]
impl crate::Resettable for DevalteSpec {
    const RESET_VALUE: u8 = 0;
}
