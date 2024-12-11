#[doc = "Register `SMBnTXF_STS` reader"]
pub type R = crate::R<SmbnTxfStsSpec>;
#[doc = "Register `SMBnTXF_STS` writer"]
pub type W = crate::W<SmbnTxfStsSpec>;
#[doc = "Field `TX_BYTES` reader - Tx-FIFO Number of Bytes"]
pub type TxBytesR = crate::FieldReader;
#[doc = "Field `TX_BYTES` writer - Tx-FIFO Number of Bytes"]
pub type TxBytesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TX_THST` reader - Tx-FIFO Threshold Status"]
pub type TxThstR = crate::BitReader;
#[doc = "Field `TX_THST` writer - Tx-FIFO Threshold Status"]
pub type TxThstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Tx-FIFO Number of Bytes"]
    #[inline(always)]
    pub fn tx_bytes(&self) -> TxBytesR {
        TxBytesR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Tx-FIFO Threshold Status"]
    #[inline(always)]
    pub fn tx_thst(&self) -> TxThstR {
        TxThstR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnTXF_STS")
            .field("tx_bytes", &self.tx_bytes())
            .field("tx_thst", &self.tx_thst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Tx-FIFO Number of Bytes"]
    #[inline(always)]
    pub fn tx_bytes(&mut self) -> TxBytesW<SmbnTxfStsSpec> {
        TxBytesW::new(self, 0)
    }
    #[doc = "Bit 6 - Tx-FIFO Threshold Status"]
    #[inline(always)]
    pub fn tx_thst(&mut self) -> TxThstW<SmbnTxfStsSpec> {
        TxThstW::new(self, 6)
    }
}
#[doc = "SMB Tx-FIFO Status Register (SMBnTXF_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_txf_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_txf_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnTxfStsSpec;
impl crate::RegisterSpec for SmbnTxfStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_txf_sts::R`](R) reader structure"]
impl crate::Readable for SmbnTxfStsSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_txf_sts::W`](W) writer structure"]
impl crate::Writable for SmbnTxfStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnTXF_STS to value 0"]
impl crate::Resettable for SmbnTxfStsSpec {
    const RESET_VALUE: u8 = 0;
}
