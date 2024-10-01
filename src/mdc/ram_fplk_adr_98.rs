#[doc = "Register `RAM_FPLK_ADR_98` reader"]
pub type R = crate::R<RamFplkAdr98Spec>;
#[doc = "Register `RAM_FPLK_ADR_98` writer"]
pub type W = crate::W<RamFplkAdr98Spec>;
#[doc = "Field `FPLK_SK0` reader - Fetch Protect Lock Sector 0"]
pub type FplkSk0R = crate::BitReader;
#[doc = "Field `FPLK_SK0` writer - Fetch Protect Lock Sector 0"]
pub type FplkSk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPLK_SK1` reader - Fetch Protect Lock Sector 1"]
pub type FplkSk1R = crate::BitReader;
#[doc = "Field `FPLK_SK1` writer - Fetch Protect Lock Sector 1"]
pub type FplkSk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPLK_SK2` reader - Fetch Protect Lock Sector 2"]
pub type FplkSk2R = crate::BitReader;
#[doc = "Field `FPLK_SK2` writer - Fetch Protect Lock Sector 2"]
pub type FplkSk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPLK_SK3` reader - Fetch Protect Lock Sector 3"]
pub type FplkSk3R = crate::BitReader;
#[doc = "Field `FPLK_SK3` writer - Fetch Protect Lock Sector 3"]
pub type FplkSk3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPLK_SK4` reader - Fetch Protect Lock Sector 4"]
pub type FplkSk4R = crate::BitReader;
#[doc = "Field `FPLK_SK4` writer - Fetch Protect Lock Sector 4"]
pub type FplkSk4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPLK_SK5` reader - Fetch Protect Lock Sector 5"]
pub type FplkSk5R = crate::BitReader;
#[doc = "Field `FPLK_SK5` writer - Fetch Protect Lock Sector 5"]
pub type FplkSk5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPLK_SK6` reader - Fetch Protect Lock Sector 6"]
pub type FplkSk6R = crate::BitReader;
#[doc = "Field `FPLK_SK6` writer - Fetch Protect Lock Sector 6"]
pub type FplkSk6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPLK_SK7` reader - Fetch Protect Lock Sector 7"]
pub type FplkSk7R = crate::BitReader;
#[doc = "Field `FPLK_SK7` writer - Fetch Protect Lock Sector 7"]
pub type FplkSk7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fetch Protect Lock Sector 0"]
    #[inline(always)]
    pub fn fplk_sk0(&self) -> FplkSk0R {
        FplkSk0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fetch Protect Lock Sector 1"]
    #[inline(always)]
    pub fn fplk_sk1(&self) -> FplkSk1R {
        FplkSk1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fetch Protect Lock Sector 2"]
    #[inline(always)]
    pub fn fplk_sk2(&self) -> FplkSk2R {
        FplkSk2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fetch Protect Lock Sector 3"]
    #[inline(always)]
    pub fn fplk_sk3(&self) -> FplkSk3R {
        FplkSk3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fetch Protect Lock Sector 4"]
    #[inline(always)]
    pub fn fplk_sk4(&self) -> FplkSk4R {
        FplkSk4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fetch Protect Lock Sector 5"]
    #[inline(always)]
    pub fn fplk_sk5(&self) -> FplkSk5R {
        FplkSk5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fetch Protect Lock Sector 6"]
    #[inline(always)]
    pub fn fplk_sk6(&self) -> FplkSk6R {
        FplkSk6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fetch Protect Lock Sector 7"]
    #[inline(always)]
    pub fn fplk_sk7(&self) -> FplkSk7R {
        FplkSk7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM_FPLK_ADR_98")
            .field("fplk_sk0", &self.fplk_sk0())
            .field("fplk_sk1", &self.fplk_sk1())
            .field("fplk_sk2", &self.fplk_sk2())
            .field("fplk_sk3", &self.fplk_sk3())
            .field("fplk_sk4", &self.fplk_sk4())
            .field("fplk_sk5", &self.fplk_sk5())
            .field("fplk_sk6", &self.fplk_sk6())
            .field("fplk_sk7", &self.fplk_sk7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Fetch Protect Lock Sector 0"]
    #[inline(always)]
    #[must_use]
    pub fn fplk_sk0(&mut self) -> FplkSk0W<RamFplkAdr98Spec> {
        FplkSk0W::new(self, 0)
    }
    #[doc = "Bit 1 - Fetch Protect Lock Sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn fplk_sk1(&mut self) -> FplkSk1W<RamFplkAdr98Spec> {
        FplkSk1W::new(self, 1)
    }
    #[doc = "Bit 2 - Fetch Protect Lock Sector 2"]
    #[inline(always)]
    #[must_use]
    pub fn fplk_sk2(&mut self) -> FplkSk2W<RamFplkAdr98Spec> {
        FplkSk2W::new(self, 2)
    }
    #[doc = "Bit 3 - Fetch Protect Lock Sector 3"]
    #[inline(always)]
    #[must_use]
    pub fn fplk_sk3(&mut self) -> FplkSk3W<RamFplkAdr98Spec> {
        FplkSk3W::new(self, 3)
    }
    #[doc = "Bit 4 - Fetch Protect Lock Sector 4"]
    #[inline(always)]
    #[must_use]
    pub fn fplk_sk4(&mut self) -> FplkSk4W<RamFplkAdr98Spec> {
        FplkSk4W::new(self, 4)
    }
    #[doc = "Bit 5 - Fetch Protect Lock Sector 5"]
    #[inline(always)]
    #[must_use]
    pub fn fplk_sk5(&mut self) -> FplkSk5W<RamFplkAdr98Spec> {
        FplkSk5W::new(self, 5)
    }
    #[doc = "Bit 6 - Fetch Protect Lock Sector 6"]
    #[inline(always)]
    #[must_use]
    pub fn fplk_sk6(&mut self) -> FplkSk6W<RamFplkAdr98Spec> {
        FplkSk6W::new(self, 6)
    }
    #[doc = "Bit 7 - Fetch Protect Lock Sector 7"]
    #[inline(always)]
    #[must_use]
    pub fn fplk_sk7(&mut self) -> FplkSk7W<RamFplkAdr98Spec> {
        FplkSk7W::new(self, 7)
    }
}
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamFplkAdr98Spec;
impl crate::RegisterSpec for RamFplkAdr98Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ram_fplk_adr_98::R`](R) reader structure"]
impl crate::Readable for RamFplkAdr98Spec {}
#[doc = "`write(|w| ..)` method takes [`ram_fplk_adr_98::W`](W) writer structure"]
impl crate::Writable for RamFplkAdr98Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RAM_FPLK_ADR_98 to value 0"]
impl crate::Resettable for RamFplkAdr98Spec {
    const RESET_VALUE: u8 = 0;
}
