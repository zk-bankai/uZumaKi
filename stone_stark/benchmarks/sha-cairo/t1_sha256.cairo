%builtins output pedersen range_check bitwise

from starkware.cairo.common.cairo_builtins import BitwiseBuiltin

func test_sha256_hello_world{bitwise_ptr: BitwiseBuiltin*, range_check_ptr}() -> felt {
    alloc_locals;

    let (hello_world) = alloc();
    assert hello_world[0] = 'hell';
    assert hello_world[1] = 'o wo';
    assert hello_world[2] = 'rld\x00';

    let (local sha256_ptr: felt*) = alloc();
    let sha256_ptr_start = sha256_ptr;
    let (hash) = sha256{sha256_ptr=sha256_ptr}(hello_world, 11);
    finalize_sha256(sha256_ptr_start=sha256_ptr_start, sha256_ptr_end=sha256_ptr);

    assert hash[0] = 3108841401;

    return hash;
}

func main(
    output_ptr: felt*, pedersen_ptr: felt*, range_check_ptr: felt*, bitwise_ptr: felt*) -> (
           output_ptr: felt*, pedersen_ptr: felt*, range_check_ptr: felt*, bitwise_ptr: felt*
        ) {
    alloc_locals;

    let hash_ptr = test_sha256_hello_world();
    assert output_ptr[0] = hash_ptr;
    assert output_ptr[1] = hash_ptr + 1;
    assert output_ptr[2] = hash_ptr + 2;
    assert output_ptr[3] = hash_ptr + 3;
    assert output_ptr[4] = hash_ptr + 4;
    assert output_ptr[5] = hash_ptr + 5;
    assert output_ptr[6] = hash_ptr + 6;
    assert output_ptr[7] = hash_ptr + 7;

    return (
        output_ptr=&output_ptr[8], pedersen_ptr=pedersen_ptr, range_check_ptr=range_check_ptr, bitwise_ptr=bitwise_ptr
    );
}
