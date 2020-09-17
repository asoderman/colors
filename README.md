# Color
Color is a #[no_std] compatible macro used to create terminal strings wrapped with escape codes.

# Usage:
To create a new color type we must pass the macro a new type identifier and a corresponding escape sequence:

`color!(Red, "[31m");`

Once our new color is defined we can wrap existing types in our new color type:

```
let red_string = Red("This is a red string");

println!("{}", red_string); 
```

The standard 8 terminal colors are already defined in colors::ansi.

# TODO:
* Individual field wrapping within types
