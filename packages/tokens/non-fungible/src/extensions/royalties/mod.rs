mod storage;
use crate::NonFungibleToken;

mod test;

use soroban_sdk::{Address, Env, Symbol};

/// Royalties Trait for Non-Fungible Token (ERC2981)
///
/// The `NonFungibleRoyalties` trait extends the `NonFungibleToken` trait to
/// provide the capability to set and query royalty information for tokens. This
/// trait is designed to be used in conjunction with the `NonFungibleToken`
/// trait.
///
/// This implementation follows the ERC2981 standard for royalties, allowing:
/// - Setting global royalties for the entire collection
/// - Setting per-token royalties that override the global setting
///
/// `storage.rs` file of this module provides the `NonFungibleRoyalties` trait
/// implementation.
///
/// # Notes
///
/// `#[contractimpl]` macro requires even the default implementations to be
/// present under its scope. To avoid confusion, we do not provide the default
/// implementations here, but we are providing a macro that generates them.
///
/// ## Example
///
/// ```ignore
/// #[default_impl] // **IMPORTANT**: place this above `#[contractimpl]`
/// #[contractimpl]
/// impl NonFungibleRoyalties for MyContract {
///     /* your overrides here (you don't have to put anything here if you don't want to override anything) */
///     /* and the macro will generate all the missing default implementations for you */
/// }
/// ```
pub trait NonFungibleRoyalties: NonFungibleToken {
    /// Sets the global default royalty information for the entire collection.
    /// This will be used for all tokens that don't have specific royalty
    /// information.
    ///
    /// # Arguments
    ///
    /// * `e` - Access to the Soroban environment.
    /// * `receiver` - The address that should receive royalty payments.
    /// * `basis_points` - The royalty percentage in basis points (100 = 1%,
    ///   10000 = 100%).
    ///
    /// # Errors
    ///
    /// * [`crate::NonFungibleTokenError::InvalidRoyaltyAmount`] - If the
    ///   royalty amount is higher than 10_000 (100%) basis points.
    ///
    /// # Events
    ///
    /// * topics - `["set_default_royalty", receiver: Address]`
    /// * data - `[basis_points: u32]`
    fn set_default_royalty(e: &Env, receiver: Address, basis_points: u32);

    /// Sets the royalty information for a specific token.
    ///
    /// # Arguments
    ///
    /// * `e` - Access to the Soroban environment.
    /// * `token_id` - The identifier of the token.
    /// * `receiver` - The address that should receive royalty payments.
    /// * `basis_points` - The royalty percentage in basis points (100 = 1%,
    ///   10000 = 100%).
    ///
    /// # Errors
    ///
    /// * [`crate::NonFungibleTokenError::InvalidRoyaltyAmount`] - If the
    ///   royalty amount is higher than 10_000 (100%) basis points.
    /// * [`crate::NonFungibleTokenError::NonExistentToken`] - If the token does
    ///   not exist.
    ///
    /// # Events
    ///
    /// * topics - `["set_token_royalty", receiver: Address]`
    /// * data - `[token_id: u32, basis_points: u32]`
    fn set_token_royalty(e: &Env, token_id: u32, receiver: Address, basis_points: u32);

    /// Returns `(Address, u32)` - A tuple containing the receiver address and
    /// the royalty amount.
    ///
    /// # Arguments
    ///
    /// * `e` - Access to the Soroban environment.
    /// * `token_id` - The identifier of the token.
    /// * `sale_price` - The sale price for which royalties are being
    ///   calculated.
    ///
    /// # Errors
    ///
    /// * [`crate::NonFungibleTokenError::NonExistentToken`] - If the token does
    ///   not exist.
    fn royalty_info(e: &Env, token_id: u32, sale_price: u32) -> (Address, u32);
}

// ################## EVENTS ##################

/// Emits an event indicating that default royalty has been set.
///
/// # Arguments
///
/// * `e` - Access to Soroban environment.
/// * `receiver` - The address that will receive royalty payments.
/// * `basis_points` - The royalty percentage in basis points (100 = 1%, 10000 =
///   100%).
///
/// # Events
///
/// * topics - `["set_default_royalty", receiver: Address]`
/// * data - `[basis_points: u32]`
pub fn emit_set_default_royalty(e: &Env, receiver: &Address, basis_points: u32) {
    let topics = (Symbol::new(e, "set_default_royalty"), receiver);
    e.events().publish(topics, basis_points);
}

/// Emits an event indicating that token royalty has been set.
///
/// # Arguments
///
/// * `e` - Access to Soroban environment.
/// * `receiver` - The address that will receive royalty payments.
/// * `token_id` - The identifier of the token.
/// * `basis_points` - The royalty percentage in basis points (100 = 1%, 10000 =
///   100%).
///
/// # Events
///
/// * topics - `["set_token_royalty", receiver: Address, token_id: u32]`
/// * data - `[basis_points: u32]`
pub fn emit_set_token_royalty(e: &Env, receiver: &Address, token_id: u32, basis_points: u32) {
    let topics = (Symbol::new(e, "set_token_royalty"), receiver, token_id);
    e.events().publish(topics, basis_points);
}
