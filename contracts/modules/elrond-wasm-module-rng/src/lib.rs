#![no_std]

elrond_wasm::imports!();

const SEED_SIZE: usize = 48;
type RngSeed = Box<[u8; SEED_SIZE]>;

#[elrond_wasm_derive::module]
pub trait RandomNumberGeneratorModule {
	/// Remember to initialize the module first
	fn init_rng_module(&self) {
		let mut init_seed = RngSeed::new([0u8; SEED_SIZE]);

		self._refresh_seed(&mut init_seed);
	}

	fn next_u8(&self) -> u8 {
		let nr_bytes = 1;
		let rand_nr = self._next_number(nr_bytes);
		
		rand_nr as u8
	}

	fn next_u16(&self) -> u16 {
		let nr_bytes = (u16::BITS / u8::BITS) as usize;
		let rand_nr = self._next_number(nr_bytes);
		
		rand_nr as u16
	}

	fn next_u32(&self) -> u32 {
		let nr_bytes = (u32::BITS / u8::BITS) as usize;
		let rand_nr = self._next_number(nr_bytes);
		
		rand_nr as u32
	}

	fn next_u64(&self) -> u64 {
		let nr_bytes = (u64::BITS / u8::BITS) as usize;
		let rand_nr = self._next_number(nr_bytes);

		// since wasm32 doesn't have u64, but only i64, we set the first bit to 0
		// this is done to prevent any potential unintended bugs that are impossible to track down
	
		rand_nr & 0x7fffffffffffffff
	}

	/// please use one of the specific methods instead of directly calling this
	fn _next_number(&self, nr_bytes: usize) -> u64 {
		if nr_bytes == 0 || nr_bytes as u32 > u64::BITS / u8::BITS {
			return 0;
		}

		let mut seed = self.rng_seed().get();
		let mut start = self.rng_seed_index().get();

		if start + nr_bytes > SEED_SIZE {
			self._refresh_seed(&mut seed);
			start = 0;
		}

		let end = start + nr_bytes;
		self.rng_seed_index().set(&end);

		u64::top_decode(&seed[start..end]).unwrap_or_default()
	}

	/// Parent contract should never call this function. Calling this directly leads to undefined behaviour.
	/// Add new seed to old seed for cases when many random numbers are needed in the same block
	/// This prevents infinite loops, where a contract may want distinct numbers (for example, lottery SC)
	fn _refresh_seed(&self, old_seed: &mut RngSeed) {
		let new_seed = self.blockchain().get_block_random_seed();

		for i in 0..SEED_SIZE {
			old_seed[i] = old_seed[i].wrapping_add(new_seed[i]);
		}

		self.rng_seed().set(old_seed);
		self.rng_seed_index().clear();
	}

	// storage

	#[storage_mapper("rng:seed_index")]
	fn rng_seed_index(&self) -> SingleValueMapper<Self::Storage, usize>;

	#[storage_mapper("rng:seed")]
	fn rng_seed(&self) -> SingleValueMapper<Self::Storage, RngSeed>;
}
