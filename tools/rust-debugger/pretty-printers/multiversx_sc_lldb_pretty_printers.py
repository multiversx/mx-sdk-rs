from functools import partial
from typing import Callable, Collection, Iterable, List, Tuple, Type
from lldb import SBValue, SBDebugger
import lldb
from pathlib import Path
import re
import struct

DEBUG_API_TYPE = "multiversx_sc_scenario::api::impl_vh::vm_hooks_api::VMHooksApi<multiversx_sc_scenario::api::impl_vh::debug_api::DebugApiBackend>"
ANY_NUMBER = "[0-9]+"
ANY_TYPE = ".*"
SOME_OR_NONE = "(Some|None)"

# 1. num_bigint library
NUM_BIG_INT_TYPE = "num_bigint::bigint::BigInt"
NUM_BIG_UINT_TYPE = "num_bigint::biguint::BigUint"

# 2. SC wasm - Managed basic types
MOD_PATH = "multiversx_sc::types::managed::basic"

BIG_INT_TYPE = f"{MOD_PATH}::big_int::BigInt<{DEBUG_API_TYPE}>"
BIG_UINT_TYPE = f"{MOD_PATH}::big_uint::BigUint<{DEBUG_API_TYPE}>"
BIG_FLOAT_TYPE = f"{MOD_PATH}::big_float::BigFloat<{DEBUG_API_TYPE}>"
MANAGED_BUFFER_TYPE = f"{MOD_PATH}::managed_buffer::ManagedBuffer<{DEBUG_API_TYPE}>"

# 3. SC wasm - Managed wrapped types
MOD_PATH = "multiversx_sc::types::managed::wrapped"

TOKEN_IDENTIFIER_TYPE = f"{MOD_PATH}::token_identifier::TokenIdentifier<{DEBUG_API_TYPE}>"
MANAGED_ADDRESS_TYPE = f"{MOD_PATH}::managed_address::ManagedAddress<{DEBUG_API_TYPE}>"
MANAGED_BYTE_ARRAY_TYPE = f"{MOD_PATH}::managed_byte_array::ManagedByteArray<{DEBUG_API_TYPE}, {ANY_NUMBER}>"

# ManagedOption
MANAGED_OPTION_INNER_TYPE_INDEX = 1
MANAGED_OPTION_NONE_HANDLE = 2147483646  # i32::MAX - 1
MANAGED_OPTION_TYPE = f"{MOD_PATH}::managed_option::ManagedOption<{DEBUG_API_TYPE}, {ANY_TYPE}>"

ESDT_TOKEN_PAYMENT_TYPE = f"{MOD_PATH}::esdt_token_payment::EsdtTokenPayment<{DEBUG_API_TYPE}>"
EGLD_OR_ESDT_TOKEN_IDENTIFIER_TYPE = f"{MOD_PATH}::egld_or_esdt_token_identifier::EgldOrEsdtTokenIdentifier<{DEBUG_API_TYPE}>"

# ManagedVec
MANAGED_VEC_INNER_TYPE_INDEX = 1
MANAGED_VEC_TYPE = f"{MOD_PATH}::managed_vec::ManagedVec<{DEBUG_API_TYPE}, {ANY_TYPE}>"

# 4. SC wasm - Managed multi value types

# 5. SC wasm - heap
MOD_PATH = "multiversx_sc::types::heap"

HEAP_ADDRESS_TYPE = f"{MOD_PATH}::h256_address::Address"
BOXED_BYTES_TYPE = f"{MOD_PATH}::boxed_bytes::BoxedBytes"

# 6. MultiversX codec - Multi-types
MOD_PATH = "multiversx_sc_codec::multi_types"

OPTIONAL_VALUE_TYPE = f"{MOD_PATH}::multi_value_optional::OptionalValue<{ANY_TYPE}>::{SOME_OR_NONE}"


class InvalidHandle(Exception):
    def __init__(self, raw_handle: int, map_: lldb.value) -> None:
        map_name = map_.sbvalue.GetName()
        error = f"<invalid handle: raw_handle {raw_handle} not found in {map_name}>"
        super().__init__(error)


def check_invalid_handle(callable: Callable) -> Callable:
    def wrapped(*args) -> str:
        try:
            return callable(*args)
        except InvalidHandle as e:
            return str(e)
    return wrapped


