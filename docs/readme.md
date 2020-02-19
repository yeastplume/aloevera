# Using Aloevera

The advent of the [Commander X16](http://commanderx16.com/X16/Ready.html) is an exciting development for those of us who enjoy poking around with a bit of retro 8-bit coding. And needless to say, just as in the old days a massive chunk of X16 development will likely involve poking, translating,manipulating and managing graphical assets.

Those old enough might remember the process of drawing their sprites and tiles onto sheets by hand and manually translating the results into hex values. Fortunately, we're no longer stuck with 80s tooling when it comes to graphical asset development; there is a wealth of modern image editors and tileset manipulation programs we can be use for X16 development. We also have the added bonus that many of these tools are very specialised for the kind of pixel-art the X16 uses thanks to the retro-renaissance of the past 10 or 15 years.

However, we still need to ensure our image data is created with VERA's more-modest-than-modern constraints in mind, and we need a method of quickly and easily transforming these images into the byte-by-byte assembly statements needed by our programs. This can potentially be quite challenging, particularly if we're working on a large, multi-person project or on one in which graphical changes can go through many revisions. Most projects will need to start by putting together a pipeline to deal with all these assets and their transformations and manage the change process in a sane manner.

Aloevera is intended to play a major role in your graphically-intense X16 project. Targeted specifically for VERA on the X16, it is a command-line tool that assembles your images into data formats ready for direct inclusion in your Assembly (or even BASIC) projects. 

In addition to this, it is also intended as a helper to ensure your image data is set up according to the needs of your particular application's targets. It is aware of all of VERA's constraints, layer display modes and pixel depths, and will ensure that the input data you provide it is conformant.

In a nutshell, Aloevera:

* **Transforms** the output of modern image editors into assembly formats that can be included and used in your X16 projects.
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
aloevera create my_project.av
```

This will create a new project file called `my_project.av` in the current directory. Feel free to look through the contents of the file, but remember its contents are not meant to be edited directly.

## The Examples

All Aloevera concepts and commands are explored and illustrated via a series of detailed example. [Samples](../samples) are also provided for each example, which contain assembly and basic code as well as a small makefile illustrating how one could potentially use Aloevera in a larger project.

### [Example 1: Intro to Aloevera - Importing a palette](./ex_001.md)
### [Example 2: Intro to Imagesets - Replacing the default font](./ex_002.md)
### [Example 3: Simple Tilemaps - Displaying a custom banner](./ex_003.md)
### [Example 4: Higher Depth Tilemaps - A more game-like tilemap](./ex_004.md)
### [Example 5: Sprites](./ex_005.md)
### [Example 6: Bitmaps](./ex_006.md)