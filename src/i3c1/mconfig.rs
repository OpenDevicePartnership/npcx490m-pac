#[doc = "Register `MCONFIG` reader"]
pub type R = crate::R<MconfigSpec>;
#[doc = "Register `MCONFIG` writer"]
pub type W = crate::W<MconfigSpec>;
#[doc = "Field `CTRENA` reader - Controller Device Enable."]
pub type CtrenaR = crate::FieldReader;
#[doc = "Field `CTRENA` writer - Controller Device Enable."]
pub type CtrenaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DISTO` reader - Disable Timeout Error"]
pub type DistoR = crate::BitReader;
#[doc = "Field `DISTO` writer - Disable Timeout Error"]
pub type DistoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODSTOP` reader - Open-Drain Stop"]
pub type OdstopR = crate::BitReader;
#[doc = "Field `ODSTOP` writer - Open-Drain Stop"]
pub type OdstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPBAUD` reader - Push-Pull Baud Rate"]
pub type PpbaudR = crate::FieldReader;
#[doc = "Field `PPBAUD` writer - Push-Pull Baud Rate"]
pub type PpbaudW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPLOW` reader - Push-Pull Low Period"]
pub type PplowR = crate::FieldReader;
#[doc = "Field `PPLOW` writer - Push-Pull Low Period"]
pub type PplowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ODBAUD` reader - Open-Drain Baud Rate"]
pub type OdbaudR = crate::FieldReader;
#[doc = "Field `ODBAUD` writer - Open-Drain Baud Rate"]
pub type OdbaudW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ODHPP` reader - Open-Drain High Period"]
pub type OdhppR = crate::BitReader;
#[doc = "Field `ODHPP` writer - Open-Drain High Period"]
pub type OdhppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CBAUD` reader - I2C Baud Rate"]
pub type I2cbaudR = crate::FieldReader;
#[doc = "Field `I2CBAUD` writer - I2C Baud Rate"]
pub type I2cbaudW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Controller Device Enable."]
    #[inline(always)]
    pub fn ctrena(&self) -> CtrenaR {
        CtrenaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Disable Timeout Error"]
    #[inline(always)]
    pub fn disto(&self) -> DistoR {
        DistoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Open-Drain Stop"]
    #[inline(always)]
    pub fn odstop(&self) -> OdstopR {
        OdstopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Push-Pull Baud Rate"]
    #[inline(always)]
    pub fn ppbaud(&self) -> PpbaudR {
        PpbaudR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Push-Pull Low Period"]
    #[inline(always)]
    pub fn pplow(&self) -> PplowR {
        PplowR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Open-Drain Baud Rate"]
    #[inline(always)]
    pub fn odbaud(&self) -> OdbaudR {
        OdbaudR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Open-Drain High Period"]
    #[inline(always)]
    pub fn odhpp(&self) -> OdhppR {
        OdhppR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:31 - I2C Baud Rate"]
    #[inline(always)]
    pub fn i2cbaud(&self) -> I2cbaudR {
        I2cbaudR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCONFIG")
            .field("ctrena", &self.ctrena())
            .field("disto", &self.disto())
            .field("odstop", &self.odstop())
            .field("ppbaud", &self.ppbaud())
            .field("pplow", &self.pplow())
            .field("odbaud", &self.odbaud())
            .field("odhpp", &self.odhpp())
            .field("i2cbaud", &self.i2cbaud())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Controller Device Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ctrena(&mut self) -> CtrenaW<MconfigSpec> {
        CtrenaW::new(self, 0)
    }
    #[doc = "Bit 3 - Disable Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn disto(&mut self) -> DistoW<MconfigSpec> {
        DistoW::new(self, 3)
    }
    #[doc = "Bit 6 - Open-Drain Stop"]
    #[inline(always)]
    #[must_use]
    pub fn odstop(&mut self) -> OdstopW<MconfigSpec> {
        OdstopW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Push-Pull Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn ppbaud(&mut self) -> PpbaudW<MconfigSpec> {
        PpbaudW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Push-Pull Low Period"]
    #[inline(always)]
    #[must_use]
    pub fn pplow(&mut self) -> PplowW<MconfigSpec> {
        PplowW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Open-Drain Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn odbaud(&mut self) -> OdbaudW<MconfigSpec> {
        OdbaudW::new(self, 16)
    }
    #[doc = "Bit 24 - Open-Drain High Period"]
    #[inline(always)]
    #[must_use]
    pub fn odhpp(&mut self) -> OdhppW<MconfigSpec> {
        OdhppW::new(self, 24)
    }
    #[doc = "Bits 28:31 - I2C Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn i2cbaud(&mut self) -> I2cbaudW<MconfigSpec> {
        I2cbaudW::new(self, 28)
    }
}
#[doc = "Controller Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MconfigSpec;
impl crate::RegisterSpec for MconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mconfig::R`](R) reader structure"]
impl crate::Readable for MconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mconfig::W`](W) writer structure"]
impl crate::Writable for MconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCONFIG to value 0"]
impl crate::Resettable for MconfigSpec {
    const RESET_VALUE: u32 = 0;
}
