%builtins output pedersen range_check bitwise

from starkware.cairo.common.cairo_builtins import HashBuiltin, BitwiseBuiltin
from starkware.cairo.common.registers import get_label_location
from starkware.cairo.common.invoke import invoke
from starkware.cairo.common.alloc import alloc

from sha256 import sha256, finalize_sha256

func hash_eight_bytes{bitwise_ptr: BitwiseBuiltin*, range_check_ptr}() -> felt* {
    alloc_locals;

    let (local sha256_ptr: felt*) = alloc();
    let sha256_ptr_start = sha256_ptr;
    let (hash) = sha256{sha256_ptr=sha256_ptr}(new ('abc\x00'), 3);
    // Disable verification
    //finalize_sha256(sha256_ptr_start=sha256_ptr_start, sha256_ptr_end=sha256_ptr);

    return hash;
}

func main(
    output_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr, bitwise_ptr: BitwiseBuiltin*) -> (
           output_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr: felt, bitwise_ptr: BitwiseBuiltin*) {

    alloc_locals;

    let res = hash_eight_bytes{bitwise_ptr=bitwise_ptr, range_check_ptr=range_check_ptr}();
    assert output_ptr[0] = res[0];
    assert output_ptr[1] = res[1];
    assert output_ptr[2] = res[2];
    assert output_ptr[3] = res[3];
    assert output_ptr[4] = res[4];
    assert output_ptr[5] = res[5];
    assert output_ptr[6] = res[6];
    assert output_ptr[7] = res[7];

    // Return the updated output_ptr.
    return (
        output_ptr=&output_ptr[8], pedersen_ptr=pedersen_ptr, range_check_ptr=range_check_ptr, bitwise_ptr=bitwise_ptr
    );
}