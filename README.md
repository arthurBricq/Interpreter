# Interpreter of ABr language

This project is an interpreter for a programming language that I created: the **ABr** language

ABR = **A**rthur **Br**icq

A programming language parser and its interpreter that I made to learn more about **parsing**, **programming language theory** and how **interpreters** work. 

## Features of the language

The language is a sort of mix between C-syntax (with the concept of *expressions* and *statements*) that is very similar to Python (*variables are typed only at runtime*)

The features of languages are the following.

- C-like differentiation of expressions and statements
- Python-like typed variables: `a = 1`
  - Currently supported types: `bool`, `int`, `list`
- Python-like functions: `fn foo(first_arg, second_arg)`
- Python-like list: `my_list = [1,2,3]`
  - access: `my_list[0]`
  - mutation: `new_list = my_list + [4]`
- If, Else-If, Else: `if (false) {foo()} else {bar()}`
- All common math operation supported and can be used in a shell.

I surely agree that the syntax of this language is weird. This is mostly because I had no clear vision of what I would do, when I would stop, and what would be more difficult. Really, this is a learning project.

## Example

### Fibonacci with recursive function

The famous Fibonacci example,  with a recursive call.

```c
fn fib(n) {
  if (n == 0) {return 0;}
  if (n == 1) {return 1;}
  return fib(n-1) + fib(n-);
}
```

## Missing features

These features are missing for ABr to be 'ready'

- || and &&
- String
- Char
- Comments
- Some built-in functions: len(), print(), etc

# Shell-like interpreter

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
