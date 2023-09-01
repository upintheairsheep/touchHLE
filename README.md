# touchHLE: high-level emulator for iPhone OS apps

**touchHLE** is a high-level emulator (HLE) for iPhone OS apps. It runs on modern desktop operating systems and Android, and is written in Rust.

As an HLE, touchHLE is radically different from a low-level emulator (LLE) like QEMU. The only code the [emulated CPU](https://github.com/merryhime/dynarmic) executes is the app binary and [a handful of libraries](touchHLE_dylibs/); touchHLE takes the place of iPhone OS and provides its own implementations of the system frameworks (Foundation, UIKit, OpenGL ES, OpenAL, etc).

The goal of this project is to run games from the early days of iOS:

* Currently: iPhone and iPod touch apps for iPhone OS 2.x. [A few of these are known to work](APP_SUPPORT.md), and of course we are trying to make the list longer. :)
* Next: iPhone OS 3.0 support.
* Longer term: iPhone OS 3.1, iPad apps (iPhone OS 3.2), iOS 4.x, …
* Never: 64-bit iOS.

Support for apps that aren't games isn't a priority: it's more complex and less fun.

Visit our homepage! <https://touchhle.org/>

If you're curious about the history and motivation behind the project, you might want to read [the original announcement](https://hikari.noyu.me/blog/2023-02-06-touchhle-anouncement-thread-tech-games-me-and-passion-projects.html). For an introduction to some of the technical details, check out [_touchHLE in depth_](https://hikari.noyu.me/blog/2023-04-13-touchhle-in-depth-1-function-calls.html).

## Important disclaimer

This project is not affiliated with or endorsed by Apple Inc in any way. iPhone, iOS, iPod, iPod touch and iPad are trademarks of Apple Inc in the United States and other countries.

Only use touchHLE to emulate software you legally own.

## Platform support

