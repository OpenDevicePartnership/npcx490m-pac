#[doc = "Register `DEVALT1` reader"]
pub type R = crate::R<Devalt1Spec>;
#[doc = "Register `DEVALT1` writer"]
pub type W = crate::W<Devalt1Spec>;
#[doc = "Field `KBRST_SL` reader - KBRST Select"]
pub type KbrstSlR = crate::BitReader;
#[doc = "Field `KBRST_SL` writer - KBRST Select"]
pub type KbrstSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMI_SL` reader - SMI Select"]
pub type SmiSlR = crate::BitReader;
#[doc = "Field `SMI_SL` writer - SMI Select"]
pub type SmiSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EC_SCI_SL` reader - EC_SCI Select"]
pub type EcSciSlR = crate::BitReader;
#[doc = "Field `EC_SCI_SL` writer - EC_SCI Select"]
pub type EcSciSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_PWRGD` reader - No PWRGD Select"]
pub type NoPwrgdR = crate::BitReader;
#[doc = "Field `NO_PWRGD` writer - No PWRGD Select"]
pub type NoPwrgdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_OUT_SL` reader - RESET_OUT Select"]
pub type RstOutSlR = crate::BitReader;
#[doc = "Field `RST_OUT_SL` writer - RESET_OUT Select"]
pub type RstOutSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKRN_SL` reader - CLKRUN Select"]
pub type ClkrnSlR = crate::BitReader;
#[doc = "Field `CLKRN_SL` writer - CLKRUN Select"]
pub type ClkrnSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_LPC_ESPI` reader - No LPC or eSPI Interface-Select"]
pub type NoLpcEspiR = crate::BitReader;
#[doc = "Field `NO_LPC_ESPI` writer - No LPC or eSPI Interface-Select"]
pub type NoLpcEspiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - KBRST Select"]
    #[inline(always)]
    pub fn kbrst_sl(&self) -> KbrstSlR {
        KbrstSlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SMI Select"]
    #[inline(always)]
    pub fn smi_sl(&self) -> SmiSlR {
        SmiSlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EC_SCI Select"]
    #[inline(always)]
    pub fn ec_sci_sl(&self) -> EcSciSlR {
        EcSciSlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No PWRGD Select"]
    #[inline(always)]
    pub fn no_pwrgd(&self) -> NoPwrgdR {
        NoPwrgdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RESET_OUT Select"]
    #[inline(always)]
    pub fn rst_out_sl(&self) -> RstOutSlR {
        RstOutSlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CLKRUN Select"]
    #[inline(always)]
    pub fn clkrn_sl(&self) -> ClkrnSlR {
        ClkrnSlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No LPC or eSPI Interface-Select"]
    #[inline(always)]
    pub fn no_lpc_espi(&self) -> NoLpcEspiR {
        NoLpcEspiR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT1")
            .field("kbrst_sl", &self.kbrst_sl())
            .field("smi_sl", &self.smi_sl())
            .field("ec_sci_sl", &self.ec_sci_sl())
            .field("no_pwrgd", &self.no_pwrgd())
            .field("rst_out_sl", &self.rst_out_sl())
            .field("clkrn_sl", &self.clkrn_sl())
            .field("no_lpc_espi", &self.no_lpc_espi())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - KBRST Select"]
    #[inline(always)]
    pub fn kbrst_sl(&mut self) -> KbrstSlW<Devalt1Spec> {
        KbrstSlW::new(self, 0)
    }
    #[doc = "Bit 2 - SMI Select"]
    #[inline(always)]
    pub fn smi_sl(&mut self) -> SmiSlW<Devalt1Spec> {
        SmiSlW::new(self, 2)
    }
    #[doc = "Bit 3 - EC_SCI Select"]
    #[inline(always)]
    pub fn ec_sci_sl(&mut self) -> EcSciSlW<Devalt1Spec> {
        EcSciSlW::new(self, 3)
    }
    #[doc = "Bit 4 - No PWRGD Select"]
    #[inline(always)]
    pub fn no_pwrgd(&mut self) -> NoPwrgdW<Devalt1Spec> {
        NoPwrgdW::new(self, 4)
    }
    #[doc = "Bit 5 - RESET_OUT Select"]
    #[inline(always)]
    pub fn rst_out_sl(&mut self) -> RstOutSlW<Devalt1Spec> {
        RstOutSlW::new(self, 5)
    }
    #[doc = "Bit 6 - CLKRUN Select"]
    #[inline(always)]
    pub fn clkrn_sl(&mut self) -> ClkrnSlW<Devalt1Spec> {
        ClkrnSlW::new(self, 6)
    }
    #[doc = "Bit 7 - No LPC or eSPI Interface-Select"]
    #[inline(always)]
    pub fn no_lpc_espi(&mut self) -> NoLpcEspiW<Devalt1Spec> {
        NoLpcEspiW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 1 Register (DEVALT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt1Spec;
impl crate::RegisterSpec for Devalt1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt1::R`](R) reader structure"]
impl crate::Readable for Devalt1Spec {}
#[doc = "`write(|w| ..)` method takes [`devalt1::W`](W) writer structure"]
impl crate::Writable for Devalt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT1 to value 0"]
impl crate::Resettable for Devalt1Spec {
    const RESET_VALUE: u8 = 0;
}
