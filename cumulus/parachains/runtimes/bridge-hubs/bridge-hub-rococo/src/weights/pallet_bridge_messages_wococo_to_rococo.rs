// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_bridge_messages`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-vmdtonbz-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("bridge-hub-rococo-dev")`, DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=pallet_bridge_messages
// --chain=bridge-hub-rococo-dev
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_messages`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_messages::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgeWococoToRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWococoToRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeRococoParachain::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachain::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWococoToRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWococoToRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `537`
		//  Estimated: `52645`
		// Minimum execution time: 42_086_000 picoseconds.
		Weight::from_parts(42_833_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWococoToRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWococoToRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeRococoParachain::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachain::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWococoToRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWococoToRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_two_messages_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `537`
		//  Estimated: `52645`
		// Minimum execution time: 51_927_000 picoseconds.
		Weight::from_parts(53_847_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWococoToRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWococoToRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeRococoParachain::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachain::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWococoToRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWococoToRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `537`
		//  Estimated: `52645`
		// Minimum execution time: 47_218_000 picoseconds.
		Weight::from_parts(48_380_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWococoToRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWococoToRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeRococoParachain::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachain::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWococoToRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWococoToRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_1_kb() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `505`
		//  Estimated: `52645`
		// Minimum execution time: 40_585_000 picoseconds.
		Weight::from_parts(41_714_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWococoToRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWococoToRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeRococoParachain::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachain::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWococoToRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWococoToRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_16_kb() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `505`
		//  Estimated: `52645`
		// Minimum execution time: 71_197_000 picoseconds.
		Weight::from_parts(73_983_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWococoToRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWococoToRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachain::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachain::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWococoToRococoMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWococoToRococoMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `3841`
		// Minimum execution time: 30_823_000 picoseconds.
		Weight::from_parts(31_501_000, 0)
			.saturating_add(Weight::from_parts(0, 3841))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeWococoToRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWococoToRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachain::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachain::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWococoToRococoMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWococoToRococoMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `3841`
		// Minimum execution time: 30_854_000 picoseconds.
		Weight::from_parts(31_663_000, 0)
			.saturating_add(Weight::from_parts(0, 3841))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeWococoToRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWococoToRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachain::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachain::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWococoToRococoMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWococoToRococoMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:2 w:2)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `6086`
		// Minimum execution time: 33_463_000 picoseconds.
		Weight::from_parts(34_290_000, 0)
			.saturating_add(Weight::from_parts(0, 6086))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgeWococoToRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWococoToRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeRococoParachain::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachain::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWococoToRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWococoToRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `XcmpQueue::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::RelevantMessagingState` (r:1 w:0)
	/// Proof: `ParachainSystem::RelevantMessagingState` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpMessages` (r:0 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpMessages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[128, 2048]`.
	/// The range of component `i` is `[128, 2048]`.
	/// The range of component `i` is `[128, 2048]`.
	/// The range of component `i` is `[128, 2048]`.
	fn receive_single_message_proof_with_dispatch(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `739`
		//  Estimated: `52645`
		// Minimum execution time: 61_523_000 picoseconds.
		Weight::from_parts(62_686_055, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			// Standard Error: 26
			.saturating_add(Weight::from_parts(6_563, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
