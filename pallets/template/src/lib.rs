#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
pub use pallet::*;
pub mod weights;
pub use weights::*;

pub mod utils;
use crate::utils::*;

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

pub type AsteroidType = u64;
pub type Energy = u32;

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
        StorageMap<_, Twox64Concat, UserAccount<T>, (Coord, Energy), OptionQuery>;

    #[pallet::storage]
    pub type AccountResources<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        UserAccount<T>,
        Twox64Concat,
        AsteroidType,
        u64, // count
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
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
        EnergyDepleted {
            owner: T::AccountId,
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

            for (user, flight) in Flights::<T>::iter() {
                if flight.end < now {
                    // Cherck asteroid coord
                    Self::collect_asteroid::<T>(user.clone(), flight.to.clone());

                    // I'm not sure about 100 energy, it should be a constant or a config value
                    let energy = ActiveShips::<T>::get(&user).map(|(_, e)| e).unwrap_or(0);
                    ActiveShips::<T>::insert(user.clone(), (flight.to.clone(), energy));
                    Flights::<T>::remove(user.clone());

                    weight += T::DbWeight::get().writes(2);
                    runtime_print!("[on_init] Flight removed {:?}", user);
                }
            }

            for (coord, (as_id, ttl_block)) in Asteroids::<T>::iter() {
                if ttl_block < now {
                    Self::remove_asteroid::<T>(coord.clone());

                    weight += T::DbWeight::get().writes(1);
                }
            }

            let type_id: AsteroidType = 0; // Moq asteroid type
            let map_size: u32 = 50; // Moq map size

            // Let’s treat it as a constant for now
            // until it becomes a real constant after refactoring
            let max_asteroids_count: usize = 10;

            let asteroids_count = Asteroids::<T>::iter().count();

            let difference = max_asteroids_count.saturating_sub(asteroids_count);

            // One more constant I need to remove from here
            let ttl_const = 5;
            if difference > 0 {
                for i in 0..difference {
                    let coord: Coord = Coord {
                        x: get_random_x::<T>(map_size, i as u32),
                        y: get_random_y::<T>(map_size, i as u32),
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

            let energy_depletion_rate: u32 = 2; // Moq energy depletion rate
                                                // Deplete energy of active ships
            for (owner, (coord, energy)) in ActiveShips::<T>::iter() {
                let new_energy = energy.saturating_sub(energy_depletion_rate);

                if new_energy == 0 {
                    runtime_print!(
                        "[on_init] Ship has no energy and is deactivated: {:?}",
                        owner
                    );
                    ActiveShips::<T>::remove(owner.clone());

                    Self::deposit_event(Event::EnergyDepleted {
                        owner: owner.clone(),
                    });
                    continue;
                }

                ActiveShips::<T>::insert(owner.clone(), (coord.clone(), new_energy));

                weight += T::DbWeight::get().writes(1);
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
                ActiveShips::<T>::insert(who.clone(), (from_coord.clone(), 100));
                runtime_print!("[on_init] Active ship added {:?}", who);
                // return Err(Error::<T>::NoneValue.into());
            } else {
                let ship_coord = ActiveShips::<T>::get(who.clone()).unwrap();
                from_coord = ship_coord.0.clone();
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

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn try_to_collect_resource(origin: OriginFor<T>, coord: Coord) -> DispatchResult {
            // Ensure the call is signed and extract the caller's account
            let who = ensure_signed(origin)?;

            // The player cannot collect resources while their ship is in flight
            if Flights::<T>::contains_key(&who) {
                runtime_print!("[try_to_collect_resource] Ship is still in flight");
                return Err(Error::<T>::NoneValue.into());
            }

            // The player must have an active ship on the map
            let ship_coord = ActiveShips::<T>::get(&who).ok_or(Error::<T>::NoneValue)?;

            // Calculate the Manhattan distance between the ship and the asteroid
            let distance = get_distance(ship_coord.0.clone(), coord.clone());

            // Will replace with a constant later
            let distance_limit = 2; // Max allowed distance to collect a resource

            if distance > distance_limit {
                runtime_print!(
            "[try_to_collect_resource] Too far to collect resource at coord {:?}, distance: {}",
            coord, distance
        );
                return Err(Error::<T>::NoneValue.into());
            }

            // Collect the asteroid (adds resource and removes asteroid)
            Self::collect_asteroid::<T>(who.clone(), coord.clone());
            runtime_print!(
                "[try_to_collect_resource] Successfully collected resource at coord {:?}",
                coord
            );
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn collect_asteroid<Runtime: Config>(user: UserAccount<T>, coord: Coord) {
            match Asteroids::<Runtime>::get(coord.clone()) {
                None => {
                    runtime_print!("[TakeAsteroid] No asteroid found at coord {:?}", coord);
                    return;
                }
                Some(asteroid) => {
                    Self::add_resource_to_account(&user, asteroid.0, 1);

                    Self::remove_asteroid::<T>(coord.clone());
                    runtime_print!("[TakeAsteroid] Asteroid taken at coord {:?}", coord);
                }
            }
        }

        fn remove_asteroid<Runtime: Config>(coord: Coord) {
            Self::deposit_event(Event::AsteroidRemoved {
                coord: coord.clone(),
            });
            runtime_print!("[on_initialize] remove coord: {:?}", coord);
            Asteroids::<T>::remove(coord);
        }

        fn add_resource_to_account(
            user: &UserAccount<T>,
            resource_type: AsteroidType,
            amount: u64,
        ) {
            AccountResources::<T>::mutate(user, resource_type, |count| {
                *count = count.saturating_add(amount);
                runtime_print!(
                "[add_resource_to_account] Added {} of resource {:?} to user {:?}. Total now: {}",
                amount, resource_type, user, *count
            );
            });
        }
    }
}
