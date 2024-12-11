#[doc = "Register `DEVALTB_LK` reader"]
pub type R = crate::R<DevaltbLkSpec>;
#[doc = "Register `DEVALTB_LK` writer"]
pub type W = crate::W<DevaltbLkSpec>;
#[doc = "Field `RXD_SL_LK` reader - RXD Select Lock"]
pub type RxdSlLkR = crate::BitReader;
#[doc = "Field `RXD_SL_LK` writer - RXD Select Lock"]
pub type RxdSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXD_SL_LK` reader - TXD Select Lock"]
pub type TxdSlLkR = crate::BitReader;
#[doc = "Field `TXD_SL_LK` writer - TXD Select Lock"]
pub type TxdSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_SL_LK` reader - RTS Select Lock"]
pub type RtsSlLkR = crate::BitReader;
#[doc = "Field `RTS_SL_LK` writer - RTS Select Lock"]
pub type RtsSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_SL_LK` reader - CTS Select Lock"]
pub type CtsSlLkR = crate::BitReader;
#[doc = "Field `CTS_SL_LK` writer - CTS Select Lock"]
pub type CtsSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD_SL_LK` reader - DCD Select Lock"]
pub type DcdSlLkR = crate::BitReader;
#[doc = "Field `DCD_SL_LK` writer - DCD Select Lock"]
pub type DcdSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR_SL_LK` reader - DSR Select Lock"]
pub type DsrSlLkR = crate::BitReader;
#[doc = "Field `DSR_SL_LK` writer - DSR Select Lock"]
pub type DsrSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXD Select Lock"]
    #[inline(always)]
    pub fn rxd_sl_lk(&self) -> RxdSlLkR {
        RxdSlLkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXD Select Lock"]
    #[inline(always)]
    pub fn txd_sl_lk(&self) -> TxdSlLkR {
        TxdSlLkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTS Select Lock"]
    #[inline(always)]
    pub fn rts_sl_lk(&self) -> RtsSlLkR {
        RtsSlLkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTS Select Lock"]
    #[inline(always)]
    pub fn cts_sl_lk(&self) -> CtsSlLkR {
        CtsSlLkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DCD Select Lock"]
    #[inline(always)]
    pub fn dcd_sl_lk(&self) -> DcdSlLkR {
        DcdSlLkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DSR Select Lock"]
    #[inline(always)]
    pub fn dsr_sl_lk(&self) -> DsrSlLkR {
        DsrSlLkR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTB_LK")
            .field("rxd_sl_lk", &self.rxd_sl_lk())
            .field("txd_sl_lk", &self.txd_sl_lk())
            .field("rts_sl_lk", &self.rts_sl_lk())
            .field("cts_sl_lk", &self.cts_sl_lk())
            .field("dcd_sl_lk", &self.dcd_sl_lk())
            .field("dsr_sl_lk", &self.dsr_sl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RXD Select Lock"]
    #[inline(always)]
    pub fn rxd_sl_lk(&mut self) -> RxdSlLkW<DevaltbLkSpec> {
        RxdSlLkW::new(self, 0)
    }
    #[doc = "Bit 1 - TXD Select Lock"]
    #[inline(always)]
    pub fn txd_sl_lk(&mut self) -> TxdSlLkW<DevaltbLkSpec> {
        TxdSlLkW::new(self, 1)
    }
    #[doc = "Bit 2 - RTS Select Lock"]
    #[inline(always)]
    pub fn rts_sl_lk(&mut self) -> RtsSlLkW<DevaltbLkSpec> {
        RtsSlLkW::new(self, 2)
    }
    #[doc = "Bit 3 - CTS Select Lock"]
    #[inline(always)]
    pub fn cts_sl_lk(&mut self) -> CtsSlLkW<DevaltbLkSpec> {
        CtsSlLkW::new(self, 3)
    }
    #[doc = "Bit 6 - DCD Select Lock"]
    #[inline(always)]
    pub fn dcd_sl_lk(&mut self) -> DcdSlLkW<DevaltbLkSpec> {
        DcdSlLkW::new(self, 6)
    }
    #[doc = "Bit 7 - DSR Select Lock"]
    #[inline(always)]
    pub fn dsr_sl_lk(&mut self) -> DsrSlLkW<DevaltbLkSpec> {
        DsrSlLkW::new(self, 7)
    }
}
#[doc = "Device Alternate Function B Lock Register (DEVALTB_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltb_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltb_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltbLkSpec;
impl crate::RegisterSpec for DevaltbLkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltb_lk::R`](R) reader structure"]
impl crate::Readable for DevaltbLkSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltb_lk::W`](W) writer structure"]
impl crate::Writable for DevaltbLkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTB_LK to value 0"]
impl crate::Resettable for DevaltbLkSpec {
    const RESET_VALUE: u8 = 0;
}
