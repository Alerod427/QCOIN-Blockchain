//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// Import post-quantum FIPS 204 primitives.
	use fips204::ml_dsa_65;
	use fips204::traits::{SerDes, Verifier};

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	/// A storage item for this pallet.
	#[pallet::storage]
	pub type Something<T> = StorageValue<_, u32>;

	/// Storage for registered Post-Quantum (ML-DSA-65) public keys.
	#[pallet::storage]
	pub type PqPublicKeys<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<u8, ConstU32<2500>>,
		OptionQuery,
	>;

	/// Counter for total post-quantum signatures verified on-chain.
	#[pallet::storage]
	pub type VerifiedPqCount<T> = StorageValue<_, u32, ValueQuery>;

	/// Events that functions in this pallet can emit.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user has successfully set a new value.
		SomethingStored {
			something: u32,
			who: T::AccountId,
		},
		/// A post-quantum ML-DSA-65 public key was registered for an account.
		PqPublicKeyRegistered {
			who: T::AccountId,
		},
		/// A post-quantum ML-DSA-65 signature was successfully verified on-chain.
		PqSignatureVerified {
			who: T::AccountId,
			verified_count: u32,
		},
	}

	/// Errors that can be returned by this pallet.
	#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,
		/// Invalid ML-DSA-65 public key length or format (expected 1952 bytes).
		InvalidPqPublicKey,
		/// Invalid ML-DSA-65 signature length or format (expected 3309 bytes).
		InvalidPqSignature,
		/// Post-Quantum ML-DSA-65 signature verification failed.
		PqVerificationFailed,
		/// No Post-Quantum public key found for the account.
		PqKeyNotFound,
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a single u32 value as a parameter, writes the value
		/// to storage and emits an event.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::<T>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });

			// Return a successful `DispatchResult`
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::<T>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage. This will cause an error in the event
					// of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::<T>::put(new);
					Ok(())
				},
			}
		}

		/// Register an ML-DSA-65 Post-Quantum Public Key for the caller account.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::register_pq_public_key())]
		pub fn register_pq_public_key(
			origin: OriginFor<T>,
			public_key: BoundedVec<u8, ConstU32<2500>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Validate length (ML-DSA-65 public key size is 1952 bytes)
			let pk_bytes: &[u8; ml_dsa_65::PK_LEN] = public_key
				.as_slice()
				.try_into()
				.map_err(|_| Error::<T>::InvalidPqPublicKey)?;

			// Validate public key structure
			let _pk = ml_dsa_65::PublicKey::try_from_bytes(*pk_bytes)
				.map_err(|_| Error::<T>::InvalidPqPublicKey)?;

			// Store public key in storage
			PqPublicKeys::<T>::insert(&who, public_key);

			// Emit event
			Self::deposit_event(Event::PqPublicKeyRegistered { who });

			Ok(())
		}

		/// Verify a Post-Quantum ML-DSA-65 signature on-chain against the stored public key of the caller.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::verify_pq_signature())]
		pub fn verify_pq_signature(
			origin: OriginFor<T>,
			message: BoundedVec<u8, ConstU32<1024>>,
			signature: BoundedVec<u8, ConstU32<4000>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Retrieve caller's registered post-quantum public key
			let stored_pk = PqPublicKeys::<T>::get(&who).ok_or(Error::<T>::PqKeyNotFound)?;

			let pk_bytes: &[u8; ml_dsa_65::PK_LEN] = stored_pk
				.as_slice()
				.try_into()
				.map_err(|_| Error::<T>::InvalidPqPublicKey)?;

			let pk = ml_dsa_65::PublicKey::try_from_bytes(*pk_bytes)
				.map_err(|_| Error::<T>::InvalidPqPublicKey)?;

			// Convert signature bytes (ML-DSA-65 signature size is 3309 bytes)
			let sig_bytes: &[u8; ml_dsa_65::SIG_LEN] = signature
				.as_slice()
				.try_into()
				.map_err(|_| Error::<T>::InvalidPqSignature)?;

			// Verify post-quantum signature on-chain (using empty domain separation context b"")
			ensure!(pk.verify(message.as_slice(), sig_bytes, b""), Error::<T>::PqVerificationFailed);

			// Increment total verified count
			let new_count = VerifiedPqCount::<T>::get()
				.checked_add(1)
				.ok_or(Error::<T>::StorageOverflow)?;
			VerifiedPqCount::<T>::put(new_count);

			Self::deposit_event(Event::PqSignatureVerified {
				who,
				verified_count: new_count,
			});

			Ok(())
		}
	}
}
