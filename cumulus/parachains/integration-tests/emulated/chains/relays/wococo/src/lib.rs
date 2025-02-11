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

// Cumulus
use emulated_integration_tests_common::{
	impl_accounts_helpers_for_relay_chain, impl_assert_events_helpers_for_relay_chain,
	impl_hrmp_channels_helpers_for_relay_chain, impl_send_transact_helpers_for_relay_chain,
	xcm_emulator::decl_test_relay_chains,
};

// Wococo declaration
decl_test_relay_chains! {
	#[api_version(8)]
	pub struct Wococo {
		genesis = rococo_emulated_chain::genesis::genesis(),
		on_init = (),
		runtime = rococo_runtime,
		core = {
			SovereignAccountOf: rococo_runtime::xcm_config::LocationConverter,
		},
		pallets = {
			XcmPallet: rococo_runtime::XcmPallet,
			Sudo: rococo_runtime::Sudo,
			Balances: rococo_runtime::Balances,
			Hrmp: rococo_runtime::Hrmp,
		}
	},
}

// Wococo implementation
impl_accounts_helpers_for_relay_chain!(Wococo);
impl_assert_events_helpers_for_relay_chain!(Wococo);
impl_hrmp_channels_helpers_for_relay_chain!(Wococo);
impl_send_transact_helpers_for_relay_chain!(Wococo);
