//! Traits for asset loading/unloading

use std::rc::Rc;

/// An asset that can be loaded and unloaded.
/// Unloading should be implemented in `Drop`.
pub trait Asset: Drop {}

/// Indicates that the asset should be loaded immediately and persist as long as the game is running
pub trait StaticAsset: Asset {}

/// A trait for converting fallible asset loading to infallible
pub trait AssetFailResponse<T: AssetLoader> {
    /// Ensures a valid object is received, making load infallible
    fn ensure(result: Result<T::Item, T::LoadError>) -> T::Item;
}

/// Indicates that the game should panic if the asset fails to load
pub struct MandatoryAsset {}

impl<T: AssetLoader> AssetFailResponse<T> for MandatoryAsset {
    fn ensure(result: Result<T::Item, T::LoadError>) -> T::Item {
        result
            .expect("mandatory asset failed to load")
    }
}

/// Indicates that the game should fallback to the asset type's default value if the asset fails to load
pub struct DefaultingAsset {}

impl<T: AssetLoader> AssetFailResponse<T> for DefaultingAsset
where
    <T as AssetLoader>::Item: Default
{
    fn ensure(result: Result<T::Item, T::LoadError>) -> T::Item {
        result
            .inspect_err(|err| println!("asset load error: {err:?}"))
            .unwrap_or_default()
    }
}

/// An asset that can be loaded and unloaded.
/// Unloading should be implemented in `Drop`.
pub trait AssetLoader: Sized {
    /// Error type for if the asset fails to load
    type LoadError: std::fmt::Debug;

    /// How the game should respond if the asset fails to load
    type FailResponse: AssetFailResponse<Self>;

    /// The argument passed to `load` and `get`
    type Source;

    /// The type of asset this loader loads
    type Item: Asset;

    /// Defines how the asset is loaded
    fn try_load(src: Self::Source) -> Result<Self::Item, Self::LoadError>;

    /// Loads the asset with built-in error handling
    fn load(src: Self::Source) -> Self::Item {
        Self::FailResponse::ensure(Self::try_load(src))
    }
}

/// Collection of all game assets
pub struct Assets<'a> {
    static_assets: Vec<&'a dyn StaticAsset>,
    assets: Vec<Rc<dyn Asset>>,
}

impl<'a> Assets<'a> {
    /// Gets the asset from the asset list or loads it immediately if it isn't there
    pub fn get_static<T: AssetLoader>(&mut self, src: T::Source) -> &'a T
    where
        <T as AssetLoader>::Item: StaticAsset
    {
        todo!()
    }

    /// Gets the asset from the asset list or loads it immediately if it isn't there
    pub fn get<T: AssetLoader>(&mut self, src: T::Source) -> &'a T {
        todo!()
    }
}
