# A small thank-you for everyone, as anyone could have done it.
clear @a minecraft:paper[minecraft:custom_data={arnis_parcel:1b}]
clear @a minecraft:leather_helmet[minecraft:custom_data={arnis_lost:1b}]
execute store result score #r arnis.story run random value 1..6
execute if score #r arnis.story matches 1 run give @a minecraft:bread 4
execute if score #r arnis.story matches 2 run give @a minecraft:poppy
execute if score #r arnis.story matches 3 run give @a minecraft:firework_rocket 3
execute if score #r arnis.story matches 4 run give @a minecraft:music_disc_cat
execute if score #r arnis.story matches 5 run give @a minecraft:emerald 2
execute if score #r arnis.story matches 6 run give @a minecraft:cake
tellraw @a {"text":"Errand done! Someone's day just got better, and there's a little thank-you in your pocket.","color":"green"}
playsound minecraft:entity.player.levelup master @a ~ ~ ~ 0.6 1.4
function arnis:story/errand/reset
