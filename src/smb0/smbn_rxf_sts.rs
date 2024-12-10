#[doc = "Register `SMBnRXF_STS` reader"]
pub type R = crate::R<SmbnRxfStsSpec>;
#[doc = "Register `SMBnRXF_STS` writer"]
pub type W = crate::W<SmbnRxfStsSpec>;
#[doc = "Field `RX_BYTES` reader - Rx-FIFO Number of Bytes"]
pub type RxBytesR = crate::FieldReader;
#[doc = "Field `RX_BYTES` writer - Rx-FIFO Number of Bytes"]
pub type RxBytesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_THST` reader - Rx-FIFO Threshold Status"]
pub type RxThstR = crate::BitReader;
#[doc = "Field `RX_THST` writer - Rx-FIFO Threshold Status"]
pub type RxThstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Rx-FIFO Number of Bytes"]
    #[inline(always)]
    pub fn rx_bytes(&self) -> RxBytesR {
        RxBytesR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Rx-FIFO Threshold Status"]
    #[inline(always)]
    pub fn rx_thst(&self) -> RxThstR {
        RxThstR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnRXF_STS")
            .field("rx_bytes", &self.rx_bytes())
            .field("rx_thst", &self.rx_thst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx-FIFO Number of Bytes"]
    #[inline(always)]
    pub fn rx_bytes(&mut self) -> RxBytesW<SmbnRxfStsSpec> {
        RxBytesW::new(self, 0)
    }
    #[doc = "Bit 6 - Rx-FIFO Threshold Status"]
    #[inline(always)]
    pub fn rx_thst(&mut self) -> RxThstW<SmbnRxfStsSpec> {
        RxThstW::new(self, 6)
    }
}
#[doc = "SMB Rx-FIFO Status Register (SMBnRXF_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_rxf_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_rxf_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnRxfStsSpec;
impl crate::RegisterSpec for SmbnRxfStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_rxf_sts::R`](R) reader structure"]
impl crate::Readable for SmbnRxfStsSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_rxf_sts::W`](W) writer structure"]
impl crate::Writable for SmbnRxfStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnRXF_STS to value 0"]
impl crate::Resettable for SmbnRxfStsSpec {
    const RESET_VALUE: u8 = 0;
}
