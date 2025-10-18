# Custom Language



## Goal language look

```ts
using std;

enum FooType {
  Default,
}

class Foo {
  data: [float],
  type: FooType,
}

class Bar : Foo {
  public name: string,

  static method new(p_name) {
    return Bar {
        data: [0],
        type: FooType.Default,
        name: p_name,
    }
  }
  
  method do_stuff() {
    std::println("Doing stuff");
  }
}

module ObjectCreator {
  function create_bar(name) -> Result<Bar, Error> {
      if (name.len() <= 0) {
          return Error("");
      }

      let instance = Bar::new(name);
      return instance;
  }
}

function main() {
  let instance = ObjectCreator.create_foo("niitroxen");
  if (instance.is_err()) {
      std::println("Error");
  }
}
```

