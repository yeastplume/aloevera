# Using Aloevera

The advent of the [Commander X16](http://commanderx16.com/X16/Ready.html) is an exciting development for those of us who enjoy poking around with a bit of retro 8-bit coding. There's something very appealing about an opportunity to go back in time just a little bit to play around with a modern 6502-based memory-mapped machine, and particularly one with a budding community around it. 

There are, however, many aspects of the old days that we no longer need to deal with in the course of our retro development. Most modern C64 development, for example, will employ cross-compilers to produce machine language code in an environment far more useful for the task than the C64's screen editor. And you don't need to feel like you're cheating by using these tools; many titles of the era, (Maniac Mansion comes to mind) were written entirely using cross-compilers on UNIX systems.

Those who are old enough might remember the process of drawing their sprites and tiles onto sheets by hand and manually translating the results into hex values. Fortunately, we're also no longer stuck with 80s tooling when it comes to graphical asset development; there is a wealth of modern image editors and tileset manipulation programs we can use for X16 development. Even better, the pixel-art renaissance of the past 10 or so years has given us the added bonus that many of these tools are very specialised for the kind of pixel-art the X16 uses.

However, we still need to ensure our image data is created with VERA's more-modest-than-modern constraints in mind, and we need a method of quickly and easily transforming these images into the byte-by-byte assembly statements needed by our programs. This can potentially be quite challenging, particularly if we're working on a large, multi-person project or on one in which graphical changes can go through many revisions. Most projects will need to start by putting together a pipeline to deal with all these assets and their transformations and manage the change process in a sane manner.

Aloevera is intended to play a major role in your graphically-intense X16 project. Targeted specifically for VERA on the X16, it is a command-line tool that assembles your images into data formats ready for direct inclusion in your Assembly (or even BASIC) projects. 

In addition to this, it is also intended as a helper to ensure your image data is set up according to the needs of your particular application's targets. It is aware of all of VERA's constraints, layer display modes and pixel depths, and will ensure that the input data you provide to it is conformant.

In a nutshell, Aloevera:

* **Transforms** the output of modern image editors into formats that can be included and used in your X16 projects.
* **Validates** your source image data, ensuring it's been set up in a manner that can be properly translated into your target VERA display modes/and pixel depth (and informs you what the issue is when your data is incorrect)
* **Integrates** into your project's build workflow as a simple command-line tool. If your build is set up correctly, you should be able to make changes directly in your image editor and have them show up instantly on the next run of your program.

The examples in this guide provide a detailed overview of how to integrate Aloevera into your project, and outline all of the considerations that need to be taken into account when putting together your source image data. Each example is accompanied by sample assembly and BASIC code that uses the produced data, as well as a small Makefile suggesting how Aloevera might be included in a larger project.

## Getting Help

Help is available for all commands with the `--help` command. For example:

```.sh
aloevera palette --help
```

Will show the available `palette` commands while:

```.sh
aloevera palette import --help
```

Will show commands and switches for the `palette import` command.

## Detailed Examples


### Project Files

All Aloevera operations are performed on project files, which persist the state of all assets and transformations between command-line invocations. Most operations require the name of a project file in order to run, which is provided with the global `-p` flag.

To create a new project file, use the `create` command as in the example below:

```.sh
aloevera create project my_project.av
```

This will create a new project file called `my_project.av` in the current directory. Feel free to look through the contents of the file, but remember its contents are not meant to be edited directly.

## The Examples

All Aloevera concepts and commands are explored and illustrated via a series of detailed example. [Samples](../samples) are also provided for each example, which contain assembly and basic code as well as a small makefile illustrating how one could potentially use Aloevera in a larger project. Although most of the examples are intended as learning exercises that include data directly within their code, the [final sample](ex_007.md) discusses .BIN output and workflows in which resources are loaded separately.


### [Example 1: Intro to Aloevera - Importing a palette](./ex_001.md)
### [Example 2: Intro to Imagesets - Replacing the default font](./ex_002.md)
### [Example 3: Simple Tilemaps - Displaying a custom banner](./ex_003.md)
### [Example 4: Higher Depth Tilemaps - A more game-like tilemap](./ex_004.md)
### [Example 5: Sprites](./ex_005.md)
### [Example 6: Bitmaps](./ex_006.md)
### [Example 7: .BIN Files, CC65 and Manual Layouts](./ex_007.md)
