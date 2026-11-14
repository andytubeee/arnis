# Stands still like a shop, unhurt by stray arrows, and leaves after four minutes or once
# nobody is near. Without AI a wandering trader neither wanders off nor turns invisible.
summon minecraft:wandering_trader ~ ~ ~ {Tags:["arnis.dealer","arnis.dealer_new"],CustomName:{"text":"Arms Dealer","color":"gold"},NoAI:1b,Invulnerable:1b,DespawnDelay:4800,Offers:{Recipes:[]}}
execute as @e[type=minecraft:wandering_trader,tag=arnis.dealer_new,distance=..1,limit=1] run function arnis:crime/stock
tellraw @a[distance=..48] {"text":"An arms dealer has set up nearby. Bring cash.","color":"gold"}
playsound minecraft:entity.wandering_trader.ambient neutral @a[distance=..48] ~ ~ ~
