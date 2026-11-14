tag @s remove arnis.cop_new
team join arnis.law @s
# The helmet keeps them from burning by day; summoned with NBT, they drew no bow of their own.
item replace entity @s armor.head with minecraft:iron_helmet
item replace entity @s weapon.mainhand with minecraft:bow
attribute @s minecraft:follow_range base set 16
attribute @s minecraft:movement_speed base set 0.2
