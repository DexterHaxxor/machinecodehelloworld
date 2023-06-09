// -----------------------------------------------------------------------------
// SPDX-License-Identifier: Apache-2.0
//
// Copyright 2023 DexterHaxxor <fox@dexterhaxxor.dev>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// -----------------------------------------------------------------------------

use std::{arch::asm, ffi::CStr, mem::transmute};

fn hello() {
    unsafe {
        asm!(
            "push 0x6F6C6C65",
            "add eax, 0x726F7720",
            "add eax, 0x0000646C",
            out("eax") _,
        );
    }
}

fn main() {
    unsafe {
        println!(
            "{}",
            CStr::from_ptr(transmute::<_, *const i8>(hello as fn()).offset(1))
                .to_str()
                .unwrap()
        );
    }
}
