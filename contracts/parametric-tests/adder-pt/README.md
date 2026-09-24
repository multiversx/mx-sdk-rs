# Parametric test example for: Adder

This is an example on how to write a basic parametric test contract.

To run fuzzing on it using Kasmer, build the contracts and simply run

```
kasmer fuzz
```


To run symbolic execution:

```
kasmer build
kasmer verify test_call_add --booster
## and / or
kasmer verify test_call_add_twice --booster
```
