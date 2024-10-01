#[doc = "Register `SPSTS` reader"]
pub type R = crate::R<SpstsSpec>;
#[doc = "Register `SPSTS` writer"]
pub type W = crate::W<SpstsSpec>;
#[doc = "Field `VHIF18_EXIST` reader - 1.8V Host Interface Supply Level"]
pub type Vhif18ExistR = crate::BitReader;
#[doc = "Field `VHIF18_EXIST` writer - 1.8V Host Interface Supply Level"]
pub type Vhif18ExistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VHIF33_EXIST` reader - 3.3V Host Interface Supply Level"]
pub type Vhif33ExistR = crate::BitReader;
#[doc = "Field `VHIF33_EXIST` writer - 3.3V Host Interface Supply Level"]
pub type Vhif33ExistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSPI18_EXIST` reader - 1.8V SPI Flash Supply Level"]
pub type Vspi18ExistR = crate::BitReader;
#[doc = "Field `VSPI18_EXIST` writer - 1.8V SPI Flash Supply Level"]
pub type Vspi18ExistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSPI33_EXIST` reader - 3.3V SPI Flash Supply Level"]
pub type Vspi33ExistR = crate::BitReader;
#[doc = "Field `VSPI33_EXIST` writer - 3.3V SPI Flash Supply Level"]
pub type Vspi33ExistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCIO18_EXIST` reader - 1.8V VCIO Supply Level"]
pub type Vcio18ExistR = crate::BitReader;
#[doc = "Field `VCIO18_EXIST` writer - 1.8V VCIO Supply Level"]
pub type Vcio18ExistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCIO33_EXIST` reader - 3.3V VCIO Supply Level"]
pub type Vcio33ExistR = crate::BitReader;
#[doc = "Field `VCIO33_EXIST` writer - 3.3V VCIO Supply Level"]
pub type Vcio33ExistW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1.8V Host Interface Supply Level"]
    #[inline(always)]
    pub fn vhif18_exist(&self) -> Vhif18ExistR {
        Vhif18ExistR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 3.3V Host Interface Supply Level"]
    #[inline(always)]
    pub fn vhif33_exist(&self) -> Vhif33ExistR {
        Vhif33ExistR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1.8V SPI Flash Supply Level"]
    #[inline(always)]
    pub fn vspi18_exist(&self) -> Vspi18ExistR {
        Vspi18ExistR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3.3V SPI Flash Supply Level"]
    #[inline(always)]
    pub fn vspi33_exist(&self) -> Vspi33ExistR {
        Vspi33ExistR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1.8V VCIO Supply Level"]
    #[inline(always)]
    pub fn vcio18_exist(&self) -> Vcio18ExistR {
        Vcio18ExistR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 3.3V VCIO Supply Level"]
    #[inline(always)]
    pub fn vcio33_exist(&self) -> Vcio33ExistR {
        Vcio33ExistR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPSTS")
            .field("vhif18_exist", &self.vhif18_exist())
            .field("vhif33_exist", &self.vhif33_exist())
            .field("vspi18_exist", &self.vspi18_exist())
            .field("vspi33_exist", &self.vspi33_exist())
            .field("vcio18_exist", &self.vcio18_exist())
            .field("vcio33_exist", &self.vcio33_exist())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1.8V Host Interface Supply Level"]
    #[inline(always)]
    #[must_use]
    pub fn vhif18_exist(&mut self) -> Vhif18ExistW<SpstsSpec> {
        Vhif18ExistW::new(self, 0)
    }
    #[doc = "Bit 1 - 3.3V Host Interface Supply Level"]
    #[inline(always)]
    #[must_use]
    pub fn vhif33_exist(&mut self) -> Vhif33ExistW<SpstsSpec> {
        Vhif33ExistW::new(self, 1)
    }
    #[doc = "Bit 2 - 1.8V SPI Flash Supply Level"]
    #[inline(always)]
    #[must_use]
    pub fn vspi18_exist(&mut self) -> Vspi18ExistW<SpstsSpec> {
        Vspi18ExistW::new(self, 2)
    }
    #[doc = "Bit 3 - 3.3V SPI Flash Supply Level"]
    #[inline(always)]
    #[must_use]
    pub fn vspi33_exist(&mut self) -> Vspi33ExistW<SpstsSpec> {
        Vspi33ExistW::new(self, 3)
    }
    #[doc = "Bit 4 - 1.8V VCIO Supply Level"]
    #[inline(always)]
    #[must_use]
    pub fn vcio18_exist(&mut self) -> Vcio18ExistW<SpstsSpec> {
        Vcio18ExistW::new(self, 4)
    }
    #[doc = "Bit 5 - 3.3V VCIO Supply Level"]
    #[inline(always)]
    #[must_use]
    pub fn vcio33_exist(&mut self) -> Vcio33ExistW<SpstsSpec> {
        Vcio33ExistW::new(self, 5)
    }
}
#[doc = "Supply Status Register (SPSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`spsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpstsSpec;
impl crate::RegisterSpec for SpstsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spsts::R`](R) reader structure"]
impl crate::Readable for SpstsSpec {}
#[doc = "`write(|w| ..)` method takes [`spsts::W`](W) writer structure"]
impl crate::Writable for SpstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SPSTS to value 0"]
impl crate::Resettable for SpstsSpec {
    const RESET_VALUE: u8 = 0;
}
