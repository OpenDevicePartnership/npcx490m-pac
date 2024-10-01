#[doc = "Register `CAPABILITY` reader"]
pub type R = crate::R<CapabilitySpec>;
#[doc = "Register `CAPABILITY` writer"]
pub type W = crate::W<CapabilitySpec>;
#[doc = "Field `IBSZ` reader - Input Buffer Size"]
pub type IbszR = crate::FieldReader;
#[doc = "Field `IBSZ` writer - Input Buffer Size"]
pub type IbszW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OBSZ` reader - 128-Byte Payload Buffer Size"]
pub type ObszR = crate::FieldReader;
#[doc = "Field `OBSZ` writer - 128-Byte Payload Buffer Size"]
pub type ObszW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Input Buffer Size"]
    #[inline(always)]
    pub fn ibsz(&self) -> IbszR {
        IbszR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - 128-Byte Payload Buffer Size"]
    #[inline(always)]
    pub fn obsz(&self) -> ObszR {
        ObszR::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPABILITY")
            .field("ibsz", &self.ibsz())
            .field("obsz", &self.obsz())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn ibsz(&mut self) -> IbszW<CapabilitySpec> {
        IbszW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 128-Byte Payload Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn obsz(&mut self) -> ObszW<CapabilitySpec> {
        ObszW::new(self, 4)
    }
}
#[doc = "SHI Capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`capability::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capability::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapabilitySpec;
impl crate::RegisterSpec for CapabilitySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`capability::R`](R) reader structure"]
impl crate::Readable for CapabilitySpec {}
#[doc = "`write(|w| ..)` method takes [`capability::W`](W) writer structure"]
impl crate::Writable for CapabilitySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CAPABILITY to value 0"]
impl crate::Resettable for CapabilitySpec {
    const RESET_VALUE: u8 = 0;
}
