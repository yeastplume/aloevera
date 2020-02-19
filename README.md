[![Build Status](https://dev.azure.com/yeastplume/aloevera/_apis/build/status/yeastplume.aloevera?branchName=master)](https://dev.azure.com/yeastplume/aloevera/_build/latest?definitionId=3&branchName=master)
[![Release Version](https://img.shields.io/github/release/yeastplume/aloevera.svg)](https://github.com/yeastplume/aloevera/releases)
[![License](https://img.shields.io/github/license/yeastplume/aloevera.svg)](https://github.com/yeastplume/aloevera/blob/master/LICENSE)

# Aloevera

Aloevera is a command-line tool that facilitates the development of graphical assets for use with the [Commander X16](https://commander-cx16.fandom.com/wiki/Commander_X16_Wiki).

In a nutshell, Aloevera transforms images created with modern editors into Assembly (or BASIC) statements that can be directly imported into your X16 development project. It aims to be a simple, easily integrated tool that: 

* Outputs resources as easily-imported Assembly (or BASIC DATA) statements.
* Assists your resource creation pipeline by validating your input data and ensuring that your souce images match the format expected by your target VERA layer mode.
* Provides helpful information to help you fix problems when your image data cannot be translated into the format expected by VERA
* Integrates into your preferred build system or development process as a simple command-line tool with minimal overhead.
* Supports all VERA modes and types, including:
    * Text
    * Tilesets
    * Tilemaps
    * Sprites
    * Bitmaps

# Usage Guide and Samples

A fully-detailed guide is provided, with several detailed examples demonstrating how to use Aloevera to output images of all types in all VERA-Compatible formats.

[Aloevera Usage Guide](./docs)

Assembly and BASIC samples are also provided for each example, along with instructions on how to run them:

[Aloevera Samples](./samples)

# Binaries

To use Aloevera, we recommend using the latest release from the [Releases page](https://github.com/yeastplume/aloevera/releases). There are distributions for Linux, MacOS and Windows.

# Building from Source

Aloevera is written entirely in Rust. You can install Rust for development on your system with [the rustup installer](https://rustup.rs/) or your preferred package manager.

Once Rust is installed, there should be no additional build pre-requisites. You should be able to build and run binaries with a simple:

```.sh
cargo run
```

in the project directory.


# Contributing

Aloevera is a project put together over a few elapsed months in a very hobbyist capacity, and I'd expect that most development for the X16 will be purely spare-time stuff. Even so, it's hoped that other members of the X16 community will chip in as and when they can to help improve Aloevera and make it the best tool it can possibly be. Contributions and feedback are welcome and encouraged.

If you're looking for a place to contribute, the [issues](./issues) page should contain quite a few pointers as to where to start. Or if there's a feature you think should be included that isn't, feel free to open an issue or PR yourself.

# Who am I?

I am Yeastplume. My main job is a full-time commitment to the completely free and open [Grin](https://github.com/mimblewimble/grin) cryptocurrency project (which I urge anyone even tangentally interested in cryptocurrencies to take a look at). My hobbies, however, are reflected by the much-higher-than-average number of Commodore computers and related hardware surrounding my workspace.

I'm a big fan of the X16 concept, particularly because I believe there's huge educational value in having a relatively popular 'modern' and available 8-bit machine that a single person can completely understand. I believe that if schools could find a way to incorporate at least some 6502-style coding into their curricula instead of fixating solely on whatever the popular language of the day is, they'd be doing their students a great service. In any case, I look forward to seeing where the X16 project goes, and am happy to contribute whatever spare cycles I can find to the community to help encourage its adoption. 

# License

Aloevera is completely free and open for use by anyone for any purpose under the Apache License v2.0.
