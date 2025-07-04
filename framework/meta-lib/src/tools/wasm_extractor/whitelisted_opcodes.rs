use wasmparser::Operator;

pub(super) const ERROR_FAIL_ALLOCATOR: &[u8; 27] = b"memory allocation forbidden";
pub(super) const WRITE_OP: &[&str] = &[
    "mBufferStorageStore",
    "storageStore",
    "int64storageStore",
    "bigIntStorageStoreUnsigned",
    "smallIntStorageStoreUnsigned",
    "smallIntStorageStoreSigned",
];

pub(super) fn is_whitelisted(op: &Operator) -> bool {
    matches!(
        op,
        Operator::Block { .. }
            | Operator::Br { .. }
            | Operator::BrIf { .. }
            | Operator::BrTable { .. }
            | Operator::Call { .. }
            | Operator::CallIndirect { .. }
            | Operator::Catch { .. }
            | Operator::CatchAll { .. }
            | Operator::Delegate { .. }
            | Operator::Drop { .. }
            | Operator::Else { .. }
            | Operator::End { .. }
            | Operator::GlobalGet { .. }
            | Operator::GlobalSet { .. }
            | Operator::I32Add { .. }
            | Operator::I32And { .. }
            | Operator::I32Clz { .. }
            | Operator::I32Const { .. }
            | Operator::I32Ctz { .. }
            | Operator::I32DivS { .. }
            | Operator::I32DivU { .. }
            | Operator::I32Eq { .. }
            | Operator::I32Eqz { .. }
            | Operator::I32Extend16S { .. }
            | Operator::I32Extend8S { .. }
            | Operator::I32GeS { .. }
            | Operator::I32GeU { .. }
            | Operator::I32GtS { .. }
            | Operator::I32GtU { .. }
            | Operator::I32LeS { .. }
            | Operator::I32LeU { .. }
            | Operator::I32Load { .. }
            | Operator::I32Load16S { .. }
            | Operator::I32Load16U { .. }
            | Operator::I32Load8S { .. }
            | Operator::I32Load8U { .. }
            | Operator::I32LtS { .. }
            | Operator::I32LtU { .. }
            | Operator::I32Mul { .. }
            | Operator::I32Ne { .. }
            | Operator::I32Or { .. }
            | Operator::I32Popcnt { .. }
            | Operator::I32RemS { .. }
            | Operator::I32RemU { .. }
            | Operator::I32Rotl { .. }
            | Operator::I32Rotr { .. }
            | Operator::I32Shl { .. }
            | Operator::I32ShrS { .. }
            | Operator::I32ShrU { .. }
            | Operator::I32Store { .. }
            | Operator::I32Store16 { .. }
            | Operator::I32Store8 { .. }
            | Operator::I32Sub { .. }
            | Operator::I32WrapI64 { .. }
            | Operator::I32Xor { .. }
            | Operator::I64Add { .. }
            | Operator::I64And { .. }
            | Operator::I64Clz { .. }
            | Operator::I64Const { .. }
            | Operator::I64Ctz { .. }
            | Operator::I64DivS { .. }
            | Operator::I64DivU { .. }
            | Operator::I64Eq { .. }
            | Operator::I64Eqz { .. }
            | Operator::I64Extend16S { .. }
            | Operator::I64Extend32S { .. }
            | Operator::I64Extend8S { .. }
            | Operator::I64ExtendI32S { .. }
            | Operator::I64ExtendI32U { .. }
            | Operator::I64GeS { .. }
            | Operator::I64GeU { .. }
            | Operator::I64GtS { .. }
            | Operator::I64GtU { .. }
            | Operator::I64LeS { .. }
            | Operator::I64LeU { .. }
            | Operator::I64Load { .. }
            | Operator::I64Load16S { .. }
            | Operator::I64Load16U { .. }
            | Operator::I64Load32S { .. }
            | Operator::I64Load32U { .. }
            | Operator::I64Load8S { .. }
            | Operator::I64Load8U { .. }
            | Operator::I64LtS { .. }
            | Operator::I64LtU { .. }
            | Operator::I64Mul { .. }
            | Operator::I64Ne { .. }
            | Operator::I64Or { .. }
            | Operator::I64Popcnt { .. }
            | Operator::I64RemS { .. }
            | Operator::I64RemU { .. }
            | Operator::I64Rotl { .. }
            | Operator::I64Rotr { .. }
            | Operator::I64Shl { .. }
            | Operator::I64ShrS { .. }
            | Operator::I64ShrU { .. }
            | Operator::I64Store { .. }
            | Operator::I64Store16 { .. }
            | Operator::I64Store32 { .. }
            | Operator::I64Store8 { .. }
            | Operator::I64Sub { .. }
            | Operator::I64Xor { .. }
            | Operator::If { .. }
            | Operator::LocalGet { .. }
            | Operator::LocalSet { .. }
            | Operator::LocalTee { .. }
            | Operator::Loop { .. }
            | Operator::MemoryGrow { .. }
            | Operator::MemorySize { .. }
            | Operator::Nop { .. }
            | Operator::RefFunc { .. }
            | Operator::RefIsNull { .. }
            | Operator::RefNull { .. }
            | Operator::Rethrow { .. }
            | Operator::Return { .. }
            | Operator::ReturnCall { .. }
            | Operator::ReturnCallIndirect { .. }
            | Operator::Select { .. }
            | Operator::TableGet { .. }
            | Operator::TableGrow { .. }
            | Operator::TableInit { .. }
            | Operator::TableSet { .. }
            | Operator::TableSize { .. }
            | Operator::Throw { .. }
            | Operator::Try { .. }
            | Operator::TypedSelect { .. }
            | Operator::Unreachable { .. }
    )
}
