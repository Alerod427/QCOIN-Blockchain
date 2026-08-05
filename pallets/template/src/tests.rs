use crate::{mock::*, Event, Something, TotalBlockRewardsMinted, RewardWallets, Pallet, UNIT, MAX_SUPPLY_CAP};
use frame_support::{assert_noop, assert_ok, traits::Hooks};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(Template::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(Something::<Test>::get(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(Template::cause_error(RuntimeOrigin::signed(1)), crate::Error::<Test>::NoneValue);
	});
}

#[test]
fn pq_signature_verification_works() {
	use fips204::ml_dsa_65;
	use fips204::traits::{SerDes, Signer};

	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// 1. Generate post-quantum ML-DSA-65 keypair
		let (pk, sk) = ml_dsa_65::try_keygen().expect("keygen works");
		let pk_bytes = pk.into_bytes();

		// 2. Register public key on-chain
		let bounded_pk: frame_support::BoundedVec<u8, frame_support::traits::ConstU32<2500>> =
			pk_bytes.to_vec().try_into().unwrap();
		assert_ok!(Template::register_pq_public_key(RuntimeOrigin::signed(1), bounded_pk));

		// 3. Sign a message off-chain
		let message = b"Hello Post-Quantum World!";
		let sig = sk.try_sign(message, b"").expect("signing works");

		let bounded_msg: frame_support::BoundedVec<u8, frame_support::traits::ConstU32<1024>> =
			message.to_vec().try_into().unwrap();
		let bounded_sig: frame_support::BoundedVec<u8, frame_support::traits::ConstU32<4000>> =
			sig.to_vec().try_into().unwrap();

		// 4. Verify post-quantum signature on-chain
		assert_ok!(Template::verify_pq_signature(
			RuntimeOrigin::signed(1),
			bounded_msg,
			bounded_sig
		));

		// 5. Assert verified count incremented to 1
		assert_eq!(crate::VerifiedPqCount::<Test>::get(), 1);
	});
}

#[test]
fn test_on_initialize_mints_block_reward_to_author() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let initial_balance = Balances::free_balance(1);

		// Trigger on_initialize for block 1
		let _weight = Template::on_initialize(1);

		// Account 1 (DummyFindAuthor) should receive 10 QCOIN (10 * 10^12 Plancks)
		let reward = 10 * UNIT;
		assert_eq!(Balances::free_balance(1), initial_balance + reward);
		assert_eq!(TotalBlockRewardsMinted::<Test>::get(), reward);
	});
}

#[test]
fn test_halving_schedule_rewards() {
	new_test_ext().execute_with(|| {
		let total_minted = 0u128;

		// Era 1 (Blocks 1..=5,000,000): 10 QCOIN
		let (r1, era1) = Pallet::<Test>::calculate_block_reward(1, total_minted);
		assert_eq!(r1, 10 * UNIT);
		assert_eq!(era1, 1);

		let (r5m, era1_end) = Pallet::<Test>::calculate_block_reward(5_000_000, total_minted);
		assert_eq!(r5m, 10 * UNIT);
		assert_eq!(era1_end, 1);

		// Era 2 (Blocks 5,000,001..=10,000,000): 5 QCOIN (Halving 1)
		let (r2, era2) = Pallet::<Test>::calculate_block_reward(5_000_001, total_minted);
		assert_eq!(r2, 5 * UNIT);
		assert_eq!(era2, 2);

		// Era 3 (Blocks 10,000,001..=15,000,000): 2.5 QCOIN (Halving 2)
		let (r3, era3) = Pallet::<Test>::calculate_block_reward(10_000_001, total_minted);
		assert_eq!(r3, (25 * UNIT) / 10);
		assert_eq!(era3, 3);

		// Era 4 (Blocks 15,000,001..=20,000,000): 1.25 QCOIN (Halving 3)
		let (r4, era4) = Pallet::<Test>::calculate_block_reward(15_000_001, total_minted);
		assert_eq!(r4, (125 * UNIT) / 100);
		assert_eq!(era4, 4);
	});
}

#[test]
fn test_custom_reward_wallet() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let author = 1u64;
		let custom_wallet = 99u64;

		// Set custom reward wallet for validator 1
		assert_ok!(Template::set_reward_wallet(RuntimeOrigin::signed(author), custom_wallet));
		assert_eq!(RewardWallets::<Test>::get(author), Some(custom_wallet));

		let initial_custom_balance = Balances::free_balance(custom_wallet);

		// Trigger block 1 initialization
		let _weight = Template::on_initialize(1);

		// Reward should go to custom_wallet (99) instead of author (1)
		let expected_reward = 10 * UNIT;
		assert_eq!(Balances::free_balance(custom_wallet), initial_custom_balance + expected_reward);
	});
}

#[test]
fn test_max_supply_cap_enforcement() {
	new_test_ext().execute_with(|| {
		// When total minted reaches MAX_SUPPLY_CAP
		let max_cap = MAX_SUPPLY_CAP;
		TotalBlockRewardsMinted::<Test>::put(max_cap);

		// Reward calculation should return 0
		let (reward, era) = Pallet::<Test>::calculate_block_reward(100, max_cap);
		assert_eq!(reward, 0);
		assert_eq!(era, 0);

		// Near max cap boundary: 5 QCOIN remaining before cap
		let near_cap = max_cap - (5 * UNIT);
		TotalBlockRewardsMinted::<Test>::put(near_cap);

		let (partial_reward, era) = Pallet::<Test>::calculate_block_reward(1, near_cap);
		// Base reward for block 1 is 10 QCOIN, but capped at remaining 5 QCOIN
		assert_eq!(partial_reward, 5 * UNIT);
		assert_eq!(era, 1);
	});
}

