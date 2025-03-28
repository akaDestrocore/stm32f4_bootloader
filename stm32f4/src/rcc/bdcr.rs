# [doc = "Register `BDCR` reader"] pub type R = crate :: R < BdcrSpec > ; # [doc = "Register `BDCR` writer"] pub type W = crate :: W < BdcrSpec > ; # [doc = "External low-speed oscillator enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Lseon { # [doc = "0: LSE oscillator Off"] Off = 0 , # [doc = "1: LSE oscillator On"] On = 1 , } impl From < Lseon > for bool { # [inline (always)] fn from (variant : Lseon) -> Self { variant as u8 != 0 } } # [doc = "Field `LSEON` reader - External low-speed oscillator enable"] pub type LseonR = crate :: BitReader < Lseon > ; impl LseonR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Lseon { match self . bits { false => Lseon :: Off , true => Lseon :: On , } } # [doc = "LSE oscillator Off"] # [inline (always)] pub fn is_off (& self) -> bool { * self == Lseon :: Off } # [doc = "LSE oscillator On"] # [inline (always)] pub fn is_on (& self) -> bool { * self == Lseon :: On } } # [doc = "Field `LSEON` writer - External low-speed oscillator enable"] pub type LseonW < 'a , REG > = crate :: BitWriter < 'a , REG , Lseon > ; impl < 'a , REG > LseonW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LSE oscillator Off"] # [inline (always)] pub fn off (self) -> & 'a mut crate :: W < REG > { self . variant (Lseon :: Off) } # [doc = "LSE oscillator On"] # [inline (always)] pub fn on (self) -> & 'a mut crate :: W < REG > { self . variant (Lseon :: On) } } # [doc = "External low-speed oscillator ready\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Lserdyr { # [doc = "0: LSE oscillator not ready"] NotReady = 0 , # [doc = "1: LSE oscillator ready"] Ready = 1 , } impl From < Lserdyr > for bool { # [inline (always)] fn from (variant : Lserdyr) -> Self { variant as u8 != 0 } } # [doc = "Field `LSERDY` reader - External low-speed oscillator ready"] pub type LserdyR = crate :: BitReader < Lserdyr > ; impl LserdyR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Lserdyr { match self . bits { false => Lserdyr :: NotReady , true => Lserdyr :: Ready , } } # [doc = "LSE oscillator not ready"] # [inline (always)] pub fn is_not_ready (& self) -> bool { * self == Lserdyr :: NotReady } # [doc = "LSE oscillator ready"] # [inline (always)] pub fn is_ready (& self) -> bool { * self == Lserdyr :: Ready } } # [doc = "External low-speed oscillator bypass\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Lsebyp { # [doc = "0: LSE crystal oscillator not bypassed"] NotBypassed = 0 , # [doc = "1: LSE crystal oscillator bypassed with external clock"] Bypassed = 1 , } impl From < Lsebyp > for bool { # [inline (always)] fn from (variant : Lsebyp) -> Self { variant as u8 != 0 } } # [doc = "Field `LSEBYP` reader - External low-speed oscillator bypass"] pub type LsebypR = crate :: BitReader < Lsebyp > ; impl LsebypR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Lsebyp { match self . bits { false => Lsebyp :: NotBypassed , true => Lsebyp :: Bypassed , } } # [doc = "LSE crystal oscillator not bypassed"] # [inline (always)] pub fn is_not_bypassed (& self) -> bool { * self == Lsebyp :: NotBypassed } # [doc = "LSE crystal oscillator bypassed with external clock"] # [inline (always)] pub fn is_bypassed (& self) -> bool { * self == Lsebyp :: Bypassed } } # [doc = "Field `LSEBYP` writer - External low-speed oscillator bypass"] pub type LsebypW < 'a , REG > = crate :: BitWriter < 'a , REG , Lsebyp > ; impl < 'a , REG > LsebypW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LSE crystal oscillator not bypassed"] # [inline (always)] pub fn not_bypassed (self) -> & 'a mut crate :: W < REG > { self . variant (Lsebyp :: NotBypassed) } # [doc = "LSE crystal oscillator bypassed with external clock"] # [inline (always)] pub fn bypassed (self) -> & 'a mut crate :: W < REG > { self . variant (Lsebyp :: Bypassed) } } # [doc = "RTC clock source selection\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum Rtcsel { # [doc = "0: No clock"] NoClock = 0 , # [doc = "1: LSE oscillator clock used as RTC clock"] Lse = 1 , # [doc = "2: LSI oscillator clock used as RTC clock"] Lsi = 2 , # [doc = "3: HSE oscillator clock divided by a prescaler used as RTC clock"] Hse = 3 , } impl From < Rtcsel > for u8 { # [inline (always)] fn from (variant : Rtcsel) -> Self { variant as _ } } impl crate :: FieldSpec for Rtcsel { type Ux = u8 ; } impl crate :: IsEnum for Rtcsel { } # [doc = "Field `RTCSEL` reader - RTC clock source selection"] pub type RtcselR = crate :: FieldReader < Rtcsel > ; impl RtcselR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Rtcsel { match self . bits { 0 => Rtcsel :: NoClock , 1 => Rtcsel :: Lse , 2 => Rtcsel :: Lsi , 3 => Rtcsel :: Hse , _ => unreachable ! () , } } # [doc = "No clock"] # [inline (always)] pub fn is_no_clock (& self) -> bool { * self == Rtcsel :: NoClock } # [doc = "LSE oscillator clock used as RTC clock"] # [inline (always)] pub fn is_lse (& self) -> bool { * self == Rtcsel :: Lse } # [doc = "LSI oscillator clock used as RTC clock"] # [inline (always)] pub fn is_lsi (& self) -> bool { * self == Rtcsel :: Lsi } # [doc = "HSE oscillator clock divided by a prescaler used as RTC clock"] # [inline (always)] pub fn is_hse (& self) -> bool { * self == Rtcsel :: Hse } } # [doc = "Field `RTCSEL` writer - RTC clock source selection"] pub type RtcselW < 'a , REG > = crate :: FieldWriter < 'a , REG , 2 , Rtcsel , crate :: Safe > ; impl < 'a , REG > RtcselW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "No clock"] # [inline (always)] pub fn no_clock (self) -> & 'a mut crate :: W < REG > { self . variant (Rtcsel :: NoClock) } # [doc = "LSE oscillator clock used as RTC clock"] # [inline (always)] pub fn lse (self) -> & 'a mut crate :: W < REG > { self . variant (Rtcsel :: Lse) } # [doc = "LSI oscillator clock used as RTC clock"] # [inline (always)] pub fn lsi (self) -> & 'a mut crate :: W < REG > { self . variant (Rtcsel :: Lsi) } # [doc = "HSE oscillator clock divided by a prescaler used as RTC clock"] # [inline (always)] pub fn hse (self) -> & 'a mut crate :: W < REG > { self . variant (Rtcsel :: Hse) } } # [doc = "RTC clock enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Rtcen { # [doc = "0: RTC clock disabled"] Disabled = 0 , # [doc = "1: RTC clock enabled"] Enabled = 1 , } impl From < Rtcen > for bool { # [inline (always)] fn from (variant : Rtcen) -> Self { variant as u8 != 0 } } # [doc = "Field `RTCEN` reader - RTC clock enable"] pub type RtcenR = crate :: BitReader < Rtcen > ; impl RtcenR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Rtcen { match self . bits { false => Rtcen :: Disabled , true => Rtcen :: Enabled , } } # [doc = "RTC clock disabled"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Rtcen :: Disabled } # [doc = "RTC clock enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Rtcen :: Enabled } } # [doc = "Field `RTCEN` writer - RTC clock enable"] pub type RtcenW < 'a , REG > = crate :: BitWriter < 'a , REG , Rtcen > ; impl < 'a , REG > RtcenW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "RTC clock disabled"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Rtcen :: Disabled) } # [doc = "RTC clock enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Rtcen :: Enabled) } } # [doc = "Backup domain software reset\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bdrst { # [doc = "0: Reset not activated"] Disabled = 0 , # [doc = "1: Reset the entire RTC domain"] Enabled = 1 , } impl From < Bdrst > for bool { # [inline (always)] fn from (variant : Bdrst) -> Self { variant as u8 != 0 } } # [doc = "Field `BDRST` reader - Backup domain software reset"] pub type BdrstR = crate :: BitReader < Bdrst > ; impl BdrstR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bdrst { match self . bits { false => Bdrst :: Disabled , true => Bdrst :: Enabled , } } # [doc = "Reset not activated"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Bdrst :: Disabled } # [doc = "Reset the entire RTC domain"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Bdrst :: Enabled } } # [doc = "Field `BDRST` writer - Backup domain software reset"] pub type BdrstW < 'a , REG > = crate :: BitWriter < 'a , REG , Bdrst > ; impl < 'a , REG > BdrstW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Reset not activated"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Bdrst :: Disabled) } # [doc = "Reset the entire RTC domain"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Bdrst :: Enabled) } } impl R { # [doc = "Bit 0 - External low-speed oscillator enable"] # [inline (always)] pub fn lseon (& self) -> LseonR { LseonR :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - External low-speed oscillator ready"] # [inline (always)] pub fn lserdy (& self) -> LserdyR { LserdyR :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - External low-speed oscillator bypass"] # [inline (always)] pub fn lsebyp (& self) -> LsebypR { LsebypR :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bits 8:9 - RTC clock source selection"] # [inline (always)] pub fn rtcsel (& self) -> RtcselR { RtcselR :: new (((self . bits >> 8) & 3) as u8) } # [doc = "Bit 15 - RTC clock enable"] # [inline (always)] pub fn rtcen (& self) -> RtcenR { RtcenR :: new (((self . bits >> 15) & 1) != 0) } # [doc = "Bit 16 - Backup domain software reset"] # [inline (always)] pub fn bdrst (& self) -> BdrstR { BdrstR :: new (((self . bits >> 16) & 1) != 0) } } impl W { # [doc = "Bit 0 - External low-speed oscillator enable"] # [inline (always)] pub fn lseon (& mut self) -> LseonW < BdcrSpec > { LseonW :: new (self , 0) } # [doc = "Bit 2 - External low-speed oscillator bypass"] # [inline (always)] pub fn lsebyp (& mut self) -> LsebypW < BdcrSpec > { LsebypW :: new (self , 2) } # [doc = "Bits 8:9 - RTC clock source selection"] # [inline (always)] pub fn rtcsel (& mut self) -> RtcselW < BdcrSpec > { RtcselW :: new (self , 8) } # [doc = "Bit 15 - RTC clock enable"] # [inline (always)] pub fn rtcen (& mut self) -> RtcenW < BdcrSpec > { RtcenW :: new (self , 15) } # [doc = "Bit 16 - Backup domain software reset"] # [inline (always)] pub fn bdrst (& mut self) -> BdrstW < BdcrSpec > { BdrstW :: new (self , 16) } } # [doc = "Backup domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct BdcrSpec ; impl crate :: RegisterSpec for BdcrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`bdcr::R`](R) reader structure"] impl crate :: Readable for BdcrSpec { } # [doc = "`write(|w| ..)` method takes [`bdcr::W`](W) writer structure"] impl crate :: Writable for BdcrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets BDCR to value 0"] impl crate :: Resettable for BdcrSpec { }