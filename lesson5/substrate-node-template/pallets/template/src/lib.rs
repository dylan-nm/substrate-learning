#![cfg_attr(not(feature = "std"), no_std)] // 如果没有标准库，则不使用标准库

pub use pallet::*; // 将 pallet 模块导出

#[frame_support::pallet] // 定义为 FRAME pallet
pub mod pallet {
	use frame_support::pallet_prelude::*; // FRAME 框架预定义的宏和结构体
	use frame_system::pallet_prelude::*; // FRAME 系统预定义的宏和结构体

	#[pallet::pallet] // 定义为 pallet
	#[pallet::generate_store(pub(super) trait Store)] // 自动生成存储代码
	pub struct Pallet<T>(_);

	#[pallet::config] // 定义配置 trait
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>; // 定义事件类型
	}

	#[pallet::event] // 定义事件
	#[pallet::generate_deposit(pub(super) fn deposit_event)] // 自动生成事件处理代码
	pub enum Event<T: Config> {
		ClaimCreated {
			// 声明 ClaimCreated 事件
			who: T::AccountId, // 声明 who 字段，类型为 T::AccountId
			claim: T::Hash,    // 声明 claim 字段，类型为 T::Hash
		},
	}

	#[pallet::error] // 定义错误
	pub enum Error<T> {
		ClaimNotExist, // 声明 ClaimNotExist 错误
	}

	#[pallet::storage] // 定义存储
	#[pallet::getter(fn claims)] // 定义 claims 存储的 getter 函数
	pub type Claims<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, T::Hash, OptionQuery>;

	#[pallet::hooks] // 定义 hooks
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call] // 定义调用
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)] // 定义权重
		pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResultWithPostInfo {
			// 声明 create_claim 函数
			let who = ensure_signed(origin)?; // 确认签名
			Claims::<T>::insert(&who, claim); // 插入 claim 到存储
			Self::deposit_event(Event::ClaimCreated { who, claim }); // 触发事件
			Ok(().into()) // 返回成功
		}
	}
}
