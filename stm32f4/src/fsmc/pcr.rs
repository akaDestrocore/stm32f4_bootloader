# [doc = "Register `PCR%s` reader"] pub type R = crate :: R < PcrSpec > ; # [doc = "Register `PCR%s` writer"] pub type W = crate :: W < PcrSpec > ; # [doc = "PWAITEN\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Pwaiten { # [doc = "0: Wait feature disabled"] Disabled = 0 , # [doc = "1: Wait feature enabled"] Enabled = 1 , } impl From < Pwaiten > for bool { # [inline (always)] fn from (variant : Pwaiten) -> Self { variant as u8 != 0 } } # [doc = "Field `PWAITEN` reader - PWAITEN"] pub type PwaitenR = crate :: BitReader < Pwaiten > ; impl PwaitenR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Pwaiten { match self . bits { false => Pwaiten :: Disabled , true => Pwaiten :: Enabled , } } # [doc = "Wait feature disabled"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Pwaiten :: Disabled } # [doc = "Wait feature enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Pwaiten :: Enabled } } # [doc = "Field `PWAITEN` writer - PWAITEN"] pub type PwaitenW < 'a , REG > = crate :: BitWriter < 'a , REG , Pwaiten > ; impl < 'a , REG > PwaitenW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Wait feature disabled"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Pwaiten :: Disabled) } # [doc = "Wait feature enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Pwaiten :: Enabled) } } # [doc = "PBKEN\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Pbken { # [doc = "0: Corresponding memory bank is disabled"] Disabled = 0 , # [doc = "1: Corresponding memory bank is enabled"] Enabled = 1 , } impl From < Pbken > for bool { # [inline (always)] fn from (variant : Pbken) -> Self { variant as u8 != 0 } } # [doc = "Field `PBKEN` reader - PBKEN"] pub type PbkenR = crate :: BitReader < Pbken > ; impl PbkenR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Pbken { match self . bits { false => Pbken :: Disabled , true => Pbken :: Enabled , } } # [doc = "Corresponding memory bank is disabled"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Pbken :: Disabled } # [doc = "Corresponding memory bank is enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Pbken :: Enabled } } # [doc = "Field `PBKEN` writer - PBKEN"] pub type PbkenW < 'a , REG > = crate :: BitWriter < 'a , REG , Pbken > ; impl < 'a , REG > PbkenW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Corresponding memory bank is disabled"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Pbken :: Disabled) } # [doc = "Corresponding memory bank is enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Pbken :: Enabled) } } # [doc = "PTYP\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Ptyp { # [doc = "1: NAND Flash"] Nandflash = 1 , } impl From < Ptyp > for bool { # [inline (always)] fn from (variant : Ptyp) -> Self { variant as u8 != 0 } } # [doc = "Field `PTYP` reader - PTYP"] pub type PtypR = crate :: BitReader < Ptyp > ; impl PtypR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Ptyp > { match self . bits { true => Some (Ptyp :: Nandflash) , _ => None , } } # [doc = "NAND Flash"] # [inline (always)] pub fn is_nandflash (& self) -> bool { * self == Ptyp :: Nandflash } } # [doc = "Field `PTYP` writer - PTYP"] pub type PtypW < 'a , REG > = crate :: BitWriter < 'a , REG , Ptyp > ; impl < 'a , REG > PtypW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NAND Flash"] # [inline (always)] pub fn nandflash (self) -> & 'a mut crate :: W < REG > { self . variant (Ptyp :: Nandflash) } } # [doc = "PWID\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum Pwid { # [doc = "0: External memory device width 8 bits"] Bits8 = 0 , # [doc = "1: External memory device width 16 bits"] Bits16 = 1 , } impl From < Pwid > for u8 { # [inline (always)] fn from (variant : Pwid) -> Self { variant as _ } } impl crate :: FieldSpec for Pwid { type Ux = u8 ; } impl crate :: IsEnum for Pwid { } # [doc = "Field `PWID` reader - PWID"] pub type PwidR = crate :: FieldReader < Pwid > ; impl PwidR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Pwid > { match self . bits { 0 => Some (Pwid :: Bits8) , 1 => Some (Pwid :: Bits16) , _ => None , } } # [doc = "External memory device width 8 bits"] # [inline (always)] pub fn is_bits8 (& self) -> bool { * self == Pwid :: Bits8 } # [doc = "External memory device width 16 bits"] # [inline (always)] pub fn is_bits16 (& self) -> bool { * self == Pwid :: Bits16 } } # [doc = "Field `PWID` writer - PWID"] pub type PwidW < 'a , REG > = crate :: FieldWriter < 'a , REG , 2 , Pwid > ; impl < 'a , REG > PwidW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "External memory device width 8 bits"] # [inline (always)] pub fn bits8 (self) -> & 'a mut crate :: W < REG > { self . variant (Pwid :: Bits8) } # [doc = "External memory device width 16 bits"] # [inline (always)] pub fn bits16 (self) -> & 'a mut crate :: W < REG > { self . variant (Pwid :: Bits16) } } # [doc = "ECCEN\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Eccen { # [doc = "0: ECC logic is disabled and reset"] Disabled = 0 , # [doc = "1: ECC logic is enabled"] Enabled = 1 , } impl From < Eccen > for bool { # [inline (always)] fn from (variant : Eccen) -> Self { variant as u8 != 0 } } # [doc = "Field `ECCEN` reader - ECCEN"] pub type EccenR = crate :: BitReader < Eccen > ; impl EccenR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Eccen { match self . bits { false => Eccen :: Disabled , true => Eccen :: Enabled , } } # [doc = "ECC logic is disabled and reset"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Eccen :: Disabled } # [doc = "ECC logic is enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Eccen :: Enabled } } # [doc = "Field `ECCEN` writer - ECCEN"] pub type EccenW < 'a , REG > = crate :: BitWriter < 'a , REG , Eccen > ; impl < 'a , REG > EccenW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ECC logic is disabled and reset"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Eccen :: Disabled) } # [doc = "ECC logic is enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Eccen :: Enabled) } } # [doc = "Field `TCLR` reader - TCLR"] pub type TclrR = crate :: FieldReader ; # [doc = "Field `TCLR` writer - TCLR"] pub type TclrW < 'a , REG > = crate :: FieldWriter < 'a , REG , 4 , u8 , crate :: Safe > ; # [doc = "Field `TAR` reader - TAR"] pub type TarR = crate :: FieldReader ; # [doc = "Field `TAR` writer - TAR"] pub type TarW < 'a , REG > = crate :: FieldWriter < 'a , REG , 4 , u8 , crate :: Safe > ; # [doc = "ECCPS\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum Eccps { # [doc = "0: ECC page size 256 bytes"] Bytes256 = 0 , # [doc = "1: ECC page size 512 bytes"] Bytes512 = 1 , # [doc = "2: ECC page size 1024 bytes"] Bytes1024 = 2 , # [doc = "3: ECC page size 2048 bytes"] Bytes2048 = 3 , # [doc = "4: ECC page size 4096 bytes"] Bytes4096 = 4 , # [doc = "5: ECC page size 8192 bytes"] Bytes8192 = 5 , } impl From < Eccps > for u8 { # [inline (always)] fn from (variant : Eccps) -> Self { variant as _ } } impl crate :: FieldSpec for Eccps { type Ux = u8 ; } impl crate :: IsEnum for Eccps { } # [doc = "Field `ECCPS` reader - ECCPS"] pub type EccpsR = crate :: FieldReader < Eccps > ; impl EccpsR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Eccps > { match self . bits { 0 => Some (Eccps :: Bytes256) , 1 => Some (Eccps :: Bytes512) , 2 => Some (Eccps :: Bytes1024) , 3 => Some (Eccps :: Bytes2048) , 4 => Some (Eccps :: Bytes4096) , 5 => Some (Eccps :: Bytes8192) , _ => None , } } # [doc = "ECC page size 256 bytes"] # [inline (always)] pub fn is_bytes256 (& self) -> bool { * self == Eccps :: Bytes256 } # [doc = "ECC page size 512 bytes"] # [inline (always)] pub fn is_bytes512 (& self) -> bool { * self == Eccps :: Bytes512 } # [doc = "ECC page size 1024 bytes"] # [inline (always)] pub fn is_bytes1024 (& self) -> bool { * self == Eccps :: Bytes1024 } # [doc = "ECC page size 2048 bytes"] # [inline (always)] pub fn is_bytes2048 (& self) -> bool { * self == Eccps :: Bytes2048 } # [doc = "ECC page size 4096 bytes"] # [inline (always)] pub fn is_bytes4096 (& self) -> bool { * self == Eccps :: Bytes4096 } # [doc = "ECC page size 8192 bytes"] # [inline (always)] pub fn is_bytes8192 (& self) -> bool { * self == Eccps :: Bytes8192 } } # [doc = "Field `ECCPS` writer - ECCPS"] pub type EccpsW < 'a , REG > = crate :: FieldWriter < 'a , REG , 3 , Eccps > ; impl < 'a , REG > EccpsW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "ECC page size 256 bytes"] # [inline (always)] pub fn bytes256 (self) -> & 'a mut crate :: W < REG > { self . variant (Eccps :: Bytes256) } # [doc = "ECC page size 512 bytes"] # [inline (always)] pub fn bytes512 (self) -> & 'a mut crate :: W < REG > { self . variant (Eccps :: Bytes512) } # [doc = "ECC page size 1024 bytes"] # [inline (always)] pub fn bytes1024 (self) -> & 'a mut crate :: W < REG > { self . variant (Eccps :: Bytes1024) } # [doc = "ECC page size 2048 bytes"] # [inline (always)] pub fn bytes2048 (self) -> & 'a mut crate :: W < REG > { self . variant (Eccps :: Bytes2048) } # [doc = "ECC page size 4096 bytes"] # [inline (always)] pub fn bytes4096 (self) -> & 'a mut crate :: W < REG > { self . variant (Eccps :: Bytes4096) } # [doc = "ECC page size 8192 bytes"] # [inline (always)] pub fn bytes8192 (self) -> & 'a mut crate :: W < REG > { self . variant (Eccps :: Bytes8192) } } impl R { # [doc = "Bit 1 - PWAITEN"] # [inline (always)] pub fn pwaiten (& self) -> PwaitenR { PwaitenR :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - PBKEN"] # [inline (always)] pub fn pbken (& self) -> PbkenR { PbkenR :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - PTYP"] # [inline (always)] pub fn ptyp (& self) -> PtypR { PtypR :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bits 4:5 - PWID"] # [inline (always)] pub fn pwid (& self) -> PwidR { PwidR :: new (((self . bits >> 4) & 3) as u8) } # [doc = "Bit 6 - ECCEN"] # [inline (always)] pub fn eccen (& self) -> EccenR { EccenR :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bits 9:12 - TCLR"] # [inline (always)] pub fn tclr (& self) -> TclrR { TclrR :: new (((self . bits >> 9) & 0x0f) as u8) } # [doc = "Bits 13:16 - TAR"] # [inline (always)] pub fn tar (& self) -> TarR { TarR :: new (((self . bits >> 13) & 0x0f) as u8) } # [doc = "Bits 17:19 - ECCPS"] # [inline (always)] pub fn eccps (& self) -> EccpsR { EccpsR :: new (((self . bits >> 17) & 7) as u8) } } impl W { # [doc = "Bit 1 - PWAITEN"] # [inline (always)] pub fn pwaiten (& mut self) -> PwaitenW < PcrSpec > { PwaitenW :: new (self , 1) } # [doc = "Bit 2 - PBKEN"] # [inline (always)] pub fn pbken (& mut self) -> PbkenW < PcrSpec > { PbkenW :: new (self , 2) } # [doc = "Bit 3 - PTYP"] # [inline (always)] pub fn ptyp (& mut self) -> PtypW < PcrSpec > { PtypW :: new (self , 3) } # [doc = "Bits 4:5 - PWID"] # [inline (always)] pub fn pwid (& mut self) -> PwidW < PcrSpec > { PwidW :: new (self , 4) } # [doc = "Bit 6 - ECCEN"] # [inline (always)] pub fn eccen (& mut self) -> EccenW < PcrSpec > { EccenW :: new (self , 6) } # [doc = "Bits 9:12 - TCLR"] # [inline (always)] pub fn tclr (& mut self) -> TclrW < PcrSpec > { TclrW :: new (self , 9) } # [doc = "Bits 13:16 - TAR"] # [inline (always)] pub fn tar (& mut self) -> TarW < PcrSpec > { TarW :: new (self , 13) } # [doc = "Bits 17:19 - ECCPS"] # [inline (always)] pub fn eccps (& mut self) -> EccpsW < PcrSpec > { EccpsW :: new (self , 17) } } # [doc = "PC Card/NAND Flash control register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct PcrSpec ; impl crate :: RegisterSpec for PcrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`pcr::R`](R) reader structure"] impl crate :: Readable for PcrSpec { } # [doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"] impl crate :: Writable for PcrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets PCR%s to value 0x18"] impl crate :: Resettable for PcrSpec { const RESET_VALUE : u32 = 0x18 ; }