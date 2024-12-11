#[doc = "Register `RAM_WPR_ADR_B8` reader"]
pub type R = crate::R<RamWprAdrB8Spec>;
#[doc = "Register `RAM_WPR_ADR_B8` writer"]
pub type W = crate::W<RamWprAdrB8Spec>;
#[doc = "Field `WPR_SK0` reader - Write Protect Sector 0"]
pub type WprSk0R = crate::BitReader;
#[doc = "Field `WPR_SK0` writer - Write Protect Sector 0"]
pub type WprSk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPR_SK1` reader - Write Protect Sector 1"]
pub type WprSk1R = crate::BitReader;
#[doc = "Field `WPR_SK1` writer - Write Protect Sector 1"]
pub type WprSk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPR_SK2` reader - Write Protect Sector 2"]
pub type WprSk2R = crate::BitReader;
#[doc = "Field `WPR_SK2` writer - Write Protect Sector 2"]
pub type WprSk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPR_SK3` reader - Write Protect Sector 3"]
pub type WprSk3R = crate::BitReader;
#[doc = "Field `WPR_SK3` writer - Write Protect Sector 3"]
pub type WprSk3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPR_SK4` reader - Write Protect Sector 4"]
pub type WprSk4R = crate::BitReader;
#[doc = "Field `WPR_SK4` writer - Write Protect Sector 4"]
pub type WprSk4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPR_SK5` reader - Write Protect Sector 5"]
pub type WprSk5R = crate::BitReader;
#[doc = "Field `WPR_SK5` writer - Write Protect Sector 5"]
pub type WprSk5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPR_SK6` reader - Write Protect Sector 6"]
pub type WprSk6R = crate::BitReader;
#[doc = "Field `WPR_SK6` writer - Write Protect Sector 6"]
pub type WprSk6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPR_SK7` reader - Write Protect Sector 7"]
pub type WprSk7R = crate::BitReader;
#[doc = "Field `WPR_SK7` writer - Write Protect Sector 7"]
pub type WprSk7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Protect Sector 0"]
    #[inline(always)]
    pub fn wpr_sk0(&self) -> WprSk0R {
        WprSk0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protect Sector 1"]
    #[inline(always)]
    pub fn wpr_sk1(&self) -> WprSk1R {
        WprSk1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protect Sector 2"]
    #[inline(always)]
    pub fn wpr_sk2(&self) -> WprSk2R {
        WprSk2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write Protect Sector 3"]
    #[inline(always)]
    pub fn wpr_sk3(&self) -> WprSk3R {
        WprSk3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Protect Sector 4"]
    #[inline(always)]
    pub fn wpr_sk4(&self) -> WprSk4R {
        WprSk4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protect Sector 5"]
    #[inline(always)]
    pub fn wpr_sk5(&self) -> WprSk5R {
        WprSk5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Protect Sector 6"]
    #[inline(always)]
    pub fn wpr_sk6(&self) -> WprSk6R {
        WprSk6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Protect Sector 7"]
    #[inline(always)]
    pub fn wpr_sk7(&self) -> WprSk7R {
        WprSk7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM_WPR_ADR_B8")
            .field("wpr_sk0", &self.wpr_sk0())
            .field("wpr_sk1", &self.wpr_sk1())
            .field("wpr_sk2", &self.wpr_sk2())
            .field("wpr_sk3", &self.wpr_sk3())
            .field("wpr_sk4", &self.wpr_sk4())
            .field("wpr_sk5", &self.wpr_sk5())
            .field("wpr_sk6", &self.wpr_sk6())
            .field("wpr_sk7", &self.wpr_sk7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect Sector 0"]
    #[inline(always)]
    pub fn wpr_sk0(&mut self) -> WprSk0W<RamWprAdrB8Spec> {
        WprSk0W::new(self, 0)
    }
    #[doc = "Bit 1 - Write Protect Sector 1"]
    #[inline(always)]
    pub fn wpr_sk1(&mut self) -> WprSk1W<RamWprAdrB8Spec> {
        WprSk1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write Protect Sector 2"]
    #[inline(always)]
    pub fn wpr_sk2(&mut self) -> WprSk2W<RamWprAdrB8Spec> {
        WprSk2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write Protect Sector 3"]
    #[inline(always)]
    pub fn wpr_sk3(&mut self) -> WprSk3W<RamWprAdrB8Spec> {
        WprSk3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protect Sector 4"]
    #[inline(always)]
    pub fn wpr_sk4(&mut self) -> WprSk4W<RamWprAdrB8Spec> {
        WprSk4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protect Sector 5"]
    #[inline(always)]
    pub fn wpr_sk5(&mut self) -> WprSk5W<RamWprAdrB8Spec> {
        WprSk5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write Protect Sector 6"]
    #[inline(always)]
    pub fn wpr_sk6(&mut self) -> WprSk6W<RamWprAdrB8Spec> {
        WprSk6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write Protect Sector 7"]
    #[inline(always)]
    pub fn wpr_sk7(&mut self) -> WprSk7W<RamWprAdrB8Spec> {
        WprSk7W::new(self, 7)
    }
}
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamWprAdrB8Spec;
impl crate::RegisterSpec for RamWprAdrB8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ram_wpr_adr_b8::R`](R) reader structure"]
impl crate::Readable for RamWprAdrB8Spec {}
#[doc = "`write(|w| ..)` method takes [`ram_wpr_adr_b8::W`](W) writer structure"]
impl crate::Writable for RamWprAdrB8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RAM_WPR_ADR_B8 to value 0"]
impl crate::Resettable for RamWprAdrB8Spec {
    const RESET_VALUE: u8 = 0;
}
