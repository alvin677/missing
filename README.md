# missing
A (terminal) package manager for Windows, written in Rust. Useful when tools are *missing*.<br/>
*I am aware this software is absolute crap, but I did it to learn Rust better.*<br/>
![image](https://github.com/alvin677/missing/assets/112005397/206a5346-23bd-4438-ba71-ff79c2a48faa)


**Usage**<br/>
First off, add the executable to **PATH**:
```bat
set PATH=%PATH%;C:\your\path\here\
```

Then to use the package manager:

`./missing` - will list all available packages<br/>
`./missing <name>` - will download the package<br/>

<br/><br/>

**packages that need to be added (cause some suck to manually install)**

`libclang`
`rust`
`nvidia`?

gotta uh, download files/file and store if it's not an exe or msi file<br/>
also maybe download files to current path where you're executing from
