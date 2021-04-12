# yawp
A command line (CLI) app for publishing social media posts

## In brief

`yawp` takes some text as an argument and publishes it to the social media accounts of your choice. Current options are Twitter and/or Mastodon.

`yawp` is specifically designed to fit within a broader toolchain: in general terms it tries to follow "the Unix philosophy":

* can take input from `stdin` (e.g. redirected from a file or another process)
* outputs the message as plaintext (i.e. if you are redirecting in, the output is the input)
* takes all configuration from `ENV` values.

## commands

TODO

If using `stdin` you must provide a hyphen (`-`) as the argument. However if you do this and are *not* redirecting `stdin` from somewhere, `yawp` will hang your shell because it will keep waiting for another line of user input forever.

## examples

1. Provide message on command line:

```bash
yawp 'Hello, World!' -t
// output: Hello, World!
// tweets: Hello, World!
```

2. Pipe in message:

```bash
echo 'Hello again, World!' | yawp - -m
// output: Hello again, World!
// toots: Hello again, World!
```

3. Read from file


```bash
(echo Line 1; echo Second line) > message.txt
yawp - <message.txt
// output: 
// Line 1
// Second line

```
