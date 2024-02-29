# Basic interpreter in Rust

- Operation on integers: +, -, *, /
- Python-like assignment
- Usage of variables

## Example

```console
> 1 + 2 * 3
7 
> (1 + 2) * 3
9 
> a = 100
100
> 3 * a
300
> b = a - 100
0
> b
0
> a - 10
90
> a    
100
> b = a - 10
90
> vars
{"a": 100, "b": 90}
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

- new operators: 
    - ** for power
    - % for modulo
    
**Next milestone** : Support float, integers, bool and string

- [ ] integer
- [ ] float
- [ ] bool
- [ ] string

**Next milestone**: functions 

- [x] multi-line parsing: introduce statements
- [x] parse simple functions
- [ ] eval simple functions
- [ ] parse a file
- [ ] parse typed functions

**Next milestone**: web-assembly deployment of the shell

