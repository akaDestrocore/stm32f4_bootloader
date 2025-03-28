# [doc = "Register `CCMR1_Output` reader"] pub type R = crate :: R < Ccmr1OutputSpec > ; # [doc = "Register `CCMR1_Output` writer"] pub type W = crate :: W < Ccmr1OutputSpec > ; # [doc = "Capture/Compare %s selection\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum Cc1s { # [doc = "0: CCx channel is configured as output"] Output = 0 , } impl From < Cc1s > for u8 { # [inline (always)] fn from (variant : Cc1s) -> Self { variant as _ } } impl crate :: FieldSpec for Cc1s { type Ux = u8 ; } impl crate :: IsEnum for Cc1s { } # [doc = "Field `CCS(1-1)` reader - Capture/Compare %s selection"] pub type CcsR = crate :: FieldReader < Cc1s > ; impl CcsR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Cc1s > { match self . bits { 0 => Some (Cc1s :: Output) , _ => None , } } # [doc = "CCx channel is configured as output"] # [inline (always)] pub fn is_output (& self) -> bool { * self == Cc1s :: Output } } # [doc = "Field `CCS(1-1)` writer - Capture/Compare %s selection"] pub type CcsW < 'a , REG > = crate :: FieldWriter < 'a , REG , 2 , Cc1s > ; impl < 'a , REG > CcsW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "CCx channel is configured as output"] # [inline (always)] pub fn output (self) -> & 'a mut crate :: W < REG > { self . variant (Cc1s :: Output) } } # [doc = "Output compare %s fast enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Oc1fe { # [doc = "0: Fast output disabled"] Disabled = 0 , # [doc = "1: Fast output enabled"] Enabled = 1 , } impl From < Oc1fe > for bool { # [inline (always)] fn from (variant : Oc1fe) -> Self { variant as u8 != 0 } } # [doc = "Field `OCFE(1-1)` reader - Output compare %s fast enable"] pub type OcfeR = crate :: BitReader < Oc1fe > ; impl OcfeR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Oc1fe { match self . bits { false => Oc1fe :: Disabled , true => Oc1fe :: Enabled , } } # [doc = "Fast output disabled"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Oc1fe :: Disabled } # [doc = "Fast output enabled"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Oc1fe :: Enabled } } # [doc = "Field `OCFE(1-1)` writer - Output compare %s fast enable"] pub type OcfeW < 'a , REG > = crate :: BitWriter < 'a , REG , Oc1fe > ; impl < 'a , REG > OcfeW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Fast output disabled"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1fe :: Disabled) } # [doc = "Fast output enabled"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1fe :: Enabled) } } # [doc = "Output compare %s preload enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Oc1pe { # [doc = "0: Preload register on CCRx disabled. New values written to CCRx are taken into account immediately"] Disabled = 0 , # [doc = "1: Preload register on CCRx enabled. Preload value is loaded into active register on each update event"] Enabled = 1 , } impl From < Oc1pe > for bool { # [inline (always)] fn from (variant : Oc1pe) -> Self { variant as u8 != 0 } } # [doc = "Field `OCPE(1-1)` reader - Output compare %s preload enable"] pub type OcpeR = crate :: BitReader < Oc1pe > ; impl OcpeR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Oc1pe { match self . bits { false => Oc1pe :: Disabled , true => Oc1pe :: Enabled , } } # [doc = "Preload register on CCRx disabled. New values written to CCRx are taken into account immediately"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Oc1pe :: Disabled } # [doc = "Preload register on CCRx enabled. Preload value is loaded into active register on each update event"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Oc1pe :: Enabled } } # [doc = "Field `OCPE(1-1)` writer - Output compare %s preload enable"] pub type OcpeW < 'a , REG > = crate :: BitWriter < 'a , REG , Oc1pe > ; impl < 'a , REG > OcpeW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Preload register on CCRx disabled. New values written to CCRx are taken into account immediately"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1pe :: Disabled) } # [doc = "Preload register on CCRx enabled. Preload value is loaded into active register on each update event"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1pe :: Enabled) } } # [doc = "Output compare %s mode\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum Oc1m { # [doc = "0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"] Frozen = 0 , # [doc = "1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"] ActiveOnMatch = 1 , # [doc = "2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"] InactiveOnMatch = 2 , # [doc = "3: OCyREF toggles when TIMx_CNT=TIMx_CCRy"] Toggle = 3 , # [doc = "4: OCyREF is forced low"] ForceInactive = 4 , # [doc = "5: OCyREF is forced high"] ForceActive = 5 , # [doc = "6: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"] PwmMode1 = 6 , # [doc = "7: Inversely to PwmMode1"] PwmMode2 = 7 , } impl From < Oc1m > for u8 { # [inline (always)] fn from (variant : Oc1m) -> Self { variant as _ } } impl crate :: FieldSpec for Oc1m { type Ux = u8 ; } impl crate :: IsEnum for Oc1m { } # [doc = "Field `OCM(1-1)` reader - Output compare %s mode"] pub type OcmR = crate :: FieldReader < Oc1m > ; impl OcmR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Oc1m { match self . bits { 0 => Oc1m :: Frozen , 1 => Oc1m :: ActiveOnMatch , 2 => Oc1m :: InactiveOnMatch , 3 => Oc1m :: Toggle , 4 => Oc1m :: ForceInactive , 5 => Oc1m :: ForceActive , 6 => Oc1m :: PwmMode1 , 7 => Oc1m :: PwmMode2 , _ => unreachable ! () , } } # [doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"] # [inline (always)] pub fn is_frozen (& self) -> bool { * self == Oc1m :: Frozen } # [doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"] # [inline (always)] pub fn is_active_on_match (& self) -> bool { * self == Oc1m :: ActiveOnMatch } # [doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"] # [inline (always)] pub fn is_inactive_on_match (& self) -> bool { * self == Oc1m :: InactiveOnMatch } # [doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"] # [inline (always)] pub fn is_toggle (& self) -> bool { * self == Oc1m :: Toggle } # [doc = "OCyREF is forced low"] # [inline (always)] pub fn is_force_inactive (& self) -> bool { * self == Oc1m :: ForceInactive } # [doc = "OCyREF is forced high"] # [inline (always)] pub fn is_force_active (& self) -> bool { * self == Oc1m :: ForceActive } # [doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"] # [inline (always)] pub fn is_pwm_mode1 (& self) -> bool { * self == Oc1m :: PwmMode1 } # [doc = "Inversely to PwmMode1"] # [inline (always)] pub fn is_pwm_mode2 (& self) -> bool { * self == Oc1m :: PwmMode2 } } # [doc = "Field `OCM(1-1)` writer - Output compare %s mode"] pub type OcmW < 'a , REG > = crate :: FieldWriter < 'a , REG , 3 , Oc1m , crate :: Safe > ; impl < 'a , REG > OcmW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"] # [inline (always)] pub fn frozen (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1m :: Frozen) } # [doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"] # [inline (always)] pub fn active_on_match (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1m :: ActiveOnMatch) } # [doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"] # [inline (always)] pub fn inactive_on_match (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1m :: InactiveOnMatch) } # [doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"] # [inline (always)] pub fn toggle (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1m :: Toggle) } # [doc = "OCyREF is forced low"] # [inline (always)] pub fn force_inactive (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1m :: ForceInactive) } # [doc = "OCyREF is forced high"] # [inline (always)] pub fn force_active (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1m :: ForceActive) } # [doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"] # [inline (always)] pub fn pwm_mode1 (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1m :: PwmMode1) } # [doc = "Inversely to PwmMode1"] # [inline (always)] pub fn pwm_mode2 (self) -> & 'a mut crate :: W < REG > { self . variant (Oc1m :: PwmMode2) } } impl R { # [doc = "Capture/Compare (1-1) selection"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CC1S` field.</div>"] # [inline (always)] pub fn ccs (& self , n : u8) -> CcsR { # [allow (clippy :: no_effect)] [() ; 1] [n as usize] ; CcsR :: new (((self . bits >> (n * 0)) & 3) as u8) } # [doc = "Iterator for array of:"] # [doc = "Capture/Compare (1-1) selection"] # [inline (always)] pub fn ccs_iter (& self) -> impl Iterator < Item = CcsR > + '_ { (0 .. 1) . map (move | n | CcsR :: new (((self . bits >> (n * 0)) & 3) as u8)) } # [doc = "Bits 0:1 - Capture/Compare 1 selection"] # [inline (always)] pub fn cc1s (& self) -> CcsR { CcsR :: new ((self . bits & 3) as u8) } # [doc = "Output compare (1-1) fast enable"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OC1FE` field.</div>"] # [inline (always)] pub fn ocfe (& self , n : u8) -> OcfeR { # [allow (clippy :: no_effect)] [() ; 1] [n as usize] ; OcfeR :: new (((self . bits >> (n * 0 + 2)) & 1) != 0) } # [doc = "Iterator for array of:"] # [doc = "Output compare (1-1) fast enable"] # [inline (always)] pub fn ocfe_iter (& self) -> impl Iterator < Item = OcfeR > + '_ { (0 .. 1) . map (move | n | OcfeR :: new (((self . bits >> (n * 0 + 2)) & 1) != 0)) } # [doc = "Bit 2 - Output compare 1 fast enable"] # [inline (always)] pub fn oc1fe (& self) -> OcfeR { OcfeR :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Output compare (1-1) preload enable"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OC1PE` field.</div>"] # [inline (always)] pub fn ocpe (& self , n : u8) -> OcpeR { # [allow (clippy :: no_effect)] [() ; 1] [n as usize] ; OcpeR :: new (((self . bits >> (n * 0 + 3)) & 1) != 0) } # [doc = "Iterator for array of:"] # [doc = "Output compare (1-1) preload enable"] # [inline (always)] pub fn ocpe_iter (& self) -> impl Iterator < Item = OcpeR > + '_ { (0 .. 1) . map (move | n | OcpeR :: new (((self . bits >> (n * 0 + 3)) & 1) != 0)) } # [doc = "Bit 3 - Output compare 1 preload enable"] # [inline (always)] pub fn oc1pe (& self) -> OcpeR { OcpeR :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Output compare (1-1) mode"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OC1M` field.</div>"] # [inline (always)] pub fn ocm (& self , n : u8) -> OcmR { # [allow (clippy :: no_effect)] [() ; 1] [n as usize] ; OcmR :: new (((self . bits >> (n * 0 + 4)) & 7) as u8) } # [doc = "Iterator for array of:"] # [doc = "Output compare (1-1) mode"] # [inline (always)] pub fn ocm_iter (& self) -> impl Iterator < Item = OcmR > + '_ { (0 .. 1) . map (move | n | OcmR :: new (((self . bits >> (n * 0 + 4)) & 7) as u8)) } # [doc = "Bits 4:6 - Output compare 1 mode"] # [inline (always)] pub fn oc1m (& self) -> OcmR { OcmR :: new (((self . bits >> 4) & 7) as u8) } } impl W { # [doc = "Capture/Compare (1-1) selection"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CC1S` field.</div>"] # [inline (always)] pub fn ccs (& mut self , n : u8) -> CcsW < Ccmr1OutputSpec > { # [allow (clippy :: no_effect)] [() ; 1] [n as usize] ; CcsW :: new (self , n * 0) } # [doc = "Bits 0:1 - Capture/Compare 1 selection"] # [inline (always)] pub fn cc1s (& mut self) -> CcsW < Ccmr1OutputSpec > { CcsW :: new (self , 0) } # [doc = "Output compare (1-1) fast enable"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OC1FE` field.</div>"] # [inline (always)] pub fn ocfe (& mut self , n : u8) -> OcfeW < Ccmr1OutputSpec > { # [allow (clippy :: no_effect)] [() ; 1] [n as usize] ; OcfeW :: new (self , n * 0 + 2) } # [doc = "Bit 2 - Output compare 1 fast enable"] # [inline (always)] pub fn oc1fe (& mut self) -> OcfeW < Ccmr1OutputSpec > { OcfeW :: new (self , 2) } # [doc = "Output compare (1-1) preload enable"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OC1PE` field.</div>"] # [inline (always)] pub fn ocpe (& mut self , n : u8) -> OcpeW < Ccmr1OutputSpec > { # [allow (clippy :: no_effect)] [() ; 1] [n as usize] ; OcpeW :: new (self , n * 0 + 3) } # [doc = "Bit 3 - Output compare 1 preload enable"] # [inline (always)] pub fn oc1pe (& mut self) -> OcpeW < Ccmr1OutputSpec > { OcpeW :: new (self , 3) } # [doc = "Output compare (1-1) mode"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OC1M` field.</div>"] # [inline (always)] pub fn ocm (& mut self , n : u8) -> OcmW < Ccmr1OutputSpec > { # [allow (clippy :: no_effect)] [() ; 1] [n as usize] ; OcmW :: new (self , n * 0 + 4) } # [doc = "Bits 4:6 - Output compare 1 mode"] # [inline (always)] pub fn oc1m (& mut self) -> OcmW < Ccmr1OutputSpec > { OcmW :: new (self , 4) } } # [doc = "capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct Ccmr1OutputSpec ; impl crate :: RegisterSpec for Ccmr1OutputSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`ccmr1_output::R`](R) reader structure"] impl crate :: Readable for Ccmr1OutputSpec { } # [doc = "`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure"] impl crate :: Writable for Ccmr1OutputSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets CCMR1_Output to value 0"] impl crate :: Resettable for Ccmr1OutputSpec { }