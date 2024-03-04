%builtins output
func main(output_ptr: felt*) -> (output_ptr: felt*) {
    alloc_locals;

    let fibonacci_claim_index = 100;
    assert output_ptr[0] = fibonacci_claim_index;
    let res = fib(1, 1, fibonacci_claim_index);
    assert output_ptr[1] = res;

    // Return the updated output_ptr.
    return (output_ptr=&output_ptr[2]);
}

func fib(first_element: felt, second_element: felt, n: felt) -> felt {
    if (n == 0) {
        return second_element;
    }

    return fib(
        first_element=second_element, second_element=first_element + second_element, n=n - 1
    );
}
