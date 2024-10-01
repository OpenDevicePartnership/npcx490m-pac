#[doc = "Register `MRDATAH` reader"]
pub type R = crate::R<MrdatahSpec>;
#[doc = "Register `MRDATAH` writer"]
pub type W = crate::W<MrdatahSpec>;
#[doc = "Field `DATA0` reader - First Data Byte"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA0` writer - First Data Byte"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` reader - Second Data Byte"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - Second Data Byte"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - First Data Byte"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Second Data Byte"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRDATAH")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - First Data Byte"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<MrdatahSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Second Data Byte"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<MrdatahSpec> {
        Data1W::new(self, 8)
    }
}
#[doc = "Controller Read Half-Word Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdatah::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrdatah::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrdatahSpec;
impl crate::RegisterSpec for MrdatahSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrdatah::R`](R) reader structure"]
impl crate::Readable for MrdatahSpec {}
#[doc = "`write(|w| ..)` method takes [`mrdatah::W`](W) writer structure"]
impl crate::Writable for MrdatahSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRDATAH to value 0"]
impl crate::Resettable for MrdatahSpec {
    const RESET_VALUE: u32 = 0;
}
