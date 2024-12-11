#[doc = "Register `MDATACTRL` reader"]
pub type R = crate::R<MdatactrlSpec>;
#[doc = "Register `MDATACTRL` writer"]
pub type W = crate::W<MdatactrlSpec>;
#[doc = "Field `FLUSHTB` reader - Flush To-Bus Buffer"]
pub type FlushtbR = crate::BitReader;
#[doc = "Field `FLUSHTB` writer - Flush To-Bus Buffer"]
pub type FlushtbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHFB` reader - Flush From-Bus Buffer"]
pub type FlushfbR = crate::BitReader;
#[doc = "Field `FLUSHFB` writer - Flush From-Bus Buffer"]
pub type FlushfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNLOCK` reader - UNLOCK"]
pub type UnlockR = crate::BitReader;
#[doc = "Field `UNLOCK` writer - UNLOCK"]
pub type UnlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIG` reader - Transmit Trigger"]
pub type TxtrigR = crate::FieldReader;
#[doc = "Field `TXTRIG` writer - Transmit Trigger"]
pub type TxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXTRIG` reader - Receive Trigger"]
pub type RxtrigR = crate::FieldReader;
#[doc = "Field `RXTRIG` writer - Receive Trigger"]
pub type RxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXCOUNT` reader - Transmit Count"]
pub type TxcountR = crate::FieldReader;
#[doc = "Field `TXCOUNT` writer - Transmit Count"]
pub type TxcountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RXCOUNT` reader - Receive Count"]
pub type RxcountR = crate::FieldReader;
#[doc = "Field `RXCOUNT` writer - Receive Count"]
pub type RxcountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TXFULL` reader - Transmit Buffer Full"]
pub type TxfullR = crate::BitReader;
#[doc = "Field `TXFULL` writer - Transmit Buffer Full"]
pub type TxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEMPTY` reader - Receive Buffer Empty"]
pub type RxemptyR = crate::BitReader;
#[doc = "Field `RXEMPTY` writer - Receive Buffer Empty"]
pub type RxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flush To-Bus Buffer"]
    #[inline(always)]
    pub fn flushtb(&self) -> FlushtbR {
        FlushtbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flush From-Bus Buffer"]
    #[inline(always)]
    pub fn flushfb(&self) -> FlushfbR {
        FlushfbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - UNLOCK"]
    #[inline(always)]
    pub fn unlock(&self) -> UnlockR {
        UnlockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Transmit Trigger"]
    #[inline(always)]
    pub fn txtrig(&self) -> TxtrigR {
        TxtrigR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Receive Trigger"]
    #[inline(always)]
    pub fn rxtrig(&self) -> RxtrigR {
        RxtrigR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Transmit Count"]
    #[inline(always)]
    pub fn txcount(&self) -> TxcountR {
        TxcountR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Receive Count"]
    #[inline(always)]
    pub fn rxcount(&self) -> RxcountR {
        RxcountR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Transmit Buffer Full"]
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive Buffer Empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDATACTRL")
            .field("flushtb", &self.flushtb())
            .field("flushfb", &self.flushfb())
            .field("unlock", &self.unlock())
            .field("txtrig", &self.txtrig())
            .field("rxtrig", &self.rxtrig())
            .field("txcount", &self.txcount())
            .field("rxcount", &self.rxcount())
            .field("txfull", &self.txfull())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flush To-Bus Buffer"]
    #[inline(always)]
    pub fn flushtb(&mut self) -> FlushtbW<MdatactrlSpec> {
        FlushtbW::new(self, 0)
    }
    #[doc = "Bit 1 - Flush From-Bus Buffer"]
    #[inline(always)]
    pub fn flushfb(&mut self) -> FlushfbW<MdatactrlSpec> {
        FlushfbW::new(self, 1)
    }
    #[doc = "Bit 3 - UNLOCK"]
    #[inline(always)]
    pub fn unlock(&mut self) -> UnlockW<MdatactrlSpec> {
        UnlockW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Transmit Trigger"]
    #[inline(always)]
    pub fn txtrig(&mut self) -> TxtrigW<MdatactrlSpec> {
        TxtrigW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Receive Trigger"]
    #[inline(always)]
    pub fn rxtrig(&mut self) -> RxtrigW<MdatactrlSpec> {
        RxtrigW::new(self, 6)
    }
    #[doc = "Bits 16:21 - Transmit Count"]
    #[inline(always)]
    pub fn txcount(&mut self) -> TxcountW<MdatactrlSpec> {
        TxcountW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Receive Count"]
    #[inline(always)]
    pub fn rxcount(&mut self) -> RxcountW<MdatactrlSpec> {
        RxcountW::new(self, 24)
    }
    #[doc = "Bit 30 - Transmit Buffer Full"]
    #[inline(always)]
    pub fn txfull(&mut self) -> TxfullW<MdatactrlSpec> {
        TxfullW::new(self, 30)
    }
    #[doc = "Bit 31 - Receive Buffer Empty"]
    #[inline(always)]
    pub fn rxempty(&mut self) -> RxemptyW<MdatactrlSpec> {
        RxemptyW::new(self, 31)
    }
}
#[doc = "Controller Data Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdatactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdatactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdatactrlSpec;
impl crate::RegisterSpec for MdatactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdatactrl::R`](R) reader structure"]
impl crate::Readable for MdatactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mdatactrl::W`](W) writer structure"]
impl crate::Writable for MdatactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDATACTRL to value 0"]
impl crate::Resettable for MdatactrlSpec {
    const RESET_VALUE: u32 = 0;
}
