#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
pub use pallet::*;
pub mod weights;
pub use weights::*;

use frame_support::sp_runtime::RuntimeDebug;
use scale_info::TypeInfo;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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

//pub type AsteroidId = u64;
pub type AsteroidType = u64;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::{pallet_prelude::*, runtime_print};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
    }

    type UserAccount<T> = <T as frame_system::Config>::AccountId;

    #[pallet::storage]
    pub type Something<T> = StorageValue<_, u32>;

    // #[pallet::storage]
    //  pub type AsteroidIds<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    pub type Asteroids<T: Config> =
        StorageMap<_, Twox64Concat, Coord, (AsteroidType, BlockNumberFor<T>), OptionQuery>;

    #[pallet::storage]
    pub type Flights<T: Config> =
        StorageMap<_, Twox64Concat, UserAccount<T>, Flight<BlockNumberFor<T>>, OptionQuery>;

    #[pallet::storage]
    pub type ActiveShips<T: Config> =
        StorageMap<_, Twox64Concat, UserAccount<T>, Coord, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SomethingStored {
            something: u32,
            who: T::AccountId,
        },

        TestEvent {
            something: u32,
        },

        AsteroidSpawned {
            resource_id: AsteroidType,
            coord: Coord,
        },

        AsteroidRemoved {
            coord: Coord,
        },

        FlightStarted {
            owner: T::AccountId,
            from: Coord,
            to: Coord,
            end: BlockNumberFor<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(now: BlockNumberFor<T>) -> Weight {
            let mut weight = Weight::zero();

            for (as_id, (coord, ttl_block)) in Asteroids::<T>::iter() {
                if ttl_block < now {
                    Self::deposit_event(Event::AsteroidRemoved {
                        coord: as_id.clone(),
                    });
                    runtime_print!(
                        "[on_initialize] remove asteroid {:?} coord: {:?}",
                        as_id,
                        coord
                    );
                    Asteroids::<T>::remove(as_id);

                    weight += T::DbWeight::get().writes(1);
                }
            }

            let type_id: AsteroidType = 0; // Moq asteroid type
            let map_size: u32 = 50; // Moq map size

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
                        x: Self::get_random_x(map_size, i as u32),
                        y: Self::get_random_y(map_size, i as u32),
                    };

                    if Asteroids::<T>::contains_key(coord.clone()) {
                        runtime_print!("[on_init] Asteroid already exists at coord {:?}", coord);
                        continue;
                    }

                    let ttl_block = now + (ttl_const + i as u32).into();

                    Asteroids::<T>::insert(coord.clone(), (type_id, ttl_block));
                    runtime_print!(
                        "[on_init] Asteroid #{:?} spawned at coord {:?}",
                        type_id,
                        coord
                    );
                    Self::deposit_event(Event::AsteroidSpawned {
                        resource_id: type_id,
                        coord: coord.clone(),
                    });

                    weight += T::DbWeight::get().writes(1);
                }
            }

            weight += T::DbWeight::get().writes(1);

            for (user, flight) in Flights::<T>::iter() {
                if flight.end < now {
                    ActiveShips::<T>::insert(user.clone(), flight.to.clone());
                    Flights::<T>::remove(user.clone());

                    weight += T::DbWeight::get().writes(2);
                    runtime_print!("[on_init] Flight removed {:?}", user);
                }
            }

            weight
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
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

            Ok(())
        }

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
