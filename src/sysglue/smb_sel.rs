#[doc = "Register `SMB_SEL` reader"]
pub type R = crate::R<SmbSelSpec>;
#[doc = "Register `SMB_SEL` writer"]
pub type W = crate::W<SmbSelSpec>;
#[doc = "Field `SMB4_SL` reader - SMBus/I2C 4 Interface-Select"]
pub type Smb4SlR = crate::BitReader;
#[doc = "Field `SMB4_SL` writer - SMBus/I2C 4 Interface-Select"]
pub type Smb4SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB5_SL` reader - SMBus/I2C 5 Interface-Select"]
pub type Smb5SlR = crate::BitReader;
#[doc = "Field `SMB5_SL` writer - SMBus/I2C 5 Interface-Select"]
pub type Smb5SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB6_SL` reader - SMBus/I2C 6 Interface-Select"]
pub type Smb6SlR = crate::BitReader;
#[doc = "Field `SMB6_SL` writer - SMBus/I2C 6 Interface-Select"]
pub type Smb6SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB7_SL` reader - SMBus/I2C 7 Interface-Select"]
pub type Smb7SlR = crate::BitReader;
#[doc = "Field `SMB7_SL` writer - SMBus/I2C 7 Interface-Select"]
pub type Smb7SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - SMBus/I2C 4 Interface-Select"]
    #[inline(always)]
    pub fn smb4_sl(&self) -> Smb4SlR {
        Smb4SlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus/I2C 5 Interface-Select"]
    #[inline(always)]
    pub fn smb5_sl(&self) -> Smb5SlR {
        Smb5SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus/I2C 6 Interface-Select"]
    #[inline(always)]
    pub fn smb6_sl(&self) -> Smb6SlR {
        Smb6SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMBus/I2C 7 Interface-Select"]
    #[inline(always)]
    pub fn smb7_sl(&self) -> Smb7SlR {
        Smb7SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMB_SEL")
            .field("smb4_sl", &self.smb4_sl())
            .field("smb5_sl", &self.smb5_sl())
            .field("smb6_sl", &self.smb6_sl())
            .field("smb7_sl", &self.smb7_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - SMBus/I2C 4 Interface-Select"]
    #[inline(always)]
    pub fn smb4_sl(&mut self) -> Smb4SlW<SmbSelSpec> {
        Smb4SlW::new(self, 4)
    }
    #[doc = "Bit 5 - SMBus/I2C 5 Interface-Select"]
    #[inline(always)]
    pub fn smb5_sl(&mut self) -> Smb5SlW<SmbSelSpec> {
        Smb5SlW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus/I2C 6 Interface-Select"]
    #[inline(always)]
    pub fn smb6_sl(&mut self) -> Smb6SlW<SmbSelSpec> {
        Smb6SlW::new(self, 6)
    }
    #[doc = "Bit 7 - SMBus/I2C 7 Interface-Select"]
    #[inline(always)]
    pub fn smb7_sl(&mut self) -> Smb7SlW<SmbSelSpec> {
        Smb7SlW::new(self, 7)
    }
}
#[doc = "SMBus/I2C Bus Select Register (SMB_SEL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smb_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smb_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbSelSpec;
impl crate::RegisterSpec for SmbSelSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smb_sel::R`](R) reader structure"]
impl crate::Readable for SmbSelSpec {}
#[doc = "`write(|w| ..)` method takes [`smb_sel::W`](W) writer structure"]
impl crate::Writable for SmbSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMB_SEL to value 0"]
impl crate::Resettable for SmbSelSpec {
    const RESET_VALUE: u8 = 0;
}
