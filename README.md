SSCHIP8 is a [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) emulator supporting curses-based command-line output on Windows.

![A Breakout clone running in SSCHIP8](/doc/img/breakout.png)

### Running the Emulator

SSCHIP8 can be launched from the command line as follows:

`SSCHIP-8.exe [romfile] (mode)`

Where 'romfile' is the rom to be loaded and 'mode' optionally enables one of several alternative modes, including SUPER-CHIP mode.

### Using the Emulator

The CHIP-8 keyboard is mapped to the left 4 columns of the QWERTY keyboard as follows:

CHIP-8 internal layout:
|   |   |   |   |
|---|---|---|---|
| 1 | 2 | 3 | C |
| 4 | 5 | 6 | D |
| 7 | 8 | 9 | E |
| A | 0 | B | F |

Mapped to:
|   |   |   |   |
|---|---|---|---|
| 1 | 2 | 3 | 4 |
| Q | W | E | R |
| A | S | D | F |
| Z | X | C | V |

### Features in progress:
* SUPER-CHIP support
* Audio Support
* Linux Support
* Graphical output via SDL
* Key Remapping