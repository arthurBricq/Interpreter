# Basic interpreter in Rust

## Format 

### Operations on integers

```
1 + 1
1 - 1
1 * 1
1 / 1
-2
```

### Group operations

```
(1 + 2) * 2
```

### Python-like Assignment

```
a = 1;
(1 + a) * 2
```

## Error handling

This is an example of how errors are handled.

```console
  > a
  UnknownVariable("a")
  > 1 + 2 + a
  UnknownVariable("a")
  > a + b
  MultipleError([UnknownVariable("a"), UnknownVariable("b")])
  > a + b + c
  MultipleError([UnknownVariable("a"), MultipleError([UnknownVariable("b"), UnknownVariable("c")])])  
```

# RoadMap

**Next milestone** : a calculator

- [ ] new operators: 
    - [ ] ** for power
    - [ ] MOD
    
**Next milestone** : Support float and integers

**Next milestone** : a file reader with indentation and scope

**Next milestone**: functions 

**Next milestone**: web-assembly deployment

