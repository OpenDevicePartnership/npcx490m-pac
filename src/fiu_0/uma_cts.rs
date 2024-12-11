#[doc = "Register `UMA_CTS` reader"]
pub type R = crate::R<UmaCtsSpec>;
#[doc = "Register `UMA_CTS` writer"]
pub type W = crate::W<UmaCtsSpec>;
#[doc = "Field `D_SIZE` reader - Data Field Size Select"]
pub type DSizeR = crate::FieldReader;
#[doc = "Field `D_SIZE` writer - Data Field Size Select"]
pub type DSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UMA_F_WR` reader - UMA Fast Write"]
pub type UmaFWrR = crate::BitReader;
#[doc = "Field `UMA_F_WR` writer - UMA Fast Write"]
pub type UmaFWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_SIZE` reader - Code Field Size Select"]
pub type CSizeR = crate::BitReader;
#[doc = "Field `C_SIZE` writer - Code Field Size Select"]
pub type CSizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_WR` reader - Read/Write Select"]
pub type RdWrR = crate::BitReader;
#[doc = "Field `RD_WR` writer - Read/Write Select"]
pub type RdWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEC_DONE` reader - Operation Execute/Done"]
pub type ExecDoneR = crate::BitReader;
#[doc = "Field `EXEC_DONE` writer - Operation Execute/Done"]
pub type ExecDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Data Field Size Select"]
    #[inline(always)]
    pub fn d_size(&self) -> DSizeR {
        DSizeR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - UMA Fast Write"]
    #[inline(always)]
    pub fn uma_f_wr(&self) -> UmaFWrR {
        UmaFWrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Code Field Size Select"]
    #[inline(always)]
    pub fn c_size(&self) -> CSizeR {
        CSizeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read/Write Select"]
    #[inline(always)]
    pub fn rd_wr(&self) -> RdWrR {
        RdWrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Operation Execute/Done"]
    #[inline(always)]
    pub fn exec_done(&self) -> ExecDoneR {
        ExecDoneR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UMA_CTS")
            .field("d_size", &self.d_size())
            .field("uma_f_wr", &self.uma_f_wr())
            .field("c_size", &self.c_size())
            .field("rd_wr", &self.rd_wr())
            .field("exec_done", &self.exec_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Field Size Select"]
    #[inline(always)]
    pub fn d_size(&mut self) -> DSizeW<UmaCtsSpec> {
        DSizeW::new(self, 0)
    }
    #[doc = "Bit 3 - UMA Fast Write"]
    #[inline(always)]
    pub fn uma_f_wr(&mut self) -> UmaFWrW<UmaCtsSpec> {
        UmaFWrW::new(self, 3)
    }
    #[doc = "Bit 4 - Code Field Size Select"]
    #[inline(always)]
    pub fn c_size(&mut self) -> CSizeW<UmaCtsSpec> {
        CSizeW::new(self, 4)
    }
    #[doc = "Bit 5 - Read/Write Select"]
    #[inline(always)]
    pub fn rd_wr(&mut self) -> RdWrW<UmaCtsSpec> {
        RdWrW::new(self, 5)
    }
    #[doc = "Bit 7 - Operation Execute/Done"]
    #[inline(always)]
    pub fn exec_done(&mut self) -> ExecDoneW<UmaCtsSpec> {
        ExecDoneW::new(self, 7)
    }
}
#[doc = "UMA Control and Status Register (UMA_CTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_cts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_cts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UmaCtsSpec;
impl crate::RegisterSpec for UmaCtsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uma_cts::R`](R) reader structure"]
impl crate::Readable for UmaCtsSpec {}
#[doc = "`write(|w| ..)` method takes [`uma_cts::W`](W) writer structure"]
impl crate::Writable for UmaCtsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UMA_CTS to value 0"]
impl crate::Resettable for UmaCtsSpec {
    const RESET_VALUE: u8 = 0;
}
