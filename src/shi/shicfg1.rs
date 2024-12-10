#[doc = "Register `SHICFG1` reader"]
pub type R = crate::R<Shicfg1Spec>;
#[doc = "Register `SHICFG1` writer"]
pub type W = crate::W<Shicfg1Spec>;
#[doc = "Field `EN` reader - SHI Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - SHI Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEN` reader - Host Write Enable"]
pub type WenR = crate::BitReader;
#[doc = "Field `WEN` writer - Host Write Enable"]
pub type WenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTIBF` reader - Automatic Input Buffer Full Indication"]
pub type AutibfR = crate::BitReader;
#[doc = "Field `AUTIBF` writer - Automatic Input Buffer Full Indication"]
pub type AutibfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAS` reader - Data after Status"]
pub type DasR = crate::BitReader;
#[doc = "Field `DAS` writer - Data after Status"]
pub type DasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWRAP` reader - Input Buffer Wraparound Enable"]
pub type IwrapR = crate::BitReader;
#[doc = "Field `IWRAP` writer - Input Buffer Wraparound Enable"]
pub type IwrapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SHI Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Host Write Enable"]
    #[inline(always)]
    pub fn wen(&self) -> WenR {
        WenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Automatic Input Buffer Full Indication"]
    #[inline(always)]
    pub fn autibf(&self) -> AutibfR {
        AutibfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Data after Status"]
    #[inline(always)]
    pub fn das(&self) -> DasR {
        DasR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Buffer Wraparound Enable"]
    #[inline(always)]
    pub fn iwrap(&self) -> IwrapR {
        IwrapR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHICFG1")
            .field("en", &self.en())
            .field("wen", &self.wen())
            .field("autibf", &self.autibf())
            .field("das", &self.das())
            .field("cpol", &self.cpol())
            .field("iwrap", &self.iwrap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SHI Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Shicfg1Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 2 - Host Write Enable"]
    #[inline(always)]
    pub fn wen(&mut self) -> WenW<Shicfg1Spec> {
        WenW::new(self, 2)
    }
    #[doc = "Bit 3 - Automatic Input Buffer Full Indication"]
    #[inline(always)]
    pub fn autibf(&mut self) -> AutibfW<Shicfg1Spec> {
        AutibfW::new(self, 3)
    }
    #[doc = "Bit 5 - Data after Status"]
    #[inline(always)]
    pub fn das(&mut self) -> DasW<Shicfg1Spec> {
        DasW::new(self, 5)
    }
    #[doc = "Bit 6 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<Shicfg1Spec> {
        CpolW::new(self, 6)
    }
    #[doc = "Bit 7 - Input Buffer Wraparound Enable"]
    #[inline(always)]
    pub fn iwrap(&mut self) -> IwrapW<Shicfg1Spec> {
        IwrapW::new(self, 7)
    }
}
#[doc = "SHI Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shicfg1Spec;
impl crate::RegisterSpec for Shicfg1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shicfg1::R`](R) reader structure"]
impl crate::Readable for Shicfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`shicfg1::W`](W) writer structure"]
impl crate::Writable for Shicfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHICFG1 to value 0"]
impl crate::Resettable for Shicfg1Spec {
    const RESET_VALUE: u8 = 0;
}
