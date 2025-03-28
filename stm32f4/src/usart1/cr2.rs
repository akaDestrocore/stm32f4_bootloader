# [doc = "Register `CR2` reader"] pub type R = crate :: R < Cr2Spec > ; # [doc = "Register `CR2` writer"] pub type W = crate :: W < Cr2Spec > ; # [doc = "Field `ADD` reader - Address of the USART node"] pub type AddR = crate :: FieldReader ; # [doc = "Field `ADD` writer - Address of the USART node"] pub type AddW < 'a , REG > = crate :: FieldWriter < 'a , REG , 4 , u8 , crate :: Safe > ; # [doc = "lin break detection length\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Lbdl { # [doc = "0: 10-bit break detection"] Lbdl10 = 0 , # [doc = "1: 11-bit break detection"] Lbdl11 = 1 , } impl From < Lbdl > for bool { # [inline (always)] fn from (variant : Lbdl) -> Self { variant as u8 != 0 } } # [doc = "Field `LBDL` reader - lin break detection length"] pub type LbdlR = crate :: BitReader < Lbdl > ; impl LbdlR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Lbdl { match self . bits { false => Lbdl :: Lbdl10 , true => Lbdl :: Lbdl11 , } } # [doc = "10-bit break detection"] # [inline (always)] pub fn is_lbdl10 (& self) -> bool { * self == Lbdl :: Lbdl10 } # [doc = "11-bit break detection"] # [inline (always)] pub fn is_lbdl11 (& self) -> bool { * self == Lbdl :: Lbdl11 } } # [doc = "Field `LBDL` writer - lin break detection length"] pub type LbdlW < 'a , REG > = crate :: BitWriter < 'a , REG , Lbdl > ; impl < 'a , REG > LbdlW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "10-bit break detection"] # [inline (always)] pub fn lbdl10 (self) -> & 'a mut crate :: W < REG > { self . variant (Lbdl :: Lbdl10) } # [doc = "11-bit break detection"] # [inline (always)] pub fn lbdl11 (self) -> & 'a mut crate :: W < REG > { self . variant (Lbdl :: Lbdl11) } } # [doc = "LIN break detection interrupt enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Lbdie { # [doc = "0: LIN break detection interrupt disabled"] Disabled = 0 , # [doc = "1: LIN break detection interrupt enabled"] Enabled = 1 , } impl From < Lbdie > for bool { # [inline (always)] fn from (variant : Lbdie) -> Self { variant as u8 != 0 } } # [doc = "Field `LBDIE` reader - LIN break detection interrupt enable"] pub type LbdieR = crate :: BitReader < Lbdie > ; impl LbdieR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Lbdie { match self . bits { false => Lbdie :: Disabled , true => Lbdie :: Enabled , } } # [doc = "LIN break detection interrupt disabled"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Lbdie :: Disabled } # [doc = "LIN break detection interrupt enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Lbdie :: Enabled } } # [doc = "Field `LBDIE` writer - LIN break detection interrupt enable"] pub type LbdieW < 'a , REG > = crate :: BitWriter < 'a , REG , Lbdie > ; impl < 'a , REG > LbdieW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LIN break detection interrupt disabled"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Lbdie :: Disabled) } # [doc = "LIN break detection interrupt enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Lbdie :: Enabled) } } # [doc = "Last bit clock pulse\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Lbcl { # [doc = "0: The clock pulse of the last data bit is not output to the CK pin"] Disabled = 0 , # [doc = "1: The clock pulse of the last data bit is output to the CK pin"] Enabled = 1 , } impl From < Lbcl > for bool { # [inline (always)] fn from (variant : Lbcl) -> Self { variant as u8 != 0 } } # [doc = "Field `LBCL` reader - Last bit clock pulse"] pub type LbclR = crate :: BitReader < Lbcl > ; impl LbclR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Lbcl { match self . bits { false => Lbcl :: Disabled , true => Lbcl :: Enabled , } } # [doc = "The clock pulse of the last data bit is not output to the CK pin"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Lbcl :: Disabled } # [doc = "The clock pulse of the last data bit is output to the CK pin"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Lbcl :: Enabled } } # [doc = "Field `LBCL` writer - Last bit clock pulse"] pub type LbclW < 'a , REG > = crate :: BitWriter < 'a , REG , Lbcl > ; impl < 'a , REG > LbclW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "The clock pulse of the last data bit is not output to the CK pin"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Lbcl :: Disabled) } # [doc = "The clock pulse of the last data bit is output to the CK pin"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Lbcl :: Enabled) } } # [doc = "Clock phase\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Cpha { # [doc = "0: The first clock transition is the first data capture edge"] First = 0 , # [doc = "1: The second clock transition is the first data capture edge"] Second = 1 , } impl From < Cpha > for bool { # [inline (always)] fn from (variant : Cpha) -> Self { variant as u8 != 0 } } # [doc = "Field `CPHA` reader - Clock phase"] pub type CphaR = crate :: BitReader < Cpha > ; impl CphaR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Cpha { match self . bits { false => Cpha :: First , true => Cpha :: Second , } } # [doc = "The first clock transition is the first data capture edge"] # [inline (always)] pub fn is_first (& self) -> bool { * self == Cpha :: First } # [doc = "The second clock transition is the first data capture edge"] # [inline (always)] pub fn is_second (& self) -> bool { * self == Cpha :: Second } } # [doc = "Field `CPHA` writer - Clock phase"] pub type CphaW < 'a , REG > = crate :: BitWriter < 'a , REG , Cpha > ; impl < 'a , REG > CphaW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "The first clock transition is the first data capture edge"] # [inline (always)] pub fn first (self) -> & 'a mut crate :: W < REG > { self . variant (Cpha :: First) } # [doc = "The second clock transition is the first data capture edge"] # [inline (always)] pub fn second (self) -> & 'a mut crate :: W < REG > { self . variant (Cpha :: Second) } } # [doc = "Clock polarity\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Cpol { # [doc = "0: Steady low value on CK pin outside transmission window"] Low = 0 , # [doc = "1: Steady high value on CK pin outside transmission window"] High = 1 , } impl From < Cpol > for bool { # [inline (always)] fn from (variant : Cpol) -> Self { variant as u8 != 0 } } # [doc = "Field `CPOL` reader - Clock polarity"] pub type CpolR = crate :: BitReader < Cpol > ; impl CpolR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Cpol { match self . bits { false => Cpol :: Low , true => Cpol :: High , } } # [doc = "Steady low value on CK pin outside transmission window"] # [inline (always)] pub fn is_low (& self) -> bool { * self == Cpol :: Low } # [doc = "Steady high value on CK pin outside transmission window"] # [inline (always)] pub fn is_high (& self) -> bool { * self == Cpol :: High } } # [doc = "Field `CPOL` writer - Clock polarity"] pub type CpolW < 'a , REG > = crate :: BitWriter < 'a , REG , Cpol > ; impl < 'a , REG > CpolW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Steady low value on CK pin outside transmission window"] # [inline (always)] pub fn low (self) -> & 'a mut crate :: W < REG > { self . variant (Cpol :: Low) } # [doc = "Steady high value on CK pin outside transmission window"] # [inline (always)] pub fn high (self) -> & 'a mut crate :: W < REG > { self . variant (Cpol :: High) } } # [doc = "Clock enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Clken { # [doc = "0: CK pin disabled"] Disabled = 0 , # [doc = "1: CK pin enabled"] Enabled = 1 , } impl From < Clken > for bool { # [inline (always)] fn from (variant : Clken) -> Self { variant as u8 != 0 } } # [doc = "Field `CLKEN` reader - Clock enable"] pub type ClkenR = crate :: BitReader < Clken > ; impl ClkenR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Clken { match self . bits { false => Clken :: Disabled , true => Clken :: Enabled , } } # [doc = "CK pin disabled"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Clken :: Disabled } # [doc = "CK pin enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Clken :: Enabled } } # [doc = "Field `CLKEN` writer - Clock enable"] pub type ClkenW < 'a , REG > = crate :: BitWriter < 'a , REG , Clken > ; impl < 'a , REG > ClkenW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CK pin disabled"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Clken :: Disabled) } # [doc = "CK pin enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Clken :: Enabled) } } # [doc = "STOP bits\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum Stop { # [doc = "0: 1 stop bit"] Stop1 = 0 , # [doc = "1: 0.5 stop bits"] Stop0p5 = 1 , # [doc = "2: 2 stop bits"] Stop2 = 2 , # [doc = "3: 1.5 stop bits"] Stop1p5 = 3 , } impl From < Stop > for u8 { # [inline (always)] fn from (variant : Stop) -> Self { variant as _ } } impl crate :: FieldSpec for Stop { type Ux = u8 ; } impl crate :: IsEnum for Stop { } # [doc = "Field `STOP` reader - STOP bits"] pub type StopR = crate :: FieldReader < Stop > ; impl StopR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Stop { match self . bits { 0 => Stop :: Stop1 , 1 => Stop :: Stop0p5 , 2 => Stop :: Stop2 , 3 => Stop :: Stop1p5 , _ => unreachable ! () , } } # [doc = "1 stop bit"] # [inline (always)] pub fn is_stop1 (& self) -> bool { * self == Stop :: Stop1 } # [doc = "0.5 stop bits"] # [inline (always)] pub fn is_stop0p5 (& self) -> bool { * self == Stop :: Stop0p5 } # [doc = "2 stop bits"] # [inline (always)] pub fn is_stop2 (& self) -> bool { * self == Stop :: Stop2 } # [doc = "1.5 stop bits"] # [inline (always)] pub fn is_stop1p5 (& self) -> bool { * self == Stop :: Stop1p5 } } # [doc = "Field `STOP` writer - STOP bits"] pub type StopW < 'a , REG > = crate :: FieldWriter < 'a , REG , 2 , Stop , crate :: Safe > ; impl < 'a , REG > StopW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "1 stop bit"] # [inline (always)] pub fn stop1 (self) -> & 'a mut crate :: W < REG > { self . variant (Stop :: Stop1) } # [doc = "0.5 stop bits"] # [inline (always)] pub fn stop0p5 (self) -> & 'a mut crate :: W < REG > { self . variant (Stop :: Stop0p5) } # [doc = "2 stop bits"] # [inline (always)] pub fn stop2 (self) -> & 'a mut crate :: W < REG > { self . variant (Stop :: Stop2) } # [doc = "1.5 stop bits"] # [inline (always)] pub fn stop1p5 (self) -> & 'a mut crate :: W < REG > { self . variant (Stop :: Stop1p5) } } # [doc = "LIN mode enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Linen { # [doc = "0: LIN mode disabled"] Disabled = 0 , # [doc = "1: LIN mode enabled"] Enabled = 1 , } impl From < Linen > for bool { # [inline (always)] fn from (variant : Linen) -> Self { variant as u8 != 0 } } # [doc = "Field `LINEN` reader - LIN mode enable"] pub type LinenR = crate :: BitReader < Linen > ; impl LinenR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Linen { match self . bits { false => Linen :: Disabled , true => Linen :: Enabled , } } # [doc = "LIN mode disabled"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Linen :: Disabled } # [doc = "LIN mode enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Linen :: Enabled } } # [doc = "Field `LINEN` writer - LIN mode enable"] pub type LinenW < 'a , REG > = crate :: BitWriter < 'a , REG , Linen > ; impl < 'a , REG > LinenW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LIN mode disabled"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Linen :: Disabled) } # [doc = "LIN mode enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Linen :: Enabled) } } impl R { # [doc = "Bits 0:3 - Address of the USART node"] # [inline (always)] pub fn add (& self) -> AddR { AddR :: new ((self . bits & 0x0f) as u8) } # [doc = "Bit 5 - lin break detection length"] # [inline (always)] pub fn lbdl (& self) -> LbdlR { LbdlR :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - LIN break detection interrupt enable"] # [inline (always)] pub fn lbdie (& self) -> LbdieR { LbdieR :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 8 - Last bit clock pulse"] # [inline (always)] pub fn lbcl (& self) -> LbclR { LbclR :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - Clock phase"] # [inline (always)] pub fn cpha (& self) -> CphaR { CphaR :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - Clock polarity"] # [inline (always)] pub fn cpol (& self) -> CpolR { CpolR :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - Clock enable"] # [inline (always)] pub fn clken (& self) -> ClkenR { ClkenR :: new (((self . bits >> 11) & 1) != 0) } # [doc = "Bits 12:13 - STOP bits"] # [inline (always)] pub fn stop (& self) -> StopR { StopR :: new (((self . bits >> 12) & 3) as u8) } # [doc = "Bit 14 - LIN mode enable"] # [inline (always)] pub fn linen (& self) -> LinenR { LinenR :: new (((self . bits >> 14) & 1) != 0) } } impl W { # [doc = "Bits 0:3 - Address of the USART node"] # [inline (always)] pub fn add (& mut self) -> AddW < Cr2Spec > { AddW :: new (self , 0) } # [doc = "Bit 5 - lin break detection length"] # [inline (always)] pub fn lbdl (& mut self) -> LbdlW < Cr2Spec > { LbdlW :: new (self , 5) } # [doc = "Bit 6 - LIN break detection interrupt enable"] # [inline (always)] pub fn lbdie (& mut self) -> LbdieW < Cr2Spec > { LbdieW :: new (self , 6) } # [doc = "Bit 8 - Last bit clock pulse"] # [inline (always)] pub fn lbcl (& mut self) -> LbclW < Cr2Spec > { LbclW :: new (self , 8) } # [doc = "Bit 9 - Clock phase"] # [inline (always)] pub fn cpha (& mut self) -> CphaW < Cr2Spec > { CphaW :: new (self , 9) } # [doc = "Bit 10 - Clock polarity"] # [inline (always)] pub fn cpol (& mut self) -> CpolW < Cr2Spec > { CpolW :: new (self , 10) } # [doc = "Bit 11 - Clock enable"] # [inline (always)] pub fn clken (& mut self) -> ClkenW < Cr2Spec > { ClkenW :: new (self , 11) } # [doc = "Bits 12:13 - STOP bits"] # [inline (always)] pub fn stop (& mut self) -> StopW < Cr2Spec > { StopW :: new (self , 12) } # [doc = "Bit 14 - LIN mode enable"] # [inline (always)] pub fn linen (& mut self) -> LinenW < Cr2Spec > { LinenW :: new (self , 14) } } # [doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct Cr2Spec ; impl crate :: RegisterSpec for Cr2Spec { type Ux = u16 ; } # [doc = "`read()` method returns [`cr2::R`](R) reader structure"] impl crate :: Readable for Cr2Spec { } # [doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"] impl crate :: Writable for Cr2Spec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets CR2 to value 0"] impl crate :: Resettable for Cr2Spec { }