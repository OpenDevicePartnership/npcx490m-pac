#[doc = "Register `UMA_AB0_3` reader"]
pub type R = crate::R<UmaAb0_3Spec>;
#[doc = "Register `UMA_AB0_3` writer"]
pub type W = crate::W<UmaAb0_3Spec>;
#[doc = "Field `ADDR_B0` reader - Address Byte 0"]
pub type AddrB0R = crate::FieldReader;
#[doc = "Field `ADDR_B0` writer - Address Byte 0"]
pub type AddrB0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR_B1` reader - Address Byte 1"]
pub type AddrB1R = crate::FieldReader;
#[doc = "Field `ADDR_B1` writer - Address Byte 1"]
pub type AddrB1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR_B2` reader - Address Byte 2"]
pub type AddrB2R = crate::FieldReader;
#[doc = "Field `ADDR_B2` writer - Address Byte 2"]
pub type AddrB2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR_B3` reader - Address Byte 3"]
pub type AddrB3R = crate::FieldReader;
#[doc = "Field `ADDR_B3` writer - Address Byte 3"]
pub type AddrB3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Address Byte 0"]
    #[inline(always)]
    pub fn addr_b0(&self) -> AddrB0R {
        AddrB0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address Byte 1"]
    #[inline(always)]
    pub fn addr_b1(&self) -> AddrB1R {
        AddrB1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address Byte 2"]
    #[inline(always)]
    pub fn addr_b2(&self) -> AddrB2R {
        AddrB2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Address Byte 3"]
    #[inline(always)]
    pub fn addr_b3(&self) -> AddrB3R {
        AddrB3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UMA_AB0_3")
            .field("addr_b0", &self.addr_b0())
            .field("addr_b1", &self.addr_b1())
            .field("addr_b2", &self.addr_b2())
            .field("addr_b3", &self.addr_b3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Address Byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn addr_b0(&mut self) -> AddrB0W<UmaAb0_3Spec> {
        AddrB0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Address Byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn addr_b1(&mut self) -> AddrB1W<UmaAb0_3Spec> {
        AddrB1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Address Byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn addr_b2(&mut self) -> AddrB2W<UmaAb0_3Spec> {
        AddrB2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Address Byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn addr_b3(&mut self) -> AddrB3W<UmaAb0_3Spec> {
        AddrB3W::new(self, 24)
    }
}
#[doc = "UMA Address Byte 0_3 Register (UMA_AB0_3)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_ab0_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_ab0_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UmaAb0_3Spec;
impl crate::RegisterSpec for UmaAb0_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uma_ab0_3::R`](R) reader structure"]
impl crate::Readable for UmaAb0_3Spec {}
#[doc = "`write(|w| ..)` method takes [`uma_ab0_3::W`](W) writer structure"]
impl crate::Writable for UmaAb0_3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UMA_AB0_3 to value 0"]
impl crate::Resettable for UmaAb0_3Spec {
    const RESET_VALUE: u32 = 0;
}
