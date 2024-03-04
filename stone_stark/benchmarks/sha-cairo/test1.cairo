%builtins output

from starkware.cairo.common.cairo_builtins import HashBuiltin, BitwiseBuiltin
from starkware.cairo.common.registers import get_label_location
from starkware.cairo.common.invoke import invoke
from starkware.cairo.common.alloc import alloc
from sha256 import sha256, finalize_sha256


func main{bitwise_ptr: BitwiseBuiltin*, range_check_ptr}(output_ptr: felt) -> (output_ptr: felt) {
    alloc_locals;

    // Load fibonacci_claim_index and copy it to the output segment.
    local sha256_bytes;
    %{ ids.sha256_bytes = program_input['sha256_bytes'] %}

    let res = test_sha256_hello_world(sha256_bytes);

    // Return the updated output_ptr.
    return (output_ptr=output_ptr);
}

func test_sha256_hello_world{bitwise_ptr: BitwiseBuiltin*, range_check_ptr}(hello_world_data: felt*) -> (hash: felt*) {
    alloc_locals;

    let (local sha256_ptr: felt*) = alloc();
    let sha256_ptr_start = sha256_ptr;
    let (hash) = sha256{sha256_ptr=sha256_ptr}(hello_world_data, 11);
    finalize_sha256(sha256_ptr_start=sha256_ptr_start, sha256_ptr_end=sha256_ptr);

    assert hash[0] = 3108841401;
    assert hash[1] = 2471312904;
    assert hash[2] = 2771276503;
    assert hash[3] = 3665669114;
    assert hash[4] = 3297046499;
    assert hash[5] = 2052292846;
    assert hash[6] = 2424895404;
    assert hash[7] = 3807366633;

    return (hash=sha256_ptr_start);
}
