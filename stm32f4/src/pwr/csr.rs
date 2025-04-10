# [doc = "Register `CSR` reader"] pub type R = crate :: R < CsrSpec > ; # [doc = "Register `CSR` writer"] pub type W = crate :: W < CsrSpec > ; # [doc = "Wakeup flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Wuf { # [doc = "0: No wakeup event occurred"] NotOccurred = 0 , # [doc = "1: A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)"] Occurred = 1 , } impl From < Wuf > for bool { # [inline (always)] fn from (variant : Wuf) -> Self { variant as u8 != 0 } } # [doc = "Field `WUF` reader - Wakeup flag"] pub type WufR = crate :: BitReader < Wuf > ; impl WufR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Wuf { match self . bits { false => Wuf :: NotOccurred , true => Wuf :: Occurred , } } # [doc = "No wakeup event occurred"] # [inline (always)] pub fn is_not_occurred (& self) -> bool { * self == Wuf :: NotOccurred } # [doc = "A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)"] # [inline (always)] pub fn is_occurred (& self) -> bool { * self == Wuf :: Occurred } } # [doc = "Standby flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Sbf { # [doc = "0: Device has not been in Standby mode"] InStandby = 0 , # [doc = "1: Device has been in Standby mode"] NotInStandby = 1 , } impl From < Sbf > for bool { # [inline (always)] fn from (variant : Sbf) -> Self { variant as u8 != 0 } } # [doc = "Field `SBF` reader - Standby flag"] pub type SbfR = crate :: BitReader < Sbf > ; impl SbfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Sbf { match self . bits { false => Sbf :: InStandby , true => Sbf :: NotInStandby , } } # [doc = "Device has not been in Standby mode"] # [inline (always)] pub fn is_in_standby (& self) -> bool { * self == Sbf :: InStandby } # [doc = "Device has been in Standby mode"] # [inline (always)] pub fn is_not_in_standby (& self) -> bool { * self == Sbf :: NotInStandby } } # [doc = "PVD output\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Pvdo { # [doc = "0: Vdd is higher than the PVD threshold selected with the PLS\\[2:0\\] bits"] Higher = 0 , # [doc = "1: Vdd is lower than the PVD threshold selected with the PLS\\[2:0\\] bits"] Lower = 1 , } impl From < Pvdo > for bool { # [inline (always)] fn from (variant : Pvdo) -> Self { variant as u8 != 0 } } # [doc = "Field `PVDO` reader - PVD output"] pub type PvdoR = crate :: BitReader < Pvdo > ; impl PvdoR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Pvdo { match self . bits { false => Pvdo :: Higher , true => Pvdo :: Lower , } } # [doc = "Vdd is higher than the PVD threshold selected with the PLS\\[2:0\\] bits"] # [inline (always)] pub fn is_higher (& self) -> bool { * self == Pvdo :: Higher } # [doc = "Vdd is lower than the PVD threshold selected with the PLS\\[2:0\\] bits"] # [inline (always)] pub fn is_lower (& self) -> bool { * self == Pvdo :: Lower } } # [doc = "Backup regulator ready\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Brr { # [doc = "0: Backup Regulator not ready"] NotReady = 0 , # [doc = "1: Backup Regulator ready"] Ready = 1 , } impl From < Brr > for bool { # [inline (always)] fn from (variant : Brr) -> Self { variant as u8 != 0 } } # [doc = "Field `BRR` reader - Backup regulator ready"] pub type BrrR = crate :: BitReader < Brr > ; impl BrrR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Brr { match self . bits { false => Brr :: NotReady , true => Brr :: Ready , } } # [doc = "Backup Regulator not ready"] # [inline (always)] pub fn is_not_ready (& self) -> bool { * self == Brr :: NotReady } # [doc = "Backup Regulator ready"] # [inline (always)] pub fn is_ready (& self) -> bool { * self == Brr :: Ready } } # [doc = "Enable WKUP pin\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Ewup { # [doc = "0: WKUP1 pin is used for general purpose I/O. An event on the WKUP1 pin does not wakeup the device from Standby mode"] Gpio = 0 , # [doc = "1: WKUP1 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP pin wakes-up the system from Standby mode)"] WakeUp = 1 , } impl From < Ewup > for bool { # [inline (always)] fn from (variant : Ewup) -> Self { variant as u8 != 0 } } # [doc = "Field `EWUP` reader - Enable WKUP pin"] pub type EwupR = crate :: BitReader < Ewup > ; impl EwupR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Ewup { match self . bits { false => Ewup :: Gpio , true => Ewup :: WakeUp , } } # [doc = "WKUP1 pin is used for general purpose I/O. An event on the WKUP1 pin does not wakeup the device from Standby mode"] # [inline (always)] pub fn is_gpio (& self) -> bool { * self == Ewup :: Gpio } # [doc = "WKUP1 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP pin wakes-up the system from Standby mode)"] # [inline (always)] pub fn is_wake_up (& self) -> bool { * self == Ewup :: WakeUp } } # [doc = "Field `EWUP` writer - Enable WKUP pin"] pub type EwupW < 'a , REG > = crate :: BitWriter < 'a , REG , Ewup > ; impl < 'a , REG > EwupW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "WKUP1 pin is used for general purpose I/O. An event on the WKUP1 pin does not wakeup the device from Standby mode"] # [inline (always)] pub fn gpio (self) -> & 'a mut crate :: W < REG > { self . variant (Ewup :: Gpio) } # [doc = "WKUP1 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP pin wakes-up the system from Standby mode)"] # [inline (always)] pub fn wake_up (self) -> & 'a mut crate :: W < REG > { self . variant (Ewup :: WakeUp) } } # [doc = "Backup regulator enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bre { # [doc = "0: Backup regulator disabled"] Disabled = 0 , # [doc = "1: Backup regulator enabled"] Enabled = 1 , } impl From < Bre > for bool { # [inline (always)] fn from (variant : Bre) -> Self { variant as u8 != 0 } } # [doc = "Field `BRE` reader - Backup regulator enable"] pub type BreR = crate :: BitReader < Bre > ; impl BreR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bre { match self . bits { false => Bre :: Disabled , true => Bre :: Enabled , } } # [doc = "Backup regulator disabled"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Bre :: Disabled } # [doc = "Backup regulator enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Bre :: Enabled } } # [doc = "Field `BRE` writer - Backup regulator enable"] pub type BreW < 'a , REG > = crate :: BitWriter < 'a , REG , Bre > ; impl < 'a , REG > BreW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Backup regulator disabled"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Bre :: Disabled) } # [doc = "Backup regulator enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Bre :: Enabled) } } # [doc = "Regulator voltage scaling output selection ready bit\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Vosrdy { # [doc = "0: Not ready"] NotReady = 0 , # [doc = "1: Ready"] Ready = 1 , } impl From < Vosrdy > for bool { # [inline (always)] fn from (variant : Vosrdy) -> Self { variant as u8 != 0 } } # [doc = "Field `VOSRDY` reader - Regulator voltage scaling output selection ready bit"] pub type VosrdyR = crate :: BitReader < Vosrdy > ; impl VosrdyR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Vosrdy { match self . bits { false => Vosrdy :: NotReady , true => Vosrdy :: Ready , } } # [doc = "Not ready"] # [inline (always)] pub fn is_not_ready (& self) -> bool { * self == Vosrdy :: NotReady } # [doc = "Ready"] # [inline (always)] pub fn is_ready (& self) -> bool { * self == Vosrdy :: Ready } } impl R { # [doc = "Bit 0 - Wakeup flag"] # [inline (always)] pub fn wuf (& self) -> WufR { WufR :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Standby flag"] # [inline (always)] pub fn sbf (& self) -> SbfR { SbfR :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - PVD output"] # [inline (always)] pub fn pvdo (& self) -> PvdoR { PvdoR :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Backup regulator ready"] # [inline (always)] pub fn brr (& self) -> BrrR { BrrR :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 8 - Enable WKUP pin"] # [inline (always)] pub fn ewup (& self) -> EwupR { EwupR :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - Backup regulator enable"] # [inline (always)] pub fn bre (& self) -> BreR { BreR :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 14 - Regulator voltage scaling output selection ready bit"] # [inline (always)] pub fn vosrdy (& self) -> VosrdyR { VosrdyR :: new (((self . bits >> 14) & 1) != 0) } } impl W { # [doc = "Bit 8 - Enable WKUP pin"] # [inline (always)] pub fn ewup (& mut self) -> EwupW < CsrSpec > { EwupW :: new (self , 8) } # [doc = "Bit 9 - Backup regulator enable"] # [inline (always)] pub fn bre (& mut self) -> BreW < CsrSpec > { BreW :: new (self , 9) } } # [doc = "power control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CsrSpec ; impl crate :: RegisterSpec for CsrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`csr::R`](R) reader structure"] impl crate :: Readable for CsrSpec { } # [doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"] impl crate :: Writable for CsrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets CSR to value 0"] impl crate :: Resettable for CsrSpec { }