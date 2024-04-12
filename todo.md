# Implementation Tasks

Parsing
- [ ] Extract functions
- [ ] Extract structs
- [ ] Extract enums
- [ ] Extract unions

Hashing
- [ ] Hash a function
- [ ] Hash a user-defined type

Modified AST Repr
- [ ] Replace function calls with their hash
- [ ] Replace types with their hash
- [ ] Am I going to need to modify the c parsing crate AST data structure?
    - [ ] A more hacky solution is make a func that maps from names to
          mangled names, which are the names that will actually be used in the
          final code.

Name -> AST Hash Map
- [ ] Get
- [ ] Put
- [ ] Update

AST Hash -> AST Map
- [ ] Get
- [ ] Put
- [ ] Update

Dependence Graph
- [ ] Update trails

Tooling
- [ ] Add
- [ ] Update
- [ ] Delete

Compiling
- [ ] Traverse the main function
- [ ] Pull in dependent funcs' AST's, and all of their deps, and all of their deps...
- [ ] Assemble into object files and rename stuff properly using that mangling
      function described above
