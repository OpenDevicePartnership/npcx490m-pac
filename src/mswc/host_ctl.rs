#[doc = "Register `HOST_CTL` reader"]
pub type R = crate::R<HostCtlSpec>;
#[doc = "Register `HOST_CTL` writer"]
pub type W = crate::W<HostCtlSpec>;
#[doc = "Field `SMI_INV` reader - SMI Invert"]
pub type SmiInvR = crate::BitReader;
#[doc = "Field `SMI_INV` writer - SMI Invert"]
pub type SmiInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCI_INV` reader - EC_SCI Invert"]
pub type SciInvR = crate::BitReader;
#[doc = "Field `SCI_INV` writer - EC_SCI Invert"]
pub type SciInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSMI_SET` reader - SMI Level Set"]
pub type NsmiSetR = crate::BitReader;
#[doc = "Field `NSMI_SET` writer - SMI Level Set"]
pub type NsmiSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SMI Invert"]
    #[inline(always)]
    pub fn smi_inv(&self) -> SmiInvR {
        SmiInvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - EC_SCI Invert"]
    #[inline(always)]
    pub fn sci_inv(&self) -> SciInvR {
        SciInvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMI Level Set"]
    #[inline(always)]
    pub fn nsmi_set(&self) -> NsmiSetR {
        NsmiSetR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_CTL")
            .field("smi_inv", &self.smi_inv())
            .field("sci_inv", &self.sci_inv())
            .field("nsmi_set", &self.nsmi_set())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - SMI Invert"]
    #[inline(always)]
    pub fn smi_inv(&mut self) -> SmiInvW<HostCtlSpec> {
        SmiInvW::new(self, 1)
    }
    #[doc = "Bit 3 - EC_SCI Invert"]
    #[inline(always)]
    pub fn sci_inv(&mut self) -> SciInvW<HostCtlSpec> {
        SciInvW::new(self, 3)
    }
    #[doc = "Bit 4 - SMI Level Set"]
    #[inline(always)]
    pub fn nsmi_set(&mut self) -> NsmiSetW<HostCtlSpec> {
        NsmiSetW::new(self, 4)
    }
}
#[doc = "Host Control Register (HOST_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCtlSpec;
impl crate::RegisterSpec for HostCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`host_ctl::R`](R) reader structure"]
impl crate::Readable for HostCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`host_ctl::W`](W) writer structure"]
impl crate::Writable for HostCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOST_CTL to value 0"]
impl crate::Resettable for HostCtlSpec {
    const RESET_VALUE: u8 = 0;
}
