give @s minecraft:paper[minecraft:custom_name={text:"Parcel",color:"gold",italic:false},minecraft:custom_data={arnis_parcel:1b}]
tellraw @s [{"selector":"@e[tag=arnis.npc_it,limit=1]","color":"aqua"},{"text":": Could you drop this parcel off at ","color":"white"},{"storage":"arnis:story","nbt":"errand.to","color":"yellow"},{"text":"? Thank you!","color":"white"}]
