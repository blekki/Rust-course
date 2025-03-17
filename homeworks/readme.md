## Basic explanation

It's an explanation why I created current reposetory.

Here contain my **Rust** homeworks... that's all.

## Install dependencies

Yep, you need do a couple action before run one of files.

### Install language

First you need install a rust language.
I use a Linux Mint, so below place described commands for similar OS (Ubuntu based). If you have Windows - improvise. I belive in you.

Install Rust using rustup tool.
```
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```
Than choice default instalation on showed list or if you wanna some play with difficult things use other item.

Next, run this command. I dont't know what it does but official documentation tell do it. So...
```
source $HOME/.cargo/env
```

### Install compiller

Nice! You did it. Next step - install a compiller.

Here everything is easier. Use follow commands:
```
sudo apt update
sudo apt install rustc
```

## How to run my code

### Compile file

Choice what you want compile. I did't push compilled file to git. That is reason you must do it.

For choiced file open a console line in folder with needy file and write this command (replace "file-name.rs" to the file you wanna compile):

```
rustc file-name.rs
```

## Start project

Use command and run a new created file:

```
./file-name
```

## PS:
That's all. I tired write readme file. So, I will not write something else.
If you wanna get more details or my explanation have broken your PS (it is possible), visit next site: ```https://www.digitalocean.com/community/tutorials/install-rust-on-ubuntu-linux```.
