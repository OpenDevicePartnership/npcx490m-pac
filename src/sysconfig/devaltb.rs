#[doc = "Register `DEVALTB` reader"]
pub type R = crate::R<DevaltbSpec>;
#[doc = "Register `DEVALTB` writer"]
pub type W = crate::W<DevaltbSpec>;
#[doc = "Field `RXD_SL` reader - RXD Select"]
pub type RxdSlR = crate::BitReader;
#[doc = "Field `RXD_SL` writer - RXD Select"]
pub type RxdSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXD_SL` reader - TXD Select"]
pub type TxdSlR = crate::BitReader;
#[doc = "Field `TXD_SL` writer - TXD Select"]
pub type TxdSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_SL` reader - RTS Select"]
pub type RtsSlR = crate::BitReader;
#[doc = "Field `RTS_SL` writer - RTS Select"]
pub type RtsSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_SL` reader - CTS Select"]
pub type CtsSlR = crate::BitReader;
#[doc = "Field `CTS_SL` writer - CTS Select"]
pub type CtsSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI_SL` reader - RI Select"]
pub type RiSlR = crate::BitReader;
#[doc = "Field `RI_SL` writer - RI Select"]
pub type RiSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR_BOUT_SL` reader - DTR_BOUT Select"]
pub type DtrBoutSlR = crate::BitReader;
#[doc = "Field `DTR_BOUT_SL` writer - DTR_BOUT Select"]
pub type DtrBoutSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD_SL` reader - DCD Select"]
pub type DcdSlR = crate::BitReader;
#[doc = "Field `DCD_SL` writer - DCD Select"]
pub type DcdSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR_SL` reader - DSR Select"]
pub type DsrSlR = crate::BitReader;
#[doc = "Field `DSR_SL` writer - DSR Select"]
pub type DsrSlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXD Select"]
    #[inline(always)]
    pub fn rxd_sl(&self) -> RxdSlR {
        RxdSlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXD Select"]
    #[inline(always)]
    pub fn txd_sl(&self) -> TxdSlR {
        TxdSlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTS Select"]
    #[inline(always)]
    pub fn rts_sl(&self) -> RtsSlR {
        RtsSlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTS Select"]
    #[inline(always)]
    pub fn cts_sl(&self) -> CtsSlR {
        CtsSlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RI Select"]
    #[inline(always)]
    pub fn ri_sl(&self) -> RiSlR {
        RiSlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DTR_BOUT Select"]
    #[inline(always)]
    pub fn dtr_bout_sl(&self) -> DtrBoutSlR {
        DtrBoutSlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DCD Select"]
    #[inline(always)]
    pub fn dcd_sl(&self) -> DcdSlR {
        DcdSlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DSR Select"]
    #[inline(always)]
    pub fn dsr_sl(&self) -> DsrSlR {
        DsrSlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTB")
            .field("rxd_sl", &self.rxd_sl())
            .field("txd_sl", &self.txd_sl())
            .field("rts_sl", &self.rts_sl())
            .field("cts_sl", &self.cts_sl())
            .field("ri_sl", &self.ri_sl())
            .field("dtr_bout_sl", &self.dtr_bout_sl())
            .field("dcd_sl", &self.dcd_sl())
            .field("dsr_sl", &self.dsr_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RXD Select"]
    #[inline(always)]
    pub fn rxd_sl(&mut self) -> RxdSlW<DevaltbSpec> {
        RxdSlW::new(self, 0)
    }
    #[doc = "Bit 1 - TXD Select"]
    #[inline(always)]
    pub fn txd_sl(&mut self) -> TxdSlW<DevaltbSpec> {
        TxdSlW::new(self, 1)
    }
    #[doc = "Bit 2 - RTS Select"]
    #[inline(always)]
    pub fn rts_sl(&mut self) -> RtsSlW<DevaltbSpec> {
        RtsSlW::new(self, 2)
    }
    #[doc = "Bit 3 - CTS Select"]
    #[inline(always)]
    pub fn cts_sl(&mut self) -> CtsSlW<DevaltbSpec> {
        CtsSlW::new(self, 3)
    }
    #[doc = "Bit 4 - RI Select"]
    #[inline(always)]
    pub fn ri_sl(&mut self) -> RiSlW<DevaltbSpec> {
        RiSlW::new(self, 4)
    }
    #[doc = "Bit 5 - DTR_BOUT Select"]
    #[inline(always)]
    pub fn dtr_bout_sl(&mut self) -> DtrBoutSlW<DevaltbSpec> {
        DtrBoutSlW::new(self, 5)
    }
    #[doc = "Bit 6 - DCD Select"]
    #[inline(always)]
    pub fn dcd_sl(&mut self) -> DcdSlW<DevaltbSpec> {
        DcdSlW::new(self, 6)
    }
    #[doc = "Bit 7 - DSR Select"]
    #[inline(always)]
    pub fn dsr_sl(&mut self) -> DsrSlW<DevaltbSpec> {
        DsrSlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function B Register (DEVALTB)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltbSpec;
impl crate::RegisterSpec for DevaltbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltb::R`](R) reader structure"]
impl crate::Readable for DevaltbSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltb::W`](W) writer structure"]
impl crate::Writable for DevaltbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTB to value 0"]
impl crate::Resettable for DevaltbSpec {
    const RESET_VALUE: u8 = 0;
}
