## BindFX

**How to use**

To create your bindings, simply run the executable. It will read the
configuration from the ```config.toml``` file and output the bindings.

**Configuration**

All BindFX configuration is inside a TOML file.
It should be called ```config.toml``` and be in the same directory as the program.

If BindFX can't find a configuration file, it will automatically create one.

Example:
```TOML
[input]
path = "./raylib/win64"
name = "raylib"
wrapper = "wrapper.h"

[output]
path = "./out"
name = "bindings"
```

**Compiling**

As BindFX is made in Rust, compiling is a 3 step process.

1. Download and install [rustup](https://rustup.rs/).
2. Clone the repository using ```git clone https://github.com/WrapFX/bindfx.git```.
3. Run ```cargo build release``` from the command line.