def map_lookup(map_: lldb.value, raw_handle: int) -> lldb.value:
    for key, value in map_.map:
        if key == raw_handle:
            return value
    raise InvalidHandle(raw_handle, map_)


def pick_big_int(managed_types: lldb.value) -> lldb.value:
    return managed_types.big_int_map


def pick_big_float(managed_types: lldb.value) -> lldb.value:
    return managed_types.big_float_map


def pick_managed_buffer(managed_types: lldb.value) -> lldb.value:
    return managed_types.managed_buffer_map


def u64_to_hex(val_u64: int) -> str:
    """
    Format int as a 0-padded 64 bit hex.
    >>> u64_to_hex(1000000)
    '00000000000f4240'
    """
    return f"{val_u64:0>16x}"


def u8_to_hex(val_u8: int) -> str:
    """
    Format int as a 0-padded 8 bit hex.

    >>> u8_to_hex(10)
    '0a'
    """
    return f"{val_u8:0>2x}"


def sb_value_to_hex(valobj_u64: SBValue) -> str:
    return u64_to_hex(valobj_u64.GetValueAsUnsigned())


def hex_to_int(value_hex: str) -> int:
    """
    Converts a hex-encoded value to an int.

    >>> hex_to_int("")
    0

    >>> hex_to_int("000003e8")
    1000
    """
    if len(value_hex) > 0:
        return int(value_hex, 16)
    else:
        return 0


def bytes_to_int(bytes: Collection[int]) -> int:
    bytes_hex = ints_to_hex(bytes)
    return hex_to_int(bytes_hex)


def num_bigint_data_to_int(value_vec_u64: lldb.value) -> int:
    value_hex = ''.join(reversed(list(map(sb_value_to_hex, value_vec_u64.sbvalue))))
    return hex_to_int(value_hex)


def ints_to_hex(ints: Iterable[int]) -> str:
    return ''.join(map(u8_to_hex, ints))


def buffer_to_bytes(buffer: lldb.value) -> List[int]:
    return list(map(int, buffer))


def buffer_to_hex(buffer: lldb.value) -> str:
    ints = buffer_to_bytes(buffer)
    return ints_to_hex(ints)


def bytes_to_handle(bytes: List[int]) -> int:
    """
    Parses a handle as a signed 32-bit int.

    >>> bytes_to_handle([255, 255, 255, 146])
    -110

    >>> bytes_to_handle([255, 255, 255, 144])
    -112
    """
    bytes_hex = ints_to_hex(bytes)
    return struct.unpack('>i', bytearray.fromhex(bytes_hex))[0]


def format_buffer_hex_string(buffer_hex: str) -> str:
    byte_count = len(buffer_hex) // 2
    return f"({byte_count}) 0x{buffer_hex}"


def format_buffer_hex(buffer: lldb.value) -> str:
    buffer_hex = buffer_to_hex(buffer)
    return format_buffer_hex_string(buffer_hex)


def ascii_to_string(buffer_iterator: Iterable[int]) -> str:
    """
    Converts ascii codes to the coresponding string.

    >>> ascii_to_string([116, 101, 115, 116])
    'test'
    """
    return ''.join(map(chr, buffer_iterator))


def buffer_as_string(buffer: lldb.value) -> str:
    buffer_string = ascii_to_string(buffer)
    return f'"{buffer_string}"'


def parse_handles_from_buffer_hex(buffer_hex: str) -> List[int]:
    """
    Parses a list of raw handles (signed 32 bit values) from a hex-encoded buffer.
    >>> parse_handles_from_buffer_hex("ffffff86ffffff83ffffff80")
    [-122, -125, -128]
    """
    raw_handles = []
    for handle_bytes_iter in zip(*[iter(buffer_hex)] * 8):
        handle_bytes_hex = ''.join(handle_bytes_iter)
        raw_handle = struct.unpack('>i', bytearray.fromhex(handle_bytes_hex))[0]
        raw_handles.append(raw_handle)
    return raw_handles


def format_vec(items: Collection[str]) -> str:
    """
    Formats a vec of items.
    >>> format_vec(["foo", "bar"])
    '(2) { [0] = foo, [1] = bar }'
    """
    count = len(items)
    indexed_items = [f"[{index}] = {item}" for index, item in enumerate(items)]
    joined_items = ", ".join(indexed_items)
    return f"({count}) {{ {joined_items} }}"


