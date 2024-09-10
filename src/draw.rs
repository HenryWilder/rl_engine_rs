//! Trait for enabling rendering on types

use super::prelude::*;

/// Defines how an object is rendered
///
/// Example:
/// ```
/// # use rl_engine_rs::prelude::*;
/// enum WoodType {
///     Oak,
///     Pine,
///     Birch,
///     Spruce,
/// }
///
/// enum Block {
///     Grass,
///     Wood(WoodType),
///     Stone,
/// }
///
/// struct BlockDrawArgs {
///     pub position: Vector3,
///     pub light: Color,
/// }
///
/// impl Draw for Block {
///     fn draw(&self, d: &mut impl RaylibDraw) {
///
///     }
/// }
/// ```
pub trait Draw {
    /// How the engine will draw the object
    fn draw(&self, d: &mut impl RaylibDraw);
}
