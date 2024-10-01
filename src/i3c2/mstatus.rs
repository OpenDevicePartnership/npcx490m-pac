#[doc = "Register `MSTATUS` reader"]
pub type R = crate::R<MstatusSpec>;
#[doc = "Register `MSTATUS` writer"]
pub type W = crate::W<MstatusSpec>;
#[doc = "Field `STATE` reader - Current Working State"]
pub type StateR = crate::FieldReader;
#[doc = "Field `STATE` writer - Current Working State"]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BETWEEN` reader - Between Messages"]
pub type BetweenR = crate::BitReader;
#[doc = "Field `BETWEEN` writer - Between Messages"]
pub type BetweenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBITYPE` reader - IBI Type"]
pub type IbitypeR = crate::FieldReader;
#[doc = "Field `IBITYPE` writer - IBI Type"]
pub type IbitypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TGTSTART` reader - Target START Detected"]
pub type TgtstartR = crate::BitReader;
#[doc = "Field `TGTSTART` writer - Target START Detected"]
pub type TgtstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCTRLDONE` reader - MCTRL Done"]
pub type MctrldoneR = crate::BitReader;
#[doc = "Field `MCTRLDONE` writer - MCTRL Done"]
pub type MctrldoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPLETE` reader - Message Completed"]
pub type CompleteR = crate::BitReader;
#[doc = "Field `COMPLETE` writer - Message Completed"]
pub type CompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEND` reader - Receive Buffer Pending"]
pub type RxpendR = crate::BitReader;
#[doc = "Field `RXPEND` writer - Receive Buffer Pending"]
pub type RxpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXNOTFULL` reader - Transmit Buffer Not Full"]
pub type TxnotfullR = crate::BitReader;
#[doc = "Field `TXNOTFULL` writer - Transmit Buffer Not Full"]
pub type TxnotfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIWON` reader - IBI Arbitration Won"]
pub type IbiwonR = crate::BitReader;
#[doc = "Field `IBIWON` writer - IBI Arbitration Won"]
pub type IbiwonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRWARN` reader - Error/Warning"]
pub type ErrwarnR = crate::BitReader;
#[doc = "Field `ERRWARN` writer - Error/Warning"]
pub type ErrwarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOWCNTLR` reader - I3CI Now Bus Controller"]
pub type NowcntlrR = crate::BitReader;
#[doc = "Field `NOWCNTLR` writer - I3CI Now Bus Controller"]
pub type NowcntlrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIADDR` reader - IBI Address"]
pub type IbiaddrR = crate::FieldReader;
#[doc = "Field `IBIADDR` writer - IBI Address"]
pub type IbiaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - Current Working State"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Between Messages"]
    #[inline(always)]
    pub fn between(&self) -> BetweenR {
        BetweenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IBI Type"]
    #[inline(always)]
    pub fn ibitype(&self) -> IbitypeR {
        IbitypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Target START Detected"]
    #[inline(always)]
    pub fn tgtstart(&self) -> TgtstartR {
        TgtstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCTRL Done"]
    #[inline(always)]
    pub fn mctrldone(&self) -> MctrldoneR {
        MctrldoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Message Completed"]
    #[inline(always)]
    pub fn complete(&self) -> CompleteR {
        CompleteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive Buffer Pending"]
    #[inline(always)]
    pub fn rxpend(&self) -> RxpendR {
        RxpendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Not Full"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TxnotfullR {
        TxnotfullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IBI Arbitration Won"]
    #[inline(always)]
    pub fn ibiwon(&self) -> IbiwonR {
        IbiwonR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Error/Warning"]
    #[inline(always)]
    pub fn errwarn(&self) -> ErrwarnR {
        ErrwarnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - I3CI Now Bus Controller"]
    #[inline(always)]
    pub fn nowcntlr(&self) -> NowcntlrR {
        NowcntlrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:30 - IBI Address"]
    #[inline(always)]
    pub fn ibiaddr(&self) -> IbiaddrR {
        IbiaddrR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTATUS")
            .field("state", &self.state())
            .field("between", &self.between())
            .field("ibitype", &self.ibitype())
            .field("tgtstart", &self.tgtstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowcntlr", &self.nowcntlr())
            .field("ibiaddr", &self.ibiaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Current Working State"]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> StateW<MstatusSpec> {
        StateW::new(self, 0)
    }
    #[doc = "Bit 4 - Between Messages"]
    #[inline(always)]
    #[must_use]
    pub fn between(&mut self) -> BetweenW<MstatusSpec> {
        BetweenW::new(self, 4)
    }
    #[doc = "Bits 6:7 - IBI Type"]
    #[inline(always)]
    #[must_use]
    pub fn ibitype(&mut self) -> IbitypeW<MstatusSpec> {
        IbitypeW::new(self, 6)
    }
    #[doc = "Bit 8 - Target START Detected"]
    #[inline(always)]
    #[must_use]
    pub fn tgtstart(&mut self) -> TgtstartW<MstatusSpec> {
        TgtstartW::new(self, 8)
    }
    #[doc = "Bit 9 - MCTRL Done"]
    #[inline(always)]
    #[must_use]
    pub fn mctrldone(&mut self) -> MctrldoneW<MstatusSpec> {
        MctrldoneW::new(self, 9)
    }
    #[doc = "Bit 10 - Message Completed"]
    #[inline(always)]
    #[must_use]
    pub fn complete(&mut self) -> CompleteW<MstatusSpec> {
        CompleteW::new(self, 10)
    }
    #[doc = "Bit 11 - Receive Buffer Pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxpend(&mut self) -> RxpendW<MstatusSpec> {
        RxpendW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Buffer Not Full"]
    #[inline(always)]
    #[must_use]
    pub fn txnotfull(&mut self) -> TxnotfullW<MstatusSpec> {
        TxnotfullW::new(self, 12)
    }
    #[doc = "Bit 13 - IBI Arbitration Won"]
    #[inline(always)]
    #[must_use]
    pub fn ibiwon(&mut self) -> IbiwonW<MstatusSpec> {
        IbiwonW::new(self, 13)
    }
    #[doc = "Bit 15 - Error/Warning"]
    #[inline(always)]
    #[must_use]
    pub fn errwarn(&mut self) -> ErrwarnW<MstatusSpec> {
        ErrwarnW::new(self, 15)
    }
    #[doc = "Bit 19 - I3CI Now Bus Controller"]
    #[inline(always)]
    #[must_use]
    pub fn nowcntlr(&mut self) -> NowcntlrW<MstatusSpec> {
        NowcntlrW::new(self, 19)
    }
    #[doc = "Bits 24:30 - IBI Address"]
    #[inline(always)]
    #[must_use]
    pub fn ibiaddr(&mut self) -> IbiaddrW<MstatusSpec> {
        IbiaddrW::new(self, 24)
    }
}
#[doc = "Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstatusSpec;
impl crate::RegisterSpec for MstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstatus::R`](R) reader structure"]
impl crate::Readable for MstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mstatus::W`](W) writer structure"]
impl crate::Writable for MstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTATUS to value 0"]
impl crate::Resettable for MstatusSpec {
    const RESET_VALUE: u32 = 0;
}