def sbdata_get_u8_at_index(sbdata: lldb.SBData, err, index: int) -> str:
    return sbdata.GetUnsignedInt8(err, index)


class Handler:
    def __init__(self) -> None:
        self.actual_type = None

    def get_actual_type(self) -> lldb.SBType:
        return self.actual_type

    def summary(self, value: lldb.value) -> str:
        pass


class ManagedType(Handler):
    def map_picker(self) -> Callable:
        return pick_managed_buffer

    def lookup(self, full_value: lldb.value) -> lldb.value:
        return full_value

    def extract_value_from_raw_handle(self, context: lldb.value, raw_handle: int, map_picker: Callable) -> lldb.value:
        managed_types = context.managed_types
        chosen_map = map_picker(managed_types)
        value = map_lookup(chosen_map, raw_handle)
        return value

    @check_invalid_handle
    def summary_from_raw_handle(self, raw_handle: int, context: lldb.value, type_info: lldb.SBType) -> str:
        map_picker = self.map_picker()
        value = self.extract_value_from_raw_handle(context, raw_handle, map_picker)
        return self.value_summary(value, context, type_info)

    def summary(self, original_value: lldb.value) -> str:
        type_info = original_value.sbvalue.GetType()
        managed_value = self.lookup(original_value)
        handle = managed_value.handle
        raw_handle = int(handle.raw_handle)
        context = handle.context
        return self.summary_from_raw_handle(raw_handle, context, type_info)

    def value_summary(self, value: lldb.value, context: lldb.value, type_info: lldb.SBType) -> str:
        pass


class ManagedVecItem(Handler):
    def item_size(self) -> int:
        pass

    def summarize_item(self, bytes: List[int], context: lldb.value, type_info: lldb.SBType) -> str:
        pass


class PlainManagedVecItem(ManagedVecItem, ManagedType):
    """
    ManagedVecItem implementation for any ManagedType which is stored in the ManagedVec only by its handle.
    """

    def item_size(self) -> int:
        return 4

    def summarize_item(self, handle_bytes: List[int], context: lldb.value, type_info: lldb.SBType) -> str:
        raw_handle = bytes_to_handle(handle_bytes)
        return self.summary_from_raw_handle(raw_handle, context, type_info)


class NumBigInt(Handler):
    def summary(self, num_big_int: lldb.value) -> str:
        value_int = num_bigint_data_to_int(num_big_int.data.data)
        if num_big_int.sign.sbvalue.GetValue() == 'num_bigint::bigint::Sign::Minus':
            return str(-value_int)
        return str(value_int)


class NumBigUint(Handler):
    def summary(self, num_big_uint: lldb.value) -> str:
        value_int = num_bigint_data_to_int(num_big_uint.data)
        return str(value_int)


class BigInt(PlainManagedVecItem, ManagedType):
    def map_picker(self) -> Callable:
        return pick_big_int

    def value_summary(self, value: lldb.value, context: lldb.value, type_info: lldb.SBType) -> str:
        return str(value.sbvalue.GetSummary())


class BigFloat(PlainManagedVecItem, ManagedType):
    def map_picker(self) -> Callable:
        return pick_big_float

    def value_summary(self, value: lldb.value, context: lldb.value, type_info: lldb.SBType) -> str:
        return str(value.sbvalue.GetValue())


class ManagedBuffer(PlainManagedVecItem, ManagedType):
    def value_summary(self, buffer: lldb.value, context: lldb.value, type_info: lldb.SBType) -> str:
        return format_buffer_hex(buffer)


class TokenIdentifier(PlainManagedVecItem, ManagedType):
    def lookup(self, token_identifier: lldb.value) -> lldb.value:
        return token_identifier.buffer

    def value_summary(self, buffer: lldb.value, context: lldb.value, type_info: lldb.SBType) -> str:
        return buffer_as_string(buffer)


class ManagedAddress(PlainManagedVecItem, ManagedType):
    def lookup(self, managed_address: lldb.value) -> lldb.value:
        return managed_address.bytes.buffer

    def value_summary(self, buffer: lldb.value, context: lldb.value, type_info: lldb.SBType) -> str:
        return format_buffer_hex(buffer)


