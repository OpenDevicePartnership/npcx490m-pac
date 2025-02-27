#[doc = "Register `FLM_CMD0` reader"]
pub type R = crate::R<FlmCmd0Spec>;
#[doc = "Register `FLM_CMD0` writer"]
pub type W = crate::W<FlmCmd0Spec>;
#[doc = "Field `CMD` reader - Command Byte"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - Command Byte"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDSZ` reader - Address Size for Limiting Command"]
pub type AddszR = crate::BitReader;
#[doc = "Field `ADDSZ` writer - Address Size for Limiting Command"]
pub type AddszW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATBPCK` reader - Data Bits Per Clock"]
pub type DatbpckR = crate::FieldReader;
#[doc = "Field `DATBPCK` writer - Data Bits Per Clock"]
pub type DatbpckW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADBPCK` reader - Address Bits Per Clock"]
pub type AdbpckR = crate::FieldReader;
#[doc = "Field `ADBPCK` writer - Address Bits Per Clock"]
pub type AdbpckW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DUMBPCK` reader - Dummy Bits Per Clock"]
pub type DumbpckR = crate::FieldReader;
#[doc = "Field `DUMBPCK` writer - Dummy Bits Per Clock"]
pub type DumbpckW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DUMB` reader - Dummy Bytes"]
pub type DumbR = crate::FieldReader;
#[doc = "Field `DUMB` writer - Dummy Bytes"]
pub type DumbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLAR` reader - Command Limited Address Range"]
pub type ClarR = crate::BitReader;
#[doc = "Field `CLAR` writer - Command Limited Address Range"]
pub type ClarW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command Byte"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 11 - Address Size for Limiting Command"]
    #[inline(always)]
    pub fn addsz(&self) -> AddszR {
        AddszR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Data Bits Per Clock"]
    #[inline(always)]
    pub fn datbpck(&self) -> DatbpckR {
        DatbpckR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Address Bits Per Clock"]
    #[inline(always)]
    pub fn adbpck(&self) -> AdbpckR {
        AdbpckR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Dummy Bits Per Clock"]
    #[inline(always)]
    pub fn dumbpck(&self) -> DumbpckR {
        DumbpckR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Dummy Bytes"]
    #[inline(always)]
    pub fn dumb(&self) -> DumbR {
        DumbR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Command Limited Address Range"]
    #[inline(always)]
    pub fn clar(&self) -> ClarR {
        ClarR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_CMD0")
            .field("cmd", &self.cmd())
            .field("addsz", &self.addsz())
            .field("datbpck", &self.datbpck())
            .field("adbpck", &self.adbpck())
            .field("dumbpck", &self.dumbpck())
            .field("dumb", &self.dumb())
            .field("clar", &self.clar())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Command Byte"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<FlmCmd0Spec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bit 11 - Address Size for Limiting Command"]
    #[inline(always)]
    pub fn addsz(&mut self) -> AddszW<FlmCmd0Spec> {
        AddszW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Data Bits Per Clock"]
    #[inline(always)]
    pub fn datbpck(&mut self) -> DatbpckW<FlmCmd0Spec> {
        DatbpckW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Address Bits Per Clock"]
    #[inline(always)]
    pub fn adbpck(&mut self) -> AdbpckW<FlmCmd0Spec> {
        AdbpckW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Dummy Bits Per Clock"]
    #[inline(always)]
    pub fn dumbpck(&mut self) -> DumbpckW<FlmCmd0Spec> {
        DumbpckW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Dummy Bytes"]
    #[inline(always)]
    pub fn dumb(&mut self) -> DumbW<FlmCmd0Spec> {
        DumbW::new(self, 18)
    }
    #[doc = "Bit 20 - Command Limited Address Range"]
    #[inline(always)]
    pub fn clar(&mut self) -> ClarW<FlmCmd0Spec> {
        ClarW::new(self, 20)
    }
}
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCmd0Spec;
impl crate::RegisterSpec for FlmCmd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cmd0::R`](R) reader structure"]
impl crate::Readable for FlmCmd0Spec {}
#[doc = "`write(|w| ..)` method takes [`flm_cmd0::W`](W) writer structure"]
impl crate::Writable for FlmCmd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CMD0 to value 0"]
impl crate::Resettable for FlmCmd0Spec {
    const RESET_VALUE: u32 = 0;
}
