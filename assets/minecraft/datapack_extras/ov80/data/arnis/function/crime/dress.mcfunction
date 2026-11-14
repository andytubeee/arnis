tag @s remove arnis.cop_new
team join arnis.law @s
# The cap keeps them from burning by day; summoned with NBT, they drew no bow of their own.
item replace entity @s armor.head with minecraft:leather_helmet[dyed_color=2241618]
item replace entity @s weapon.mainhand with minecraft:bow
attribute @s minecraft:follow_range base set 16
attribute @s minecraft:movement_speed base set 0.2
execute if score #swat arnis.cr matches 1 run data modify entity @s CustomName set value {text:"SWAT",color:"dark_gray"}
execute if score #swat arnis.cr matches 1 run item replace entity @s armor.chest with minecraft:iron_chestplate
