%builtins output pedersen range_check bitwise

from starkware.cairo.common.cairo_builtins import HashBuiltin, BitwiseBuiltin
from starkware.cairo.common.registers import get_label_location
from starkware.cairo.common.invoke import invoke
from starkware.cairo.common.alloc import alloc

from sha256 import sha256, finalize_sha256

func hash_hundred_bytes{bitwise_ptr: BitwiseBuiltin*, range_check_ptr}() -> felt* {
    alloc_locals;

    let (input) = alloc();
    assert input[0] = 'abcd';
    assert input[1] = 'efgh';
    assert input[2] = 'bcde';
    assert input[3] = 'fghi';
    assert input[4] = 'cdef';
    assert input[5] = 'ghij';
    assert input[6] = 'defg';
    assert input[7] = 'hijk';
    assert input[8] = 'efgh';
    assert input[9] = 'ijkl';
    assert input[10] = 'fghi';
    assert input[11] = 'jklm';
    assert input[12] = 'ghij';
    assert input[13] = 'klmn';
    assert input[14] = 'hijk';
    assert input[15] = 'lmno';
    assert input[16] = 'ijkl';
    assert input[17] = 'mnop';
    assert input[18] = 'jklm';
    assert input[19] = 'nopq';
    assert input[20] = 'klmn';
    assert input[21] = 'opqr';
    assert input[22] = 'lmno';
    assert input[23] = 'pqrs';
    assert input[24] = 'mnop';

    let (local sha256_ptr: felt*) = alloc();
    let sha256_ptr_start = sha256_ptr;
    let (hash) = sha256{sha256_ptr=sha256_ptr}(input, 100);
    // Disable Verification
    //finalize_sha256(sha256_ptr_start=sha256_ptr_start, sha256_ptr_end=sha256_ptr);

    return hash;
}

func main(
    output_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr, bitwise_ptr: BitwiseBuiltin*) -> (
           output_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr: felt, bitwise_ptr: BitwiseBuiltin*) {

    alloc_locals;

    let res = hash_hundred_bytes{bitwise_ptr=bitwise_ptr, range_check_ptr=range_check_ptr}();
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
