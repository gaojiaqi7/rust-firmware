// Copyright © 2019 Intel Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

mod enum_builder;
pub mod r_efi_wrapper;

pub mod boot_mode;
pub mod fv;
pub mod hob;

pub mod pi {
    pub use crate::boot_mode;
    pub use crate::fv;
    pub use crate::hob;
    pub use crate::r_efi_wrapper::Guid;
}
