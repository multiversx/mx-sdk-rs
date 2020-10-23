pub mod arg_types;
pub mod arg_types_multi;
pub mod arg_loader_endpoint;
pub mod arg_loader_cd;
pub mod arg_loader_err;
pub mod arg_serialize;
pub mod finish;

pub use arg_types::*;
pub use arg_types_multi::*;
pub use arg_loader_endpoint::*;
pub use arg_loader_cd::*;
pub use arg_loader_err::*;
pub use arg_serialize::*;
pub use finish::*;

#[cfg(test)]
pub mod test_arg_load {
    use crate::*;

    pub struct PanickingDynArgErrHandler;

    impl DynArgErrHandler for PanickingDynArgErrHandler {
        fn handle_sc_error(&self, _err: SCError) -> ! {
            panic!("PanickingDynArgErrHandler panicked")
        }
    }
    
    #[test]
    fn test_simple_args() {
        let input: &[u8] = b"func@1111@2222";
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let arg1: i32 = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler, &[]);
        assert_eq!(arg1, 0x1111i32);

        let arg2: &i32 = &load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler, &[]);
        assert_eq!(arg2, &0x2222i32);

        assert!(!DynArgLoader::<()>::has_next(&cd_loader));
    }

    #[test]
    fn test_simple_vec_arg() {
        let input: &[u8] = b"some_other_func@000000020000000300000006";
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let arg1: Vec<usize> = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler, &[]);
        assert_eq!(arg1, [2usize, 3usize, 6usize].to_vec());
        assert!(!DynArgLoader::<()>::has_next(&cd_loader));
    }

    #[test]
    fn test_var_args() {
        let input: &[u8] = b"func@1111@2222";
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let var_arg: VarArgs<i32> = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler, &[]);
        let arg_vec = var_arg.into_vec();
        assert_eq!(arg_vec.len(), 2);
        assert_eq!(arg_vec[0], 0x1111i32);
        assert_eq!(arg_vec[1], 0x2222i32);
    }

    #[test]
    fn test_multi_arg_2() {
        let input: &[u8] = b"func@1111@2222";
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let tuple_arg: MultiArg2<i32, i32> = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler, &[]);
        let tuple = tuple_arg.into_tuple();
        assert_eq!(tuple.0, 0x1111i32);
        assert_eq!(tuple.1, 0x2222i32);
    }

    #[test]
    fn test_var_multi_arg_2() {
        let input: &[u8] = b"func@1111@2222";
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let tuple_arg: VarArgs<MultiArg2<i32, i32>> = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler, &[]);
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
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let opt_tuple_arg: OptionalArg<MultiArg2<i32, i32>> = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler, &[]);
        match opt_tuple_arg {
            OptionalArg::Some(tuple_arg) => {
                let tuple = tuple_arg.into_tuple();
                assert_eq!(tuple.0, 0x1111i32);
                assert_eq!(tuple.1, 0x2222i32);
            },
            OptionalArg::None => {
                panic!("OptionalArg::Some expected");
            }
        }
    }

    #[test]
    fn test_async_call_result_ok() {
        let input: &[u8] = b"func@@1111@2222";
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let acr: AsyncCallResult<MultiArg2<i32, i32>> = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler, &[]);
        match acr {
            AsyncCallResult::Ok(tuple_arg) => {
                let tuple = tuple_arg.into_tuple();
                assert_eq!(tuple.0, 0x1111i32);
                assert_eq!(tuple.1, 0x2222i32);
            },
            AsyncCallResult::Err(_) => {
                panic!("AsyncCallResult::Ok expected");
            }
        }
    }

    #[test]
    fn test_async_call_result_ok2() {
        let input: &[u8] = b"func@00";
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let acr: AsyncCallResult<VarArgs<MultiArg2<i32, i32>>> = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler, &[]);
        match acr {
            AsyncCallResult::Ok(var_args) => {
                assert_eq!(var_args.len(), 0);
            },
            AsyncCallResult::Err(_) => {
                panic!("AsyncCallResult::Ok expected");
            }
        }
    }

    #[test]
    fn test_async_call_result_err() {
        let input: &[u8] = b"func@0123@1111";
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let acr: AsyncCallResult<MultiArg2<i32, i32>> = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler, &[]);
        match acr {
            AsyncCallResult::Ok(_) => {
                panic!("AsyncCallResult::Err expected");
            },
            AsyncCallResult::Err(async_call_error) => {
                assert_eq!(async_call_error.err_code, 0x0123);
                assert_eq!(async_call_error.err_msg, [0x11u8, 0x11u8].to_vec());
            }
        }
    }
}
