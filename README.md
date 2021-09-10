# Daleks

Escape from evil robots who want to exterminate you.

![screenshot](screenshot.png)

Daleks pits you against evil robots, who are trying to kill you (which is why
they are evil).  
Fortunately for you, even though they are evil, they are not very bright
and have a habit of bumping into each other, thus destroying themselves.
In order to survive, you must get them to kill each other off.  
Since you are stuck without offensive weaponry, you are endowed with one
piece of defensive weaponry: a teleportation device.  
If a robot runs into you, you die.  
When two robots run into each other or a junk pile, they die.  
When all the robots die,
you start on the next field.  
This keeps up until they finally get you or you complete the game.

The game is inspired by `gnome-robots` which in turn was inspired by `BSD robots` (aka [`Chase`](https://en.wikipedia.org/wiki/Chase_(video_game))), and so we are back to terminal.


## How to play

```
 Directions:
 
    7   8   9        y     k     u
      \ | /           \    |   /
    4 - 5 - 6        h - SPACE - l
      / | \           /    |   \
    1   2   3        b     j     n

 Commands:
    w          : Wait for end
                 You can stay put and wait for the robots to finish making all their moves.
                 This is not advisable unless there are very few robots and you are absolutely
                 sure that they will all perish before reaching you.
                 Every robot died when you wait increment your safe teleports number (depending on
                 profile)
    + or ENTER : safe teleport
                 Safe teleports move the main character to a location that is safe from enemy robots.
                 In each game only a few safe teleports are allowed.
    - or t     : random teleport
                 Use random teleports to teleport to a random location that may be safe or may not be safe.
                 You can use as many random teleports as you want during a game.
    q          : quit
    ?          : this help
    
Legend:     
    @:  you
    +:  robot1
    #:  robot2
    *:  junk heap    
```

## Options

```
Usage:
  daleks [OPTIONS]

Optional arguments:
  -h,--help             Show this help message and exit
  -s,--safe-moves       Prevent accidental moves that result in getting killed
  --no-safe-moves       Don't prevent accidental moves that result in getting
                        killed
  -p,--profile PROFILE  Set the game profile (CLASSIC, ROBOTS2, NIGHTMARE,
                        ROBOTS2EASY, CLASSICWITHSAFETELEPORTS)
  -c,--colors           Enable terminal colors
  --no-colors           Disable terminal colors
  -a,--asciionly        Use only ascii characters
  --no-asciionly        Use extended unicode characters
  -b,--boardtype BOARDTYPE
                        Set the board layout (NORMAL, BSD)
  -x,--exterminate      Use at your own risk
  --defaults            Restore default values
  --save-conf           Save current configuration
```

### BSD

Launching the program with the following options you play (quite) the same game as `bsd-robots`

```
daleks -a -p CLASSIC -b BSD --no-colors
```

## Installation

### From sources
Follow these instructions to compile `daleks` (requires [rust](https://www.rust-lang.org/) installed).

1\. Clone the project 
 
 ```
 git clone https://github.com/geckoblu-games/daleks && cd daleks
 ```
 
2\. Build the project
 
 ```
 cargo build --release
 ```
 
3\. Once complete, the binary will be located at

```
target/release/daleks
```

4\. Copy the binary in one of the directories listed in your $PATH

### From crates.io
If you're a rusticean, `daleks` can be installed with `cargo`.

```
cargo install daleks
```

### For Debian / Ubuntu
You can use the .deb file provided with the [release](https://github.com/geckoblu-games/daleks/releases/).
