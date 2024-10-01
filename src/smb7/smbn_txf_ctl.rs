#[doc = "Register `SMBnTXF_CTL` reader"]
pub type R = crate::R<SmbnTxfCtlSpec>;
#[doc = "Register `SMBnTXF_CTL` writer"]
pub type W = crate::W<SmbnTxfCtlSpec>;
#[doc = "Field `TX_THR` reader - Tx-FIFO Threshold"]
pub type TxThrR = crate::FieldReader;
#[doc = "Field `TX_THR` writer - Tx-FIFO Threshold"]
pub type TxThrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `THR_TXIE` reader - Threshold Tx-FIFO Interrupt Enable"]
pub type ThrTxieR = crate::BitReader;
#[doc = "Field `THR_TXIE` writer - Threshold Tx-FIFO Interrupt Enable"]
pub type ThrTxieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Tx-FIFO Threshold"]
    #[inline(always)]
    pub fn tx_thr(&self) -> TxThrR {
        TxThrR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Threshold Tx-FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn thr_txie(&self) -> ThrTxieR {
        ThrTxieR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnTXF_CTL")
            .field("tx_thr", &self.tx_thr())
            .field("thr_txie", &self.thr_txie())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Tx-FIFO Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tx_thr(&mut self) -> TxThrW<SmbnTxfCtlSpec> {
        TxThrW::new(self, 0)
    }
    #[doc = "Bit 6 - Threshold Tx-FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn thr_txie(&mut self) -> ThrTxieW<SmbnTxfCtlSpec> {
        ThrTxieW::new(self, 6)
    }
}
#[doc = "SMB Tx-FIFO Control Register (SMBnTXF_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_txf_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_txf_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnTxfCtlSpec;
impl crate::RegisterSpec for SmbnTxfCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_txf_ctl::R`](R) reader structure"]
impl crate::Readable for SmbnTxfCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_txf_ctl::W`](W) writer structure"]
impl crate::Writable for SmbnTxfCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnTXF_CTL to value 0"]
impl crate::Resettable for SmbnTxfCtlSpec {
    const RESET_VALUE: u8 = 0;
}
