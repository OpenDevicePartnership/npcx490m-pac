#[doc = "Register `RAM_WPLK_ADR_C8` reader"]
pub type R = crate::R<RamWplkAdrC8Spec>;
#[doc = "Register `RAM_WPLK_ADR_C8` writer"]
pub type W = crate::W<RamWplkAdrC8Spec>;
#[doc = "Field `WPLK_SK0` reader - Write Protect Lock Sector 0"]
pub type WplkSk0R = crate::BitReader;
#[doc = "Field `WPLK_SK0` writer - Write Protect Lock Sector 0"]
pub type WplkSk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPLK_SK1` reader - Write Protect Lock Sector 1"]
pub type WplkSk1R = crate::BitReader;
#[doc = "Field `WPLK_SK1` writer - Write Protect Lock Sector 1"]
pub type WplkSk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPLK_SK2` reader - Write Protect Lock Sector 2"]
pub type WplkSk2R = crate::BitReader;
#[doc = "Field `WPLK_SK2` writer - Write Protect Lock Sector 2"]
pub type WplkSk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPLK_SK3` reader - Write Protect Lock Sector 3"]
pub type WplkSk3R = crate::BitReader;
#[doc = "Field `WPLK_SK3` writer - Write Protect Lock Sector 3"]
pub type WplkSk3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPLK_SK4` reader - Write Protect Lock Sector 4"]
pub type WplkSk4R = crate::BitReader;
#[doc = "Field `WPLK_SK4` writer - Write Protect Lock Sector 4"]
pub type WplkSk4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPLK_SK5` reader - Write Protect Lock Sector 5"]
pub type WplkSk5R = crate::BitReader;
#[doc = "Field `WPLK_SK5` writer - Write Protect Lock Sector 5"]
pub type WplkSk5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPLK_SK6` reader - Write Protect Lock Sector 6"]
pub type WplkSk6R = crate::BitReader;
#[doc = "Field `WPLK_SK6` writer - Write Protect Lock Sector 6"]
pub type WplkSk6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPLK_SK7` reader - Write Protect Lock Sector 7"]
pub type WplkSk7R = crate::BitReader;
#[doc = "Field `WPLK_SK7` writer - Write Protect Lock Sector 7"]
pub type WplkSk7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Protect Lock Sector 0"]
    #[inline(always)]
    pub fn wplk_sk0(&self) -> WplkSk0R {
        WplkSk0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protect Lock Sector 1"]
    #[inline(always)]
    pub fn wplk_sk1(&self) -> WplkSk1R {
        WplkSk1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protect Lock Sector 2"]
    #[inline(always)]
    pub fn wplk_sk2(&self) -> WplkSk2R {
        WplkSk2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write Protect Lock Sector 3"]
    #[inline(always)]
    pub fn wplk_sk3(&self) -> WplkSk3R {
        WplkSk3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Protect Lock Sector 4"]
    #[inline(always)]
    pub fn wplk_sk4(&self) -> WplkSk4R {
        WplkSk4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protect Lock Sector 5"]
    #[inline(always)]
    pub fn wplk_sk5(&self) -> WplkSk5R {
        WplkSk5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Protect Lock Sector 6"]
    #[inline(always)]
    pub fn wplk_sk6(&self) -> WplkSk6R {
        WplkSk6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Protect Lock Sector 7"]
    #[inline(always)]
    pub fn wplk_sk7(&self) -> WplkSk7R {
        WplkSk7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM_WPLK_ADR_C8")
            .field("wplk_sk0", &self.wplk_sk0())
            .field("wplk_sk1", &self.wplk_sk1())
            .field("wplk_sk2", &self.wplk_sk2())
            .field("wplk_sk3", &self.wplk_sk3())
            .field("wplk_sk4", &self.wplk_sk4())
            .field("wplk_sk5", &self.wplk_sk5())
            .field("wplk_sk6", &self.wplk_sk6())
            .field("wplk_sk7", &self.wplk_sk7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect Lock Sector 0"]
    #[inline(always)]
    pub fn wplk_sk0(&mut self) -> WplkSk0W<RamWplkAdrC8Spec> {
        WplkSk0W::new(self, 0)
    }
    #[doc = "Bit 1 - Write Protect Lock Sector 1"]
    #[inline(always)]
    pub fn wplk_sk1(&mut self) -> WplkSk1W<RamWplkAdrC8Spec> {
        WplkSk1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write Protect Lock Sector 2"]
    #[inline(always)]
    pub fn wplk_sk2(&mut self) -> WplkSk2W<RamWplkAdrC8Spec> {
        WplkSk2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write Protect Lock Sector 3"]
    #[inline(always)]
    pub fn wplk_sk3(&mut self) -> WplkSk3W<RamWplkAdrC8Spec> {
        WplkSk3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protect Lock Sector 4"]
    #[inline(always)]
    pub fn wplk_sk4(&mut self) -> WplkSk4W<RamWplkAdrC8Spec> {
        WplkSk4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protect Lock Sector 5"]
    #[inline(always)]
    pub fn wplk_sk5(&mut self) -> WplkSk5W<RamWplkAdrC8Spec> {
        WplkSk5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write Protect Lock Sector 6"]
    #[inline(always)]
    pub fn wplk_sk6(&mut self) -> WplkSk6W<RamWplkAdrC8Spec> {
        WplkSk6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write Protect Lock Sector 7"]
    #[inline(always)]
    pub fn wplk_sk7(&mut self) -> WplkSk7W<RamWplkAdrC8Spec> {
        WplkSk7W::new(self, 7)
    }
}
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamWplkAdrC8Spec;
impl crate::RegisterSpec for RamWplkAdrC8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ram_wplk_adr_c8::R`](R) reader structure"]
impl crate::Readable for RamWplkAdrC8Spec {}
#[doc = "`write(|w| ..)` method takes [`ram_wplk_adr_c8::W`](W) writer structure"]
impl crate::Writable for RamWplkAdrC8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RAM_WPLK_ADR_C8 to value 0"]
impl crate::Resettable for RamWplkAdrC8Spec {
    const RESET_VALUE: u8 = 0;
}
