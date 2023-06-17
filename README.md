# Rust Workshop Repo

A repo to accompany the Rust workshop classes

# Rust and Cargo Installation Guide

This guide provides instructions on how to install Rust programming language and its package manager Cargo on macOS and Windows systems using `rustup`, the recommended way to install the Rust programming language.

## Installing Rust and Cargo
### macOS

1. **Install Rust and Cargo**

    Open Terminal and download and install `rustup` by running the following command:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    The above command will download a script and start the installation of the `rustup` toolchain installer.

    It will prompt you for installation. Press "1" and hit enter to proceed with default installation.

2. **Add Rust to your system path**

    After the installation is complete, you will see a message suggesting you to add `rustc` to your system PATH. Close your terminal and reopen it. If `rustc` and `cargo` aren't available, you might need to add Rust to your system path manually.

    If you're using the Bash shell, you can do this by adding the following line to your `~/.bash_profile` or `~/.bashrc` file:

    ```bash
    source $HOME/.cargo/env
    ```

3. **Confirm Installation**

    Verify the installation and check the installed versions of Rust and Cargo by running:

    ```bash
    rustc --version
    cargo --version
    ```

### Windows

1. **Download and run rustup-init.exe**

    Download the [rustup-init.exe](https://win.rustup.rs/) executable and run it. Follow the onscreen instructions. It will install Rust and Cargo.

2. **Add Rust to your system path**

    The installer should do this automatically, but if you find that the command `rustc` is not recognized in a new command prompt, you may need to add it manually:

    - Open the start menu, search for "Environment Variables," then click on "Edit the system environment variables."
    - Click on "Environment Variables."
    - Under "System variables" (the bottom pane), scroll until you find the variable named "Path". Click on it, then click on "Edit."
    - Click on "New," then enter the path to your `.cargo\bin` directory, which by default will be `%USERPROFILE%\.cargo\bin`.

3. **Confirm Installation**

    Open a new command prompt and confirm the installation and versions of Rust and Cargo:

    ```powershell
    rustc --version
    cargo --version
    ```

## Updating Rust and Cargo

Rust can be updated to the latest version by using the following command:

```bash
rustup update
```

## Initializing a Rust Project with Cargo

To initialize a new Rust project, run the following command:

```bash
cargo new <project-name>
```

In this case, however, we have an empty directory and we want to initialize a new Rust project inside it. To do this, we need to change directories to the folder we want to initialize our project in (notes) and use 'cargo init' like so:


```bash
cd student_notes
cargo init .
```

Once you have done this, drag and drop all the rust files that were initially in the notes folder into the src folder that cargo init created.

This readme just contains instructions for setting up rust...
Readmes containing activity instructions are in the folder corresponding to each activity.
