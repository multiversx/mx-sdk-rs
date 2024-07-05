# Parametric test example for: Multisig

This is an exampel on how to write a basic parametric test contract.

To run fuzzing on it using Kasmer, buld the contracts and simply run

```
kasmer fuzz
```


To run symbolic execution:

```
kasmer build
kasmer verify test_change_quorum --booster
```
