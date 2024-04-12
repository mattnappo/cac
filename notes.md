# IMPLEMENTING CONTENT-ADDRESSED CODE FOR C

# Maps

Big idea with these maps: the state of your code text file has nothing to do with the state of the codebase data structure. You can have functions in/missing from your text file regardless of whether the functions are in/missing the codebase (stuff you commit).

# Map 1

Map 1: Names --> AST fragment hashes
* key: a function name
* value: a hash of the AST representation of a function

## Notes

1. This map is a cache that is never invalidated. That is, entries are never removed (unless the user explicitly says so).

2. Two funcs/types with the same name cannot exist. That is, you can't have

```
X = abc
X = def
```

in your code

IDEA: The value should actually have some metadata in it, like the function name. Issue with this: refactoring would have to go into the values, not just the keys. So, TODO: look into Unison's internal codebase data structures, and see how Unison does it.

# Map 2

Map 2: AST fragment hashes --> AST fragments
* key: The hash of the AST representation of a function
* value: The AST representation of that function

ISSUE: how to handle dependencies in the value of this map? Meaning, when is just the AST repr of a function NOT enough to execute that function?

Potential Soln: Do I need to annotate the AST to maintain a list of dependencies?

Answer: yeah, you keep a dependence graph -- good thinking matt!
But I'd store that as a separate structure...

# Operations

1. User writes an entirely new function

Make a new entry in the hashmap.

The name of the function

2. User deletes a function

3. User modifies the contents of an existing function

4. User modifies the name of an existing function

5. User modifies the contents AND name of an existing function

# Custom Compiler

I need a way to serialize everything -- types and functions.

This is NOT the same as the regular C AST... references to functions need to be replaced with their hash (impl idea: name mangling?) and types need to be replaced with their hash.

What about pointers?

# Important Considerations

## External Libraries
I'm going to need to index this / populate the massive code database with all the standard library stuff.

For example, if a function uses a standard library function, we need to hunt down the code of that function (and its dependencies recursively)

## Recursion

What if function a calls function b, and function b calls function a?

## User-Defined Types

This discussion so far has been only about functions. But I also want to extend this idea to user-defined typedefs, structs/unions, and enums. Is it even possible to implement this for functions without implementing this for types?

## Constants and Globals

Similar to above ("user defined types"), do we also need to implement this idea for global constants / globals? Say a function depends on a global... how do we handle that?

## Macros

Same thing as above with macros -- `#define MACRO`, but even worse. This is because C macros are textual/preprocessor. So you can't "link" a macro like you can link an object file: the actual text `"#define MACRO"` needs to be in the file, and even worse, with the right visibility.

Solution: wait, is this even an issue? I should just run the preprocessor first, then do all the Unison indexing stuff! So it actually shouldn't be an issue!

## Missing (Deleted) Function Dependences

Say function A calls function B, and you store both these functions in the "codebase". In Unison, deleting function B in your code, doesn't do anything. It doesn't "commit" that delete to the codebase. Therefore, function A can still run just fine, with no issues.

You can only delete a function F by manually saying "delete F" in `ucm`. But, the delete will only be successful when all dependences of F (all functions that call F) are also deleted.

### This idea in C

Same setup as above: Func A calls Func B, and you delete func B (in your code textfile). The deleted function (B) still needs to be linked in the final compiled executable... or else you get linker errors like "cannot find function B". 

So, when building an executable, look at the main function, find all its dependences, and for each dependence, compile it into its own object file. Then link all the obj files together.

But what if there is no main function? What if we want to compile a library, or into a .so perhaps? To be considered further....

### Modifying Dependences

If A calls B, and you modify B, but you don't modify A, which version of B will A use?

This relates to the overall question of which version of a function does the caller use? The most-recently-committed version? Or the most recent version at the time of the caller's indexing? (surely the latter...). My thinking of why the latter is because the hash of the dependence is serialized into the caller's AST, which is in turn serialized. This is because `lookup()` happens at compile time, not run time (right? I am assuming this because I can't see it working efficiently any other way...).

ANSWER: (see discord thread) Hashes within an AST are actually __references__ (kind of). Unison (UCM) maintains a dependence graph, and what really happens when you run an `update B` is that UCM is smart enough to go and automatically update all the references from `B#old` to `B#new` in anything that depends on B. So,
```
upon update B:
recompute the hash of B
for all d which depend on B:
    update the hash of B in d's AST
```
Key idea: but now, the hash of A has changed, since we changed one of the hashes within A's AST (to be the new version/hash of B). So, we then go ahead and repeat this process for A... and all the things it depends on, recursively, so on and so forth.

### Does lookup happen at compile or run time?
If it happened at run time: could be very slow to "fetch" the dependence if it doesn't have it already.

### FFI

Oh boy.. I don't know...

# Unison First Impressions

Upon installation, Unison created a dir in my home dir to "store the codebase". This is where the append-only cache is stored, as a massive sqlite DB.

When initializing a new project, Unison "fetched" the standard library, and I assume it "stored" it in the codebase. Because in order for Unison to work, EVERYTHINGGG (including the standard library and all dependencies) must be indexed in the codebase, just like it were any other user-supplied code.

I have to manually say "add" to commit new files to this local repository

# Resources
- https://www.unison-lang.org/docs/the-big-idea
- https://www.unison.cloud/our-approach/
- https://www.unison-lang.org/articles/distributed-datasets/
- https://www.youtube.com/watch?v=gCWtkvDQ2ZI
