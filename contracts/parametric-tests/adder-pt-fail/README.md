# Parametric test example: failing tests for Adder

A companion to [adder-pt](../adder-pt). The tests in this contract are **expected to fail**.
They check the tooling rather than the Adder contract: a fuzzer or prover that reports them as
passing is broken.

| Test | Expected result |
|---|---|
| `test_control_passes` | Passes. Same property as `test_call_add` in `adder-pt`; shows the setup is sound |
| `test_wrong_sum` | Fails for every input. Fuzzing and symbolic execution must both report it |
| `test_rare_failure` | Fails for exactly one input (`value == 123456784`). Fuzzing will almost certainly miss it; symbolic execution must report it |
