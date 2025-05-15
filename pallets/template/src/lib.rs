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

use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
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

use frame_support::sp_runtime::RuntimeDebug;
use scale_info::TypeInfo;

#[derive(
    Encode,
    Decode,
    DecodeWithMemTracking,
    MaxEncodedLen,
    Clone,
    PartialEq,
    Eq,
    RuntimeDebug,
    TypeInfo,
)]
pub struct Coord {
    x: u32,
    y: u32,
}

#[derive(
    Encode,
    Decode,
    DecodeWithMemTracking,
    MaxEncodedLen,
    Clone,
    PartialEq,
    Eq,
    RuntimeDebug,
    TypeInfo,
)]
pub struct Flight<BlockNumber> {
    pub from: Coord,
    pub to: Coord,
    pub start: BlockNumber,
    pub end: BlockNumber,
}

pub type AsteroidId = u64;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    use core::{hash, result};

    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::{pallet_prelude::*, runtime_print};
    use frame_system::pallet_prelude::*;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    ///
    /// All our types and constants a pallet depends on must be declared here.
    /// These types are defined generically and made concrete when the pallet is declared in the
    /// `runtime/src/lib.rs` file of your chain.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
    }

    type UserAccount<T> = <T as frame_system::Config>::AccountId;

    #[pallet::storage]
    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::storage]
    pub type AsteroidIds<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    pub type Asteroids<T: Config> =
        StorageMap<_, Twox64Concat, AsteroidId, (Coord, BlockNumberFor<T>), OptionQuery>;

    #[pallet::storage]
    pub type Flights<T: Config> =
        StorageMap<_, Twox64Concat, UserAccount<T>, Flight<BlockNumberFor<T>>, OptionQuery>;

    #[pallet::storage]
    pub type ActiveShips<T: Config> =
        StorageMap<_, Twox64Concat, UserAccount<T>, Coord, OptionQuery>;

    ///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
    /// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
    /// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A user has successfully set a new value.
        SomethingStored {
            /// The new value set.
            something: u32,
            /// The account who set the new value.
            who: T::AccountId,
        },

        TestEvent {
            /// The new value set.
            something: u32,
        },

        AsteroidSpawned {
            resource_id: AsteroidId,
            coord: Coord,
        },

        AsteroidRemoved {
            id: AsteroidId,
        },

        FlightStarted {
            owner: T::AccountId,
            from: Coord,
            to: Coord,
            end: BlockNumberFor<T>,
        },
    }

    /// Errors that can be returned by this pallet.
    ///
    /// Errors tell users that something went wrong so it's important that their naming is
    /// informative. Similar to events, error documentation is added to a node's metadata so it's
    /// equally important that they have helpful documentation associated with them.
    ///
    /// This type of runtime error can be up to 4 bytes in size should you want to return additional
    /// information.
    #[pallet::error]
    pub enum Error<T> {
        /// The value retrieved was `None` as no value was previously set.
        NoneValue,
        /// There was an attempt to increment the value in storage over `u32::MAX`.
        StorageOverflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(now: BlockNumberFor<T>) -> Weight {
            let mut weight = Weight::zero();

            for (as_id, (coord, ttl_block)) in Asteroids::<T>::iter() {
                if ttl_block < now {
                    Asteroids::<T>::remove(as_id);

                    weight += T::DbWeight::get().writes(1);
                    Self::deposit_event(Event::AsteroidRemoved { id: as_id });
                    runtime_print!(
                        "[on_initialize] remove asteroid {:?} coord: {:?}",
                        as_id,
                        coord
                    );
                }
            }

            let mut id = AsteroidIds::<T>::get();
            // Let’s treat it as a constant for now
            // until it becomes a real constant after refactoring
            let max_asteroids_count = 10;

            let asteroids_count = Asteroids::<T>::iter().count();

            let difference = max_asteroids_count - asteroids_count;

            // One more constant I need to remove from here
            let ttl_const = 5;
            if difference > 0 {
                for i in 0..difference {
                    let coord: Coord = Coord {
                        x: Self::get_random_x(50, i as u32),
                        y: Self::get_random_y(50, i as u32),
                    };

                    let ttl_block = now + (ttl_const + i as u32).into();

                    Asteroids::<T>::insert(id, (coord.clone(), ttl_block));
                    runtime_print!("[on_init] Asteroid #{:?} spawned at coord {:?}", id, coord);
                    Self::deposit_event(Event::AsteroidSpawned {
                        resource_id: id,
                        coord: coord.clone(),
                    });

                    id += 1;
                    weight += T::DbWeight::get().writes(1);
                }
            }
            AsteroidIds::<T>::put(id);
            weight += T::DbWeight::get().writes(1);

            for (user, flight) in Flights::<T>::iter() {
                if flight.end < now {
                    ActiveShips::<T>::insert(user.clone(), flight.to.clone());
                    Flights::<T>::remove(user.clone());

                    weight += T::DbWeight::get().writes(2);
                    runtime_print!("[on_init] Flight removed {:?}", user);
                }
            }

            // Self::deposit_event(Event::TestEvent {
            //     something: id as u32,
            // });

            //  weight += T::DbWeight::get().reads_writes(1, 2);

            weight
        }
    }
    /// The pallet's dispatchable functions ([`Call`]s).
    ///
    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// They must always return a `DispatchResult` and be annotated with a weight and call index.
    ///
    /// The [`call_index`] macro is used to explicitly
    /// define an index for calls in the [`Call`] enum. This is useful for pallets that may
    /// introduce new dispatchables over time. If the order of a dispatchable changes, its index
    /// will also change which will break backwards compatibility.
    ///
    /// The [`weight`] macro is used to assign a weight to each call.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a single u32 value as a parameter, writes the value
        /// to storage and emits an event.
        ///
        /// It checks that the _origin_ for this call is _Signed_ and returns a dispatch
        /// error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn do_something(origin: OriginFor<T>, coord: Coord) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            if Flights::<T>::contains_key(who.clone()) {
                return Err(Error::<T>::NoneValue.into());
            }

            let mut from_coord = Coord { x: 0, y: 0 };

            if !ActiveShips::<T>::contains_key(who.clone()) {
                ActiveShips::<T>::insert(who.clone(), from_coord.clone());
                runtime_print!("[on_init] Active ship added {:?}", who);
                // return Err(Error::<T>::NoneValue.into());
            } else {
                let ship_coord = ActiveShips::<T>::get(who.clone()).unwrap();
                from_coord = ship_coord;
            }

            let block_number = <frame_system::Pallet<T>>::block_number();
            let end_block = block_number + 2u32.into();
            Flights::<T>::insert(
                who.clone(),
                Flight {
                    from: from_coord.clone(),
                    to: coord.clone(),
                    start: block_number.clone(),

                    end: end_block.clone(),
                },
            );
            runtime_print!("[on_init] Flight added {:?}", who);

            Self::deposit_event(Event::FlightStarted {
                owner: who.clone(),
                from: from_coord.clone(),
                to: coord.clone(),
                end: end_block,
            });
            // Self::deposit_event(Event::SomethingStored {
            //     something,
            //     who: who.clone(),
            // });

            // Return a successful `DispatchResult`
            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        ///
        /// It checks that the caller is a signed origin and reads the current value from the
        /// `Something` storage item. If a current value exists, it is incremented by 1 and then
        /// written back to storage.
        ///
        /// ## Errors
        ///
        /// The function will return an error under the following conditions:
        ///
        /// - If no value has been set ([`Error::NoneValue`])
        /// - If incrementing the value in storage causes an arithmetic overflow
        ///   ([`Error::StorageOverflow`])
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
                }
            }
        }
    }

    impl<T: Config> Pallet<T> {
        // Don’t do that. it’s a bad practice
        // I’m just gonna harry up
        fn get_hash_u32() -> u32 {
            let hash = <frame_system::Pallet<T>>::parent_hash();
            let bytes = hash.as_ref();
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        }

        fn get_random(seed: u32, skip: u32, max: u32) -> u32 {
            let mut local_skip = skip;
            if skip == 0 {
                local_skip = 1;
            }

            let new_seed = seed / local_skip;
            new_seed % max
        }

        pub fn get_random_x(max: u32, index: u32) -> u32 {
            let hash = Self::get_hash_u32();
            let result = Self::get_random(hash, index, max);
            runtime_print!("[on_init] x:{:?}", result);

            result
        }

        pub fn get_random_y(max: u32, index: u32) -> u32 {
            let hash = Self::get_hash_u32();
            let result = Self::get_random(hash, 100 + index, max);
            runtime_print!("[on_init] y:{:?}", result);

            result
        }
    }
}
