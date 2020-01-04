
extern {
    fn int64getArgument(id: i32) -> i64;
    fn int64storageStore(keyOffset: *const u8, value: i64) -> i32;
    fn int64storageLoad(keyOffset: *const u8) -> i64;
    fn int64finish(value: i64);
}

pub fn get_argument_i64(arg_id: i32) -> i64 {
    unsafe { int64getArgument(arg_id) }
}

// pub fn get_argument_i32(arg_id: i32) -> i32 {
//     unsafe { 
//         let int64 = int64getArgument(arg_id);
//         int64.try_into().unwrap()
//         //i32::try_from(int64).unwrap()
//     }
// }

pub fn finish_i64(value: i64) {
    unsafe { int64finish(value); }
}