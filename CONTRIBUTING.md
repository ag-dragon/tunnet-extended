# Contributing

Pull requests and any contributions in general are welcome!

## Planned Features
* Logging
    * Due to the nature of how TE works using dll/so injection, debugging is difficult. Adding a log file should ideally make things easier.
* Player Teleportation
    * Should be another simple qol addition. The main difficulty would come from finding where player position is stored in memory. Could maybe use reloading of different save files with different player positions to narrow it down.
* Lua Scripting
    * The eventual plan for TE is to turn it into a modding api that anyone can use to create mods for Tunnet.
    * API Features ideas (some of these are very long-term)
        * get/set player values (name, health, stamina, pos, etc.)
            * Shouldn't be too difficult to expose an api for these, already have most of the values anyways.
        * get/set companion values (exists, movement, position, etc.)
            * Could be neat to mess around with the companion, especially with custom dialogue. Also shouldn't be too difficult to implement (though figuring out where the companion position is stored might be difficult. Maybe first figure out where companion exists boolean is stored and check what reads that value?)
        * Dialogue
            * A way to modify game dialogue would allow for a lot of custom content options. Modifying the actual strings themselves isn't too difficult, the difficulty comes with allowing more complex changes, such as the number of lines of dialogue, or amount of responses and what they do. Requires more fully exploring/reverse-engineering the game's dialogue system.
        * UI
            * Custom UI is basically an essential feature for any sort of complex modding. Actually implementing it though will be difficult. Minor changes, such as modifying the strings used for on-screen text, are pretty simple and a good starting point. Anything more complex will require more research (intercept draw calls?)
        * World Gen
            * Could open up interesting modding options, especially with custom room types. Requires more reverse-engineering to determine how difficult it would be to modify.
        * Tools
            * Both modifying current tools and adding new tools. Would greatly benefit from custom UI. Requires more reverse-engineering.
        * Network nodes
            * Again, both modifying current nodes and adding new types. Would also greatly benefit from custom UI. Requires more reverse-engineering.
    
