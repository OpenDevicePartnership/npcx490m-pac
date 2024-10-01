#[doc = "Register `RAM_FPR_ADR_78` reader"]
pub type R = crate::R<RamFprAdr78Spec>;
#[doc = "Register `RAM_FPR_ADR_78` writer"]
pub type W = crate::W<RamFprAdr78Spec>;
#[doc = "Field `FPR_SK0` reader - Fetch Protect Sector 0"]
pub type FprSk0R = crate::BitReader;
#[doc = "Field `FPR_SK0` writer - Fetch Protect Sector 0"]
pub type FprSk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPR_SK1` reader - Fetch Protect Sector 1"]
pub type FprSk1R = crate::BitReader;
#[doc = "Field `FPR_SK1` writer - Fetch Protect Sector 1"]
pub type FprSk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPR_SK2` reader - Fetch Protect Sector 2"]
pub type FprSk2R = crate::BitReader;
#[doc = "Field `FPR_SK2` writer - Fetch Protect Sector 2"]
pub type FprSk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPR_SK3` reader - Fetch Protect Sector 3"]
pub type FprSk3R = crate::BitReader;
#[doc = "Field `FPR_SK3` writer - Fetch Protect Sector 3"]
pub type FprSk3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPR_SK4` reader - Fetch Protect Sector 4"]
pub type FprSk4R = crate::BitReader;
#[doc = "Field `FPR_SK4` writer - Fetch Protect Sector 4"]
pub type FprSk4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPR_SK5` reader - Fetch Protect Sector 5"]
pub type FprSk5R = crate::BitReader;
#[doc = "Field `FPR_SK5` writer - Fetch Protect Sector 5"]
pub type FprSk5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPR_SK6` reader - Fetch Protect Sector 6"]
pub type FprSk6R = crate::BitReader;
#[doc = "Field `FPR_SK6` writer - Fetch Protect Sector 6"]
pub type FprSk6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPR_SK7` reader - Fetch Protect Sector 7"]
pub type FprSk7R = crate::BitReader;
#[doc = "Field `FPR_SK7` writer - Fetch Protect Sector 7"]
pub type FprSk7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fetch Protect Sector 0"]
    #[inline(always)]
    pub fn fpr_sk0(&self) -> FprSk0R {
        FprSk0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fetch Protect Sector 1"]
    #[inline(always)]
    pub fn fpr_sk1(&self) -> FprSk1R {
        FprSk1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fetch Protect Sector 2"]
    #[inline(always)]
    pub fn fpr_sk2(&self) -> FprSk2R {
        FprSk2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fetch Protect Sector 3"]
    #[inline(always)]
    pub fn fpr_sk3(&self) -> FprSk3R {
        FprSk3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fetch Protect Sector 4"]
    #[inline(always)]
    pub fn fpr_sk4(&self) -> FprSk4R {
        FprSk4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fetch Protect Sector 5"]
    #[inline(always)]
    pub fn fpr_sk5(&self) -> FprSk5R {
        FprSk5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fetch Protect Sector 6"]
    #[inline(always)]
    pub fn fpr_sk6(&self) -> FprSk6R {
        FprSk6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fetch Protect Sector 7"]
    #[inline(always)]
    pub fn fpr_sk7(&self) -> FprSk7R {
        FprSk7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM_FPR_ADR_78")
            .field("fpr_sk0", &self.fpr_sk0())
            .field("fpr_sk1", &self.fpr_sk1())
            .field("fpr_sk2", &self.fpr_sk2())
            .field("fpr_sk3", &self.fpr_sk3())
            .field("fpr_sk4", &self.fpr_sk4())
            .field("fpr_sk5", &self.fpr_sk5())
            .field("fpr_sk6", &self.fpr_sk6())
            .field("fpr_sk7", &self.fpr_sk7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Fetch Protect Sector 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpr_sk0(&mut self) -> FprSk0W<RamFprAdr78Spec> {
        FprSk0W::new(self, 0)
    }
    #[doc = "Bit 1 - Fetch Protect Sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpr_sk1(&mut self) -> FprSk1W<RamFprAdr78Spec> {
        FprSk1W::new(self, 1)
    }
    #[doc = "Bit 2 - Fetch Protect Sector 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpr_sk2(&mut self) -> FprSk2W<RamFprAdr78Spec> {
        FprSk2W::new(self, 2)
    }
    #[doc = "Bit 3 - Fetch Protect Sector 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpr_sk3(&mut self) -> FprSk3W<RamFprAdr78Spec> {
        FprSk3W::new(self, 3)
    }
    #[doc = "Bit 4 - Fetch Protect Sector 4"]
    #[inline(always)]
    #[must_use]
    pub fn fpr_sk4(&mut self) -> FprSk4W<RamFprAdr78Spec> {
        FprSk4W::new(self, 4)
    }
    #[doc = "Bit 5 - Fetch Protect Sector 5"]
    #[inline(always)]
    #[must_use]
    pub fn fpr_sk5(&mut self) -> FprSk5W<RamFprAdr78Spec> {
        FprSk5W::new(self, 5)
    }
    #[doc = "Bit 6 - Fetch Protect Sector 6"]
    #[inline(always)]
    #[must_use]
    pub fn fpr_sk6(&mut self) -> FprSk6W<RamFprAdr78Spec> {
        FprSk6W::new(self, 6)
    }
    #[doc = "Bit 7 - Fetch Protect Sector 7"]
    #[inline(always)]
    #[must_use]
    pub fn fpr_sk7(&mut self) -> FprSk7W<RamFprAdr78Spec> {
        FprSk7W::new(self, 7)
    }
}
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_78::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_78::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamFprAdr78Spec;
impl crate::RegisterSpec for RamFprAdr78Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ram_fpr_adr_78::R`](R) reader structure"]
impl crate::Readable for RamFprAdr78Spec {}
#[doc = "`write(|w| ..)` method takes [`ram_fpr_adr_78::W`](W) writer structure"]
impl crate::Writable for RamFprAdr78Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RAM_FPR_ADR_78 to value 0"]
impl crate::Resettable for RamFprAdr78Spec {
    const RESET_VALUE: u8 = 0;
}