* Officially supported: x64 Windows, x64 macOS and AArch64 Android.
  * These are the platforms with binary releases. (Android support is not in a release yet. It's coming soon, please wait!)
  * If you're an Apple Silicon Mac user, the x64 build reportedly works in Rosetta.
* Probably works, but you must build it yourself: AArch64 macOS, x64 Linux, AArch64 Linux.
* Never?: other architectures.

Input methods:

- For simulated touch input, there are three options:
  - Mouse/trackpad input (tap/hold/drag by pressing the left mouse button)
  - Virtual cursor using the right analog stick on a game controller (tap/hold/drag by pressing the stick or the right shoulder button)
  - Real touch input, if you're on a device that has a touch screen
- For simulated acceleremeter input, there are two options:
  - Tilt control simulation using the left analog stick of a game controller
  - Real accelerometer input, if you are using a phone, tablet or some other device with a built-in accelerometer (TODO: support game controllers with accelerometers)

## Development status

Real development started in December 2022, and this is so far [a single person](https://hikari.noyu.me/)'s full-time passion project. There's only been a handful of releases so far and no promises can be made about the future. Please be patient.

Currently, the supported functionality is not much more than what's needed by a handful of supported apps, although the code tries to be reasonably complete where it can. The completeness varies a lot between APIs, e.g. UIKit is easily the most hacky and incomplete of the large frameworks that have been implemented, whereas the OpenGL ES and OpenAL implementations are probably complete enough to cover a large number of early apps.

# Usage

First obtain touchHLE, either a [binary release](https://github.com/hikari-no-yume/touchHLE/releases) or by building it yourself (see the next section). If using Windows, it is reccomended to use the `winget` method as described below.

You'll then need an app that you can run (check [the list of supported apps](APP_SUPPORT.md)). Note that the app binary must be decrypted to be usable.

There's a few ways you can run an app in touchHLE.

## Installation via `winget` on Windows

To simplify the installation and management process on Windows, you can use `winget` to install touchHLE. Run `winget install touchHLE` in the command line to install touchHLE on Windows and `winget upgrade touchHLE` in the command line to update touchHLE to the latest version. To uninstall touchHLE, run `winget uninstall touchHLE --preserve` to keep settings and app data, and `winget uninstall touchHLE` to uninstall touchHLE while also permanently deleting app data and settings. Please note the process of updating and uninstalling touchHLE via `winget` only apply to when touchHLE is installed via `winget`, and that `winget` is not availble on certain older versions of Windows 10 and below. To create a shortcut to access the touchHLE directory, run `powershell "$s=(New-Object -COM WScript.Shell).CreateShortcut('%userprofile%\Documents\touchHLE.lnk');$s.TargetPath='%userprofile%\AppData\Local\Microsoft\WinGet\Packages\hikari-no-yume.touchHLE';$s.Save()"` in the command line. When touchHLE is installed via `winget`, `touchHLE` will be added as a new command, and can be used in any folder. `.\touchHLE.exe` will not work, use `touchHLE` instead when installed via `winget`.

## Special Android notes

Windows, Mac and Linux users can skip this section.

On Android, only the graphical user interface (app picker) is available.

On some Android devices, you may be able to use the “Files” app to put apps in the `touchHLE_apps` directory. However, the most reliable method is to use ADB. If you're unfamiliar with ADB, try these steps:

- First, make sure you have installed touchHLE on your Android device, and opened it at least one time. You should see a screen telling you the touchHLE\_apps directory is empty.
- Visit <https://yume-chan.github.io/ya-webadb/> in Google Chrome or another browser with WebUSB, then:
  - Connect your Android device over USB.
  - “Connect” to the device in the web interface.
  - Go to “File Manager” and then navigate to “sdcard” > “Android” > “data” > “org.touchhle.android” > “files” > “touchHLE\_apps”.
  - Click “Upload” and select a “.ipa” file.
  - “Disconnect” from the device in the web interface.
  - Disconnect your Android device.

By making use of the “Download” and “Delete” buttons you can also, for example, edit the options or back up your game saves.

This will be made easier in a future release!

## Graphical user interface

* You can put an app's `.ipa` file or `.app` bundle in the `touchHLE_apps` directory, then when you run touchHLE (on Windows: double-click on `touchHLE.exe`) you can select the app from the app picker.
* On Windows, you can also directly drag and drop an app's `.ipa` file or `.app` bundle onto `touchHLE.exe`.

To configure the options, you can then edit the `touchHLE_options.txt` file. To get a list of options, look in the `OPTIONS_HELP.txt` file.

## Command-line user interface

**This section does not apply on Android.**

You can see the command-line usage by passing the `--help` flag.

If you're a Windows user and unfamiliar with the command line, these instructions may help you get started:

1. Move the `.ipa` file or `.app` bundle to the same folder as `touchHLE.exe`.
2. Hold the Shift key and right-click on the empty space in the folder window.
3. Click “Open with PowerShell”.
4. Type `.\touchHLE.exe "YourAppNameHere.ipa"` (or `.app` as appropriate) and press Enter. If you want to specify options, add a space after the app name (outside the quotes) and then type the options, separated by spaces.

## Other stuff

Currently language detection doesn't work on Windows. To change the language preference reported to the app, you can type `SET LANG=` followed by an [ISO 639-1 language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes), then press Enter, before running the app in the command line. Some common language codes are: `en` (English), `de` (Deutsch), `es` (español), `fr` (français), `it` (italiano) and `ja` (日本語). Bear in mind that it's the app itself that determines which languages are supported, not the emulator.

Any data saved by the app (e.g. **saved games**) are stored in the `touchHLE_sandbox` folder.

If the emulator crashes almost immediately while running a game **listed as supported**, please check whether you have any overlays turned on like the Steam overlay, Discord overlay, RivaTuner Statistics Server, etc. Sadly, as useful as these tools are, they work by injecting themselves into other apps or games and don't always clean up after themselves, so they can break touchHLE… it's not our fault. 😢 Currently only RivaTuner Statistics Server is known to be a problem. If you find another overlay that doesn't work, please tell us about it.

# Building and contributing

Please see the BUILDING.md, DEBUGGING.md and CONTRIBUTING.md files in the git repo.

# License

touchHLE © 2023 hikari\_no\_yume and other contributors.

The source code of touchHLE itself (not its dependencies) is licensed under the Mozilla Public License, version 2.0.

Due to license compatibility concerns, binaries are under the GNU General Public License version 3 or later.

For a best effort listing of all licenses of dependencies, build touchHLE and pass the `--copyright` flag when running it.

Please note that different licensing terms apply to the bundled dynamic libraries (in `touchHLE_dylibs/`) and fonts (in `touchHLE_fonts/`). Please consult the respective directories for more information.

# Thanks

We stand on the shoulders of giants. Thank you to:

* Everyone who has contributed to the project or supported it financially.
* The authors of and contributors to the many libraries used by this project: [dynarmic](https://github.com/merryhime/dynarmic), [rust-macho](https://github.com/flier/rust-macho), [SDL](https://libsdl.org/), [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2), [stb\_image](https://github.com/nothings/stb), Imagination Technologies' [PVRTC decompressor](https://github.com/powervr-graphics/Native_SDK/blob/master/framework/PVRCore/texture/PVRTDecompress.cpp), [openal-soft](https://github.com/kcat/openal-soft), [hound](https://github.com/ruuda/hound), [caf](https://github.com/rustaudio/caf), [dr\_mp3](https://github.com/mackron/dr_libs), [RustType](https://gitlab.redox-os.org/redox-os/rusttype), [the Liberation fonts](https://github.com/liberationfonts/liberation-fonts), [the Noto CJK fonts](https://github.com/googlefonts/noto-cjk), [rust-plist](https://github.com/ebarnard/rust-plist), [gl-rs](https://github.com/brendanzab/gl-rs), [cargo-license](https://github.com/onur/cargo-license), [cc-rs](https://github.com/rust-lang/cc-rs), [cmake-rs](https://github.com/rust-lang/cmake-rs), [cargo-ndk](https://github.com/bbqsrc/cargo-ndk), [cargo-ndk-android-gradle](https://github.com/willir/cargo-ndk-android-gradle), and the Rust standard library.
* The [Rust project](https://www.rust-lang.org/) generally.
* The various people out there who've documented the iPhone OS platform, officially or otherwise. Much of this documentation is linked to within this codebase!
* The iOS hacking/jailbreaking community.
* The Free Software Foundation, for making libgcc and libstdc++ copyleft and therefore saving this project from ABI hell.
* The National Security Agency of the United States of America, for [Ghidra](https://ghidra-sre.org/).
* Many friends who took an interest in the project and gave suggestions and encouragement.
* Developers of early iPhone OS apps. What treasures you created!
* Apple, and NeXT before them, for creating such fantastic platforms.
