#[doc = "Register `FLASH_RPMC_CFG_2` reader"]
pub type R = crate::R<FlashRpmcCfg2Spec>;
#[doc = "Register `FLASH_RPMC_CFG_2` writer"]
pub type W = crate::W<FlashRpmcCfg2Spec>;
#[doc = "Field `RPMC_CNTR_3` reader - RPMC Counter the 3rd RPMC Flash Device"]
pub type RpmcCntr3R = crate::FieldReader;
#[doc = "Field `RPMC_CNTR_3` writer - RPMC Counter the 3rd RPMC Flash Device"]
pub type RpmcCntr3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RPMC_OP1_3` reader - RPMC OP1 Opcode on the 3rd RPMC Flash Device"]
pub type RpmcOp1_3R = crate::FieldReader;
#[doc = "Field `RPMC_OP1_3` writer - RPMC OP1 Opcode on the 3rd RPMC Flash Device"]
pub type RpmcOp1_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RPMC_CNTR_4` reader - RPMC Counter the 4th RPMC Flash Device"]
pub type RpmcCntr4R = crate::FieldReader;
#[doc = "Field `RPMC_CNTR_4` writer - RPMC Counter the 4th RPMC Flash Device"]
pub type RpmcCntr4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RPMC_OP1_4` reader - RPMC OP1 Opcode on the 4th RPMC Flash Device"]
pub type RpmcOp1_4R = crate::FieldReader;
#[doc = "Field `RPMC_OP1_4` writer - RPMC OP1 Opcode on the 4th RPMC Flash Device"]
pub type RpmcOp1_4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - RPMC Counter the 3rd RPMC Flash Device"]
    #[inline(always)]
    pub fn rpmc_cntr_3(&self) -> RpmcCntr3R {
        RpmcCntr3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - RPMC OP1 Opcode on the 3rd RPMC Flash Device"]
    #[inline(always)]
    pub fn rpmc_op1_3(&self) -> RpmcOp1_3R {
        RpmcOp1_3R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - RPMC Counter the 4th RPMC Flash Device"]
    #[inline(always)]
    pub fn rpmc_cntr_4(&self) -> RpmcCntr4R {
        RpmcCntr4R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - RPMC OP1 Opcode on the 4th RPMC Flash Device"]
    #[inline(always)]
    pub fn rpmc_op1_4(&self) -> RpmcOp1_4R {
        RpmcOp1_4R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_RPMC_CFG_2")
            .field("rpmc_cntr_3", &self.rpmc_cntr_3())
            .field("rpmc_op1_3", &self.rpmc_op1_3())
            .field("rpmc_cntr_4", &self.rpmc_cntr_4())
            .field("rpmc_op1_4", &self.rpmc_op1_4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - RPMC Counter the 3rd RPMC Flash Device"]
    #[inline(always)]
    #[must_use]
    pub fn rpmc_cntr_3(&mut self) -> RpmcCntr3W<FlashRpmcCfg2Spec> {
        RpmcCntr3W::new(self, 0)
    }
    #[doc = "Bits 4:11 - RPMC OP1 Opcode on the 3rd RPMC Flash Device"]
    #[inline(always)]
    #[must_use]
    pub fn rpmc_op1_3(&mut self) -> RpmcOp1_3W<FlashRpmcCfg2Spec> {
        RpmcOp1_3W::new(self, 4)
    }
    #[doc = "Bits 12:15 - RPMC Counter the 4th RPMC Flash Device"]
    #[inline(always)]
    #[must_use]
    pub fn rpmc_cntr_4(&mut self) -> RpmcCntr4W<FlashRpmcCfg2Spec> {
        RpmcCntr4W::new(self, 12)
    }
    #[doc = "Bits 16:23 - RPMC OP1 Opcode on the 4th RPMC Flash Device"]
    #[inline(always)]
    #[must_use]
    pub fn rpmc_op1_4(&mut self) -> RpmcOp1_4W<FlashRpmcCfg2Spec> {
        RpmcOp1_4W::new(self, 16)
    }
}
#[doc = "Flash RPMC Configuration Register 2 (FLASH_RPMC_CFG_2)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rpmc_cfg_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rpmc_cfg_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashRpmcCfg2Spec;
impl crate::RegisterSpec for FlashRpmcCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_rpmc_cfg_2::R`](R) reader structure"]
impl crate::Readable for FlashRpmcCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`flash_rpmc_cfg_2::W`](W) writer structure"]
impl crate::Writable for FlashRpmcCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_RPMC_CFG_2 to value 0"]
impl crate::Resettable for FlashRpmcCfg2Spec {
    const RESET_VALUE: u32 = 0;
}
