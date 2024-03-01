# Setting a Rust Executable's Icon in Windows

by [Mason Remaley](https://twitter.com/MasonRemaley) • 2021-01-05 •
[way-of-rhea](https://anthropicstudios.com/blog/tags/way-of-rhea) |
[games](https://anthropicstudios.com/blog/tags/games) |
[gamedev](https://anthropicstudios.com/blog/tags/gamedev) |
[tech](https://anthropicstudios.com/blog/tags/tech)

[Please, **read the original article!**](https://anthropicstudios.com/2021/01/05/setting-a-rust-windows-exe-icon/)



This morning, I decided it was long overdue that [Way of Rhea](https://anthropicstudios.com/way-of-rhea)
get its own icon.

I believe that if you’re building a project in Visual Studio there’s a UI through which
you can change your exe’s icon–but I’m not a Visual Studio user.
It took me quite a while to figure out how to set an exe’s icon from the command line,
so I figured I’d document what I learned here in the hopes of saving someone else some time.

It’s possible to dynamically load and set a _window_ icon via code,
but for the icon to show up in the file explorer it actually needs to be baked into the executable.
This makes sense–`explorer.exe` shouldn’t have to have to run an executable to determine its icon!

The rest of this post will walk you through how to do this.
The only Rust specific bit is the syntax by which I pass arguments to the linker.



## Table of contents

- [Table of contents](#table-of-contents)
- [Instructions](#instructions)
  1. [Create an icon](#1-create-an-icon)
  2. [Create a resources file](#2-create-a-resources-file)
  3. [Compile the resources file](#3-compile-the-resources-file)
  4. [Link with the compiled resources](#4)
- [Quick update: Window icons](#)



## Instructions

### 1. Create an icon

First, you’ll need to create
[a **square** image ideally at least 256x256px](https://docs.microsoft.com/en-us/windows/win32/uxguide/vis-icons),
and save it as a `.ico`. If you’re not sure how to create a `.ico`, you can use
[GIMP](https://www.gimp.org/) or [ImageMagick](https://imagemagick.org/).



### 2. Create a resources file

Next, you’ll need to create a
[`.rc` file](https://docs.microsoft.com/en-us/windows/win32/menurc/about-resource-files)
that [provides the icon path](https://docs.microsoft.com/en-us/windows/win32/menurc/icon-resource).
Here’s what it should look like assuming you aren’t including any other resources:

**resources.rc**

```
arbitrary_name_here ICON "path\to\your\icon.ico"
```



### 3. Compile the resources file

Next, you’ll need to compile your `.rc` file. The official way to do this is via
[`rc.exe`](https://docs.microsoft.com/en-us/windows/win32/menurc/resource-compiler).

Unfortunately, `rc.exe` is not in the path by default, so you’ll need to find it.
Mine is located at `C:\Program Files\ (x86)\Windows Kits\10\bin\10.0.18362.0\x86\rc.exe`.
It was likely placed there when I installed Visual Studio.

Once you’ve located `rc.exe`, you can use it to compile your `.rc` file into a `.res` file:

```
rc resources.rc
```

Programmatically determining the path to `rc.exe` is, unfortunately, not easy.
If you need to do this, here are some options:

- Require the user to provide the path
- Call it from within a
  [Developer Command Prompt](https://docs.microsoft.com/en-us/dotnet/framework/tools/developer-command-prompt-for-vs)
  where it’s already in the path
- Use [LLVM](https://llvm.org/)’s implementation [`llvm-rc`](https://github.com/llvm/llvm-project/tree/62ec4ac90738a5f2d209ed28c822223e58aaaeb7/llvm/tools/llvm-rc)
- Use the [GNU implementation](https://man7.org/linux/man-pages/man1/windres.1.html)
  (if cross compiling from Linux)
- Check out [how the Zig compiler finds similar files](https://github.com/ziglang/zig/blob/master/src/windows_sdk.cpp),
  and write up something similar for `rc.exe`
- Use a library like [embed-resource](https://crates.io/crates/embed-resource)
  or [winres](https://crates.io/crates/winres) to handle the compilation step for you
  (I haven’t tried either but they seem convenient, thanks Reddit & Twitter for calling them out!)

_If you’ve found a better way to do this, or know if it’s possible to use [vswhere](https://github.com/Microsoft/vswhere) for this purpose,
[let me know](mailto:mason@anthropicstudios.com) and I’ll update this post!_


### 4. Link with the compiled resources

Lastly, you need to link with the `.res` file when building your executable.
How exactly you do this depends on your compiler and linker.

If you’ve used a library to handle the resource compilation step for you,
it will likely automate this step as well. If not,
here’s how I did it in Rust with unrelated options removed for clarity:

```
cargo rustc -- -C link-args="resources.res"
```

_**Update**: This can also be done via `build.rs`._

---

That’s it! If everything has gone well, your executable should display
your icon in the file explorer and the task bar.

![taskbar screenshot with way of rhea icon](https://anthropicstudios.com/assets/monsters-and-sprites/setting-a-rust-windows-exe-icon/task-bar.png)

If things _aren’t_ working correctly, there are third party tools like _Resource Hacker_ that
you can use to compare your final executable’s resources to that of an executable with a working icon.



## Quick update: Window icons

Window icons and executable icons are set separately.

If you’d like to set a window icon to be equivalent to your executable icon,
load the icon with `LoadIconA` and the arbitrary name you specified in your resources file.
Once it’s loaded, store the result in `hIcon` and `hIconSm` on your
[window class](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexa).

```
HICON icon = LoadIconA(hInstance, "arbitrary_name_here");
global_window_class.hIcon = icon;
global_window_class.hIconSm = icon;
```

_`LoadIconA` is defined in `winuser.h` (included in `Windows.h`) and requires linking with `User32.lib`._

If the icon _doesn’t_ exist, `LoadIconA` will return `NULL` and a `GetLastError` flag will be set.

Feel free to leave comments on [Twitter](https://twitter.com/masonremaley/status/1346594278802399232),
[Reddit](https://www.reddit.com/r/rust_gamedev/comments/kraink/setting_a_rust_executables_icon_in_windows/),
[Discord](https://discord.gg/JGeVt5XwPP), or [send us an email!](mailto:hello@anthropicstudios.com)

---

- This markdown is a backup of the original article if the URL is broken someday.
- Original author: [Mason Remaley](https://twitter.com/MasonRemaley) (2021-01-05)
- [Please, read the original article!](https://anthropicstudios.com/2021/01/05/setting-a-rust-windows-exe-icon/)
