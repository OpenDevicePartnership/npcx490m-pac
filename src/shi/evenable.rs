#[doc = "Register `EVENABLE` reader"]
pub type R = crate::R<EvenableSpec>;
#[doc = "Register `EVENABLE` writer"]
pub type W = crate::W<EvenableSpec>;
#[doc = "Field `OBEEN` reader - 128-Byte Payload Buffer Empty Interrupt Enable"]
pub type ObeenR = crate::BitReader;
#[doc = "Field `OBEEN` writer - 128-Byte Payload Buffer Empty Interrupt Enable"]
pub type ObeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBHEEN` reader - 128-Byte Payload Buffer Half Empty Interrupt Enable"]
pub type ObheenR = crate::BitReader;
#[doc = "Field `OBHEEN` writer - 128-Byte Payload Buffer Half Empty Interrupt Enable"]
pub type ObheenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBFEN` reader - Input Buffer Full Interrupt Enable"]
pub type IbfenR = crate::BitReader;
#[doc = "Field `IBFEN` writer - Input Buffer Full Interrupt Enable"]
pub type IbfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBHFEN` reader - Input Buffer Half Full Interrupt Enable"]
pub type IbhfenR = crate::BitReader;
#[doc = "Field `IBHFEN` writer - Input Buffer Half Full Interrupt Enable"]
pub type IbhfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOREN` reader - End-of-Data for Read Transaction Interrupt Enable"]
pub type EorenR = crate::BitReader;
#[doc = "Field `EOREN` writer - End-of-Data for Read Transaction Interrupt Enable"]
pub type EorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOWEN` reader - End-of-Data for Write Transaction Interrupt Enable"]
pub type EowenR = crate::BitReader;
#[doc = "Field `EOWEN` writer - End-of-Data for Write Transaction Interrupt Enable"]
pub type EowenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSREN` reader - Status Read Interrupt Enable"]
pub type StsrenR = crate::BitReader;
#[doc = "Field `STSREN` writer - Status Read Interrupt Enable"]
pub type StsrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 128-Byte Payload Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn obeen(&self) -> ObeenR {
        ObeenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 128-Byte Payload Buffer Half Empty Interrupt Enable"]
    #[inline(always)]
    pub fn obheen(&self) -> ObheenR {
        ObheenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn ibfen(&self) -> IbfenR {
        IbfenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input Buffer Half Full Interrupt Enable"]
    #[inline(always)]
    pub fn ibhfen(&self) -> IbhfenR {
        IbhfenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End-of-Data for Read Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn eoren(&self) -> EorenR {
        EorenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End-of-Data for Write Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn eowen(&self) -> EowenR {
        EowenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status Read Interrupt Enable"]
    #[inline(always)]
    pub fn stsren(&self) -> StsrenR {
        StsrenR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVENABLE")
            .field("obeen", &self.obeen())
            .field("obheen", &self.obheen())
            .field("ibfen", &self.ibfen())
            .field("ibhfen", &self.ibhfen())
            .field("eoren", &self.eoren())
            .field("eowen", &self.eowen())
            .field("stsren", &self.stsren())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 128-Byte Payload Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn obeen(&mut self) -> ObeenW<EvenableSpec> {
        ObeenW::new(self, 0)
    }
    #[doc = "Bit 1 - 128-Byte Payload Buffer Half Empty Interrupt Enable"]
    #[inline(always)]
    pub fn obheen(&mut self) -> ObheenW<EvenableSpec> {
        ObheenW::new(self, 1)
    }
    #[doc = "Bit 2 - Input Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn ibfen(&mut self) -> IbfenW<EvenableSpec> {
        IbfenW::new(self, 2)
    }
    #[doc = "Bit 3 - Input Buffer Half Full Interrupt Enable"]
    #[inline(always)]
    pub fn ibhfen(&mut self) -> IbhfenW<EvenableSpec> {
        IbhfenW::new(self, 3)
    }
    #[doc = "Bit 4 - End-of-Data for Read Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn eoren(&mut self) -> EorenW<EvenableSpec> {
        EorenW::new(self, 4)
    }
    #[doc = "Bit 5 - End-of-Data for Write Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn eowen(&mut self) -> EowenW<EvenableSpec> {
        EowenW::new(self, 5)
    }
    #[doc = "Bit 6 - Status Read Interrupt Enable"]
    #[inline(always)]
    pub fn stsren(&mut self) -> StsrenW<EvenableSpec> {
        StsrenW::new(self, 6)
    }
}
#[doc = "Event Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`evenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvenableSpec;
impl crate::RegisterSpec for EvenableSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evenable::R`](R) reader structure"]
impl crate::Readable for EvenableSpec {}
#[doc = "`write(|w| ..)` method takes [`evenable::W`](W) writer structure"]
impl crate::Writable for EvenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EVENABLE to value 0"]
impl crate::Resettable for EvenableSpec {
    const RESET_VALUE: u8 = 0;
}
