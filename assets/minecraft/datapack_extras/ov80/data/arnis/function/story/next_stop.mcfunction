# Where the story goes now, said in chat; a riddle's place is for the players to work out.
execute if data storage arnis:story cur{kind:"riddle"} run return run tellraw @a [{"text":"Riddle: ","color":"gold"},{"storage":"arnis:story","nbt":"cur.hint","color":"yellow"},{"text":". Work out the place and walk there.","color":"gray"}]
execute if data storage arnis:story cur{kind:"search"} run return run tellraw @a [{"text":"Search around ","color":"gold"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"},{"text":" for the ","color":"gold"},{"storage":"arnis:story","nbt":"cur.thing","color":"yellow"},{"text":". The bar at the top says warmer or colder.","color":"gray"}]
tellraw @a [{"text":"Next stop: ","color":"gold"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"}]
