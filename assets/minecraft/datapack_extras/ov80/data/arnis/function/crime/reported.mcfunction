tellraw @s {"text":"A witness called the police!","color":"red"}
function arnis:crime/spotted
# The police know where it happened, but no officer has seen the player yet.
scoreboard players set @s arnis.unseen 1
