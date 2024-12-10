#[doc = "Register `SMCE_CTL` reader"]
pub type R = crate::R<SmceCtlSpec>;
#[doc = "Register `SMCE_CTL` writer"]
pub type W = crate::W<SmceCtlSpec>;
#[doc = "Field `HEVREG3_EN` reader - Host Event Registers Window 3 Enable"]
pub type Hevreg3EnR = crate::BitReader;
#[doc = "Field `HEVREG3_EN` writer - Host Event Registers Window 3 Enable"]
pub type Hevreg3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEVREG4_EN` reader - Host Event Registers Window 4 Enable"]
pub type Hevreg4EnR = crate::BitReader;
#[doc = "Field `HEVREG4_EN` writer - Host Event Registers Window 4 Enable"]
pub type Hevreg4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEM3_IE` reader - Enable Interrupt by Host Semaphore 3 Written"]
pub type Hsem3IeR = crate::BitReader;
#[doc = "Field `HSEM3_IE` writer - Enable Interrupt by Host Semaphore 3 Written"]
pub type Hsem3IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEM4_IE` reader - Enable Interrupt by Host Semaphore 4 Written"]
pub type Hsem4IeR = crate::BitReader;
#[doc = "Field `HSEM4_IE` writer - Enable Interrupt by Host Semaphore 4 Written"]
pub type Hsem4IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host Event Registers Window 3 Enable"]
    #[inline(always)]
    pub fn hevreg3_en(&self) -> Hevreg3EnR {
        Hevreg3EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Event Registers Window 4 Enable"]
    #[inline(always)]
    pub fn hevreg4_en(&self) -> Hevreg4EnR {
        Hevreg4EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Interrupt by Host Semaphore 3 Written"]
    #[inline(always)]
    pub fn hsem3_ie(&self) -> Hsem3IeR {
        Hsem3IeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Interrupt by Host Semaphore 4 Written"]
    #[inline(always)]
    pub fn hsem4_ie(&self) -> Hsem4IeR {
        Hsem4IeR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCE_CTL")
            .field("hevreg3_en", &self.hevreg3_en())
            .field("hevreg4_en", &self.hevreg4_en())
            .field("hsem3_ie", &self.hsem3_ie())
            .field("hsem4_ie", &self.hsem4_ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Host Event Registers Window 3 Enable"]
    #[inline(always)]
    pub fn hevreg3_en(&mut self) -> Hevreg3EnW<SmceCtlSpec> {
        Hevreg3EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Event Registers Window 4 Enable"]
    #[inline(always)]
    pub fn hevreg4_en(&mut self) -> Hevreg4EnW<SmceCtlSpec> {
        Hevreg4EnW::new(self, 1)
    }
    #[doc = "Bit 3 - Enable Interrupt by Host Semaphore 3 Written"]
    #[inline(always)]
    pub fn hsem3_ie(&mut self) -> Hsem3IeW<SmceCtlSpec> {
        Hsem3IeW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Interrupt by Host Semaphore 4 Written"]
    #[inline(always)]
    pub fn hsem4_ie(&mut self) -> Hsem4IeW<SmceCtlSpec> {
        Hsem4IeW::new(self, 4)
    }
}
#[doc = "Extended Shared Memory Core Control Register (SMCE_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smce_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smce_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmceCtlSpec;
impl crate::RegisterSpec for SmceCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smce_ctl::R`](R) reader structure"]
impl crate::Readable for SmceCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`smce_ctl::W`](W) writer structure"]
impl crate::Writable for SmceCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMCE_CTL to value 0"]
impl crate::Resettable for SmceCtlSpec {
    const RESET_VALUE: u8 = 0;
}
