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

const DEFAULT_ENERGY: u32 = 100;
// Deplete energy of active ships
const ENERGY_DEPLETION_RATE: u32 = 2;
const MAX_ASTEROIDS_COUNT: u32 = 10;
const MAP_SIZE: u32 = 50;
const ASTEROID_TTL_CONST: u32 = 10;
const RESOURCE_DISTANCE_LIMIT: u32 = 2;
const DEFAULT_DOT_STAKE: u64 = 5;
/// Minimum number of blocks that must pass before another NFT asteroid can spawn
const NFT_SPAWN_COOLDOWN_BLOCKS: u32 = 10;
/// Maximum percentage of the DOT prize pool that can be emitted as DOT asteroids
const DOT_EMISSION_LIMIT_RATIO: u64 = 10; // 10%

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

#[derive(
    Encode,
    Decode,
    Clone,
    Copy,
    PartialEq,
    Eq,
    RuntimeDebug,
    TypeInfo,
    MaxEncodedLen,
    DecodeWithMemTracking,
)]
pub enum AsteroidKind {
    Energy = 0,
    Gold = 1,
    Dot0 = 2,
    Dot1 = 3,
    Dot2 = 4,
    Nft0 = 5,
    Nft1 = 6,
    Nft2 = 7,
}

pub type AsteroidType = AsteroidKind;
pub type Energy = u32;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::{pallet_prelude::*, runtime_print, storage::child::get};
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

    #[pallet::storage]
    pub type DotPrizePool<T> = StorageValue<_, u64, ValueQuery>; // Total amount of DOT deposited into the prize pool

    #[pallet::storage]
    pub type DotEmittedTotal<T> = StorageValue<_, u64, ValueQuery>; // Total amount of DOT already emitted through asteroids

    #[pallet::storage]
    pub type LastNftSpawnBlock<T> = StorageValue<_, BlockNumberFor<T>, ValueQuery>; // The block number when the last NFT asteroid was spawned

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

        GameStarted {
            owner: T::AccountId,
            coord: Coord,
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
                if flight.end >= now {
                    continue;
                }

                let coord = flight.to.clone();

                Self::collect_asteroid::<T>(user.clone(), coord.clone());

                let energy = ActiveShips::<T>::get(&user).map(|(_, e)| e).unwrap_or(0);

                ActiveShips::<T>::insert(user.clone(), (coord, energy));
                Flights::<T>::remove(&user);

                weight += T::DbWeight::get().writes(2);
                runtime_print!("[on_init] Flight removed {:?}", user);
            }

            for (coord, (as_id, ttl_block)) in Asteroids::<T>::iter() {
                if ttl_block < now {
                    Self::remove_asteroid::<T>(as_id, coord.clone());

                    weight += T::DbWeight::get().writes(1);
                }
            }

            let map_size: u32 = MAP_SIZE; // Moq map size

            let asteroids_count = Asteroids::<T>::iter().count();

            let difference =
                MAX_ASTEROIDS_COUNT.saturating_sub(asteroids_count.try_into().unwrap_or(0));

            // One more constant I need to remove from here
            let ttl_const = ASTEROID_TTL_CONST;
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

                    let asteroid_type = Self::get_random_asteroid_type::<T>(i, now);

                    if matches!(
                        asteroid_type,
                        AsteroidKind::Dot0 | AsteroidKind::Dot1 | AsteroidKind::Dot2
                    ) {
                        let dot_amount = Self::get_dot_amount::<T>(asteroid_type);

                        // Add the DOT to the emitted total
                        DotEmittedTotal::<T>::mutate(|total| {
                            *total = total.saturating_add(dot_amount);
                        });
                    } else if matches!(
                        asteroid_type,
                        AsteroidKind::Nft0 | AsteroidKind::Nft1 | AsteroidKind::Nft2
                    ) {
                        LastNftSpawnBlock::<T>::put(now);
                    }

                    let ttl_block = now + (ttl_const + i as u32).into();

                    Asteroids::<T>::insert(coord.clone(), (asteroid_type, ttl_block));
                    runtime_print!(
                        "[on_init] Asteroid #{:?} spawned at coord {:?}",
                        asteroid_type,
                        coord
                    );
                    Self::deposit_event(Event::AsteroidSpawned {
                        resource_id: asteroid_type,
                        coord: coord.clone(),
                    });

                    weight += T::DbWeight::get().writes(1);
                }
            }

            weight += T::DbWeight::get().writes(1);

            for (owner, (coord, energy)) in ActiveShips::<T>::iter() {
                let new_energy = energy.saturating_sub(ENERGY_DEPLETION_RATE);

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

            if !ActiveShips::<T>::contains_key(who.clone()) {
                return Err(Error::<T>::NoneValue.into());
            }

            let ship_coord = ActiveShips::<T>::get(who.clone()).unwrap();
            let from_coord = ship_coord.0.clone();

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

            if distance > RESOURCE_DISTANCE_LIMIT {
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

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn start_game(origin: OriginFor<T>, coord: Coord) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            if ActiveShips::<T>::contains_key(who.clone()) {
                runtime_print!("[start_game] Player already has an active ship: {:?}", who);
                return Err(Error::<T>::NoneValue.into());
            }

            ActiveShips::<T>::insert(who.clone(), (coord.clone(), DEFAULT_ENERGY));

            Self::deposit_event(Event::GameStarted {
                owner: who.clone(),
                coord: coord.clone(),
            });

            // Add the value to the total DOT prize pool
            DotPrizePool::<T>::mutate(|pool| {
                *pool = pool.saturating_add(DEFAULT_DOT_STAKE);
            });
            runtime_print!("[on_init] Active ship added {:?} coord: {:?}", who, coord);

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
                    if asteroid.0 == AsteroidKind::Energy {
                        // If the asteroid is Energy, we just add it to the user's energy
                        let energy = ActiveShips::<T>::get(&user).map(|(_, e)| e).unwrap_or(0);

                        let new_energy = energy.saturating_add(10); // Add 10 energy

                        ActiveShips::<T>::insert(user.clone(), (coord.clone(), new_energy));

                        runtime_print!(
                            "[TakeAsteroid] Energy collected for user {:?}, new energy: {}",
                            user,
                            new_energy
                        );
                    } else if matches!(
                        asteroid.0,
                        AsteroidKind::Nft0
                            | AsteroidKind::Nft1
                            | AsteroidKind::Nft2
                            | AsteroidKind::Gold
                    ) {
                        Self::add_resource_to_account(&user, asteroid.0, 1);
                    } else {
                        let mut amount: u64 = 1;
                        if matches!(
                            asteroid.0,
                            AsteroidKind::Dot0 | AsteroidKind::Dot1 | AsteroidKind::Dot2
                        ) {
                            amount = Self::get_dot_amount::<Runtime>(asteroid.0);
                        }

                        Self::add_resource_to_account(&user, asteroid.0, amount);
                    }

                    Self::remove_asteroid::<T>(asteroid.0, coord.clone());

                    runtime_print!("[TakeAsteroid] Asteroid taken at coord {:?}", coord);
                }
            }
        }

        fn remove_asteroid<Runtime: Config>(resource_type: AsteroidKind, coord: Coord) {
            if matches!(
                resource_type,
                AsteroidKind::Dot0 | AsteroidKind::Dot1 | AsteroidKind::Dot2
            ) {
                let dot_amount = Self::get_dot_amount::<Runtime>(resource_type);
                // If it's a DOT asteroid, we need to remove it from the total emitted DOT
                DotEmittedTotal::<T>::mutate(|total| {
                    *total = total.saturating_sub(dot_amount);
                });
            }

            Self::deposit_event(Event::AsteroidRemoved {
                coord: coord.clone(),
            });
            runtime_print!("[on_initialize] remove coord: {:?}", coord);
            Asteroids::<T>::remove(coord);
        }

        fn get_dot_amount<Runtime: Config>(asteroid_type: AsteroidKind) -> u64 {
            match asteroid_type {
                AsteroidKind::Dot0 => 1,
                AsteroidKind::Dot1 => 2,
                AsteroidKind::Dot2 => 3,
                _ => 0,
            }
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

        fn get_random_asteroid_type<Runtime: Config>(
            index: u32,
            block: BlockNumberFor<T>,
        ) -> AsteroidKind {
            let roll = get_random::<T>(100, index + 500); // 0–99

            let pool_size = 100; // DotPrizePool::<T>::get();
            let dot_emitted = DotEmittedTotal::<T>::get();
            let last_nft_block = LastNftSpawnBlock::<T>::get();

            // Calculate the number of players
            let players_count = ActiveShips::<T>::iter().count() as u32;

            if roll < 5 && dot_emitted < pool_size / DOT_EMISSION_LIMIT_RATIO {
                // 5% chance for Dot
                if roll < 3 {
                    AsteroidKind::Dot0 // 3% chance for Dot0
                } else if roll == 3 {
                    AsteroidKind::Dot1 // 1% chance for Dot1
                } else {
                    AsteroidKind::Dot2 // 1% chance for Dot2
                }
            } else if roll < 30 {
                AsteroidKind::Energy
            } else if roll < 50 && block > last_nft_block + NFT_SPAWN_COOLDOWN_BLOCKS.into() {
                if players_count < 3 {
                    // 20% chance for NFT0 if there are less than 5 players
                    AsteroidKind::Nft0
                } else if players_count < 4 {
                    // 10% chance for NFT1 if there are less than 10 players
                    AsteroidKind::Nft1
                } else {
                    // 5% chance for NFT2 if there are more than 10 players
                    AsteroidKind::Nft2
                }
            } else {
                AsteroidKind::Gold
            }
        }
    }
}
