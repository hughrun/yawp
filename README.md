# yawp
A command line (CLI) app for publishing social media posts.

## In brief

`yawp` takes some text as an argument and publishes it to the social media accounts of your choice. No need to read the comments, just send your [yawp](https://www.wordnik.com/words/yawp) and move on with your day. Current options are Twitter and Mastodon, it's possible more will be added in future (or not).

`yawp` is specifically designed to fit within a broader toolchain: in general terms it tries to follow "the Unix philosophy":

* can take input from `stdin` (e.g. redirected from a file or another process)
* outputs the message as plaintext to `stdout` (i.e. the output is the input)
* takes all configuration from environment (`ENV`) values to enable flexibility

## Installation

### Homebrew

```bash
brew tap hughrun/yawp && brew install yawp
```

### From binary on MacOS or Linux

1. Download the relevant binary file from [the latest release](https://github.com/hughrun/yawp/releases/latest).
2. Rename it to `yawp`
3. Make the file executable: `chmod +x yawp`
4. Save it somewhere in your `PATH`, e.g. in `/usr/local/bin/`. Alternatively you can symlink it from wherever you want to save it, like this: 
```bash
ln -s /my/awesome/directory/yawp /usr/local/bin/
```

### From source

If you're using another platform or don't trust my binaries you can build your own from source:

1. `git clone` or download the repository as a zip.
2. `cargo build --release`
3. Save it somewhere in your `PATH`, e.g. in `/usr/local/bin/`. Alternatively you can symlink it from `target/release`, like this: 
```bash
ln -s /Users/YOURNAME/YAWP_PATH/target/release/yawp /usr/local/bin/
```

## Usage:
`yawp [FLAGS] [OPTIONS] <YAWP>`

## Flags:
`-h`, `--help` Prints help information  
`-m`, `--mastodon` Send toot  
`-q`, `--quiet` Suppress output (error messages will still be sent to `stderr`)  
`-t`, `--twitter` Send tweet  
`-V`, `--version` Prints version information  

## Options:
`-e`, `--env <env>` path to env file  

## Args:
`<YAWP>` Message (post) to send.

If using `stdin` you must provide a hyphen (`-`) as the argument. However if you do this and are *not* redirecting `stdin` from somewhere, `yawp` will hang your shell unless you supply `EOF` by pressing `Ctrl + D`. (See **example 5** below).

## Environment variables

`yawp` requires some environment variables in order to actually publish your message. You can set these in a number of ways depending on your operating system. `yawp` also allows you to call them in from a file. See **example 6** for using a file or **example 7** for setting environment values at the same time you call `yawp`.

An example environment variables file is provided at `example.env`.

The possible values are:

### Mastodon

For Mastodon you need the base url of your instance (server), and an API access token.

* `MASTODON_ACCESS_TOKEN` - You can create a token at `settings - applications` in your Mastodon account. You require `write:statuses` permission.
* `MASTODON_BASE_URL` - This is the base URL of your server. e.g. `https://mastodon.social`

### Twitter

For Twitter you need the four tokens provided when you create an app at `https://developer.twitter.com/en/apps`.

* `TWITTER_CONSUMER_KEY`
* `TWITTER_CONSUMER_SECRET`
* `TWITTER_ACCESS_TOKEN`
* `TWITTER_ACCESS_SECRET`

## Examples

1. Provide message on command line:

```bash
yawp 'Hello, World!' -t
# Output: Hello, World!
# Tweets: Hello, World!
```

2. Pipe in message:

```bash
echo 'Hello again, World!' | yawp - -m

# Output: Hello again, World!
# Toots: Hello again, World!
```

3. Read from file

```bash
# create a file
(echo Hello fronds; echo "     It's me"; echo ...a tree 🌳) > message.txt

# run yawp and direct file content into it
yawp - <message.txt

# this does the same thing:
cat message.txt | yawp -

# Output: 
#Hello fronds
#     It's me
#...a tree 🌳

```
4. Chain commands

You can redirect the output of `yawp` as well as the input:

```bash
cat message.txt | yawp - > output.txt

# the message.txt and output.txt files are now identical.
```

5. Read from user input

This is not really recommended, but you may find yourself facing a user input prompt if you use a hyphen without providing any redirected input. i.e. if you do this:

```bash
yawp - 
# machine awaits user further input from command line
```
Don't panic, you can provide the message text by typing it in at the command prompt. There is a catch, however, in that `yawp` will wait for further input until it reaches `EOF` (End of File). This will not happen when you press `Enter` but can usually be provided by pressing `Ctrl + D`:

```bash
yawp -t - 
# machine awaits user further input from command line
Awoo!
[Ctrl + D]
# Output: Awoo!
# Tweets: Awoo!
```

6. Provide environment variables from file

In some situtations (e.g. [when using Docker Compose](https://docs.docker.com/compose/environment-variables/) or a `systemd` unit file) you may have already set environment variables specific to those needed by `yawp`. If not, you can call them in from a file by providing the filepath using `-e` or `--env`:

```bash
yawp -t --env ~/yawp.env 'I love to tweet!'
```

7. Provide environment variables on command line

You could also set `ENV` settings manually when you call `yawp`:

```bash
MASTODON_BASE_URL=https://ausglam.space MASTODON_ACCESS_TOKEN=abcd1234 yawp '🎺 I am tooting!' -m
```