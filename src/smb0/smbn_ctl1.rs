#[doc = "Register `SMBnCTL1` reader"]
pub type R = crate::R<SmbnCtl1Spec>;
#[doc = "Register `SMBnCTL1` writer"]
pub type W = crate::W<SmbnCtl1Spec>;
#[doc = "Field `START` reader - MASTER in SMBnST register is set to 0"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - MASTER in SMBnST register is set to 0"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - default"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - default"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN` reader - Interrupt Enable"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - Interrupt Enable"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOBINTE` reader - End of Busy Interrupt Enable"]
pub type EobinteR = crate::BitReader;
#[doc = "Field `EOBINTE` writer - End of Busy Interrupt Enable"]
pub type EobinteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - Acknowledge"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - Acknowledge"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCMEN` reader - Global Call Match Enable"]
pub type GcmenR = crate::BitReader;
#[doc = "Field `GCMEN` writer - Global Call Match Enable"]
pub type GcmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMINTE` reader - New Match Interrupt Enable"]
pub type NminteR = crate::BitReader;
#[doc = "Field `NMINTE` writer - New Match Interrupt Enable"]
pub type NminteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STASTRE` reader - Stall After Start Enable"]
pub type StastreR = crate::BitReader;
#[doc = "Field `STASTRE` writer - Stall After Start Enable"]
pub type StastreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MASTER in SMBnST register is set to 0"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - default"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Busy Interrupt Enable"]
    #[inline(always)]
    pub fn eobinte(&self) -> EobinteR {
        EobinteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Global Call Match Enable"]
    #[inline(always)]
    pub fn gcmen(&self) -> GcmenR {
        GcmenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - New Match Interrupt Enable"]
    #[inline(always)]
    pub fn nminte(&self) -> NminteR {
        NminteR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stall After Start Enable"]
    #[inline(always)]
    pub fn stastre(&self) -> StastreR {
        StastreR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnCTL1")
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("inten", &self.inten())
            .field("eobinte", &self.eobinte())
            .field("ack", &self.ack())
            .field("gcmen", &self.gcmen())
            .field("nminte", &self.nminte())
            .field("stastre", &self.stastre())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MASTER in SMBnST register is set to 0"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<SmbnCtl1Spec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - default"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<SmbnCtl1Spec> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<SmbnCtl1Spec> {
        IntenW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Busy Interrupt Enable"]
    #[inline(always)]
    pub fn eobinte(&mut self) -> EobinteW<SmbnCtl1Spec> {
        EobinteW::new(self, 3)
    }
    #[doc = "Bit 4 - Acknowledge"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<SmbnCtl1Spec> {
        AckW::new(self, 4)
    }
    #[doc = "Bit 5 - Global Call Match Enable"]
    #[inline(always)]
    pub fn gcmen(&mut self) -> GcmenW<SmbnCtl1Spec> {
        GcmenW::new(self, 5)
    }
    #[doc = "Bit 6 - New Match Interrupt Enable"]
    #[inline(always)]
    pub fn nminte(&mut self) -> NminteW<SmbnCtl1Spec> {
        NminteW::new(self, 6)
    }
    #[doc = "Bit 7 - Stall After Start Enable"]
    #[inline(always)]
    pub fn stastre(&mut self) -> StastreW<SmbnCtl1Spec> {
        StastreW::new(self, 7)
    }
}
#[doc = "SMB Control 1 Register (SMBnCTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnCtl1Spec;
impl crate::RegisterSpec for SmbnCtl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_ctl1::R`](R) reader structure"]
impl crate::Readable for SmbnCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`smbn_ctl1::W`](W) writer structure"]
impl crate::Writable for SmbnCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnCTL1 to value 0"]
impl crate::Resettable for SmbnCtl1Spec {
    const RESET_VALUE: u8 = 0;
}
