# Flerry

A simple programming language meant to be the playground for rosewud's development. rosewud is another programming language, but I have suspended development on it.

## Example

```
// Main.flerry

func Hello() =
    print("Hello, World")
end

func Add(x, y) =
    return x + y
end

func Main() =
    Hello()
    sum = Add(2, 4) // 6
end
```
