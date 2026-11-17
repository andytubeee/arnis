data modify entity @s PickupDelay set value 0s
tag @s remove arnis.heavy
tellraw @a[distance=..16] {"text":"Heave! It comes free.","color":"green"}
playsound minecraft:block.barrel.open master @a[distance=..16] ~ ~ ~ 1 0.7
