#[doc = "Register `IBIRULES` reader"]
pub type R = crate::R<IbirulesSpec>;
#[doc = "Register `IBIRULES` writer"]
pub type W = crate::W<IbirulesSpec>;
#[doc = "Field `ADDR0` reader - Target 0 Dynamic Address"]
pub type Addr0R = crate::FieldReader;
#[doc = "Field `ADDR0` writer - Target 0 Dynamic Address"]
pub type Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADDR1` reader - Target 1 Dynamic Address"]
pub type Addr1R = crate::FieldReader;
#[doc = "Field `ADDR1` writer - Target 1 Dynamic Address"]
pub type Addr1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADDR2` reader - Target 2 Dynamic Address"]
pub type Addr2R = crate::FieldReader;
#[doc = "Field `ADDR2` writer - Target 2 Dynamic Address"]
pub type Addr2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADDR3` reader - Target 3 Dynamic Address"]
pub type Addr3R = crate::FieldReader;
#[doc = "Field `ADDR3` writer - Target 3 Dynamic Address"]
pub type Addr3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADDR4` reader - Target 4 Dynamic Address"]
pub type Addr4R = crate::FieldReader;
#[doc = "Field `ADDR4` writer - Target 4 Dynamic Address"]
pub type Addr4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MSB0` reader - MSb is 0"]
pub type Msb0R = crate::BitReader;
#[doc = "Field `MSB0` writer - MSb is 0"]
pub type Msb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOBYTE` reader - No Mandatory Byte"]
pub type NobyteR = crate::BitReader;
#[doc = "Field `NOBYTE` writer - No Mandatory Byte"]
pub type NobyteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Target 0 Dynamic Address"]
    #[inline(always)]
    pub fn addr0(&self) -> Addr0R {
        Addr0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Target 1 Dynamic Address"]
    #[inline(always)]
    pub fn addr1(&self) -> Addr1R {
        Addr1R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Target 2 Dynamic Address"]
    #[inline(always)]
    pub fn addr2(&self) -> Addr2R {
        Addr2R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Target 3 Dynamic Address"]
    #[inline(always)]
    pub fn addr3(&self) -> Addr3R {
        Addr3R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Target 4 Dynamic Address"]
    #[inline(always)]
    pub fn addr4(&self) -> Addr4R {
        Addr4R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - MSb is 0"]
    #[inline(always)]
    pub fn msb0(&self) -> Msb0R {
        Msb0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - No Mandatory Byte"]
    #[inline(always)]
    pub fn nobyte(&self) -> NobyteR {
        NobyteR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBIRULES")
            .field("addr0", &self.addr0())
            .field("addr1", &self.addr1())
            .field("addr2", &self.addr2())
            .field("addr3", &self.addr3())
            .field("addr4", &self.addr4())
            .field("msb0", &self.msb0())
            .field("nobyte", &self.nobyte())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Target 0 Dynamic Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr0(&mut self) -> Addr0W<IbirulesSpec> {
        Addr0W::new(self, 0)
    }
    #[doc = "Bits 6:11 - Target 1 Dynamic Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr1(&mut self) -> Addr1W<IbirulesSpec> {
        Addr1W::new(self, 6)
    }
    #[doc = "Bits 12:17 - Target 2 Dynamic Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr2(&mut self) -> Addr2W<IbirulesSpec> {
        Addr2W::new(self, 12)
    }
    #[doc = "Bits 18:23 - Target 3 Dynamic Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr3(&mut self) -> Addr3W<IbirulesSpec> {
        Addr3W::new(self, 18)
    }
    #[doc = "Bits 24:29 - Target 4 Dynamic Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr4(&mut self) -> Addr4W<IbirulesSpec> {
        Addr4W::new(self, 24)
    }
    #[doc = "Bit 30 - MSb is 0"]
    #[inline(always)]
    #[must_use]
    pub fn msb0(&mut self) -> Msb0W<IbirulesSpec> {
        Msb0W::new(self, 30)
    }
    #[doc = "Bit 31 - No Mandatory Byte"]
    #[inline(always)]
    #[must_use]
    pub fn nobyte(&mut self) -> NobyteW<IbirulesSpec> {
        NobyteW::new(self, 31)
    }
}
#[doc = "IBI Registry and Rules Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibirules::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibirules::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IbirulesSpec;
impl crate::RegisterSpec for IbirulesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibirules::R`](R) reader structure"]
impl crate::Readable for IbirulesSpec {}
#[doc = "`write(|w| ..)` method takes [`ibirules::W`](W) writer structure"]
impl crate::Writable for IbirulesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IBIRULES to value 0"]
impl crate::Resettable for IbirulesSpec {
    const RESET_VALUE: u32 = 0;
}
