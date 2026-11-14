# Night falls for the scene, unless a Zombie Outbreak keeps the clock.
execute if score #outbreak arnis.story matches 1 run return 0
time set night
tellraw @a {"text":"Night falls over the city...","color":"dark_aqua","italic":true}
