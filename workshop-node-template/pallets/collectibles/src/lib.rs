#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_support::{pallet_prelude::{Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen}, traits::{Currency, Randomness}};

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Color {
	Red,
	Yellow,
	Blue,
	Green
}

#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Collectible<T: Config> {
	// Unsigned integers of 16 bytes to represent a unique identifier
	pub uniqueId: [u8; 16],
	// `None` assumes not for sale
	pub price: Option<BalanceOf<T>>,
	pub color: Color,
	pub owner: T::AccountId,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
		type Currency: Currency<Self::AccountId>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		#[pallet::constant]
		type MaximumOwned: Get<u32>;
    }
}