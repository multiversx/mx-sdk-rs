pub mod arg_de_input;
pub mod arg_serialize;
pub mod dyn_arg;
pub mod dyn_arg_input;
pub mod dyn_arg_input_cd;
pub mod dyn_arg_input_endpoint;
pub mod finish;
pub mod macro_helpers;
pub mod signal_error;

pub use arg_de_input::*;
pub use arg_serialize::*;
pub use dyn_arg::*;
pub use dyn_arg_input::*;
pub use dyn_arg_input_cd::*;
pub use dyn_arg_input_endpoint::*;
pub use finish::*;
pub use macro_helpers::*;
pub use signal_error::*;

#[cfg(test)]
pub mod test_arg_load {
	use crate::*;

	#[test]
	fn test_simple_args() {
		let input: &[u8] = b"func@1111@2222";
		let de = HexCallDataDeserializer::new(input);
		let mut cd_loader = CallDataArgLoader::new(de, PanickingSignalError);
		let arg1: i32 = load_dyn_arg(&mut cd_loader, ArgId::empty());
		assert_eq!(arg1, 0x1111i32);

		let arg2: &i32 = &load_dyn_arg(&mut cd_loader, ArgId::empty());
		assert_eq!(arg2, &0x2222i32);

		cd_loader.assert_no_more_args();
	}

	#[test]
	fn test_simple_vec_arg() {
		let input: &[u8] = b"some_other_func@000000020000000300000006";
		let de = HexCallDataDeserializer::new(input);
		let mut cd_loader = CallDataArgLoader::new(de, PanickingSignalError);
		let arg1: Vec<usize> = load_dyn_arg(&mut cd_loader, ArgId::empty());
		assert_eq!(arg1, [2usize, 3usize, 6usize].to_vec());

		cd_loader.assert_no_more_args();
	}

	#[test]
	fn test_var_args() {
		let input: &[u8] = b"func@1111@2222";
		let de = HexCallDataDeserializer::new(input);
		let mut cd_loader = CallDataArgLoader::new(de, PanickingSignalError);
		let var_arg: VarArgs<i32> = load_dyn_arg(&mut cd_loader, ArgId::empty());
		let arg_vec = var_arg.into_vec();
		assert_eq!(arg_vec.len(), 2);
		assert_eq!(arg_vec[0], 0x1111i32);
		assert_eq!(arg_vec[1], 0x2222i32);
	}

	#[test]
	fn test_multi_arg_2() {
		let input: &[u8] = b"func@1111@2222";
		let de = HexCallDataDeserializer::new(input);
		let mut cd_loader = CallDataArgLoader::new(de, PanickingSignalError);
		let tuple_arg: MultiArg2<i32, i32> = load_dyn_arg(&mut cd_loader, ArgId::empty());
		let tuple = tuple_arg.into_tuple();
		assert_eq!(tuple.0, 0x1111i32);
		assert_eq!(tuple.1, 0x2222i32);
	}

	#[test]
	fn test_var_multi_arg_2() {
		let input: &[u8] = b"func@1111@2222";
		let de = HexCallDataDeserializer::new(input);
		let mut cd_loader = CallDataArgLoader::new(de, PanickingSignalError);
		let tuple_arg: VarArgs<MultiArg2<i32, i32>> = load_dyn_arg(&mut cd_loader, ArgId::empty());
		let tuple_vec = tuple_arg.into_vec();
		assert_eq!(tuple_vec.len(), 1);
		let mut iter = tuple_vec.into_iter();
		let tuple = iter.next().unwrap().into_tuple();
		assert_eq!(tuple.0, 0x1111i32);
		assert_eq!(tuple.1, 0x2222i32);
	}

	#[test]
	fn test_opt_multi_arg_2() {
		let input: &[u8] = b"func@1111@2222";
		let de = HexCallDataDeserializer::new(input);
		let mut cd_loader = CallDataArgLoader::new(de, PanickingSignalError);
		let opt_tuple_arg: OptionalArg<MultiArg2<i32, i32>> =
			load_dyn_arg(&mut cd_loader, ArgId::empty());
		match opt_tuple_arg {
			OptionalArg::Some(tuple_arg) => {
				let tuple = tuple_arg.into_tuple();
				assert_eq!(tuple.0, 0x1111i32);
				assert_eq!(tuple.1, 0x2222i32);
			},
			OptionalArg::None => {
				panic!("OptionalArg::Some expected");
			},
		}
	}

	#[test]
	fn test_async_call_result_ok() {
		let input: &[u8] = b"func@@1111@2222";
		let de = HexCallDataDeserializer::new(input);
		let mut cd_loader = CallDataArgLoader::new(de, PanickingSignalError);
		let acr: AsyncCallResult<MultiArg2<i32, i32>> =
			load_dyn_arg(&mut cd_loader, ArgId::empty());
		match acr {
			AsyncCallResult::Ok(tuple_arg) => {
				let tuple = tuple_arg.into_tuple();
				assert_eq!(tuple.0, 0x1111i32);
				assert_eq!(tuple.1, 0x2222i32);
			},
			AsyncCallResult::Err(_) => {
				panic!("AsyncCallResult::Ok expected");
			},
		}
	}

	#[test]
	fn test_async_call_result_ok2() {
		let input: &[u8] = b"func@00";
		let de = HexCallDataDeserializer::new(input);
		let mut cd_loader = CallDataArgLoader::new(de, PanickingSignalError);
		let acr: AsyncCallResult<VarArgs<MultiArg2<i32, i32>>> =
			load_dyn_arg(&mut cd_loader, ArgId::empty());
		match acr {
			AsyncCallResult::Ok(var_args) => {
				assert_eq!(var_args.len(), 0);
			},
			AsyncCallResult::Err(_) => {
				panic!("AsyncCallResult::Ok expected");
			},
		}
	}

	#[test]
	fn test_async_call_result_err() {
		let input: &[u8] = b"func@0123@1111";
		let de = HexCallDataDeserializer::new(input);
		let mut cd_loader = CallDataArgLoader::new(de, PanickingSignalError);
		let acr: AsyncCallResult<MultiArg2<i32, i32>> =
			load_dyn_arg(&mut cd_loader, ArgId::empty());
		match acr {
			AsyncCallResult::Ok(_) => {
				panic!("AsyncCallResult::Err expected");
			},
			AsyncCallResult::Err(async_call_error) => {
				assert_eq!(async_call_error.err_code, 0x0123);
				assert_eq!(async_call_error.err_msg, [0x11u8, 0x11u8].to_vec());
			},
		}
	}
}
