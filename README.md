<h1 align="center"> <code> archividian </code> </h1>

A CLI tool for archiving file system metadata, built in Rust!


<br>


## Quickstart

Build the project with [Cargo<sup>↗</sup>](https://doc.rust-lang.org/cargo/):

```bash
/archividian> cargo build --release
```

Execute the binary:

```bash
/archividian> target/release/archividian.exe
```

> [!Tip]
> If you’re on Windows, add `archividian.exe` to PATH so that you simply run `archividian` anywhere.

Customise paths:

```bash
/archividian> target/release/archividian.exe C:/Desktop/ --export "example.md"
```

Use `--help` for a full summary of all the available options.


<br>


## Rationale

I use Windows and I have a lot of binaries installed (not just applications, but things like languages, toolchaining, etc.). I also have a nice intuitive folder structure for organising stuff.

When my laptop nuked itself and I lost everything, I realised I had zero record of what I had installed prior. I do have my Desktop on Git, but obviously I’m not committing massive binaries like DaVinci Resolve, and not all directories were being tracked.

What I really needed was just a text record of what my desktop looked like before, even if the files weren’t there.

*Archividian* is just that – a quick-and-simple tool that descends the filesystem, collects paths and metadata, and exports it all to a lovely Markdown file.
