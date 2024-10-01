#[doc = "Register `DEVALTF` reader"]
pub type R = crate::R<DevaltfSpec>;
#[doc = "Register `DEVALTF` writer"]
pub type W = crate::W<DevaltfSpec>;
#[doc = "Field `AD5_SL` reader - AD5 Select"]
pub type Ad5SlR = crate::BitReader;
#[doc = "Field `AD5_SL` writer - AD5 Select"]
pub type Ad5SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD6_SL` reader - AD6 Select"]
pub type Ad6SlR = crate::BitReader;
#[doc = "Field `AD6_SL` writer - AD6 Select"]
pub type Ad6SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD7_SL` reader - AD7 Select"]
pub type Ad7SlR = crate::BitReader;
#[doc = "Field `AD7_SL` writer - AD7 Select"]
pub type Ad7SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD8_SL` reader - AD8 Select"]
pub type Ad8SlR = crate::BitReader;
#[doc = "Field `AD8_SL` writer - AD8 Select"]
pub type Ad8SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD9_SL` reader - AD9 Select"]
pub type Ad9SlR = crate::BitReader;
#[doc = "Field `AD9_SL` writer - AD9 Select"]
pub type Ad9SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD10_SL` reader - AD10 Select"]
pub type Ad10SlR = crate::BitReader;
#[doc = "Field `AD10_SL` writer - AD10 Select"]
pub type Ad10SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD11_SL` reader - AD11 Select"]
pub type Ad11SlR = crate::BitReader;
#[doc = "Field `AD11_SL` writer - AD11 Select"]
pub type Ad11SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD12_SL` reader - AD12 Select"]
pub type Ad12SlR = crate::BitReader;
#[doc = "Field `AD12_SL` writer - AD12 Select"]
pub type Ad12SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AD5 Select"]
    #[inline(always)]
    pub fn ad5_sl(&self) -> Ad5SlR {
        Ad5SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD6 Select"]
    #[inline(always)]
    pub fn ad6_sl(&self) -> Ad6SlR {
        Ad6SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD7 Select"]
    #[inline(always)]
    pub fn ad7_sl(&self) -> Ad7SlR {
        Ad7SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD8 Select"]
    #[inline(always)]
    pub fn ad8_sl(&self) -> Ad8SlR {
        Ad8SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD9 Select"]
    #[inline(always)]
    pub fn ad9_sl(&self) -> Ad9SlR {
        Ad9SlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AD10 Select"]
    #[inline(always)]
    pub fn ad10_sl(&self) -> Ad10SlR {
        Ad10SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AD11 Select"]
    #[inline(always)]
    pub fn ad11_sl(&self) -> Ad11SlR {
        Ad11SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AD12 Select"]
    #[inline(always)]
    pub fn ad12_sl(&self) -> Ad12SlR {
        Ad12SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTF")
            .field("ad5_sl", &self.ad5_sl())
            .field("ad6_sl", &self.ad6_sl())
            .field("ad7_sl", &self.ad7_sl())
            .field("ad8_sl", &self.ad8_sl())
            .field("ad9_sl", &self.ad9_sl())
            .field("ad10_sl", &self.ad10_sl())
            .field("ad11_sl", &self.ad11_sl())
            .field("ad12_sl", &self.ad12_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AD5 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad5_sl(&mut self) -> Ad5SlW<DevaltfSpec> {
        Ad5SlW::new(self, 0)
    }
    #[doc = "Bit 1 - AD6 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad6_sl(&mut self) -> Ad6SlW<DevaltfSpec> {
        Ad6SlW::new(self, 1)
    }
    #[doc = "Bit 2 - AD7 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad7_sl(&mut self) -> Ad7SlW<DevaltfSpec> {
        Ad7SlW::new(self, 2)
    }
    #[doc = "Bit 3 - AD8 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad8_sl(&mut self) -> Ad8SlW<DevaltfSpec> {
        Ad8SlW::new(self, 3)
    }
    #[doc = "Bit 4 - AD9 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad9_sl(&mut self) -> Ad9SlW<DevaltfSpec> {
        Ad9SlW::new(self, 4)
    }
    #[doc = "Bit 5 - AD10 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad10_sl(&mut self) -> Ad10SlW<DevaltfSpec> {
        Ad10SlW::new(self, 5)
    }
    #[doc = "Bit 6 - AD11 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad11_sl(&mut self) -> Ad11SlW<DevaltfSpec> {
        Ad11SlW::new(self, 6)
    }
    #[doc = "Bit 7 - AD12 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad12_sl(&mut self) -> Ad12SlW<DevaltfSpec> {
        Ad12SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function F Register (DEVALTF)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltfSpec;
impl crate::RegisterSpec for DevaltfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltf::R`](R) reader structure"]
impl crate::Readable for DevaltfSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltf::W`](W) writer structure"]
impl crate::Writable for DevaltfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTF to value 0"]
impl crate::Resettable for DevaltfSpec {
    const RESET_VALUE: u8 = 0;
}
