In terms of configuration,

we should probably include a `bevygen.toml`
file in the root of the directory.


## Command ideas
- `bevygen init`
Will create a `bevygen.toml` file in the root directory.

- `bevygen new`
Will scaffold a project in the root directory, like `loco new`



## Path to 0.2
- `bevygen scaffold palettes` -> create palette directory
    - includes `template.js` (maybe `.json`?)
Then refactor such that
- `bevygen init` -> create `bevygen.toml` that will identify the palette directory
Then
- `bevygen gen` -> update palettes in palette directory based on changed template
