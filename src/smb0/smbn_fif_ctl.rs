#[doc = "Register `SMBnFIF_CTL` reader"]
pub type R = crate::R<SmbnFifCtlSpec>;
#[doc = "Register `SMBnFIF_CTL` writer"]
pub type W = crate::W<SmbnFifCtlSpec>;
#[doc = "Field `FIFO_EN` reader - Enable FIFO Mode"]
pub type FifoEnR = crate::BitReader;
#[doc = "Field `FIFO_EN` writer - Enable FIFO Mode"]
pub type FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Enable FIFO Mode"]
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnFIF_CTL")
            .field("fifo_en", &self.fifo_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Enable FIFO Mode"]
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FifoEnW<SmbnFifCtlSpec> {
        FifoEnW::new(self, 4)
    }
}
#[doc = "SMB FIFO Control Register (SMBnFIF_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_fif_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_fif_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnFifCtlSpec;
impl crate::RegisterSpec for SmbnFifCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_fif_ctl::R`](R) reader structure"]
impl crate::Readable for SmbnFifCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_fif_ctl::W`](W) writer structure"]
impl crate::Writable for SmbnFifCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnFIF_CTL to value 0"]
impl crate::Resettable for SmbnFifCtlSpec {
    const RESET_VALUE: u8 = 0;
}
