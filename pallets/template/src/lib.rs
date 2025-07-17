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
const MAX_ASTEROIDS_COUNT: u32 = 30;
const MAP_SIZE: u32 = 50;
const ASTEROID_TTL_CONST: u32 = 10;
const RESOURCE_DISTANCE_LIMIT: u32 = 5;
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
    DecodeWithMemTracking,
    MaxEncodedLen,
    Clone,
    PartialEq,
    Eq,
    RuntimeDebug,
    TypeInfo,
)]
pub struct Starship {
    pub pos: Coord,
    pub energy: Energy,
    pub nft_skin: u32,
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
    Nft0 = 5, //Uncommon
    Nft1 = 6, //Rare
    Nft2 = 7, //Mystical
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

    #[pallet::storage]
    pub type MapSize<T> = StorageValue<_, u32>;

    #[pallet::storage]
    pub type MaxAsteroidsCount<T> = StorageValue<_, u32>;

    #[pallet::storage]
    pub type PlayersCount<T> = StorageValue<_, u32, ValueQuery>;

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
        StorageMap<_, Twox64Concat, UserAccount<T>, Starship, OptionQuery>;

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
            nft_skin: u32,
        },

        EnergyDepleted {
            owner: T::AccountId,
        },

        GameStarted {
            owner: T::AccountId,
            coord: Coord,
            nft_skin: u32,
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

                if let Some(mut ship) = ActiveShips::<T>::get(&user) {
                    ship.pos = coord.clone();
                    ActiveShips::<T>::insert(user.clone(), ship);
                }

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

            let map_size = MapSize::<T>::get().unwrap_or(MAP_SIZE);

            let asteroids_count = Asteroids::<T>::iter().count();

            let max_asteroids_count = MaxAsteroidsCount::<T>::get().unwrap_or(MAX_ASTEROIDS_COUNT);
            let difference =
                max_asteroids_count.saturating_sub(asteroids_count.try_into().unwrap_or(0));

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

            for (owner, ship) in ActiveShips::<T>::iter() {
                let new_energy = ship.energy.saturating_sub(ENERGY_DEPLETION_RATE);

                if new_energy == 0 {
                    runtime_print!(
                        "[on_init] Ship has no energy and is deactivated: {:?}",
                        owner
                    );
                    ActiveShips::<T>::remove(owner.clone());

                    Self::deposit_event(Event::EnergyDepleted {
                        owner: owner.clone(),
                    });

                    PlayersCount::<T>::mutate(|player_count| {
                        *player_count = player_count.saturating_sub(1);
                    });
                    continue;
                }

                ActiveShips::<T>::insert(
                    owner.clone(),
                    Starship {
                        pos: ship.pos,
                        energy: new_energy,
                        nft_skin: ship.nft_skin,
                    },
                );

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
            let from_coord = ship_coord.pos.clone();

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
                nft_skin: ship_coord.nft_skin,
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
            let distance = get_distance(ship_coord.pos.clone(), coord.clone());

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
        pub fn start_game(origin: OriginFor<T>, coord: Coord, nft_skin: u32) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            if ActiveShips::<T>::contains_key(who.clone()) {
                runtime_print!("[start_game] Player already has an active ship: {:?}", who);
                return Err(Error::<T>::NoneValue.into());
            }

            let map_size = MapSize::<T>::get().unwrap_or(MAP_SIZE);
            if coord.x >= map_size || coord.y >= map_size {
                runtime_print!("[start_game] Coordinates are out of bounds: {:?}", coord);
                return Err(Error::<T>::NoneValue.into());
            }

            // ! We decided not to require players to pay with Gold for participation,
            // ! since Gold can serve better as a leaderboard score.
            // ! Instead, we chose to use DOT for participation and created a dedicated DOT prize pool for this purpose.
            // ! So while the logic of paying to participate remains, the project now uses a shared
            // ! DOT pool to spawn DOT asteroids — which we believe is a better approach than requiring
            // ! Gold for entry.
            // let user_gold = AccountResources::<T>::get(&who, AsteroidKind::Dot0);
            // let user_gold = AccountResources::<T>::get(&who, AsteroidKind::Gold);
            // if user_gold < 20 {
            //     runtime_print!(
            //         "[start_game] Player does not have enough Gold: {:?}, has only {}",
            //         who,
            //         user_gold
            //     );
            //     return Err(Error::<T>::NoneValue.into());
            // }

            if nft_skin != 0 {
                let asteroid_kind = match nft_skin {
                    5 => AsteroidKind::Nft0,
                    6 => AsteroidKind::Nft1,
                    7 => AsteroidKind::Nft2,
                    _ => {
                        runtime_print!("[start_game] Invalid nft_skin: {}", nft_skin);
                        return Err(Error::<T>::NoneValue.into());
                    }
                };

                let has_nft = AccountResources::<T>::get(&who, asteroid_kind) > 0;
                if !has_nft {
                    runtime_print!(
                        "[start_game] Player does not have the required NFT: {:?}",
                        asteroid_kind
                    );
                    return Err(Error::<T>::NoneValue.into());
                }
            }

            ActiveShips::<T>::insert(
                who.clone(),
                Starship {
                    pos: coord.clone(),
                    energy: DEFAULT_ENERGY,
                    nft_skin: nft_skin,
                },
            );

            Self::deposit_event(Event::GameStarted {
                owner: who.clone(),
                coord: coord.clone(),
                nft_skin: nft_skin,
            });

            // Add the value to the total DOT prize pool
            DotPrizePool::<T>::mutate(|pool| {
                *pool = pool.saturating_add(DEFAULT_DOT_STAKE);
            });
            runtime_print!("[on_init] Active ship added {:?} coord: {:?}", who, coord);

            PlayersCount::<T>::mutate(|player_count| {
                *player_count = player_count.saturating_add(1);
            });
            Ok(())
        }

        // ! -------------------------------------------
        // ! Admin calls are implemented to allow faster testing of the game with different parameters.
        // ! That’s why they are not restricted at the moment.
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn admin_set_map_size(origin: OriginFor<T>, size: u32) -> DispatchResult {
            MapSize::<T>::put(size);
            runtime_print!("[set_map_size] Map size set to: {}", size);
            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn admin_set_max_asteroids_count(origin: OriginFor<T>, count: u32) -> DispatchResult {
            MaxAsteroidsCount::<T>::put(count);
            runtime_print!(
                "[set_max_asteroids_count] Max asteroids count set to: {}",
                count
            );
            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn admin_reset_game(origin: OriginFor<T>) -> DispatchResult {
            let map_size = MapSize::<T>::get().unwrap_or(MAP_SIZE);

            for (owner, mut ship) in ActiveShips::<T>::iter() {
                ship.energy = DEFAULT_ENERGY;
                ship.pos = Coord { x: 0, y: 0 };

                ActiveShips::<T>::insert(&owner, ship);
                runtime_print!(
                    "[admin_reset_game] Reset ship for {:?} to energy={} pos=(0,0)",
                    owner,
                    DEFAULT_ENERGY
                );
            }

            for (user, _) in Flights::<T>::iter() {
                Flights::<T>::remove(&user);
                runtime_print!("[admin_reset_game] Cleared flight for {:?}", user);
            }
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
                        if let Some(mut ship) = ActiveShips::<T>::get(&user) {
                            ship.energy = ship.energy.saturating_add(10);
                            ship.pos = coord.clone();

                            runtime_print!(
                                "[TakeAsteroid] Energy collected for user {:?}, new energy: {}",
                                user,
                                ship.energy
                            );

                            ActiveShips::<T>::insert(user.clone(), ship);
                        }
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
                            // Temporarily solution
                            Self::add_resource_to_account(&user, AsteroidKind::Dot0, amount);
                        } else {
                            Self::add_resource_to_account(&user, asteroid.0, amount);
                        }
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
            //let players_count = ActiveShips::<T>::iter().count() as u32;
            let players_count = PlayersCount::<T>::get();

            if roll < 10 && dot_emitted < pool_size / DOT_EMISSION_LIMIT_RATIO {
                // 10% chance for Dot
                if roll < 5 {
                    AsteroidKind::Dot0
                } else if roll < 7 {
                    AsteroidKind::Dot1
                } else {
                    AsteroidKind::Dot2
                }
            } else if roll < 30 {
                AsteroidKind::Energy
            } else if roll < 50 && block > last_nft_block + NFT_SPAWN_COOLDOWN_BLOCKS.into() {
                // Here you can implement custom logic for NFT spawning.

                if players_count > 2 {
                    AsteroidKind::Nft2
                } else if players_count > 1 {
                    AsteroidKind::Nft1
                } else {
                    AsteroidKind::Nft0
                }
            } else {
                AsteroidKind::Gold
            }
        }
    }
}
