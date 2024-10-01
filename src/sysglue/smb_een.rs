#[doc = "Register `SMB_EEN` reader"]
pub type R = crate::R<SmbEenSpec>;
#[doc = "Register `SMB_EEN` writer"]
pub type W = crate::W<SmbEenSpec>;
#[doc = "Field `SMB0EEN` reader - SMBus/I2C 0 Event Enable"]
pub type Smb0eenR = crate::BitReader;
#[doc = "Field `SMB0EEN` writer - SMBus/I2C 0 Event Enable"]
pub type Smb0eenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB1EEN` reader - SMBus/I2C 1 Event Enable"]
pub type Smb1eenR = crate::BitReader;
#[doc = "Field `SMB1EEN` writer - SMBus/I2C 1 Event Enable"]
pub type Smb1eenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB2EEN` reader - SMBus/I2C 2 Event Enable"]
pub type Smb2eenR = crate::BitReader;
#[doc = "Field `SMB2EEN` writer - SMBus/I2C 2 Event Enable"]
pub type Smb2eenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB3EEN` reader - SMBus/I2C 3 Event Enable"]
pub type Smb3eenR = crate::BitReader;
#[doc = "Field `SMB3EEN` writer - SMBus/I2C 3 Event Enable"]
pub type Smb3eenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB4EEN` reader - SMBus/I2C 4 Event Enable"]
pub type Smb4eenR = crate::BitReader;
#[doc = "Field `SMB4EEN` writer - SMBus/I2C 4 Event Enable"]
pub type Smb4eenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB5EEN` reader - SMBus/I2C 5 Event Enable"]
pub type Smb5eenR = crate::BitReader;
#[doc = "Field `SMB5EEN` writer - SMBus/I2C 5 Event Enable"]
pub type Smb5eenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB6EEN` reader - SMBus/I2C 6 Event Enable"]
pub type Smb6eenR = crate::BitReader;
#[doc = "Field `SMB6EEN` writer - SMBus/I2C 6 Event Enable"]
pub type Smb6eenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB7EEN` reader - SMBus/I2C 7 Event Enable"]
pub type Smb7eenR = crate::BitReader;
#[doc = "Field `SMB7EEN` writer - SMBus/I2C 7 Event Enable"]
pub type Smb7eenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SMBus/I2C 0 Event Enable"]
    #[inline(always)]
    pub fn smb0een(&self) -> Smb0eenR {
        Smb0eenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus/I2C 1 Event Enable"]
    #[inline(always)]
    pub fn smb1een(&self) -> Smb1eenR {
        Smb1eenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMBus/I2C 2 Event Enable"]
    #[inline(always)]
    pub fn smb2een(&self) -> Smb2eenR {
        Smb2eenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus/I2C 3 Event Enable"]
    #[inline(always)]
    pub fn smb3een(&self) -> Smb3eenR {
        Smb3eenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMBus/I2C 4 Event Enable"]
    #[inline(always)]
    pub fn smb4een(&self) -> Smb4eenR {
        Smb4eenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus/I2C 5 Event Enable"]
    #[inline(always)]
    pub fn smb5een(&self) -> Smb5eenR {
        Smb5eenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus/I2C 6 Event Enable"]
    #[inline(always)]
    pub fn smb6een(&self) -> Smb6eenR {
        Smb6eenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMBus/I2C 7 Event Enable"]
    #[inline(always)]
    pub fn smb7een(&self) -> Smb7eenR {
        Smb7eenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMB_EEN")
            .field("smb0een", &self.smb0een())
            .field("smb1een", &self.smb1een())
            .field("smb2een", &self.smb2een())
            .field("smb3een", &self.smb3een())
            .field("smb4een", &self.smb4een())
            .field("smb5een", &self.smb5een())
            .field("smb6een", &self.smb6een())
            .field("smb7een", &self.smb7een())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SMBus/I2C 0 Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb0een(&mut self) -> Smb0eenW<SmbEenSpec> {
        Smb0eenW::new(self, 0)
    }
    #[doc = "Bit 1 - SMBus/I2C 1 Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb1een(&mut self) -> Smb1eenW<SmbEenSpec> {
        Smb1eenW::new(self, 1)
    }
    #[doc = "Bit 2 - SMBus/I2C 2 Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb2een(&mut self) -> Smb2eenW<SmbEenSpec> {
        Smb2eenW::new(self, 2)
    }
    #[doc = "Bit 3 - SMBus/I2C 3 Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb3een(&mut self) -> Smb3eenW<SmbEenSpec> {
        Smb3eenW::new(self, 3)
    }
    #[doc = "Bit 4 - SMBus/I2C 4 Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb4een(&mut self) -> Smb4eenW<SmbEenSpec> {
        Smb4eenW::new(self, 4)
    }
    #[doc = "Bit 5 - SMBus/I2C 5 Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb5een(&mut self) -> Smb5eenW<SmbEenSpec> {
        Smb5eenW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus/I2C 6 Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb6een(&mut self) -> Smb6eenW<SmbEenSpec> {
        Smb6eenW::new(self, 6)
    }
    #[doc = "Bit 7 - SMBus/I2C 7 Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb7een(&mut self) -> Smb7eenW<SmbEenSpec> {
        Smb7eenW::new(self, 7)
    }
}
#[doc = "SMBus/I2C Event Enable Register (SMB_EEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`smb_een::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smb_een::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbEenSpec;
impl crate::RegisterSpec for SmbEenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smb_een::R`](R) reader structure"]
impl crate::Readable for SmbEenSpec {}
#[doc = "`write(|w| ..)` method takes [`smb_een::W`](W) writer structure"]
impl crate::Writable for SmbEenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMB_EEN to value 0"]
impl crate::Resettable for SmbEenSpec {
    const RESET_VALUE: u8 = 0;
}
