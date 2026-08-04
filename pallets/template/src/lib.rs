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
	use frame_support::traits::Currency;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;

	// Import post-quantum FIPS 204 primitives.
	use fips204::ml_dsa_65;
	use fips204::traits::{SerDes, Verifier};

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Automatically mints and distributes block rewards on every finalized block initialization.
		/// Rewards are credited to approved active validator reward wallets.
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			let block_num: u32 = TryInto::<u32>::try_into(n).unwrap_or(1);
			let (reward_amount, era) = Self::calculate_block_reward(block_num);

			// Collect all active approved validator accounts
			let approved: Vec<T::AccountId> = ApprovedValidators::<T>::iter()
				.filter_map(|(acc, is_approved)| if is_approved { Some(acc) } else { None })
				.collect();

			if !approved.is_empty() {
				// Round-robin validator selection based on block number
				let index = (block_num as usize) % approved.len();
				let validator = &approved[index];

				// Determine recipient: custom RewardWallet or validator's own account
				let recipient = RewardWallets::<T>::get(validator).unwrap_or_else(|| validator.clone());

				// Mint real QCOIN tokens into the recipient's wallet balance
				if let Ok(amount) = <T::Currency as Currency<T::AccountId>>::Balance::try_from(reward_amount) {
					let _imbalance = T::Currency::deposit_creating(&recipient, amount);

					// Update cumulative minted rewards
					let new_total = TotalBlockRewardsMinted::<T>::get().saturating_add(reward_amount);
					TotalBlockRewardsMinted::<T>::put(new_total);

					// Emit reward distribution event
					Self::deposit_event(Event::BlockRewardDistributed {
						block_number: block_num,
						reward_amount,
						era,
						recipient,
					});
				}
			}

			T::DbWeight::get().reads_writes(3, 2)
		}
	}

	/// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
		/// The currency mechanism for minting block rewards.
		type Currency: Currency<Self::AccountId>;
	}

	/// Storage item for this pallet.
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

	/// Storage for total cumulative block rewards minted by the network.
	#[pallet::storage]
	pub type TotalBlockRewardsMinted<T> = StorageValue<_, u128, ValueQuery>;

	/// Storage for approved network validator accounts.
	#[pallet::storage]
	pub type ApprovedValidators<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		bool,
		ValueQuery,
	>;

	/// Storage mapping each validator to their chosen reward wallet address.
	/// If not set, rewards go to the validator's own account.
	#[pallet::storage]
	pub type RewardWallets<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		T::AccountId,
		OptionQuery,
	>;

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
		/// A block mining reward was minted and distributed with Halving schedule.
		BlockRewardDistributed {
			block_number: u32,
			reward_amount: u128,
			era: u32,
			recipient: T::AccountId,
		},
		/// A new validator node was approved by Sudo master key.
		ValidatorApproved {
			who: T::AccountId,
			session_key: BoundedVec<u8, ConstU32<64>>,
		},
		/// A validator node authorization was revoked by Sudo master key.
		ValidatorRevoked {
			who: T::AccountId,
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
		/// Caller is not a registered/approved validator.
		NotApprovedValidator,
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

		/// Claim block reward with Halving Schedule for active validator node.
		/// Rewards are minted and deposited into the validator's chosen reward wallet.
		/// If no reward wallet is configured, rewards go to the caller's own account.
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::claim_block_reward())]
		pub fn claim_block_reward(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Verify the caller is an approved validator
			ensure!(ApprovedValidators::<T>::get(&who), Error::<T>::NotApprovedValidator);

			let current_block = <frame_system::Pallet<T>>::block_number();
			let block_num: u32 = TryInto::<u32>::try_into(current_block).unwrap_or(1);

			// Calculate block reward & current era
			let (reward_amount, era) = Self::calculate_block_reward(block_num);

			// Determine the reward recipient: custom wallet or validator's own account
			let recipient = RewardWallets::<T>::get(&who).unwrap_or(who.clone());

			// Mint the reward coins and deposit them into the recipient's account
			let amount = <T::Currency as Currency<T::AccountId>>::Balance::try_from(reward_amount)
				.map_err(|_| Error::<T>::StorageOverflow)?;
			let _imbalance = T::Currency::deposit_creating(&recipient, amount);

			// Update total rewards storage
			let new_total = TotalBlockRewardsMinted::<T>::get()
				.saturating_add(reward_amount);
			TotalBlockRewardsMinted::<T>::put(new_total);

			// Emit block reward distribution event
			Self::deposit_event(Event::BlockRewardDistributed {
				block_number: block_num,
				reward_amount,
				era,
				recipient,
			});

			Ok(())
		}

		/// Approve a new validator node by Sudo Master Key.
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::add_validator())]
		pub fn add_validator(
			origin: OriginFor<T>,
			validator: T::AccountId,
			session_key: BoundedVec<u8, ConstU32<64>>,
		) -> DispatchResult {
			ensure_root(origin)?;

			ApprovedValidators::<T>::insert(&validator, true);

			Self::deposit_event(Event::ValidatorApproved {
				who: validator,
				session_key,
			});

			Ok(())
		}

		/// Revoke validator node approval by Sudo Master Key.
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::remove_validator())]
		pub fn remove_validator(
			origin: OriginFor<T>,
			validator: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			ApprovedValidators::<T>::remove(&validator);

			Self::deposit_event(Event::ValidatorRevoked {
				who: validator,
			});

			Ok(())
		}

		/// Register self as an approved validator node by providing the node's local Session Key.
		/// The caller's signed AccountId (`who`) is mapped as the reward recipient wallet for this validator node.
		/// Optionally specify a different reward_wallet to receive block mining rewards.
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::register_validator())]
		pub fn register_validator(
			origin: OriginFor<T>,
			session_key: BoundedVec<u8, ConstU32<64>>,
			reward_wallet: Option<T::AccountId>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ApprovedValidators::<T>::insert(&who, true);

			// If the user specified a custom reward wallet, store it
			if let Some(ref wallet) = reward_wallet {
				RewardWallets::<T>::insert(&who, wallet);
			}

			Self::deposit_event(Event::ValidatorApproved {
				who: who.clone(),
				session_key,
			});

			Ok(())
		}

		/// Change the reward wallet address for an already-registered validator.
		/// The caller must be an approved validator. Rewards from future claims will go to the new wallet.
		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::register_validator())]
		pub fn set_reward_wallet(
			origin: OriginFor<T>,
			new_wallet: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Only approved validators can change their reward wallet
			ensure!(ApprovedValidators::<T>::get(&who), Error::<T>::NotApprovedValidator);

			RewardWallets::<T>::insert(&who, &new_wallet);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Calculates the block reward and era based on current block height and Halving schedule.
		///
		/// - Era 1 (Blocks 1 - 5,000,000): 10 QCOIN (10,000,000,000,000 Plancks)
		/// - Era 2 (Blocks 5,000,001 - 10,000,000): 5 QCOIN (5,000,000,000,000 Plancks) [Halving 1]
		/// - Era 3 (Blocks 10,000,001 - 15,000,000): 2.5 QCOIN (2,500,000,000,000 Plancks) [Halving 2]
		/// - Era 4+ (Blocks 15,000,001+): 1.25 QCOIN (1,250,000,000,000 Plancks) [Halving 3]
		pub fn calculate_block_reward(block_number: u32) -> (u128, u32) {
			const UNIT: u128 = 1_000_000_000_000;
			if block_number <= 5_000_000 {
				(10 * UNIT, 1)
			} else if block_number <= 10_000_000 {
				(5 * UNIT, 2)
			} else if block_number <= 15_000_000 {
				(2_500_000_000_000, 3)
			} else {
				(1_250_000_000_000, 4)
			}
		}
	}
}
