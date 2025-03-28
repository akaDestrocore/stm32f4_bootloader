# [doc = "Register `RF%sR` reader"] pub type R = crate :: R < RfrSpec > ; # [doc = "Register `RF%sR` writer"] pub type W = crate :: W < RfrSpec > ; # [doc = "Field `FMP` reader - FMP0"] pub type FmpR = crate :: FieldReader ; # [doc = "FULL0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Full0r { # [doc = "0: FIFO x is not full"] NotFull = 0 , # [doc = "1: FIFO x is full"] Full = 1 , } impl From < Full0r > for bool { # [inline (always)] fn from (variant : Full0r) -> Self { variant as u8 != 0 } } # [doc = "Field `FULL` reader - FULL0"] pub type FullR = crate :: BitReader < Full0r > ; impl FullR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Full0r { match self . bits { false => Full0r :: NotFull , true => Full0r :: Full , } } # [doc = "FIFO x is not full"] # [inline (always)] pub fn is_not_full (& self) -> bool { * self == Full0r :: NotFull } # [doc = "FIFO x is full"] # [inline (always)] pub fn is_full (& self) -> bool { * self == Full0r :: Full } } # [doc = "FULL0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Full0wWO { # [doc = "1: Clear flag"] Clear = 1 , } impl From < Full0wWO > for bool { # [inline (always)] fn from (variant : Full0wWO) -> Self { variant as u8 != 0 } } # [doc = "Field `FULL` writer - FULL0"] pub type FullW < 'a , REG > = crate :: BitWriter < 'a , REG , Full0wWO > ; impl < 'a , REG > FullW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Clear flag"] # [inline (always)] pub fn clear (self) -> & 'a mut crate :: W < REG > { self . variant (Full0wWO :: Clear) } } # [doc = "FOVR0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Fovr0r { # [doc = "0: No FIFO x overrun"] NoOverrun = 0 , # [doc = "1: FIFO x overrun"] Overrun = 1 , } impl From < Fovr0r > for bool { # [inline (always)] fn from (variant : Fovr0r) -> Self { variant as u8 != 0 } } # [doc = "Field `FOVR` reader - FOVR0"] pub type FovrR = crate :: BitReader < Fovr0r > ; impl FovrR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Fovr0r { match self . bits { false => Fovr0r :: NoOverrun , true => Fovr0r :: Overrun , } } # [doc = "No FIFO x overrun"] # [inline (always)] pub fn is_no_overrun (& self) -> bool { * self == Fovr0r :: NoOverrun } # [doc = "FIFO x overrun"] # [inline (always)] pub fn is_overrun (& self) -> bool { * self == Fovr0r :: Overrun } } # [doc = "FOVR0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Fovr0wWO { # [doc = "1: Clear flag"] Clear = 1 , } impl From < Fovr0wWO > for bool { # [inline (always)] fn from (variant : Fovr0wWO) -> Self { variant as u8 != 0 } } # [doc = "Field `FOVR` writer - FOVR0"] pub type FovrW < 'a , REG > = crate :: BitWriter < 'a , REG , Fovr0wWO > ; impl < 'a , REG > FovrW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Clear flag"] # [inline (always)] pub fn clear (self) -> & 'a mut crate :: W < REG > { self . variant (Fovr0wWO :: Clear) } } # [doc = "RFOM0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Rfom0w { # [doc = "1: Set by software to release the output mailbox of the FIFO"] Release = 1 , } impl From < Rfom0w > for bool { # [inline (always)] fn from (variant : Rfom0w) -> Self { variant as u8 != 0 } } # [doc = "Field `RFOM` reader - RFOM0"] pub type RfomR = crate :: BitReader < Rfom0w > ; impl RfomR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Rfom0w > { match self . bits { true => Some (Rfom0w :: Release) , _ => None , } } # [doc = "Set by software to release the output mailbox of the FIFO"] # [inline (always)] pub fn is_release (& self) -> bool { * self == Rfom0w :: Release } } # [doc = "Field `RFOM` writer - RFOM0"] pub type RfomW < 'a , REG > = crate :: BitWriter < 'a , REG , Rfom0w > ; impl < 'a , REG > RfomW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Set by software to release the output mailbox of the FIFO"] # [inline (always)] pub fn release (self) -> & 'a mut crate :: W < REG > { self . variant (Rfom0w :: Release) } } impl R { # [doc = "Bits 0:1 - FMP0"] # [inline (always)] pub fn fmp (& self) -> FmpR { FmpR :: new ((self . bits & 3) as u8) } # [doc = "Bit 3 - FULL0"] # [inline (always)] pub fn full (& self) -> FullR { FullR :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - FOVR0"] # [inline (always)] pub fn fovr (& self) -> FovrR { FovrR :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - RFOM0"] # [inline (always)] pub fn rfom (& self) -> RfomR { RfomR :: new (((self . bits >> 5) & 1) != 0) } } impl W { # [doc = "Bit 3 - FULL0"] # [inline (always)] pub fn full (& mut self) -> FullW < RfrSpec > { FullW :: new (self , 3) } # [doc = "Bit 4 - FOVR0"] # [inline (always)] pub fn fovr (& mut self) -> FovrW < RfrSpec > { FovrW :: new (self , 4) } # [doc = "Bit 5 - RFOM0"] # [inline (always)] pub fn rfom (& mut self) -> RfomW < RfrSpec > { RfomW :: new (self , 5) } } # [doc = "receive FIFO %s register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct RfrSpec ; impl crate :: RegisterSpec for RfrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`rfr::R`](R) reader structure"] impl crate :: Readable for RfrSpec { } # [doc = "`write(|w| ..)` method takes [`rfr::W`](W) writer structure"] impl crate :: Writable for RfrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets RF%sR to value 0"] impl crate :: Resettable for RfrSpec { }