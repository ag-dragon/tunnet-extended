# Tunnet Extended
A quality of life mod for the indie horror game Tunnet by puzzled_squid.

Disclaimers:
* I highly recommend you backup your saves before using this, and to just backup them up frequently in general. (save files are located in %appdata%\tunnet or ~/local/share/tunnet depending on windows vs linux)
* If the game gets updated it might take me a little while to port TE to the new version. I recommend turning off auto-updates if you are worried about this.

Features:
* Infinite stamina
* Terraform inside of rooms
* Dig rock slightly faster (doesn't affect drill radious when digging rock, only makes it break instantly instead of becoming damaged)
* Toggleable ability to drill any material (e.g. metal walls, planks, etc.)
* Hotkey to make placed relays lights regardless of # of connected relays
* And finally, selectable terraform material! (For if you want to build planks, metal, grass, etc.)

Here is a video I made briefly showcasing building with TE: https://youtu.be/yLXGrKHqlK8

# Installing
Download one of the releases depending on which os and version of the game you are using.
Export everything in the zip into the directory that contains the game executable.
(For steam this would be C:\Program Files (x86)\Steam\steamapps\common\Tunnet\ or ~/.local/share/Steam/steamapps/common/Tunnet/ depending on windows vs linux)

Launch tunnet-extended-loader.exe to run the game with the mod loaded. If you want to play without the mod, just launch the game normally.
Settings and keybinds are set based off of te_config.toml

# Config
\[patches\] contains toggles for certain features.
| key              | type | example |
| ---------------- | ---- | ------- |
| infinite_stamina | bool | true    |
| build_in_rooms   | bool | true    |
| dig_rock_fast    | bool | true    |

\[keybinds\] contains the keybinds that can be set.
Check [this rust file](tunnet-extended/src/settings.rs) to see a list of accepted keybinds.
| key                 | type    | example |
| ------------------- | ------- | ------- |
| dig_anywhere_toggle | Keybind | 'k'     |
| force_light         | Keybind | 'l'     |
| material_down       | Keybind | '-'     |
| material_up         | Keybind | '='     |

# Compiling
This is optional, there are already releases for the different game versions available for download. However, if you do wish to compile TE yourself
you will need to set the TE_VERSION environment variable to either 'Steam' or 'Itchio' depending on what version of the game you want to compile for.
TE also uses rust 1.81.0-nightly
