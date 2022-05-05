#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Action {
		Increment,
		Decrement,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// Errors.
	#[pallet::error]
	pub enum Error<T> {
		CannotGoBelowZero,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ToggleAction(T::AccountId, Action),
		ActionMade(T::AccountId, Action, i32),
	}

	#[pallet::type_value]
  pub(super) fn DefaultNumber<T: Config>() -> i32 { 0 }
	#[pallet::storage]
	#[pallet::getter(fn unary_number)]
	// Store count in relation to unary action.
	pub(super) type UnaryNumber<T: Config> = StorageValue<Value = i32, QueryKind = ValueQuery, OnEmpty = DefaultNumber<T>>;

	#[pallet::type_value]
  pub(super) fn DefaultAction<T: Config>() -> Option<Action> { Some(Action::Increment) }
	#[pallet::storage]
	#[pallet::getter(fn unary_action)]
	/// Store unary action.
	pub(super) type UnaryAction<T: Config> = StorageValue<Value = Option<Action>, QueryKind = ValueQuery, OnEmpty = DefaultAction<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Toggle unary.
		/// 
		/// Toggle unary action based on previous unary
		#[pallet::weight(100)]
		pub fn toggle_action (origin: OriginFor<T>) -> DispatchResult {
			// ensure sender is signed.
			let sender = ensure_signed(origin)?;

			match <UnaryAction<T>>::get().unwrap() {
				Action::Increment => <UnaryAction<T>>::put(Some(Action::Decrement)),
				Action::Decrement => <UnaryAction<T>>::put(Some(Action::Increment)),
			}

			Self::deposit_event(Event::ToggleAction(sender, <UnaryAction<T>>::get().unwrap()));
			Ok(())
		}

		/// Make action
		///
		/// Increment or Decrement number based on unary action.
		#[pallet::weight(100)]
		pub fn make_action (origin: OriginFor<T>) -> DispatchResult {
			// ensure sender is signed.
			let sender = ensure_signed(origin)?;

			match <UnaryAction<T>>::get().unwrap() {
				Action::Increment => <UnaryNumber<T>>::put(<UnaryNumber<T>>::get() + 1),
				Action::Decrement => {
					// Number must not go below zero.
					ensure!((<UnaryNumber<T>>::get() - 1) > -1,  <Error<T>>::CannotGoBelowZero);
					<UnaryNumber<T>>::put(<UnaryNumber<T>>::get() - 1);
				},
			}

			Self::deposit_event(Event::ActionMade(sender, <UnaryAction<T>>::get().unwrap(), <UnaryNumber<T>>::get()));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {}
}
