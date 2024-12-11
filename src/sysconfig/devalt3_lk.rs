#[doc = "Register `DEVALT3_LK` reader"]
pub type R = crate::R<Devalt3LkSpec>;
#[doc = "Register `DEVALT3_LK` writer"]
pub type W = crate::W<Devalt3LkSpec>;
#[doc = "Field `PS2_1_SL_LK` reader - PS/2 #1 Select Lock"]
pub type Ps2_1SlLkR = crate::BitReader;
#[doc = "Field `PS2_1_SL_LK` writer - PS/2 #1 Select Lock"]
pub type Ps2_1SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_2_SL_LK` reader - PS/2 #2 Select Lock"]
pub type Ps2_2SlLkR = crate::BitReader;
#[doc = "Field `PS2_2_SL_LK` writer - PS/2 #2 Select Lock"]
pub type Ps2_2SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA1_TACH1_SL1_LK` reader - TA1_TACH1 Select-1 Lock"]
pub type Ta1Tach1Sl1LkR = crate::BitReader;
#[doc = "Field `TA1_TACH1_SL1_LK` writer - TA1_TACH1 Select-1 Lock"]
pub type Ta1Tach1Sl1LkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - PS/2 #1 Select Lock"]
    #[inline(always)]
    pub fn ps2_1_sl_lk(&self) -> Ps2_1SlLkR {
        Ps2_1SlLkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PS/2 #2 Select Lock"]
    #[inline(always)]
    pub fn ps2_2_sl_lk(&self) -> Ps2_2SlLkR {
        Ps2_2SlLkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TA1_TACH1 Select-1 Lock"]
    #[inline(always)]
    pub fn ta1_tach1_sl1_lk(&self) -> Ta1Tach1Sl1LkR {
        Ta1Tach1Sl1LkR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT3_LK")
            .field("ps2_1_sl_lk", &self.ps2_1_sl_lk())
            .field("ps2_2_sl_lk", &self.ps2_2_sl_lk())
            .field("ta1_tach1_sl1_lk", &self.ta1_tach1_sl1_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - PS/2 #1 Select Lock"]
    #[inline(always)]
    pub fn ps2_1_sl_lk(&mut self) -> Ps2_1SlLkW<Devalt3LkSpec> {
        Ps2_1SlLkW::new(self, 1)
    }
    #[doc = "Bit 2 - PS/2 #2 Select Lock"]
    #[inline(always)]
    pub fn ps2_2_sl_lk(&mut self) -> Ps2_2SlLkW<Devalt3LkSpec> {
        Ps2_2SlLkW::new(self, 2)
    }
    #[doc = "Bit 4 - TA1_TACH1 Select-1 Lock"]
    #[inline(always)]
    pub fn ta1_tach1_sl1_lk(&mut self) -> Ta1Tach1Sl1LkW<Devalt3LkSpec> {
        Ta1Tach1Sl1LkW::new(self, 4)
    }
}
#[doc = "Device Alternate Function 3 Lock Register (DEVALT3_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt3_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt3_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt3LkSpec;
impl crate::RegisterSpec for Devalt3LkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt3_lk::R`](R) reader structure"]
impl crate::Readable for Devalt3LkSpec {}
#[doc = "`write(|w| ..)` method takes [`devalt3_lk::W`](W) writer structure"]
impl crate::Writable for Devalt3LkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT3_LK to value 0"]
impl crate::Resettable for Devalt3LkSpec {
    const RESET_VALUE: u8 = 0;
}
