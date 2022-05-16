use crate as pallet_supersig;
use frame_support::{parameter_types, traits::Everything, PalletId};
use frame_system as system;
use sp_core::{
    H256, sr25519, Public, Pair,
};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
	MultiSignature,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;

#[frame_support::pallet]
pub mod nothing {
	// use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Nothing {},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1000)]
		pub fn do_nothing(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			Ok(().into())
		}
	}
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Supersig: pallet_supersig::{Pallet, Call, Storage, Event<T>},
		Nothing: nothing::{Pallet, Call, Storage, Event<T>},

		Balances: pallet_balances,
	}
);

impl nothing::Config for Test {
	type Event = Event;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub type Balance = u64;

parameter_types! {
	pub const ExistentialDeposit: Balance = 1_000;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const SupersigPalletId: PalletId = PalletId(*b"id/dsupersig");
	pub const SupersigPreimageByteDeposit: Balance = 1000;
}

impl pallet_supersig::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type PalletId = SupersigPalletId;
	// type Call = Call;
	type SupersigPreimageByteDeposit = SupersigPreimageByteDeposit;
}

pub type NoCall = nothing::Call<Test>;

type AccountPublic = <MultiSignature as Verify>::Signer;

/// Helper function to generate a crypto pair from seeds
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Mock users AccountId
#[allow(non_snake_case)]
pub fn ALICE() -> AccountId {
    get_account_id_from_seed::<sr25519::Public>("Alice")
}
#[allow(non_snake_case)]
pub fn BOB() -> AccountId {
    get_account_id_from_seed::<sr25519::Public>("Bob")
}
#[allow(non_snake_case)]
pub fn CHARLIE() -> AccountId {
    get_account_id_from_seed::<sr25519::Public>("Charlie")
}
pub struct ExtBuilder {
	caps_endowed_accounts: Vec<(u64, u64)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder {
			caps_endowed_accounts: vec![(ALICE, 1_000_000), (BOB, 100_000), (CHARLIE, 100_000)],
		}
	}
}

impl ExtBuilder {
	pub fn balances(mut self, accounts: Vec<(u64, u64)>) -> Self {
		for account in accounts {
			self.caps_endowed_accounts.push(account);
		}
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		pallet_balances::GenesisConfig::<Test> { balances: self.caps_endowed_accounts }
			.assimilate_storage(&mut t)
			.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
