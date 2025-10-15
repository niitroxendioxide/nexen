# Custom Language

Should have these features:

- Rust-like syntax
- Static Typing
- RAII
- Classes
  - Inheritance
  - Operator overload

## Goal language look

```
using std;

enum Types {
    Default = 0,
}

class Foo {
    pub data: [u8],
}

impl Foo {
    pub function new() {
        return Foo {
            data: [0],
        }
    }
}

class Bar : Foo {
    pub name: string,
}

impl Bar {
    pub function new(name: string) {
        return Bar {
            data: [0],
            name: name,
        }
    }
}

module ObjectCreator {
    pub function create_bar(name: string) -> Result<Bar, std::Error> {
        let instance = Bar::new();
        if (name.len() <= 0) {
            return Error("");
        }

        return instance;
    }
}

function main() {
    let instance = ObjectCreator.create_foo();
    if (instance.is_err()) {
        std::println("Error");
    }
}
```