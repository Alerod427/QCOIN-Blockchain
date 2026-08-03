use crate::{mock::*, Error, Event, Something};
use frame_support::{assert_noop, assert_ok};

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
		assert_noop!(Template::cause_error(RuntimeOrigin::signed(1)), Error::<Test>::NoneValue);
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
