#[doc = "Register `MWDATAH` reader"]
pub type R = crate::R<MwdatahSpec>;
#[doc = "Register `MWDATAH` writer"]
pub type W = crate::W<MwdatahSpec>;
#[doc = "Field `DATA0` reader - First Data Byte"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA0` writer - First Data Byte"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` reader - Second Data Byte"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - Second Data Byte"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `END` reader - End of Data"]
pub type EndR = crate::BitReader;
#[doc = "Field `END` writer - End of Data"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 16 - End of Data"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MWDATAH")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .field("end", &self.end())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - First Data Byte"]
    #[inline(always)]
    pub fn data0(&mut self) -> Data0W<MwdatahSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Second Data Byte"]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<MwdatahSpec> {
        Data1W::new(self, 8)
    }
    #[doc = "Bit 16 - End of Data"]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<MwdatahSpec> {
        EndW::new(self, 16)
    }
}
#[doc = "Controller Write Half-Word Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mwdatah::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatah::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwdatahSpec;
impl crate::RegisterSpec for MwdatahSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mwdatah::R`](R) reader structure"]
impl crate::Readable for MwdatahSpec {}
#[doc = "`write(|w| ..)` method takes [`mwdatah::W`](W) writer structure"]
impl crate::Writable for MwdatahSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWDATAH to value 0"]
impl crate::Resettable for MwdatahSpec {
    const RESET_VALUE: u32 = 0;
}
