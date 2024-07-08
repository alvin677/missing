# missing
A package manager for Windows, written in Rust. Useful when tools are *missing*.

## not yet finished gg
example: `missing python`, tool checks if package is installed. if it is, ask to update if there's an available update.
if it's not installed, install the installer or whatever and add to PATH if necessary.

a batch file could be installed (or created from data of an installed json file containing metadata) whenever trying to get a package. the script can automatically install everything.
this would make it easier for packages to be added to the package repository without possibly having to touch the rs file.

packages that need to be added (cause some suck to manually install):

`libclang` `nasm` `cmake`

`python` `node` `rust`

`nvidia`?
