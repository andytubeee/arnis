execute if score #kind arnis.story matches 1 run function arnis:story/errand/arrived with storage arnis:story errand
execute if score #kind arnis.story matches 2 if entity @a[nbt={Inventory:[{components:{"minecraft:custom_data":{arnis_lost:1b}}}]}] run function arnis:story/errand/done
