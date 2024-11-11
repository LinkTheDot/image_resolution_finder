This is a tool for recursively searching a given directory or directories for
images that fall under certain parameters.

The way this is configured was a bit of an experiment at the time. So it's a
little jank.

# Building for your own target

```
cargo build --release
```

If you want logging, you can build with the -l flag:

```
cargo build --release -- -l
```

# Usage

Upon running the program, you will be prompted with a window that will describe
how to setup the configuration.

A summary of said prompt would be: Edit the new config file with your desired
parameters. Save the file, then hit Ok on the window with the instructions to
run.

Upon running, the program will search every desired directory recursively for
every image within the given parameters. Copying each one to a destination file.
