// Copyright 2024 RISC Zero, Inc.
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

mod hal;
#[cfg(test)]
mod tests;
mod witgen;

use anyhow::Result;
use cfg_if::cfg_if;
use risc0_zkp::core::{digest::Digest, hash::poseidon2::Poseidon2HashSuite};

use crate::{execute::segment::Segment, zirgen::CircuitImpl};

const GLOBAL_MIX: usize = 0;
const GLOBAL_OUT: usize = 1;

pub type Seal = Vec<u32>;

pub trait SegmentProver {
    fn prove(&self, segment: &Segment) -> Result<Seal>;

    fn verify(&self, seal: &Seal) -> Result<()> {
        let hash_suite = Poseidon2HashSuite::new_suite();

        // We don't have a `code' buffer to verify.
        let check_code_fn = |_: u32, _: &Digest| Ok(());

        Ok(risc0_zkp::verify::verify(
            &CircuitImpl,
            &hash_suite,
            seal,
            check_code_fn,
        )?)
    }
}

pub fn segment_prover() -> Result<Box<dyn SegmentProver>> {
    cfg_if! {
        if #[cfg(feature = "cuda")] {
            self::hal::cuda::segment_prover()
        // } else if #[cfg(any(all(target_os = "macos", target_arch = "aarch64"), target_os = "ios"))] {
        // self::hal::metal::segment_prover(hashfn)
        } else {
            self::hal::cpu::segment_prover()
        }
    }
}