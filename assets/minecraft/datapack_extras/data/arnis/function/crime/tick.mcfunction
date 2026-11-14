# Crime Spree needs 1.21.6; the real one lives in the ov80 overlay.
execute as @a unless score @s arnis.cr.seen matches 1.. run function arnis:crime/unsupported
