#[doc = "Register `SMBnFIF_CTS` reader"]
pub type R = crate::R<SmbnFifCtsSpec>;
#[doc = "Register `SMBnFIF_CTS` writer"]
pub type W = crate::W<SmbnFifCtsSpec>;
#[doc = "Field `RXF_TXE` reader - Rx-FIFO Full, Tx-FIFO Empty Status"]
pub type RxfTxeR = crate::BitReader;
#[doc = "Field `RXF_TXE` writer - Rx-FIFO Full, Tx-FIFO Empty Status"]
pub type RxfTxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFTE_IE` reader - Rx-FIFO Full, Tx-FIFO Empty Interrupt Enable"]
pub type RfteIeR = crate::BitReader;
#[doc = "Field `RFTE_IE` writer - Rx-FIFO Full, Tx-FIFO Empty Interrupt Enable"]
pub type RfteIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_FIFO` reader - Clear FIFOs"]
pub type ClrFifoR = crate::BitReader;
#[doc = "Field `CLR_FIFO` writer - Clear FIFOs"]
pub type ClrFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVRSTR` reader - Slave Start or Restart"]
pub type SlvrstrR = crate::BitReader;
#[doc = "Field `SLVRSTR` writer - Slave Start or Restart"]
pub type SlvrstrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Rx-FIFO Full, Tx-FIFO Empty Status"]
    #[inline(always)]
    pub fn rxf_txe(&self) -> RxfTxeR {
        RxfTxeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx-FIFO Full, Tx-FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rfte_ie(&self) -> RfteIeR {
        RfteIeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear FIFOs"]
    #[inline(always)]
    pub fn clr_fifo(&self) -> ClrFifoR {
        ClrFifoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave Start or Restart"]
    #[inline(always)]
    pub fn slvrstr(&self) -> SlvrstrR {
        SlvrstrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnFIF_CTS")
            .field("rxf_txe", &self.rxf_txe())
            .field("rfte_ie", &self.rfte_ie())
            .field("clr_fifo", &self.clr_fifo())
            .field("slvrstr", &self.slvrstr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Rx-FIFO Full, Tx-FIFO Empty Status"]
    #[inline(always)]
    pub fn rxf_txe(&mut self) -> RxfTxeW<SmbnFifCtsSpec> {
        RxfTxeW::new(self, 1)
    }
    #[doc = "Bit 3 - Rx-FIFO Full, Tx-FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rfte_ie(&mut self) -> RfteIeW<SmbnFifCtsSpec> {
        RfteIeW::new(self, 3)
    }
    #[doc = "Bit 6 - Clear FIFOs"]
    #[inline(always)]
    pub fn clr_fifo(&mut self) -> ClrFifoW<SmbnFifCtsSpec> {
        ClrFifoW::new(self, 6)
    }
    #[doc = "Bit 7 - Slave Start or Restart"]
    #[inline(always)]
    pub fn slvrstr(&mut self) -> SlvrstrW<SmbnFifCtsSpec> {
        SlvrstrW::new(self, 7)
    }
}
#[doc = "SMB FIFO Control and Status Register (SMBnFIF_CTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_fif_cts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_fif_cts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnFifCtsSpec;
impl crate::RegisterSpec for SmbnFifCtsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_fif_cts::R`](R) reader structure"]
impl crate::Readable for SmbnFifCtsSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_fif_cts::W`](W) writer structure"]
impl crate::Writable for SmbnFifCtsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnFIF_CTS to value 0"]
impl crate::Resettable for SmbnFifCtsSpec {
    const RESET_VALUE: u8 = 0;
}
