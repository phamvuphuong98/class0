#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;
	use scale_info::TypeInfo;

	#[derive(Encode, Decode)]
	#[derive(TypeInfo)]
    pub struct User {
		pub name: Vec<u8>,
		pub age: u8
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
	pub struct Pallet<T>(_);
		// The pallet's runtime storage items.
		// https://docs.substrate.io/v3/runtime/storage

		// Learn more about declaring storage items:
		// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	#[pallet::storage]
	pub(super) type Something<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId,Option<User>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		MemberRegistered(Vec<u8>, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		MemberAlreadyRegister,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn register(origin: OriginFor<T>, name: Vec<u8>, age: u8) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			// Verify that the specified proof has not already been claimed.
            ensure!(!Something::<T>::contains_key(&who), Error::<T>::MemberAlreadyRegister);

            // Get the block number from the FRAME System pallet.
          
			let name_clone = name.clone();
			let user_new = User {
				name: name,
				age: age
			};
            // Store the proof with the sender and block number.
            Something::<T>::insert( &who,Some(user_new));

			// Emit an event.
			Self::deposit_event(Event::MemberRegistered(name_clone, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		
	}
}
