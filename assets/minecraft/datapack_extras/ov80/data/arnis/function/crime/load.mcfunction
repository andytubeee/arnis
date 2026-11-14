scoreboard objectives add arnis.cr dummy
scoreboard objectives add arnis.cr.seen dummy
scoreboard objectives add arnis.cash dummy
# 1 while the police are after a player, the seconds since an officer last saw them, and
# the countdown that starts after three of those.
scoreboard objectives add arnis.wanted dummy
scoreboard objectives add arnis.hide dummy
scoreboard objectives add arnis.unseen dummy
scoreboard objectives add arnis.sneak minecraft.custom:minecraft.sneak_time
scoreboard objectives add arnis.sneak.last dummy
scoreboard objectives add arnis.cr.die deathCount
scoreboard objectives add arnis.cr.hit minecraft.custom:minecraft.damage_dealt
# Mobs leave alone whoever shares their team, so calm police and the law-abiding share one.
# Friendly fire stays on, as it is by default, so hitting an officer still lands.
team add arnis.law
scoreboard players set #cr_t arnis.cr 0
scoreboard objectives add arnis.cr.ctl trigger
# Settings from the pause menu outlast a reload; until one is picked, Normal.
execute unless score #countdown arnis.cr matches 1.. run function arnis:crime/difficulty {delay:3,countdown:30,hunt:6}
execute unless score #calm arnis.cr matches 1.. run scoreboard players set #calm arnis.cr 4
# A server takes its difficulty from server.properties, and Peaceful deletes every officer.
execute store result score #difficulty arnis.cr run difficulty
execute if score #difficulty arnis.cr matches 0 run difficulty easy
