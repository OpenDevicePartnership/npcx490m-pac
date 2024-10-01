#[doc = "Register `UMA_DB0_3` reader"]
pub type R = crate::R<UmaDb0_3Spec>;
#[doc = "Register `UMA_DB0_3` writer"]
pub type W = crate::W<UmaDb0_3Spec>;
#[doc = "Field `DAT_B0` reader - Data Byte 0"]
pub type DatB0R = crate::FieldReader;
#[doc = "Field `DAT_B0` writer - Data Byte 0"]
pub type DatB0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DAT_B1` reader - Data Byte 1"]
pub type DatB1R = crate::FieldReader;
#[doc = "Field `DAT_B1` writer - Data Byte 1"]
pub type DatB1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DAT_B2` reader - Data Byte 2"]
pub type DatB2R = crate::FieldReader;
#[doc = "Field `DAT_B2` writer - Data Byte 2"]
pub type DatB2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DAT_B3` reader - Data Byte 3"]
pub type DatB3R = crate::FieldReader;
#[doc = "Field `DAT_B3` writer - Data Byte 3"]
pub type DatB3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 0"]
    #[inline(always)]
    pub fn dat_b0(&self) -> DatB0R {
        DatB0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 1"]
    #[inline(always)]
    pub fn dat_b1(&self) -> DatB1R {
        DatB1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Byte 2"]
    #[inline(always)]
    pub fn dat_b2(&self) -> DatB2R {
        DatB2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data Byte 3"]
    #[inline(always)]
    pub fn dat_b3(&self) -> DatB3R {
        DatB3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UMA_DB0_3")
            .field("dat_b0", &self.dat_b0())
            .field("dat_b1", &self.dat_b1())
            .field("dat_b2", &self.dat_b2())
            .field("dat_b3", &self.dat_b3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn dat_b0(&mut self) -> DatB0W<UmaDb0_3Spec> {
        DatB0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn dat_b1(&mut self) -> DatB1W<UmaDb0_3Spec> {
        DatB1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data Byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn dat_b2(&mut self) -> DatB2W<UmaDb0_3Spec> {
        DatB2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data Byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn dat_b3(&mut self) -> DatB3W<UmaDb0_3Spec> {
        DatB3W::new(self, 24)
    }
}
#[doc = "UMA Data Bytes 0-3 Register (UMA_DB0_3)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_db0_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_db0_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UmaDb0_3Spec;
impl crate::RegisterSpec for UmaDb0_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uma_db0_3::R`](R) reader structure"]
impl crate::Readable for UmaDb0_3Spec {}
#[doc = "`write(|w| ..)` method takes [`uma_db0_3::W`](W) writer structure"]
impl crate::Writable for UmaDb0_3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UMA_DB0_3 to value 0"]
impl crate::Resettable for UmaDb0_3Spec {
    const RESET_VALUE: u32 = 0;
}
