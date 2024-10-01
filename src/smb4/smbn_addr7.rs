#[doc = "Register `SMBnADDR7` reader"]
pub type R = crate::R<SmbnAddr7Spec>;
#[doc = "Register `SMBnADDR7` writer"]
pub type W = crate::W<SmbnAddr7Spec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SAEN` reader - Slave Address Enable"]
pub type SaenR = crate::BitReader;
#[doc = "Field `SAEN` writer - Slave Address Enable"]
pub type SaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Slave Address Enable"]
    #[inline(always)]
    pub fn saen(&self) -> SaenR {
        SaenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnADDR7")
            .field("addr", &self.addr())
            .field("saen", &self.saen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<SmbnAddr7Spec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 7 - Slave Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn saen(&mut self) -> SaenW<SmbnAddr7Spec> {
        SaenW::new(self, 7)
    }
}
#[doc = "SMB Own Address 1 - 8 Register (SMBnADDR1-8)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_addr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_addr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnAddr7Spec;
impl crate::RegisterSpec for SmbnAddr7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_addr7::R`](R) reader structure"]
impl crate::Readable for SmbnAddr7Spec {}
#[doc = "`write(|w| ..)` method takes [`smbn_addr7::W`](W) writer structure"]
impl crate::Writable for SmbnAddr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnADDR7 to value 0"]
impl crate::Resettable for SmbnAddr7Spec {
    const RESET_VALUE: u8 = 0;
}