class ManagedByteArray(PlainManagedVecItem, ManagedType):
    def lookup(self, managed_byte_array: lldb.value) -> lldb.value:
        return managed_byte_array.buffer

    def value_summary(self, buffer: lldb.value, context: lldb.value, type_info: lldb.SBType) -> str:
        return format_buffer_hex(buffer)


class ManagedOption(PlainManagedVecItem, ManagedType):
    def summary_from_raw_handle(self, raw_handle: int, context: lldb.value, type_info: lldb.SBType) -> str:
        if raw_handle == MANAGED_OPTION_NONE_HANDLE:
            return "ManagedOption::none()"
        inner_type_handler, inner_type = get_inner_type_handler(type_info, MANAGED_OPTION_INNER_TYPE_INDEX)
        assert(isinstance(inner_type_handler, ManagedType))
        inner_summary = inner_type_handler.summary_from_raw_handle(raw_handle, context, inner_type)
        return f"ManagedOption::some({inner_summary})"


def split_bytes(bytes: List[int], sizes: Iterable[int]) -> List[List[int]]:
    """
    Split a byte array into multiple chunks where the length varies.
    >>> split_bytes([1, 2, 3, 4, 5, 6], [2, 3, 1])
    [[1, 2], [3, 4, 5], [6]]
    """
    chunks = []
    i = 0
    for size in sizes:
        chunks.append(bytes[i: i + size])
        i += size
    return chunks


def split_bytes_fixed_size(bytes: List[int], size: int) -> List[List[int]]:
    """
    Split a byte array into fixed-size chunks.
    >>> split_bytes_fixed_size([1, 2, 3, 4, 5, 6, 7, 8, 9], 3)
    [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    """
    chunks = []
    i = 0
    byte_count = len(bytes)
    while i < byte_count:
        chunks.append(bytes[i: i + size])
        i += size
    return chunks


class EsdtTokenPayment(ManagedVecItem, ManagedType):
    COMPONENT_SIZES = [4, 8, 4]

    def summary(self, payment: lldb.value) -> str:
        token_id = payment.token_identifier.sbvalue.GetSummary()
        nonce = int(payment.token_nonce)
        amount = payment.amount.sbvalue.GetSummary()
        return self.to_string(token_id, nonce, amount)

    def item_size(self) -> int:
        return sum(self.COMPONENT_SIZES)

    def summarize_item(self, bytes: List[int], context: lldb.value, type_info: lldb.SBType) -> str:
        token_id_handle_bytes, nonce_bytes, amount_handle_bytes = split_bytes(bytes, self.COMPONENT_SIZES)
        token_id = TokenIdentifier().summarize_item(token_id_handle_bytes, context, None)
        nonce = bytes_to_int(nonce_bytes)
        amount = BigInt().summarize_item(amount_handle_bytes, context, None)
        return self.to_string(token_id, nonce, amount)

    def to_string(self, token_id: str, nonce: int, amount: str) -> str:
        return f"{{ token_identifier: {token_id}, nonce: {nonce}, amount: {amount} }}"


class EgldOrEsdtTokenIdentifier(PlainManagedVecItem, ManagedType):
    def lookup(self, egld_or_esdt_token_identifier: lldb.value) -> lldb.value:
        return egld_or_esdt_token_identifier.data

    @check_invalid_handle
    def summary_from_raw_handle(self, raw_handle: int, context: lldb.value, type_info: lldb.SBType) -> str:
        if raw_handle == MANAGED_OPTION_NONE_HANDLE:
            return "EgldOrEsdtTokenIdentifier::egld()"
        token_summary = TokenIdentifier().summary_from_raw_handle(raw_handle, context, None)
        return f"EgldOrEsdtTokenIdentifier::esdt({token_summary})"


class ManagedVec(PlainManagedVecItem, ManagedType):
    def lookup(self, managed_vec: lldb.value) -> lldb.value:
        return managed_vec.buffer

    def value_summary(self, value: lldb.value, context: lldb.value, type_info: lldb.SBType) -> str:
        item_handler, inner_type = get_inner_type_handler(type_info, MANAGED_VEC_INNER_TYPE_INDEX)
        assert(isinstance(item_handler, ManagedVecItem))
        buffer_bytes = buffer_to_bytes(value)
        item_size = item_handler.item_size()
        bytes_of_all_items = split_bytes_fixed_size(buffer_bytes, item_size)
        items = [item_handler.summarize_item(bytes, context, inner_type) for bytes in bytes_of_all_items]
        return format_vec(items)


