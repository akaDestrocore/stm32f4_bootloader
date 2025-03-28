# [doc = "Register `MACMIIAR` reader"] pub type R = crate :: R < MacmiiarSpec > ; # [doc = "Register `MACMIIAR` writer"] pub type W = crate :: W < MacmiiarSpec > ; # [doc = "MII busy\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Mb { # [doc = "1: This bit is set to 1 by the application to indicate that a read or write access is in progress"] Busy = 1 , } impl From < Mb > for bool { # [inline (always)] fn from (variant : Mb) -> Self { variant as u8 != 0 } } # [doc = "Field `MB` reader - MII busy"] pub type MbR = crate :: BitReader < Mb > ; impl MbR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Mb > { match self . bits { true => Some (Mb :: Busy) , _ => None , } } # [doc = "This bit is set to 1 by the application to indicate that a read or write access is in progress"] # [inline (always)] pub fn is_busy (& self) -> bool { * self == Mb :: Busy } } # [doc = "Field `MB` writer - MII busy"] pub type MbW < 'a , REG > = crate :: BitWriter < 'a , REG , Mb > ; impl < 'a , REG > MbW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "This bit is set to 1 by the application to indicate that a read or write access is in progress"] # [inline (always)] pub fn busy (self) -> & 'a mut crate :: W < REG > { self . variant (Mb :: Busy) } } # [doc = "MII write\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Mw { # [doc = "0: Read operation"] Read = 0 , # [doc = "1: Write operation"] Write = 1 , } impl From < Mw > for bool { # [inline (always)] fn from (variant : Mw) -> Self { variant as u8 != 0 } } # [doc = "Field `MW` reader - MII write"] pub type MwR = crate :: BitReader < Mw > ; impl MwR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Mw { match self . bits { false => Mw :: Read , true => Mw :: Write , } } # [doc = "Read operation"] # [inline (always)] pub fn is_read (& self) -> bool { * self == Mw :: Read } # [doc = "Write operation"] # [inline (always)] pub fn is_write (& self) -> bool { * self == Mw :: Write } } # [doc = "Field `MW` writer - MII write"] pub type MwW < 'a , REG > = crate :: BitWriter < 'a , REG , Mw > ; impl < 'a , REG > MwW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Read operation"] # [inline (always)] pub fn read (self) -> & 'a mut crate :: W < REG > { self . variant (Mw :: Read) } # [doc = "Write operation"] # [inline (always)] pub fn write (self) -> & 'a mut crate :: W < REG > { self . variant (Mw :: Write) } } # [doc = "Clock range\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum Cr { # [doc = "0: 60-100MHz HCLK/42"] Cr60_100 = 0 , # [doc = "1: 100-150 MHz HCLK/62"] Cr100_150 = 1 , # [doc = "2: 20-35MHz HCLK/16"] Cr20_35 = 2 , # [doc = "3: 35-60MHz HCLK/16"] Cr35_60 = 3 , # [doc = "4: 150-168MHz HCLK/102"] Cr150_168 = 4 , } impl From < Cr > for u8 { # [inline (always)] fn from (variant : Cr) -> Self { variant as _ } } impl crate :: FieldSpec for Cr { type Ux = u8 ; } impl crate :: IsEnum for Cr { } # [doc = "Field `CR` reader - Clock range"] pub type CrR = crate :: FieldReader < Cr > ; impl CrR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Cr > { match self . bits { 0 => Some (Cr :: Cr60_100) , 1 => Some (Cr :: Cr100_150) , 2 => Some (Cr :: Cr20_35) , 3 => Some (Cr :: Cr35_60) , 4 => Some (Cr :: Cr150_168) , _ => None , } } # [doc = "60-100MHz HCLK/42"] # [inline (always)] pub fn is_cr_60_100 (& self) -> bool { * self == Cr :: Cr60_100 } # [doc = "100-150 MHz HCLK/62"] # [inline (always)] pub fn is_cr_100_150 (& self) -> bool { * self == Cr :: Cr100_150 } # [doc = "20-35MHz HCLK/16"] # [inline (always)] pub fn is_cr_20_35 (& self) -> bool { * self == Cr :: Cr20_35 } # [doc = "35-60MHz HCLK/16"] # [inline (always)] pub fn is_cr_35_60 (& self) -> bool { * self == Cr :: Cr35_60 } # [doc = "150-168MHz HCLK/102"] # [inline (always)] pub fn is_cr_150_168 (& self) -> bool { * self == Cr :: Cr150_168 } } # [doc = "Field `CR` writer - Clock range"] pub type CrW < 'a , REG > = crate :: FieldWriter < 'a , REG , 3 , Cr > ; impl < 'a , REG > CrW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "60-100MHz HCLK/42"] # [inline (always)] pub fn cr_60_100 (self) -> & 'a mut crate :: W < REG > { self . variant (Cr :: Cr60_100) } # [doc = "100-150 MHz HCLK/62"] # [inline (always)] pub fn cr_100_150 (self) -> & 'a mut crate :: W < REG > { self . variant (Cr :: Cr100_150) } # [doc = "20-35MHz HCLK/16"] # [inline (always)] pub fn cr_20_35 (self) -> & 'a mut crate :: W < REG > { self . variant (Cr :: Cr20_35) } # [doc = "35-60MHz HCLK/16"] # [inline (always)] pub fn cr_35_60 (self) -> & 'a mut crate :: W < REG > { self . variant (Cr :: Cr35_60) } # [doc = "150-168MHz HCLK/102"] # [inline (always)] pub fn cr_150_168 (self) -> & 'a mut crate :: W < REG > { self . variant (Cr :: Cr150_168) } } # [doc = "Field `MR` reader - MII register - select the desired MII register in the PHY device"] pub type MrR = crate :: FieldReader ; # [doc = "Field `MR` writer - MII register - select the desired MII register in the PHY device"] pub type MrW < 'a , REG > = crate :: FieldWriter < 'a , REG , 5 , u8 , crate :: Safe > ; # [doc = "Field `PA` reader - PHY address - select which of possible 32 PHYs is being accessed"] pub type PaR = crate :: FieldReader ; # [doc = "Field `PA` writer - PHY address - select which of possible 32 PHYs is being accessed"] pub type PaW < 'a , REG > = crate :: FieldWriter < 'a , REG , 5 , u8 , crate :: Safe > ; impl R { # [doc = "Bit 0 - MII busy"] # [inline (always)] pub fn mb (& self) -> MbR { MbR :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - MII write"] # [inline (always)] pub fn mw (& self) -> MwR { MwR :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bits 2:4 - Clock range"] # [inline (always)] pub fn cr (& self) -> CrR { CrR :: new (((self . bits >> 2) & 7) as u8) } # [doc = "Bits 6:10 - MII register - select the desired MII register in the PHY device"] # [inline (always)] pub fn mr (& self) -> MrR { MrR :: new (((self . bits >> 6) & 0x1f) as u8) } # [doc = "Bits 11:15 - PHY address - select which of possible 32 PHYs is being accessed"] # [inline (always)] pub fn pa (& self) -> PaR { PaR :: new (((self . bits >> 11) & 0x1f) as u8) } } impl W { # [doc = "Bit 0 - MII busy"] # [inline (always)] pub fn mb (& mut self) -> MbW < MacmiiarSpec > { MbW :: new (self , 0) } # [doc = "Bit 1 - MII write"] # [inline (always)] pub fn mw (& mut self) -> MwW < MacmiiarSpec > { MwW :: new (self , 1) } # [doc = "Bits 2:4 - Clock range"] # [inline (always)] pub fn cr (& mut self) -> CrW < MacmiiarSpec > { CrW :: new (self , 2) } # [doc = "Bits 6:10 - MII register - select the desired MII register in the PHY device"] # [inline (always)] pub fn mr (& mut self) -> MrW < MacmiiarSpec > { MrW :: new (self , 6) } # [doc = "Bits 11:15 - PHY address - select which of possible 32 PHYs is being accessed"] # [inline (always)] pub fn pa (& mut self) -> PaW < MacmiiarSpec > { PaW :: new (self , 11) } } # [doc = "Ethernet MAC MII address register\n\nYou can [`read`](crate::Reg::read) this register and get [`macmiiar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiiar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct MacmiiarSpec ; impl crate :: RegisterSpec for MacmiiarSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`macmiiar::R`](R) reader structure"] impl crate :: Readable for MacmiiarSpec { } # [doc = "`write(|w| ..)` method takes [`macmiiar::W`](W) writer structure"] impl crate :: Writable for MacmiiarSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets MACMIIAR to value 0"] impl crate :: Resettable for MacmiiarSpec { }