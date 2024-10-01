#[doc = "Register `FLASH_RPMC_CFG_1` reader"]
pub type R = crate::R<FlashRpmcCfg1Spec>;
#[doc = "Register `FLASH_RPMC_CFG_1` writer"]
pub type W = crate::W<FlashRpmcCfg1Spec>;
#[doc = "Field `RPMC_CNTR_1` reader - RPMC Counter on the 1st RPMC Flash Device"]
pub type RpmcCntr1R = crate::FieldReader;
#[doc = "Field `RPMC_CNTR_1` writer - RPMC Counter on the 1st RPMC Flash Device"]
pub type RpmcCntr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RPMC_OP1_1` reader - RPMC OP1 Opcode on the 1st RPMC Flash Device"]
pub type RpmcOp1_1R = crate::FieldReader;
#[doc = "Field `RPMC_OP1_1` writer - RPMC OP1 Opcode on the 1st RPMC Flash Device"]
pub type RpmcOp1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RPMC_CNTR_2` reader - RPMC Counter on the 2nd RPMC Flash Device"]
pub type RpmcCntr2R = crate::FieldReader;
#[doc = "Field `RPMC_CNTR_2` writer - RPMC Counter on the 2nd RPMC Flash Device"]
pub type RpmcCntr2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RPMC_OP1_2` reader - RPMC OP1 Opcode on the 2nd RPMC Flash Device"]
pub type RpmcOp1_2R = crate::FieldReader;
#[doc = "Field `RPMC_OP1_2` writer - RPMC OP1 Opcode on the 2nd RPMC Flash Device"]
pub type RpmcOp1_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RPMC_DEV_SUPP` reader - Number of TAF RPMC Flash Devices Supported"]
pub type RpmcDevSuppR = crate::FieldReader;
#[doc = "Field `RPMC_DEV_SUPP` writer - Number of TAF RPMC Flash Devices Supported"]
pub type RpmcDevSuppW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRGRPMCSUPP` reader - Target RPMC Supported"]
pub type TrgrpmcsuppR = crate::FieldReader;
#[doc = "Field `TRGRPMCSUPP` writer - Target RPMC Supported"]
pub type TrgrpmcsuppW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - RPMC Counter on the 1st RPMC Flash Device"]
    #[inline(always)]
    pub fn rpmc_cntr_1(&self) -> RpmcCntr1R {
        RpmcCntr1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - RPMC OP1 Opcode on the 1st RPMC Flash Device"]
    #[inline(always)]
    pub fn rpmc_op1_1(&self) -> RpmcOp1_1R {
        RpmcOp1_1R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - RPMC Counter on the 2nd RPMC Flash Device"]
    #[inline(always)]
    pub fn rpmc_cntr_2(&self) -> RpmcCntr2R {
        RpmcCntr2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - RPMC OP1 Opcode on the 2nd RPMC Flash Device"]
    #[inline(always)]
    pub fn rpmc_op1_2(&self) -> RpmcOp1_2R {
        RpmcOp1_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Number of TAF RPMC Flash Devices Supported"]
    #[inline(always)]
    pub fn rpmc_dev_supp(&self) -> RpmcDevSuppR {
        RpmcDevSuppR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - Target RPMC Supported"]
    #[inline(always)]
    pub fn trgrpmcsupp(&self) -> TrgrpmcsuppR {
        TrgrpmcsuppR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_RPMC_CFG_1")
            .field("rpmc_cntr_1", &self.rpmc_cntr_1())
            .field("rpmc_op1_1", &self.rpmc_op1_1())
            .field("rpmc_cntr_2", &self.rpmc_cntr_2())
            .field("rpmc_op1_2", &self.rpmc_op1_2())
            .field("rpmc_dev_supp", &self.rpmc_dev_supp())
            .field("trgrpmcsupp", &self.trgrpmcsupp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - RPMC Counter on the 1st RPMC Flash Device"]
    #[inline(always)]
    #[must_use]
    pub fn rpmc_cntr_1(&mut self) -> RpmcCntr1W<FlashRpmcCfg1Spec> {
        RpmcCntr1W::new(self, 0)
    }
    #[doc = "Bits 4:11 - RPMC OP1 Opcode on the 1st RPMC Flash Device"]
    #[inline(always)]
    #[must_use]
    pub fn rpmc_op1_1(&mut self) -> RpmcOp1_1W<FlashRpmcCfg1Spec> {
        RpmcOp1_1W::new(self, 4)
    }
    #[doc = "Bits 12:15 - RPMC Counter on the 2nd RPMC Flash Device"]
    #[inline(always)]
    #[must_use]
    pub fn rpmc_cntr_2(&mut self) -> RpmcCntr2W<FlashRpmcCfg1Spec> {
        RpmcCntr2W::new(self, 12)
    }
    #[doc = "Bits 16:23 - RPMC OP1 Opcode on the 2nd RPMC Flash Device"]
    #[inline(always)]
    #[must_use]
    pub fn rpmc_op1_2(&mut self) -> RpmcOp1_2W<FlashRpmcCfg1Spec> {
        RpmcOp1_2W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Number of TAF RPMC Flash Devices Supported"]
    #[inline(always)]
    #[must_use]
    pub fn rpmc_dev_supp(&mut self) -> RpmcDevSuppW<FlashRpmcCfg1Spec> {
        RpmcDevSuppW::new(self, 24)
    }
    #[doc = "Bits 26:31 - Target RPMC Supported"]
    #[inline(always)]
    #[must_use]
    pub fn trgrpmcsupp(&mut self) -> TrgrpmcsuppW<FlashRpmcCfg1Spec> {
        TrgrpmcsuppW::new(self, 26)
    }
}
#[doc = "Flash RPMC Configuration 1 Register (FLASH_RPMC_CFG_1)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rpmc_cfg_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rpmc_cfg_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashRpmcCfg1Spec;
impl crate::RegisterSpec for FlashRpmcCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_rpmc_cfg_1::R`](R) reader structure"]
impl crate::Readable for FlashRpmcCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`flash_rpmc_cfg_1::W`](W) writer structure"]
impl crate::Writable for FlashRpmcCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_RPMC_CFG_1 to value 0"]
impl crate::Resettable for FlashRpmcCfg1Spec {
    const RESET_VALUE: u32 = 0;
}
