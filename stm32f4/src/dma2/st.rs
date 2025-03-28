# [repr (C)] # [doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"] # [doc (alias = "ST")] pub struct St { cr : Cr , ndtr : Ndtr , par : Par , m0ar : M0ar , m1ar : M1ar , fcr : Fcr , } impl St { # [doc = "0x00 - stream x configuration register"] # [inline (always)] pub const fn cr (& self) -> & Cr { & self . cr } # [doc = "0x04 - stream x number of data register"] # [inline (always)] pub const fn ndtr (& self) -> & Ndtr { & self . ndtr } # [doc = "0x08 - stream x peripheral address register"] # [inline (always)] pub const fn par (& self) -> & Par { & self . par } # [doc = "0x0c - stream x memory 0 address register"] # [inline (always)] pub const fn m0ar (& self) -> & M0ar { & self . m0ar } # [doc = "0x10 - stream x memory 1 address register"] # [inline (always)] pub const fn m1ar (& self) -> & M1ar { & self . m1ar } # [doc = "0x14 - stream x FIFO control register"] # [inline (always)] pub const fn fcr (& self) -> & Fcr { & self . fcr } } # [doc = "CR (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"] # [doc (alias = "CR")] pub type Cr = crate :: Reg < cr :: CrSpec > ; # [doc = "stream x configuration register"] pub mod cr ; # [doc = "NDTR (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndtr`] module"] # [doc (alias = "NDTR")] pub type Ndtr = crate :: Reg < ndtr :: NdtrSpec > ; # [doc = "stream x number of data register"] pub mod ndtr ; # [doc = "PAR (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`par::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@par`] module"] # [doc (alias = "PAR")] pub type Par = crate :: Reg < par :: ParSpec > ; # [doc = "stream x peripheral address register"] pub mod par ; # [doc = "M0AR (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0ar`] module"] # [doc (alias = "M0AR")] pub type M0ar = crate :: Reg < m0ar :: M0arSpec > ; # [doc = "stream x memory 0 address register"] pub mod m0ar ; # [doc = "M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1ar`] module"] # [doc (alias = "M1AR")] pub type M1ar = crate :: Reg < m1ar :: M1arSpec > ; # [doc = "stream x memory 1 address register"] pub mod m1ar ; # [doc = "FCR (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"] # [doc (alias = "FCR")] pub type Fcr = crate :: Reg < fcr :: FcrSpec > ; # [doc = "stream x FIFO control register"] pub mod fcr ;