# [doc = "Register `CLKCR` reader"] pub type R = crate :: R < ClkcrSpec > ; # [doc = "Register `CLKCR` writer"] pub type W = crate :: W < ClkcrSpec > ; # [doc = "Field `CLKDIV` reader - Clock divide factor"] pub type ClkdivR = crate :: FieldReader ; # [doc = "Field `CLKDIV` writer - Clock divide factor"] pub type ClkdivW < 'a , REG > = crate :: FieldWriter < 'a , REG , 8 , u8 , crate :: Safe > ; # [doc = "Clock enable bit\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Clken { # [doc = "0: Disable clock"] Disabled = 0 , # [doc = "1: Enable clock"] Enabled = 1 , } impl From < Clken > for bool { # [inline (always)] fn from (variant : Clken) -> Self { variant as u8 != 0 } } # [doc = "Field `CLKEN` reader - Clock enable bit"] pub type ClkenR = crate :: BitReader < Clken > ; impl ClkenR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Clken { match self . bits { false => Clken :: Disabled , true => Clken :: Enabled , } } # [doc = "Disable clock"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Clken :: Disabled } # [doc = "Enable clock"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Clken :: Enabled } } # [doc = "Field `CLKEN` writer - Clock enable bit"] pub type ClkenW < 'a , REG > = crate :: BitWriter < 'a , REG , Clken > ; impl < 'a , REG > ClkenW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Disable clock"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Clken :: Disabled) } # [doc = "Enable clock"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Clken :: Enabled) } } # [doc = "Power saving configuration bit\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Pwrsav { # [doc = "0: SDIO_CK clock is always enabled"] Enabled = 0 , # [doc = "1: SDIO_CK is only enabled when the bus is active"] Disabled = 1 , } impl From < Pwrsav > for bool { # [inline (always)] fn from (variant : Pwrsav) -> Self { variant as u8 != 0 } } # [doc = "Field `PWRSAV` reader - Power saving configuration bit"] pub type PwrsavR = crate :: BitReader < Pwrsav > ; impl PwrsavR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Pwrsav { match self . bits { false => Pwrsav :: Enabled , true => Pwrsav :: Disabled , } } # [doc = "SDIO_CK clock is always enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Pwrsav :: Enabled } # [doc = "SDIO_CK is only enabled when the bus is active"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Pwrsav :: Disabled } } # [doc = "Field `PWRSAV` writer - Power saving configuration bit"] pub type PwrsavW < 'a , REG > = crate :: BitWriter < 'a , REG , Pwrsav > ; impl < 'a , REG > PwrsavW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "SDIO_CK clock is always enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Pwrsav :: Enabled) } # [doc = "SDIO_CK is only enabled when the bus is active"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Pwrsav :: Disabled) } } # [doc = "Clock divider bypass enable bit\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bypass { # [doc = "0: SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal."] Disabled = 0 , # [doc = "1: SDIOCLK directly drives the SDIO_CK output signal"] Enabled = 1 , } impl From < Bypass > for bool { # [inline (always)] fn from (variant : Bypass) -> Self { variant as u8 != 0 } } # [doc = "Field `BYPASS` reader - Clock divider bypass enable bit"] pub type BypassR = crate :: BitReader < Bypass > ; impl BypassR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bypass { match self . bits { false => Bypass :: Disabled , true => Bypass :: Enabled , } } # [doc = "SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal."] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Bypass :: Disabled } # [doc = "SDIOCLK directly drives the SDIO_CK output signal"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Bypass :: Enabled } } # [doc = "Field `BYPASS` writer - Clock divider bypass enable bit"] pub type BypassW < 'a , REG > = crate :: BitWriter < 'a , REG , Bypass > ; impl < 'a , REG > BypassW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal."] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Bypass :: Disabled) } # [doc = "SDIOCLK directly drives the SDIO_CK output signal"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Bypass :: Enabled) } } # [doc = "Wide bus mode enable bit\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum Widbus { # [doc = "0: 1 lane wide bus"] BusWidth1 = 0 , # [doc = "1: 4 lane wide bus"] BusWidth4 = 1 , # [doc = "2: 8 lane wide bus"] BusWidth8 = 2 , } impl From < Widbus > for u8 { # [inline (always)] fn from (variant : Widbus) -> Self { variant as _ } } impl crate :: FieldSpec for Widbus { type Ux = u8 ; } impl crate :: IsEnum for Widbus { } # [doc = "Field `WIDBUS` reader - Wide bus mode enable bit"] pub type WidbusR = crate :: FieldReader < Widbus > ; impl WidbusR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Widbus > { match self . bits { 0 => Some (Widbus :: BusWidth1) , 1 => Some (Widbus :: BusWidth4) , 2 => Some (Widbus :: BusWidth8) , _ => None , } } # [doc = "1 lane wide bus"] # [inline (always)] pub fn is_bus_width1 (& self) -> bool { * self == Widbus :: BusWidth1 } # [doc = "4 lane wide bus"] # [inline (always)] pub fn is_bus_width4 (& self) -> bool { * self == Widbus :: BusWidth4 } # [doc = "8 lane wide bus"] # [inline (always)] pub fn is_bus_width8 (& self) -> bool { * self == Widbus :: BusWidth8 } } # [doc = "Field `WIDBUS` writer - Wide bus mode enable bit"] pub type WidbusW < 'a , REG > = crate :: FieldWriter < 'a , REG , 2 , Widbus > ; impl < 'a , REG > WidbusW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "1 lane wide bus"] # [inline (always)] pub fn bus_width1 (self) -> & 'a mut crate :: W < REG > { self . variant (Widbus :: BusWidth1) } # [doc = "4 lane wide bus"] # [inline (always)] pub fn bus_width4 (self) -> & 'a mut crate :: W < REG > { self . variant (Widbus :: BusWidth4) } # [doc = "8 lane wide bus"] # [inline (always)] pub fn bus_width8 (self) -> & 'a mut crate :: W < REG > { self . variant (Widbus :: BusWidth8) } } # [doc = "SDIO_CK dephasing selection bit\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Negedge { # [doc = "0: SDIO_CK generated on the rising edge"] Rising = 0 , # [doc = "1: SDIO_CK generated on the falling edge"] Falling = 1 , } impl From < Negedge > for bool { # [inline (always)] fn from (variant : Negedge) -> Self { variant as u8 != 0 } } # [doc = "Field `NEGEDGE` reader - SDIO_CK dephasing selection bit"] pub type NegedgeR = crate :: BitReader < Negedge > ; impl NegedgeR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Negedge { match self . bits { false => Negedge :: Rising , true => Negedge :: Falling , } } # [doc = "SDIO_CK generated on the rising edge"] # [inline (always)] pub fn is_rising (& self) -> bool { * self == Negedge :: Rising } # [doc = "SDIO_CK generated on the falling edge"] # [inline (always)] pub fn is_falling (& self) -> bool { * self == Negedge :: Falling } } # [doc = "Field `NEGEDGE` writer - SDIO_CK dephasing selection bit"] pub type NegedgeW < 'a , REG > = crate :: BitWriter < 'a , REG , Negedge > ; impl < 'a , REG > NegedgeW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "SDIO_CK generated on the rising edge"] # [inline (always)] pub fn rising (self) -> & 'a mut crate :: W < REG > { self . variant (Negedge :: Rising) } # [doc = "SDIO_CK generated on the falling edge"] # [inline (always)] pub fn falling (self) -> & 'a mut crate :: W < REG > { self . variant (Negedge :: Falling) } } # [doc = "HW Flow Control enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum HwfcEn { # [doc = "0: HW Flow Control is disabled"] Disabled = 0 , # [doc = "1: HW Flow Control is enabled"] Enabled = 1 , } impl From < HwfcEn > for bool { # [inline (always)] fn from (variant : HwfcEn) -> Self { variant as u8 != 0 } } # [doc = "Field `HWFC_EN` reader - HW Flow Control enable"] pub type HwfcEnR = crate :: BitReader < HwfcEn > ; impl HwfcEnR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> HwfcEn { match self . bits { false => HwfcEn :: Disabled , true => HwfcEn :: Enabled , } } # [doc = "HW Flow Control is disabled"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == HwfcEn :: Disabled } # [doc = "HW Flow Control is enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == HwfcEn :: Enabled } } # [doc = "Field `HWFC_EN` writer - HW Flow Control enable"] pub type HwfcEnW < 'a , REG > = crate :: BitWriter < 'a , REG , HwfcEn > ; impl < 'a , REG > HwfcEnW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "HW Flow Control is disabled"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (HwfcEn :: Disabled) } # [doc = "HW Flow Control is enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (HwfcEn :: Enabled) } } impl R { # [doc = "Bits 0:7 - Clock divide factor"] # [inline (always)] pub fn clkdiv (& self) -> ClkdivR { ClkdivR :: new ((self . bits & 0xff) as u8) } # [doc = "Bit 8 - Clock enable bit"] # [inline (always)] pub fn clken (& self) -> ClkenR { ClkenR :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - Power saving configuration bit"] # [inline (always)] pub fn pwrsav (& self) -> PwrsavR { PwrsavR :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - Clock divider bypass enable bit"] # [inline (always)] pub fn bypass (& self) -> BypassR { BypassR :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bits 11:12 - Wide bus mode enable bit"] # [inline (always)] pub fn widbus (& self) -> WidbusR { WidbusR :: new (((self . bits >> 11) & 3) as u8) } # [doc = "Bit 13 - SDIO_CK dephasing selection bit"] # [inline (always)] pub fn negedge (& self) -> NegedgeR { NegedgeR :: new (((self . bits >> 13) & 1) != 0) } # [doc = "Bit 14 - HW Flow Control enable"] # [inline (always)] pub fn hwfc_en (& self) -> HwfcEnR { HwfcEnR :: new (((self . bits >> 14) & 1) != 0) } } impl W { # [doc = "Bits 0:7 - Clock divide factor"] # [inline (always)] pub fn clkdiv (& mut self) -> ClkdivW < ClkcrSpec > { ClkdivW :: new (self , 0) } # [doc = "Bit 8 - Clock enable bit"] # [inline (always)] pub fn clken (& mut self) -> ClkenW < ClkcrSpec > { ClkenW :: new (self , 8) } # [doc = "Bit 9 - Power saving configuration bit"] # [inline (always)] pub fn pwrsav (& mut self) -> PwrsavW < ClkcrSpec > { PwrsavW :: new (self , 9) } # [doc = "Bit 10 - Clock divider bypass enable bit"] # [inline (always)] pub fn bypass (& mut self) -> BypassW < ClkcrSpec > { BypassW :: new (self , 10) } # [doc = "Bits 11:12 - Wide bus mode enable bit"] # [inline (always)] pub fn widbus (& mut self) -> WidbusW < ClkcrSpec > { WidbusW :: new (self , 11) } # [doc = "Bit 13 - SDIO_CK dephasing selection bit"] # [inline (always)] pub fn negedge (& mut self) -> NegedgeW < ClkcrSpec > { NegedgeW :: new (self , 13) } # [doc = "Bit 14 - HW Flow Control enable"] # [inline (always)] pub fn hwfc_en (& mut self) -> HwfcEnW < ClkcrSpec > { HwfcEnW :: new (self , 14) } } # [doc = "SDI clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct ClkcrSpec ; impl crate :: RegisterSpec for ClkcrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`clkcr::R`](R) reader structure"] impl crate :: Readable for ClkcrSpec { } # [doc = "`write(|w| ..)` method takes [`clkcr::W`](W) writer structure"] impl crate :: Writable for ClkcrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets CLKCR to value 0"] impl crate :: Resettable for ClkcrSpec { }