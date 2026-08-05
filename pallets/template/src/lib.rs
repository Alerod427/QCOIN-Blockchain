//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::{Currency, FindAuthor};
	use frame_system::pallet_prelude::*;

	// Import post-quantum FIPS 204 primitives.
	use fips204::ml_dsa_65;
	use fips204::traits::{SerDes, Verifier};

	/// 1 QCOIN in base indivisible units (Plancks). 1 QCOIN = 1_000_000_000_000 Plancks.
	pub const UNIT: u128 = 1_000_000_000_000;

	/// Maximum Total Supply Cap for QCOIN block rewards: 100,000,000 QCOIN.
	pub const MAX_SUPPLY_CAP: u128 = 100_000_000 * UNIT;

	/// Block interval per Halving Era (5,000,000 blocks ~ approx 1 year at 6s block time).
	pub const HALVING_INTERVAL: u32 = 5_000_000;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Automatically mints and distributes block rewards on every finalized block initialization.
		/// Rewards are credited ONLY to the validator node that ACTUALLY mined and signed the block.
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			let block_num: u32 = TryInto::<u32>::try_into(n).unwrap_or(1);
			let total_minted = TotalBlockRewardsMinted::<T>::get();
			let (reward_amount, era) = Self::calculate_block_reward(block_num, total_minted);

			if reward_amount > 0 {
				// Extract digest logs from current block header to find block author
				let digest = <frame_system::Pallet<T>>::digest();
				let pre_digests = digest.logs().iter().filter_map(|d| d.as_pre_runtime());

				// Find the actual author account who signed/produced this block
				if let Some(author) = T::FindAuthor::find_author(pre_digests) {
					// Determine recipient: custom RewardWallet or validator's own account
					let recipient = RewardWallets::<T>::get(&author).unwrap_or_else(|| author.clone());

					// Mint real QCOIN tokens into the recipient's wallet balance
					if let Ok(amount) = <T::Currency as Currency<T::AccountId>>::Balance::try_from(reward_amount) {
						let _imbalance = T::Currency::deposit_creating(&recipient, amount);

						// Update cumulative minted rewards
						let new_total = total_minted.saturating_add(reward_amount);
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
		/// Mechanism to find the actual author who signed and produced the block.
		type FindAuthor: frame_support::traits::FindAuthor<Self::AccountId>;
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
			let who = ensure_signed(origin)?;
			Something::<T>::put(something);
			Self::deposit_event(Event::SomethingStored { something, who });
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			match Something::<T>::get() {
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
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

			let pk_bytes: &[u8; ml_dsa_65::PK_LEN] = public_key
				.as_slice()
				.try_into()
				.map_err(|_| Error::<T>::InvalidPqPublicKey)?;

			let _pk = ml_dsa_65::PublicKey::try_from_bytes(*pk_bytes)
				.map_err(|_| Error::<T>::InvalidPqPublicKey)?;

			PqPublicKeys::<T>::insert(&who, public_key);
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

			let stored_pk = PqPublicKeys::<T>::get(&who).ok_or(Error::<T>::PqKeyNotFound)?;

			let pk_bytes: &[u8; ml_dsa_65::PK_LEN] = stored_pk
				.as_slice()
				.try_into()
				.map_err(|_| Error::<T>::InvalidPqPublicKey)?;

			let pk = ml_dsa_65::PublicKey::try_from_bytes(*pk_bytes)
				.map_err(|_| Error::<T>::InvalidPqPublicKey)?;

			let sig_bytes: &[u8; ml_dsa_65::SIG_LEN] = signature
				.as_slice()
				.try_into()
				.map_err(|_| Error::<T>::InvalidPqSignature)?;

			ensure!(pk.verify(message.as_slice(), sig_bytes, b""), Error::<T>::PqVerificationFailed);

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

		/// Approve a new validator node by Sudo Master Key.
		#[pallet::call_index(4)]
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
		#[pallet::call_index(5)]
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
		/// Optionally specify a custom reward_wallet to receive block mining rewards.
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::register_validator())]
		pub fn register_validator(
			origin: OriginFor<T>,
			session_key: BoundedVec<u8, ConstU32<64>>,
			reward_wallet: Option<T::AccountId>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ApprovedValidators::<T>::insert(&who, true);

			if let Some(ref wallet) = reward_wallet {
				RewardWallets::<T>::insert(&who, wallet);
			}

			Self::deposit_event(Event::ValidatorApproved {
				who: who.clone(),
				session_key,
			});

			Ok(())
		}

		/// Change the reward wallet address for a validator node.
		/// Rewards from future mined blocks will go to the new wallet.
		#[pallet::call_index(7)]
		#[pallet::weight((T::WeightInfo::register_validator(), DispatchClass::Normal, Pays::No))]
		pub fn set_reward_wallet(
			origin: OriginFor<T>,
			new_wallet: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			RewardWallets::<T>::insert(&who, &new_wallet);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Calculates the block reward and era based on current block height, total minted rewards,
		/// and Bitcoin-style Halving schedule.
		///
		/// - Era 1 (Blocks 1 - 5,000,000): 10 QCOIN (10,000,000,000,000 Plancks)
		/// - Era 2 (Blocks 5,000,001 - 10,000,000): 5 QCOIN (5,000,000,000,000 Plancks) [Halving 1]
		/// - Era 3 (Blocks 10,000,001 - 15,000,000): 2.5 QCOIN (2,500,000,000,000 Plancks) [Halving 2]
		/// - Era 4 (Blocks 15,000,001 - 20,000,000): 1.25 QCOIN (1,250,000,000,000 Plancks) [Halving 3]
		/// - Era N: Halves reward every 5,000,000 blocks until max supply cap (100,000,000 QCOIN).
		pub fn calculate_block_reward(block_number: u32, total_minted: u128) -> (u128, u32) {
			if total_minted >= MAX_SUPPLY_CAP || block_number == 0 {
				return (0, 0);
			}

			// Determine Era (1-indexed)
			let era = ((block_number.saturating_sub(1)) / HALVING_INTERVAL) + 1;

			let initial_reward: u128 = 10 * UNIT;
			let shift = era.saturating_sub(1);

			let base_reward = if shift >= 64 {
				0
			} else {
				initial_reward >> shift
			};

			if base_reward == 0 {
				return (0, era);
			}

			// Enforce Maximum Total Supply Cap
			let max_remaining = MAX_SUPPLY_CAP.saturating_sub(total_minted);
			let reward_amount = base_reward.min(max_remaining);

			(reward_amount, era)
		}
	}
}

