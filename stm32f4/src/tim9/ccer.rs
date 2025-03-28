# [doc = "Register `CCER` reader"] pub type R = crate :: R < CcerSpec > ; # [doc = "Register `CCER` writer"] pub type W = crate :: W < CcerSpec > ; # [doc = "Capture/Compare %s output enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Cc1e { # [doc = "0: Capture disabled"] Disabled = 0 , # [doc = "1: Capture enabled"] Enabled = 1 , } impl From < Cc1e > for bool { # [inline (always)] fn from (variant : Cc1e) -> Self { variant as u8 != 0 } } # [doc = "Field `CCE(1-2)` reader - Capture/Compare %s output enable"] pub type CceR = crate :: BitReader < Cc1e > ; impl CceR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Cc1e { match self . bits { false => Cc1e :: Disabled , true => Cc1e :: Enabled , } } # [doc = "Capture disabled"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Cc1e :: Disabled } # [doc = "Capture enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Cc1e :: Enabled } } # [doc = "Field `CCE(1-2)` writer - Capture/Compare %s output enable"] pub type CceW < 'a , REG > = crate :: BitWriter < 'a , REG , Cc1e > ; impl < 'a , REG > CceW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Capture disabled"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Cc1e :: Disabled) } # [doc = "Capture enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Cc1e :: Enabled) } } # [doc = "Capture/Compare %s output Polarity\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Cc1p { # [doc = "0: Noninverted/rising edge"] RisingEdge = 0 , # [doc = "1: Inverted/falling edge"] FallingEdge = 1 , } impl From < Cc1p > for bool { # [inline (always)] fn from (variant : Cc1p) -> Self { variant as u8 != 0 } } # [doc = "Field `CCP(1-2)` reader - Capture/Compare %s output Polarity"] pub type CcpR = crate :: BitReader < Cc1p > ; impl CcpR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Cc1p { match self . bits { false => Cc1p :: RisingEdge , true => Cc1p :: FallingEdge , } } # [doc = "Noninverted/rising edge"] # [inline (always)] pub fn is_rising_edge (& self) -> bool { * self == Cc1p :: RisingEdge } # [doc = "Inverted/falling edge"] # [inline (always)] pub fn is_falling_edge (& self) -> bool { * self == Cc1p :: FallingEdge } } # [doc = "Field `CCP(1-2)` writer - Capture/Compare %s output Polarity"] pub type CcpW < 'a , REG > = crate :: BitWriter < 'a , REG , Cc1p > ; impl < 'a , REG > CcpW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Noninverted/rising edge"] # [inline (always)] pub fn rising_edge (self) -> & 'a mut crate :: W < REG > { self . variant (Cc1p :: RisingEdge) } # [doc = "Inverted/falling edge"] # [inline (always)] pub fn falling_edge (self) -> & 'a mut crate :: W < REG > { self . variant (Cc1p :: FallingEdge) } } # [doc = "Field `CCNP(1-2)` reader - Capture/Compare %s output Polarity"] pub type CcnpR = crate :: BitReader ; # [doc = "Field `CCNP(1-2)` writer - Capture/Compare %s output Polarity"] pub type CcnpW < 'a , REG > = crate :: BitWriter < 'a , REG > ; impl R { # [doc = "Capture/Compare (1-2) output enable"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CC1E` field.</div>"] # [inline (always)] pub fn cce (& self , n : u8) -> CceR { # [allow (clippy :: no_effect)] [() ; 2] [n as usize] ; CceR :: new (((self . bits >> (n * 4)) & 1) != 0) } # [doc = "Iterator for array of:"] # [doc = "Capture/Compare (1-2) output enable"] # [inline (always)] pub fn cce_iter (& self) -> impl Iterator < Item = CceR > + '_ { (0 .. 2) . map (move | n | CceR :: new (((self . bits >> (n * 4)) & 1) != 0)) } # [doc = "Bit 0 - Capture/Compare 1 output enable"] # [inline (always)] pub fn cc1e (& self) -> CceR { CceR :: new ((self . bits & 1) != 0) } # [doc = "Bit 4 - Capture/Compare 2 output enable"] # [inline (always)] pub fn cc2e (& self) -> CceR { CceR :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Capture/Compare (1-2) output Polarity"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CC1P` field.</div>"] # [inline (always)] pub fn ccp (& self , n : u8) -> CcpR { # [allow (clippy :: no_effect)] [() ; 2] [n as usize] ; CcpR :: new (((self . bits >> (n * 4 + 1)) & 1) != 0) } # [doc = "Iterator for array of:"] # [doc = "Capture/Compare (1-2) output Polarity"] # [inline (always)] pub fn ccp_iter (& self) -> impl Iterator < Item = CcpR > + '_ { (0 .. 2) . map (move | n | CcpR :: new (((self . bits >> (n * 4 + 1)) & 1) != 0)) } # [doc = "Bit 1 - Capture/Compare 1 output Polarity"] # [inline (always)] pub fn cc1p (& self) -> CcpR { CcpR :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 5 - Capture/Compare 2 output Polarity"] # [inline (always)] pub fn cc2p (& self) -> CcpR { CcpR :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Capture/Compare (1-2) output Polarity"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CC1NP` field.</div>"] # [inline (always)] pub fn ccnp (& self , n : u8) -> CcnpR { # [allow (clippy :: no_effect)] [() ; 2] [n as usize] ; CcnpR :: new (((self . bits >> (n * 4 + 3)) & 1) != 0) } # [doc = "Iterator for array of:"] # [doc = "Capture/Compare (1-2) output Polarity"] # [inline (always)] pub fn ccnp_iter (& self) -> impl Iterator < Item = CcnpR > + '_ { (0 .. 2) . map (move | n | CcnpR :: new (((self . bits >> (n * 4 + 3)) & 1) != 0)) } # [doc = "Bit 3 - Capture/Compare 1 output Polarity"] # [inline (always)] pub fn cc1np (& self) -> CcnpR { CcnpR :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 7 - Capture/Compare 2 output Polarity"] # [inline (always)] pub fn cc2np (& self) -> CcnpR { CcnpR :: new (((self . bits >> 7) & 1) != 0) } } impl W { # [doc = "Capture/Compare (1-2) output enable"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CC1E` field.</div>"] # [inline (always)] pub fn cce (& mut self , n : u8) -> CceW < CcerSpec > { # [allow (clippy :: no_effect)] [() ; 2] [n as usize] ; CceW :: new (self , n * 4) } # [doc = "Bit 0 - Capture/Compare 1 output enable"] # [inline (always)] pub fn cc1e (& mut self) -> CceW < CcerSpec > { CceW :: new (self , 0) } # [doc = "Bit 4 - Capture/Compare 2 output enable"] # [inline (always)] pub fn cc2e (& mut self) -> CceW < CcerSpec > { CceW :: new (self , 4) } # [doc = "Capture/Compare (1-2) output Polarity"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CC1P` field.</div>"] # [inline (always)] pub fn ccp (& mut self , n : u8) -> CcpW < CcerSpec > { # [allow (clippy :: no_effect)] [() ; 2] [n as usize] ; CcpW :: new (self , n * 4 + 1) } # [doc = "Bit 1 - Capture/Compare 1 output Polarity"] # [inline (always)] pub fn cc1p (& mut self) -> CcpW < CcerSpec > { CcpW :: new (self , 1) } # [doc = "Bit 5 - Capture/Compare 2 output Polarity"] # [inline (always)] pub fn cc2p (& mut self) -> CcpW < CcerSpec > { CcpW :: new (self , 5) } # [doc = "Capture/Compare (1-2) output Polarity"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CC1NP` field.</div>"] # [inline (always)] pub fn ccnp (& mut self , n : u8) -> CcnpW < CcerSpec > { # [allow (clippy :: no_effect)] [() ; 2] [n as usize] ; CcnpW :: new (self , n * 4 + 3) } # [doc = "Bit 3 - Capture/Compare 1 output Polarity"] # [inline (always)] pub fn cc1np (& mut self) -> CcnpW < CcerSpec > { CcnpW :: new (self , 3) } # [doc = "Bit 7 - Capture/Compare 2 output Polarity"] # [inline (always)] pub fn cc2np (& mut self) -> CcnpW < CcerSpec > { CcnpW :: new (self , 7) } } # [doc = "capture/compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CcerSpec ; impl crate :: RegisterSpec for CcerSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`ccer::R`](R) reader structure"] impl crate :: Readable for CcerSpec { } # [doc = "`write(|w| ..)` method takes [`ccer::W`](W) writer structure"] impl crate :: Writable for CcerSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets CCER to value 0"] impl crate :: Resettable for CcerSpec { }