elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::utils::{events, owner_endpoints, storage};

#[elrond_wasm_derive::module]
pub trait TokenMethods:
	storage::StorageModule + events::EventsModule + owner_endpoints::OwnerEndpointsModule
{
	#[endpoint(ftLocalMint)]
	fn local_mint(
		&self,
		identifier: TokenIdentifier,
		nonce: u64,
		amount: Self::BigUint,
	) -> SCResult<()> {
		self.send().esdt_local_mint(&identifier, nonce, &amount);
		self.bonding_curve(&identifier).update(|bonding_curve| {
			bonding_curve.arguments.available_supply += &amount;
			bonding_curve.arguments.balance += &amount;
		});

		self.nonce_amount(&identifier, nonce).update(|value| {
			*value += &amount;
		});
		Ok(())
	}

	#[endpoint(ftLocalBurn)]
	fn local_burn(&self, identifier: TokenIdentifier, nonce: u64, amount: Self::BigUint) {
		self.send().esdt_local_burn(&identifier, nonce, &amount);
		self.bonding_curve(&identifier).update(|bonding_curve| {
			bonding_curve.arguments.available_supply -= &amount;
			bonding_curve.arguments.balance -= &amount;
		});

		self.nonce_amount(&identifier, nonce).update(|value| {
			*value -= &amount;
		});
	}

	#[endpoint(nftCreate)]
	#[allow(clippy::too_many_arguments)]
	fn create(
		&self,
		identifier: TokenIdentifier,
		amount: Self::BigUint,
		name: BoxedBytes,
		royalties: Self::BigUint,
		hash: BoxedBytes,
		attributes: BoxedBytes,
		uri: BoxedBytes,
		#[var_args] payment: OptionalArg<TokenIdentifier>,
	) -> SCResult<()> {
		self.send().esdt_nft_create(
			&identifier,
			&amount,
			&name,
			&royalties,
			&hash,
			&attributes,
			&[uri],
		);

		let nonce = self
			.blockchain()
			.get_current_esdt_nft_nonce(&self.blockchain().get_sc_address(), &identifier);
		require!(
			!self.token_details(&identifier).is_empty(),
			"Token not issued"
		);

		let mut set_payment = TokenIdentifier::egld();
		if self.bonding_curve(&identifier).is_empty() {
			set_payment = payment
				.into_option()
				.ok_or("Expected provided accepted_payment for the token")?;
		}

		self.token_details(&identifier)
			.update(|details| details.token_nonces.push(nonce));

		self.nonce_amount(&identifier, nonce).set(&amount);
		self.set_curve_storage(&identifier, amount, set_payment);

		Ok(())
	}
}
