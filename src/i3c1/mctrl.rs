#[doc = "Register `MCTRL` reader"]
pub type R = crate::R<MctrlSpec>;
#[doc = "Register `MCTRL` writer"]
pub type W = crate::W<MctrlSpec>;
#[doc = "Field `REQUEST` reader - Bus Operation Request"]
pub type RequestR = crate::FieldReader;
#[doc = "Field `REQUEST` writer - Bus Operation Request"]
pub type RequestW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TYPE` reader - Operation Type"]
pub type TypeR = crate::FieldReader;
#[doc = "Field `TYPE` writer - Operation Type"]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IBIRESP` reader - IBI Response"]
pub type IbirespR = crate::FieldReader;
#[doc = "Field `IBIRESP` writer - IBI Response"]
pub type IbirespW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIR` reader - Transfer Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Transfer Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RDTERM` reader - Read Termination Counter"]
pub type RdtermR = crate::FieldReader;
#[doc = "Field `RDTERM` writer - Read Termination Counter"]
pub type RdtermW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Bus Operation Request"]
    #[inline(always)]
    pub fn request(&self) -> RequestR {
        RequestR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Operation Type"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - IBI Response"]
    #[inline(always)]
    pub fn ibiresp(&self) -> IbirespR {
        IbirespR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Read Termination Counter"]
    #[inline(always)]
    pub fn rdterm(&self) -> RdtermR {
        RdtermR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCTRL")
            .field("request", &self.request())
            .field("type_", &self.type_())
            .field("ibiresp", &self.ibiresp())
            .field("dir", &self.dir())
            .field("addr", &self.addr())
            .field("rdterm", &self.rdterm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Bus Operation Request"]
    #[inline(always)]
    #[must_use]
    pub fn request(&mut self) -> RequestW<MctrlSpec> {
        RequestW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Operation Type"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TypeW<MctrlSpec> {
        TypeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - IBI Response"]
    #[inline(always)]
    #[must_use]
    pub fn ibiresp(&mut self) -> IbirespW<MctrlSpec> {
        IbirespW::new(self, 6)
    }
    #[doc = "Bit 8 - Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<MctrlSpec> {
        DirW::new(self, 8)
    }
    #[doc = "Bits 9:13 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<MctrlSpec> {
        AddrW::new(self, 9)
    }
    #[doc = "Bits 16:23 - Read Termination Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rdterm(&mut self) -> RdtermW<MctrlSpec> {
        RdtermW::new(self, 16)
    }
}
#[doc = "Controller Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctrlSpec;
impl crate::RegisterSpec for MctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctrl::R`](R) reader structure"]
impl crate::Readable for MctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mctrl::W`](W) writer structure"]
impl crate::Writable for MctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MctrlSpec {
    const RESET_VALUE: u32 = 0;
}
