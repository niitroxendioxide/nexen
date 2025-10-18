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
    Default,
}

struct Foo {
    data: [u8],
}

struct Bar : Foo {
    pub name: string,
    
    pub function new(p_name) {
        return Bar {
            data: [0],
            name: p_name,
        }
    }
}

module ObjectCreator {
    pub function create_bar(name) -> Result<Bar, std::Error> {
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