class HeapAddress(Handler):
    def summary(self, heap_address: lldb.value) -> str:
        buffer = lldb.value(heap_address.sbvalue.GetChildAtIndex(0).GetChildAtIndex(0))
        return format_buffer_hex(buffer)


class BoxedBytes(Handler):
    def summary(self, boxed_bytes: lldb.value) -> str:
        box = lldb.value(boxed_bytes.sbvalue.GetChildAtIndex(0))
        length = int(box.length)
        data = box.data_ptr.sbvalue.GetPointeeData(0, length)
        raw = lldb.SBData.read_data_helper(data, sbdata_get_u8_at_index, 1).all()
        buffer_hex = ints_to_hex(raw)
        return format_buffer_hex_string(buffer_hex)


class OptionalValue(Handler):
    def summary(self, optional_value: lldb.value) -> str:
        if optional_value.sbvalue.GetType().GetName().endswith('::Some'):
            summary = optional_value.sbvalue.GetChildAtIndex(0).GetSummary()
            return f"OptionalValue::Some({summary})"
        return "OptionalValue::None"


ELROND_WASM_TYPE_HANDLERS = [
    # 1. num_bigint library
    (NUM_BIG_INT_TYPE, NumBigInt),
    (NUM_BIG_UINT_TYPE, NumBigUint),
    # 2. SC wasm - Managed basic types
    (BIG_INT_TYPE, BigInt),
    (BIG_UINT_TYPE, BigInt),
    (BIG_FLOAT_TYPE, BigFloat),
    (MANAGED_BUFFER_TYPE, ManagedBuffer),
    # 3. SC wasm - Managed wrapped types
    (TOKEN_IDENTIFIER_TYPE, TokenIdentifier),
    (MANAGED_ADDRESS_TYPE, ManagedAddress),
    (MANAGED_BYTE_ARRAY_TYPE, ManagedByteArray),
    (MANAGED_OPTION_TYPE, ManagedOption),
    (ESDT_TOKEN_PAYMENT_TYPE, EsdtTokenPayment),
    (EGLD_OR_ESDT_TOKEN_IDENTIFIER_TYPE, EgldOrEsdtTokenIdentifier),
    (MANAGED_VEC_TYPE, ManagedVec),
    # 4. SC wasm - Managed multi value types
    # 5. SC wasm - heap
    (HEAP_ADDRESS_TYPE, HeapAddress),
    (BOXED_BYTES_TYPE, BoxedBytes),
    # 6. MultiversX codec - Multi-types
    (OPTIONAL_VALUE_TYPE, OptionalValue),
]


class UnknownType(Exception):
    def __init__(self, type_name: str) -> None:
        super().__init__(f'unknown type: {type_name}')


def get_inner_type_handler(type_info: lldb.SBType, inner_type_index: int) -> Tuple[Handler, lldb.SBType]:
    inner_type = type_info.GetTemplateArgumentType(inner_type_index).GetCanonicalType()
    handler = get_handler(inner_type.GetName())
    return handler, inner_type


def get_handler(type_name: str) -> Handler:
    for rust_type, handler_class in ELROND_WASM_TYPE_HANDLERS:
        if re.fullmatch(rust_type, type_name) is not None:
            return handler_class()
    raise UnknownType(type_name)


def summarize_handler(handler_type: Type[Handler], valobj: SBValue, dictionary) -> str:
    handler: Handler = handler_type()
    value = lldb.value(valobj)
    return handler.summary(value)


def __lldb_init_module(debugger: SBDebugger, dict):
    python_module_name = Path(__file__).with_suffix('').name

    for rust_type, handler_class in ELROND_WASM_TYPE_HANDLERS:
        # Add summary binding
        summary_function_name = f"handle{handler_class.__name__}"
        globals()[summary_function_name] = partial(summarize_handler, handler_class)

        summary_command = f'type summary add -x "^{rust_type}$" -F {python_module_name}.{summary_function_name} --category multiversx-sc'
        debugger.HandleCommand(summary_command)
        # print(f"Registered: {summary_command}")

    # Enable categories
    debugger.HandleCommand('type category enable multiversx-sc')
