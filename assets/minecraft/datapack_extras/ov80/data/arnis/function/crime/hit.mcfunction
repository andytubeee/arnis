# Damage this player dealt since last tick, with an officer just hurt in bow range, was an
# attack on the police. A hit leaves HurtTime at 10, or 9 if the officer moved later that tick.
scoreboard players set @s arnis.cr.hit 0
execute unless entity @e[type=minecraft:skeleton,tag=arnis.cop,distance=..48,nbt={HurtTime:10s}] unless entity @e[type=minecraft:skeleton,tag=arnis.cop,distance=..48,nbt={HurtTime:9s}] run return 0
# Assaulting an officer is two stars at least.
execute unless score @s arnis.wanted matches 1.. run function arnis:crime/spotted
execute unless score @s arnis.wanted matches 2.. run function arnis:crime/spotted
function arnis:crime/sighted
