# constant-folder

Simple constant evaluator for a language

**Note:** _It does not currently check if an expression can be evaluated as a constant, as other code is not implemented to do so yet. eg. Function calls_

## Example

```
let a = 2 * 4;
{ let b = a + 3 * 2; }
let c = a + b;
{ let b = c / a; }
```

Will result in a table like so:

```
Scope {
    level: 0,
    items: {
        "a": Number(
            8,
        ),
        "c": Number(
            22,
        ),
    },
},
Scope {
    level: 1,
    items: {
        "b": Number(
            14,
        ),
    },
},
Scope {
    level: 1,
    items: {
        "b": Number(
            2,
        ),
    },
},
```
