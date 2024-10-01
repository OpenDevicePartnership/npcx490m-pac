#[doc = "Register `MEM_PAR_EN` reader"]
pub type R = crate::R<MemParEnSpec>;
#[doc = "Register `MEM_PAR_EN` writer"]
pub type W = crate::W<MemParEnSpec>;
#[doc = "Field `RAM_PAR_EN` reader - RAM Parity Check Enable"]
pub type RamParEnR = crate::FieldReader;
#[doc = "Field `RAM_PAR_EN` writer - RAM Parity Check Enable"]
pub type RamParEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ROM_PAR_EN` reader - ROM Parity Check Enable"]
pub type RomParEnR = crate::FieldReader;
#[doc = "Field `ROM_PAR_EN` writer - ROM Parity Check Enable"]
pub type RomParEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RAM Parity Check Enable"]
    #[inline(always)]
    pub fn ram_par_en(&self) -> RamParEnR {
        RamParEnR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - ROM Parity Check Enable"]
    #[inline(always)]
    pub fn rom_par_en(&self) -> RomParEnR {
        RomParEnR::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_PAR_EN")
            .field("ram_par_en", &self.ram_par_en())
            .field("rom_par_en", &self.rom_par_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM Parity Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ram_par_en(&mut self) -> RamParEnW<MemParEnSpec> {
        RamParEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - ROM Parity Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rom_par_en(&mut self) -> RomParEnW<MemParEnSpec> {
        RomParEnW::new(self, 4)
    }
}
#[doc = "Memory Parity Check Enable Register (MEM_PAR_EN)\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_par_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_par_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemParEnSpec;
impl crate::RegisterSpec for MemParEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mem_par_en::R`](R) reader structure"]
impl crate::Readable for MemParEnSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_par_en::W`](W) writer structure"]
impl crate::Writable for MemParEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MEM_PAR_EN to value 0"]
impl crate::Resettable for MemParEnSpec {
    const RESET_VALUE: u8 = 0;
}
