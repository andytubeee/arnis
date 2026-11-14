# Damage this player dealt since last tick, with an officer just hurt in bow range, was an
# attack on the police. A hit leaves HurtTime at 10, or 9 if the officer moved later that tick.
scoreboard players set @s arnis.cr.hit 0
execute if entity @e[type=minecraft:skeleton,tag=arnis.cop,distance=..48,nbt={HurtTime:10s}] run return run function arnis:crime/spotted
execute if entity @e[type=minecraft:skeleton,tag=arnis.cop,distance=..48,nbt={HurtTime:9s}] run function arnis:crime/spotted
