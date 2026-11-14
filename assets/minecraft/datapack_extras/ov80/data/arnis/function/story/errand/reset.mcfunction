# No errand: the next is offered in 3 to 5 minutes.
scoreboard players set #errand arnis.story 0
scoreboard players set #errand_t arnis.story 0
execute store result score #errand_wait arnis.story run random value 180..300
