pub mod arg_types;
pub mod arg_loader_endpoint;
pub mod arg_loader_cd;
pub mod arg_loader_err;
pub mod finish;
pub mod sc_error;

pub use arg_types::*;
pub use arg_loader_endpoint::*;
pub use arg_loader_cd::*;
pub use arg_loader_err::*;
pub use finish::*;
pub use sc_error::*;

#[cfg(test)]
pub mod test_arg_load {
    use crate::*;

    pub struct PanickingDynArgErrHandler;

    impl DynArgErrHandler for PanickingDynArgErrHandler {
        fn handle_sc_error(&self, _err: SCError) -> ! {
            panic!()
        }
    }
    
    #[test]
    fn test_simple_args() {
        let input: &[u8] = b"func@1111@2222";
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let arg1: i32 = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler);
        assert_eq!(arg1, 0x1111i32);

        let arg2: &i32 = &load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler);
        assert_eq!(arg2, &0x2222i32);
    }

    #[test]
    fn test_var_args() {
        let input: &[u8] = b"func@1111@2222";
        let de = CallDataDeserializer::new(input);
        let mut cd_loader = CallDataArgLoader::new(de);
        let var_arg: VarArgs<i32> = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler);
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
        let tuple_arg: MultiArg2<i32, i32> = load_dyn_arg(&mut cd_loader, &PanickingDynArgErrHandler);
        let tuple = tuple_arg.into_tuple();
        assert_eq!(tuple.0, 0x1111i32);
        assert_eq!(tuple.1, 0x2222i32);
    }
}
