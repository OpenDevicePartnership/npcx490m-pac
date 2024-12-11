#[doc = "Register `SMB_SBD` reader"]
pub type R = crate::R<SmbSbdSpec>;
#[doc = "Register `SMB_SBD` writer"]
pub type W = crate::W<SmbSbdSpec>;
#[doc = "Field `SMB0SBD` reader - SMBus/I2C 0 Start Bit Detection"]
pub type Smb0sbdR = crate::BitReader;
#[doc = "Field `SMB0SBD` writer - SMBus/I2C 0 Start Bit Detection"]
pub type Smb0sbdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB1SBD` reader - SMBus/I2C Module 1 Start Bit Detection"]
pub type Smb1sbdR = crate::BitReader;
#[doc = "Field `SMB1SBD` writer - SMBus/I2C Module 1 Start Bit Detection"]
pub type Smb1sbdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB2SBD` reader - SMBus/I2C Module 2 Start Bit Detection"]
pub type Smb2sbdR = crate::BitReader;
#[doc = "Field `SMB2SBD` writer - SMBus/I2C Module 2 Start Bit Detection"]
pub type Smb2sbdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB3SBD` reader - SMBus/I2C Module 3 Start Bit Detection"]
pub type Smb3sbdR = crate::BitReader;
#[doc = "Field `SMB3SBD` writer - SMBus/I2C Module 3 Start Bit Detection"]
pub type Smb3sbdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB4SBD` reader - SMBus/I2C Module 4 Start Bit Detection"]
pub type Smb4sbdR = crate::BitReader;
#[doc = "Field `SMB4SBD` writer - SMBus/I2C Module 4 Start Bit Detection"]
pub type Smb4sbdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB5SBD` reader - SMBus/I2C Module 5 Start Bit Detection"]
pub type Smb5sbdR = crate::BitReader;
#[doc = "Field `SMB5SBD` writer - SMBus/I2C Module 5 Start Bit Detection"]
pub type Smb5sbdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB6SBD` reader - SMBus/I2C Module 6 Start Bit Detection"]
pub type Smb6sbdR = crate::BitReader;
#[doc = "Field `SMB6SBD` writer - SMBus/I2C Module 6 Start Bit Detection"]
pub type Smb6sbdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB7SBD` reader - SMBus/I2C Module 7 Start Bit Detection"]
pub type Smb7sbdR = crate::BitReader;
#[doc = "Field `SMB7SBD` writer - SMBus/I2C Module 7 Start Bit Detection"]
pub type Smb7sbdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SMBus/I2C 0 Start Bit Detection"]
    #[inline(always)]
    pub fn smb0sbd(&self) -> Smb0sbdR {
        Smb0sbdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus/I2C Module 1 Start Bit Detection"]
    #[inline(always)]
    pub fn smb1sbd(&self) -> Smb1sbdR {
        Smb1sbdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMBus/I2C Module 2 Start Bit Detection"]
    #[inline(always)]
    pub fn smb2sbd(&self) -> Smb2sbdR {
        Smb2sbdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus/I2C Module 3 Start Bit Detection"]
    #[inline(always)]
    pub fn smb3sbd(&self) -> Smb3sbdR {
        Smb3sbdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMBus/I2C Module 4 Start Bit Detection"]
    #[inline(always)]
    pub fn smb4sbd(&self) -> Smb4sbdR {
        Smb4sbdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 5 Start Bit Detection"]
    #[inline(always)]
    pub fn smb5sbd(&self) -> Smb5sbdR {
        Smb5sbdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 6 Start Bit Detection"]
    #[inline(always)]
    pub fn smb6sbd(&self) -> Smb6sbdR {
        Smb6sbdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 7 Start Bit Detection"]
    #[inline(always)]
    pub fn smb7sbd(&self) -> Smb7sbdR {
        Smb7sbdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMB_SBD")
            .field("smb0sbd", &self.smb0sbd())
            .field("smb1sbd", &self.smb1sbd())
            .field("smb2sbd", &self.smb2sbd())
            .field("smb3sbd", &self.smb3sbd())
            .field("smb4sbd", &self.smb4sbd())
            .field("smb5sbd", &self.smb5sbd())
            .field("smb6sbd", &self.smb6sbd())
            .field("smb7sbd", &self.smb7sbd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SMBus/I2C 0 Start Bit Detection"]
    #[inline(always)]
    pub fn smb0sbd(&mut self) -> Smb0sbdW<SmbSbdSpec> {
        Smb0sbdW::new(self, 0)
    }
    #[doc = "Bit 1 - SMBus/I2C Module 1 Start Bit Detection"]
    #[inline(always)]
    pub fn smb1sbd(&mut self) -> Smb1sbdW<SmbSbdSpec> {
        Smb1sbdW::new(self, 1)
    }
    #[doc = "Bit 2 - SMBus/I2C Module 2 Start Bit Detection"]
    #[inline(always)]
    pub fn smb2sbd(&mut self) -> Smb2sbdW<SmbSbdSpec> {
        Smb2sbdW::new(self, 2)
    }
    #[doc = "Bit 3 - SMBus/I2C Module 3 Start Bit Detection"]
    #[inline(always)]
    pub fn smb3sbd(&mut self) -> Smb3sbdW<SmbSbdSpec> {
        Smb3sbdW::new(self, 3)
    }
    #[doc = "Bit 4 - SMBus/I2C Module 4 Start Bit Detection"]
    #[inline(always)]
    pub fn smb4sbd(&mut self) -> Smb4sbdW<SmbSbdSpec> {
        Smb4sbdW::new(self, 4)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 5 Start Bit Detection"]
    #[inline(always)]
    pub fn smb5sbd(&mut self) -> Smb5sbdW<SmbSbdSpec> {
        Smb5sbdW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 6 Start Bit Detection"]
    #[inline(always)]
    pub fn smb6sbd(&mut self) -> Smb6sbdW<SmbSbdSpec> {
        Smb6sbdW::new(self, 6)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 7 Start Bit Detection"]
    #[inline(always)]
    pub fn smb7sbd(&mut self) -> Smb7sbdW<SmbSbdSpec> {
        Smb7sbdW::new(self, 7)
    }
}
#[doc = "SMBus/I2C Start Bit Detection Register (SMB_SBD)\n\nYou can [`read`](crate::Reg::read) this register and get [`smb_sbd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smb_sbd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbSbdSpec;
impl crate::RegisterSpec for SmbSbdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smb_sbd::R`](R) reader structure"]
impl crate::Readable for SmbSbdSpec {}
#[doc = "`write(|w| ..)` method takes [`smb_sbd::W`](W) writer structure"]
impl crate::Writable for SmbSbdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMB_SBD to value 0"]
impl crate::Resettable for SmbSbdSpec {
    const RESET_VALUE: u8 = 0;
}
