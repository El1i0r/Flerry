# Flerry

A powerful yet minimal systems programming language utilizing dependent types, contracts, and a strong builtin proofing system.The minimal aspect stems from the fact that types, structs, functions and contracts are the 4 abstractions of the language. With contracts being a way to rule some type, to restrict a type, to proof some operation, or to do all (define rules, define proofs, and all that).

## Example

```
// Main.flerry
import IO

func hello() =
    println("Hello, World")
end

func add(x, y) =
    return x + y
end

func main() =
    hello()
    sum = add(2, 4) // 6
end
```
