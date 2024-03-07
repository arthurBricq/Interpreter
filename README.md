# Basic interpreter in Rust

This project is an interpreter for a programming that I created. 

I did this language only to **learn more about programming language theory and intepreters** ! 

It's a C-like


# Shell-like intrepreter

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
    
**Next milestone** : Typing - Support float, integers, bool and string

- [ ] bool
- [ ] integer
- [ ] float
- [ ] string

**Next milestone**: functions and logic

- [ ] loop + break
- [ ] typed functions

**Next milestone**: web-assembly deployment of the shell

