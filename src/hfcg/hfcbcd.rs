#[doc = "Register `HFCBCD` reader"]
pub type R = crate::R<HfcbcdSpec>;
#[doc = "Register `HFCBCD` writer"]
pub type W = crate::W<HfcbcdSpec>;
#[doc = "Field `AHB6CLK_BLK` reader - AHB6 Clock Block"]
pub type Ahb6clkBlkR = crate::BitReader;
#[doc = "Field `AHB6CLK_BLK` writer - AHB6 Clock Block"]
pub type Ahb6clkBlkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIU0DIV` reader - FIU0 Clock Divider"]
pub type Fiu0divR = crate::FieldReader;
#[doc = "Field `FIU0DIV` writer - FIU0 Clock Divider"]
pub type Fiu0divW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIU1DIV` reader - FIU1 Clock Divider"]
pub type Fiu1divR = crate::FieldReader;
#[doc = "Field `FIU1DIV` writer - FIU1 Clock Divider"]
pub type Fiu1divW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - AHB6 Clock Block"]
    #[inline(always)]
    pub fn ahb6clk_blk(&self) -> Ahb6clkBlkR {
        Ahb6clkBlkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - FIU0 Clock Divider"]
    #[inline(always)]
    pub fn fiu0div(&self) -> Fiu0divR {
        Fiu0divR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - FIU1 Clock Divider"]
    #[inline(always)]
    pub fn fiu1div(&self) -> Fiu1divR {
        Fiu1divR::new((self.bits >> 4) & 3)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCBCD")
            .field("ahb6clk_blk", &self.ahb6clk_blk())
            .field("fiu0div", &self.fiu0div())
            .field("fiu1div", &self.fiu1div())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - AHB6 Clock Block"]
    #[inline(always)]
    pub fn ahb6clk_blk(&mut self) -> Ahb6clkBlkW<HfcbcdSpec> {
        Ahb6clkBlkW::new(self, 1)
    }
    #[doc = "Bits 2:3 - FIU0 Clock Divider"]
    #[inline(always)]
    pub fn fiu0div(&mut self) -> Fiu0divW<HfcbcdSpec> {
        Fiu0divW::new(self, 2)
    }
    #[doc = "Bits 4:5 - FIU1 Clock Divider"]
    #[inline(always)]
    pub fn fiu1div(&mut self) -> Fiu1divW<HfcbcdSpec> {
        Fiu1divW::new(self, 4)
    }
}
#[doc = "HFCG Bus Clock Dividers Register (HFCBCD)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcbcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcbcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfcbcdSpec;
impl crate::RegisterSpec for HfcbcdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcbcd::R`](R) reader structure"]
impl crate::Readable for HfcbcdSpec {}
#[doc = "`write(|w| ..)` method takes [`hfcbcd::W`](W) writer structure"]
impl crate::Writable for HfcbcdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCBCD to value 0x60"]
impl crate::Resettable for HfcbcdSpec {
    const RESET_VALUE: u8 = 0x60;
}
