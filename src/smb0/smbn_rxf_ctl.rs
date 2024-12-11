#[doc = "Register `SMBnRXF_CTL` reader"]
pub type R = crate::R<SmbnRxfCtlSpec>;
#[doc = "Register `SMBnRXF_CTL` writer"]
pub type W = crate::W<SmbnRxfCtlSpec>;
#[doc = "Field `RX_THR` reader - Rx-FIFO Threshold"]
pub type RxThrR = crate::FieldReader;
#[doc = "Field `RX_THR` writer - Rx-FIFO Threshold"]
pub type RxThrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `THR_RXIE` reader - Threshold Rx-FIFO Interrupt Enable"]
pub type ThrRxieR = crate::BitReader;
#[doc = "Field `THR_RXIE` writer - Threshold Rx-FIFO Interrupt Enable"]
pub type ThrRxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST_PEC` reader - Last Byte or PEC Byte"]
pub type LastPecR = crate::BitReader;
#[doc = "Field `LAST_PEC` writer - Last Byte or PEC Byte"]
pub type LastPecW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Rx-FIFO Threshold"]
    #[inline(always)]
    pub fn rx_thr(&self) -> RxThrR {
        RxThrR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Threshold Rx-FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn thr_rxie(&self) -> ThrRxieR {
        ThrRxieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Last Byte or PEC Byte"]
    #[inline(always)]
    pub fn last_pec(&self) -> LastPecR {
        LastPecR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnRXF_CTL")
            .field("rx_thr", &self.rx_thr())
            .field("thr_rxie", &self.thr_rxie())
            .field("last_pec", &self.last_pec())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx-FIFO Threshold"]
    #[inline(always)]
    pub fn rx_thr(&mut self) -> RxThrW<SmbnRxfCtlSpec> {
        RxThrW::new(self, 0)
    }
    #[doc = "Bit 6 - Threshold Rx-FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn thr_rxie(&mut self) -> ThrRxieW<SmbnRxfCtlSpec> {
        ThrRxieW::new(self, 6)
    }
    #[doc = "Bit 7 - Last Byte or PEC Byte"]
    #[inline(always)]
    pub fn last_pec(&mut self) -> LastPecW<SmbnRxfCtlSpec> {
        LastPecW::new(self, 7)
    }
}
#[doc = "SMB Rx-FIFO Control Register (SMBnRXF_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_rxf_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_rxf_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnRxfCtlSpec;
impl crate::RegisterSpec for SmbnRxfCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_rxf_ctl::R`](R) reader structure"]
impl crate::Readable for SmbnRxfCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_rxf_ctl::W`](W) writer structure"]
impl crate::Writable for SmbnRxfCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnRXF_CTL to value 0"]
impl crate::Resettable for SmbnRxfCtlSpec {
    const RESET_VALUE: u8 = 0;
}
