# JBMF
JBMF is a java bytecode manipulation framework written in rust.
It does currently not support (but hopefully will in the future)
- Lifting to jbmf-ir
- Several optimization passes
- idk lol
- Symbolic execution (The main feature of this project)
- Building an ast from an smt statement and compiling it back to bytecode
- (Far from now, but I'll include it anyways) Compiling to llvm-ir (at least some parts (might interop with jni))

This project is designed to be used for deobfuscation of heavily obfuscated binary, by outlining the semantics of certain code parts, simplifying them and recompiling them back

# Building
To build just simply clone the repository
```$ git clone https://github.com/Wazed-Matthis/JBMF.git```

Then execute the build command
```$ cargo build (--release)```

Make sure you have the correct rust version installed for more on that refer to [this](https://rustup.rs/)
