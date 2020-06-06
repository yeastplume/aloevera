# Aloevera Samples

The samples in this directory complement the examples found in the [Aloevera Usage Guide](../docs). Most samples contain both Assembly and BASIC examples, as well as a Makefile to assemble them.

## Prerequisites

`Make` must be installed and in your system path.

For the time being, sample assembly code is only provided for ca65, the assembler tool of the [cc65 project](https://github.com/cc65/). If you're interested in running the assembly samples, you'll therefore need to have the tools `ca65` and `ld65` on your system path. Both of these tools are provided as part of the `cc65` suite.

You'll also need the [x16 Emulator](https://github.com/commanderx16/x16-emulator/releases) and [ROM](https://github.com/commanderx16/x16-rom) available on your path. At the time of this writing, the target release for both is R37.

And of course, you'll need the Aloevera executable somewhere on your machine, either built locally or from a release.

## Configuring the samples

You'll need to configure a few paths in [Makefile.vars](Makefile.vars) as follows:

* set `ALOEVERA` to the built Aloevera executable (or you can just put Aloevera on your path and use `aloevera[.exe]` here)
* set `X16_EMU` to the x16 emulator executable
* set `X16_ROM` to the x16 rom

Makefile.vars provides a few other common flags you can tweak to your liking. 

## Running the samples

To run a sample, ensure you're in the desired sample directory and run (for example):

```.sh
[samples/palettes]$ make run_asm
```

To build and run the assembly version of the sample, and

```.sh
[samples/palettes]$ make run_bas
```

To build and run the BASIC version.

Clean the output with 

```.sh
make clean
```

Which you may need to do to get changes picked up every time you modify the Makefile directly. Changes to the code or resource files should be picked up automatically.

These examples are meant to be tweaked and enhanced for your understanding pleasure, so feel free to play with them and modify them as you see fit!









