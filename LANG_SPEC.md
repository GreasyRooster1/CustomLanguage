
```
name str:"hi" 
x i:20
y f:3.14


fn do_stuff_param_ret(x:i, y:f):f{
    loop{

    }
    loop {

    }
    loop i 0->10{

    }
    loop j 0->10 by 2{

    }

    if x>10 {
        ret 8;
    }
    ret 10;
}

fn do_again(x:i, y:f):f{
  ret 10;
}

```

## Types
- f - 32-bit float
- i - 32-bit signed int

## Interfaces
interfaces allow for turing control flow execution into sys calls
they are the basis of a standard library
