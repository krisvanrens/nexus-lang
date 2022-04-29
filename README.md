# nexus-lang

The Nexus programming language.

## Features

* ...


# Examples

## Example 1

```
// Comment.
func free(a, b, c) {
  return 42;
}

func main() {
  func local() {
    return "Hello".length();
  }

  free(1, 2, local());

  return func { return 17; }()
}
```

## Example 2

```
// Comment.
func free(a, b, c) {
  ret 42;
}

func main() {
  func local() {
    ret "Hello".length();
  }

  free(1, 2, local());

  ret func { ret 17; }()
}
```
