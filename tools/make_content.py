#!/usr/bin/env python3
"""Write every JSON/TOML file of the Underbedrock content mod and its data pack.

Run: python3 tools/make_content.py
The files are the deliverable; this script only keeps them reproducible.

The world under the bedrock floor, top to bottom:
  y -54 .. -58    ordinary deepslate (the Overworld's own floor)
  y -59 .. -64    a thin bedrock transition, jagged as vanilla's floor
  y -65 .. -72    voidshell: the mod's gate block, dug only by the void pickaxe
  y -73 .. -564   the Abyss: voidstone, carved caves (air, never lava), ores
  y -565 .. -576  abyss bedrock: unbreakable, so nobody falls out of the world
"""
import json
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MOD = ROOT / "content-mod"
PACK = ROOT / "datapack" / "underbedrock_world"

ID = "underbedrock"

# Start clean: files an older version of this script wrote must not survive.
for stale in (MOD / "data", PACK / "data"):
    shutil.rmtree(stale, ignore_errors=True)
for stale in (MOD / "assets" / ID / "blockstates", MOD / "assets" / ID / "models",
              MOD / "assets" / ID / "items", MOD / "assets" / ID / "lang"):
    shutil.rmtree(stale, ignore_errors=True)


def write(path: Path, data) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if isinstance(data, str):
        path.write_text(data, encoding="utf-8")
    else:
        path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


# --------------------------------------------------------------------------- manifest
write(MOD / "horizon-mod.toml", f'''id = "{ID}"
name = "Underbedrock"
version = "0.0.1"
description = "A dark world under the bedrock floor: voidstone caverns lit by blue crystals, voidshell gates, and a void-alloy pickaxe that digs bedrock."
authors = ["FoxVukOff"]
# A content mod: blocks, items and mob effects from data/ and assets/, no code (API 0.2).
api = "0.2"
side = "both"
# registries: "May add blocks, items, mob effects and entity types to your worlds".
capabilities = ["registries"]
''')

# --------------------------------------------------------------------------- blocks
BLOCKS = {
    # The stone of the Abyss itself.
    "voidstone": {
        "template": "minecraft:stone",
        "properties": {
            "hardness": 3.0,
            "explosion_resistance": 6.0,
            "requires_tool": True,
            "sounds": "minecraft:deepslate",
        },
        "item": True,
    },
    "voidstone_bricks": {
        "template": "minecraft:stone_bricks",
        "properties": {
            "hardness": 3.0,
            "explosion_resistance": 6.0,
            "requires_tool": True,
            "sounds": "minecraft:deepslate",
        },
        "item": True,
    },
    "void_gravel": {
        "template": "minecraft:gravel",
        "properties": {
            "hardness": 1.2,
            "explosion_resistance": 1.2,
            "sounds": "minecraft:gravel",
        },
        "item": True,
    },
    # The ore: blue crystals in voidstone, faintly lit.
    "void_crystal_ore": {
        "template": "minecraft:deepslate_diamond_ore",
        "properties": {
            "hardness": 4.5,
            "explosion_resistance": 3.0,
            "requires_tool": True,
            "light": 4,
            "sounds": "minecraft:deepslate",
        },
        "item": True,
    },
    "void_crystal_block": {
        "template": "minecraft:diamond_block",
        "properties": {
            "hardness": 5.0,
            "explosion_resistance": 6.0,
            "requires_tool": True,
            "light": 6,
            "sounds": "minecraft:iron_block",
        },
        "item": True,
    },
    "crystal_lamp": {
        "template": "minecraft:glowstone",
        "properties": {
            "hardness": 1.8,
            "explosion_resistance": 1.8,
            "light": 15,
            "sounds": "minecraft:glass",
        },
        "item": True,
    },
    # A glowing sprout, for cave floors and builds.
    "crystal_sprout": {
        "template": "minecraft:poppy",
        "properties": {
            "hardness": 0.0,
            "explosion_resistance": 0.0,
            "light": 7,
            "sounds": "minecraft:short_grass",
        },
        "item": True,
    },
    # The hot expanse: magma-like rock with ember cracks, faintly lit.
    "emberstone": {
        "template": "minecraft:netherrack",
        "properties": {
            "hardness": 2.5,
            "explosion_resistance": 6.0,
            "requires_tool": True,
            "light": 4,
            "sounds": "minecraft:netherrack",
        },
        "item": True,
    },
    # The inactive hot expanse: cooled ash-grey rock.
    "scorched_stone": {
        "template": "minecraft:stone",
        "properties": {
            "hardness": 3.0,
            "explosion_resistance": 6.0,
            "requires_tool": True,
            "sounds": "minecraft:basalt",
        },
        "item": True,
    },
    # The frozen wastes: pale ice stone.
    "froststone": {
        "template": "minecraft:calcite",
        "properties": {
            "hardness": 3.0,
            "explosion_resistance": 6.0,
            "requires_tool": True,
            "sounds": "minecraft:calcite",
        },
        "item": True,
    },
    "frost_gravel": {
        "template": "minecraft:gravel",
        "properties": {
            "hardness": 1.2,
            "explosion_resistance": 1.2,
            "sounds": "minecraft:gravel",
        },
        "item": True,
    },
    # The gate under the bedrock floor: the engine never breaks it (hardness
    # -1); the native half lets the void-alloy pickaxe dig it, as bedrock.
    "voidshell": {
        "template": "minecraft:obsidian",
        "properties": {
            "hardness": -1.0,
            "explosion_resistance": 1200.0,
            "sounds": "minecraft:deepslate",
        },
        "item": True,
    },
    # The world's bottom: unbreakable for everyone, pickaxe included. It has
    # an item form only for the creative tab and builders' /give.
    "abyss_bedrock": {
        "template": "minecraft:bedrock",
        "properties": {
            "hardness": -1.0,
            "explosion_resistance": 3600000.0,
            "sounds": "minecraft:stone",
        },
        "item": True,
    },
}
for name, body in BLOCKS.items():
    write(MOD / f"data/{ID}/horizon/block/{name}.json", body)

# --------------------------------------------------------------------------- items
write(MOD / f"data/{ID}/horizon/item/void_crystal.json",
      {"template": "minecraft:diamond", "rarity": "uncommon"})
write(MOD / f"data/{ID}/horizon/item/void_alloy.json",
      {"template": "minecraft:iron_ingot", "rarity": "rare"})
# The pickaxe that digs bedrock and voidshell. Its tool rules replace the
# netherite pickaxe's: one rule over the whole mineable/pickaxe tag, faster
# and top-tier, so it also harvests the mod's blocks (they join that tag).
write(MOD / f"data/{ID}/horizon/item/void_alloy_pickaxe.json", {
    "template": "minecraft:netherite_pickaxe",
    "rarity": "epic",
    "components": {
        "minecraft:max_damage": 2800,
        "minecraft:enchantable": {"value": 22},
        "minecraft:repairable": {"items": f"{ID}:void_alloy"},
        "minecraft:tool": {
            "default_mining_speed": 1.0,
            "damage_per_block": 1,
            "rules": [
                {"blocks": "#minecraft:mineable/pickaxe", "speed": 14.0, "correct_for_drops": True}
            ],
        },
    },
})

# The full void gear set: every tool and armor piece, netherite-tier from the
# template and repaired with void alloy. No enchantment glint: the purple
# shine made the set read as enchanted netherite, so the texture speaks alone.
GEAR = {
    "void_alloy_sword": ("minecraft:netherite_sword", 2200),
    "void_alloy_axe": ("minecraft:netherite_axe", 2100),
    "void_alloy_hoe": ("minecraft:netherite_hoe", 2000),
    "void_alloy_shovel": ("minecraft:netherite_shovel", 2100),
    "void_alloy_helmet": ("minecraft:netherite_helmet", 700),
    "void_alloy_chestplate": ("minecraft:netherite_chestplate", 1000),
    "void_alloy_leggings": ("minecraft:netherite_leggings", 900),
    "void_alloy_boots": ("minecraft:netherite_boots", 800),
}
for gear_name, (gear_template, gear_damage) in GEAR.items():
    write(MOD / f"data/{ID}/horizon/item/{gear_name}.json", {
        "template": gear_template,
        "rarity": "epic",
        "components": {
            "minecraft:max_damage": gear_damage,
            "minecraft:enchantable": {"value": 22},
            "minecraft:repairable": {"items": f"{ID}:void_alloy"},
        },
    })

# Layer shards: trophies of the hot and frozen expanse, and the stalker's own
# shard (the native half renders it as the stalker's visible body).
write(MOD / f"data/{ID}/horizon/item/ember_shard.json",
      {"template": "minecraft:diamond", "rarity": "uncommon"})
write(MOD / f"data/{ID}/horizon/item/frost_shard.json",
      {"template": "minecraft:diamond", "rarity": "uncommon"})
write(MOD / f"data/{ID}/horizon/item/stalker_shard.json",
      {"template": "minecraft:echo_shard", "rarity": "epic"})

# --------------------------------------------------------------------------- creative tab
write(MOD / f"data/{ID}/horizon/creative_tab/{ID}.json", {
    "title": f"itemGroup.{ID}",
    "icon": f"{ID}:void_alloy_pickaxe",
    "items": [
        # the dig and the set it digs with
        f"{ID}:void_alloy_pickaxe",
        f"{ID}:void_alloy_sword",
        f"{ID}:void_alloy_axe",
        f"{ID}:void_alloy_shovel",
        f"{ID}:void_alloy_hoe",
        f"{ID}:void_alloy_helmet",
        f"{ID}:void_alloy_chestplate",
        f"{ID}:void_alloy_leggings",
        f"{ID}:void_alloy_boots",
        # what the set is made of
        f"{ID}:void_crystal_ore",
        f"{ID}:void_crystal",
        f"{ID}:void_alloy",
        f"{ID}:ember_shard",
        f"{ID}:frost_shard",
        f"{ID}:stalker_shard",
        # the Abyss, layer by layer, top down
        f"{ID}:voidshell",
        f"{ID}:emberstone",
        f"{ID}:scorched_stone",
        f"{ID}:voidstone",
        f"{ID}:voidstone_bricks",
        f"{ID}:void_gravel",
        f"{ID}:froststone",
        f"{ID}:frost_gravel",
        f"{ID}:abyss_bedrock",
        # and what grows or glows down there
        f"{ID}:void_crystal_block",
        f"{ID}:crystal_lamp",
        f"{ID}:crystal_sprout",
    ],
})

# --------------------------------------------------------------------------- loot
def drops_itself(name):
    return {
        "type": "minecraft:block",
        "pools": [{
            "condition": {"type": "minecraft:survives_explosion"},
            "entries": [{"type": "minecraft:item", "name": f"{ID}:{name}"}],
            "rolls": 1,
        }],
        "random_sequence": f"{ID}:blocks/{name}",
    }

for name in ("voidstone", "voidstone_bricks", "void_gravel", "void_crystal_block",
             "crystal_lamp", "crystal_sprout", "voidshell",
             "emberstone", "scorched_stone", "froststone", "frost_gravel"):
    write(MOD / f"data/{ID}/loot_table/blocks/{name}.json", drops_itself(name))
write(MOD / f"data/{ID}/loot_table/blocks/void_crystal_ore.json", {
    "type": "minecraft:block",
    "pools": [{
        "condition": {"type": "minecraft:survives_explosion"},
        "entries": [{"type": "minecraft:item", "name": f"{ID}:void_crystal"}],
        "rolls": 1,
    }],
    "random_sequence": f"{ID}:blocks/void_crystal_ore",
})
# The world's bottom drops nothing and never breaks.
write(MOD / f"data/{ID}/loot_table/blocks/abyss_bedrock.json", {
    "type": "minecraft:block",
    "pools": [],
    "random_sequence": f"{ID}:blocks/abyss_bedrock",
})

# --------------------------------------------------------------------------- recipes
write(MOD / f"data/{ID}/recipe/void_crystal_from_smelting.json", {
    "type": "minecraft:smelting",
    "category": "misc",
    "ingredient": f"{ID}:void_crystal_ore",
    "result": {"id": f"{ID}:void_crystal"},
    "experience": 1.2,
    "cookingtime": 200,
})
write(MOD / f"data/{ID}/recipe/void_crystal_from_blasting.json", {
    "type": "minecraft:blasting",
    "category": "misc",
    "ingredient": f"{ID}:void_crystal_ore",
    "result": {"id": f"{ID}:void_crystal"},
    "experience": 1.2,
    "cookingtime": 100,
})
write(MOD / f"data/{ID}/recipe/void_alloy.json", {
    "type": "minecraft:crafting_shapeless",
    "category": "misc",
    "ingredients": [f"{ID}:void_crystal", "minecraft:iron_ingot"],
    "result": {"id": f"{ID}:void_alloy"},
})
write(MOD / f"data/{ID}/recipe/void_alloy_pickaxe.json", {
    "type": "minecraft:crafting_shaped",
    "category": "equipment",
    "group": "void_alloy_pickaxe",
    "pattern": ["AAA", " S ", " S "],
    "key": {"A": f"{ID}:void_alloy", "S": "minecraft:stick"},
    "result": {"id": f"{ID}:void_alloy_pickaxe"},
})
write(MOD / f"data/{ID}/recipe/void_crystal_block.json", {
    "type": "minecraft:crafting_shaped",
    "category": "building",
    "pattern": ["CCC", "CCC", "CCC"],
    "key": {"C": f"{ID}:void_crystal"},
    "result": {"id": f"{ID}:void_crystal_block"},
})
write(MOD / f"data/{ID}/recipe/void_crystal_from_block.json", {
    "type": "minecraft:crafting_shapeless",
    "category": "misc",
    "ingredients": [f"{ID}:void_crystal_block"],
    "result": {"id": f"{ID}:void_crystal", "count": 9},
})
write(MOD / f"data/{ID}/recipe/voidstone_bricks.json", {
    "type": "minecraft:crafting_shaped",
    "category": "building",
    "pattern": ["VV", "VV"],
    "key": {"V": f"{ID}:voidstone"},
    "result": {"id": f"{ID}:voidstone_bricks", "count": 4},
})
write(MOD / f"data/{ID}/recipe/crystal_lamp.json", {
    "type": "minecraft:crafting_shaped",
    "category": "building",
    "pattern": [" V ", "VCV", " V "],
    "key": {"V": f"{ID}:voidstone", "C": f"{ID}:void_crystal"},
    "result": {"id": f"{ID}:crystal_lamp"},
})
write(MOD / f"data/{ID}/recipe/crystal_sprout.json", {
    "type": "minecraft:crafting_shapeless",
    "category": "misc",
    "ingredients": [f"{ID}:void_crystal", "minecraft:bone_meal"],
    "result": {"id": f"{ID}:crystal_sprout"},
})

for gear_name in GEAR:
    kind = gear_name.removeprefix("void_alloy_")
    if kind in ("sword",):
        pattern, key = ["A", "A", "S"], {"A": f"{ID}:void_alloy", "S": "minecraft:stick"}
    elif kind in ("helmet",):
        pattern, key = ["AAA", "A A"], {"A": f"{ID}:void_alloy"}
    elif kind in ("chestplate", "boots"):
        pattern, key = ["A A", "AAA", "AAA"], {"A": f"{ID}:void_alloy"}
    elif kind in ("leggings",):
        pattern, key = ["AAA", "A A", "A A"], {"A": f"{ID}:void_alloy"}
    else:  # axe, hoe, shovel: head of two ingots on a stick, axe mirrored
        pattern, key = ["AA", "AS", " S"], {"A": f"{ID}:void_alloy", "S": "minecraft:stick"}
        if kind == "shovel":
            pattern = ["A", "S", "S"]
        elif kind == "hoe":
            pattern = ["AA", " S", " S"]
    write(MOD / f"data/{ID}/recipe/{gear_name}.json", {
        "type": "minecraft:crafting_shaped",
        "category": "equipment",
        "group": gear_name,
        "pattern": pattern,
        "key": key,
        "result": {"id": f"{ID}:{gear_name}"},
    })
write(MOD / f"data/{ID}/recipe/ember_shard.json", {
    "type": "minecraft:smelting",
    "category": "misc",
    "ingredient": f"{ID}:emberstone",
    "result": {"id": f"{ID}:ember_shard"},
    "experience": 0.7,
    "cookingtime": 200,
})
write(MOD / f"data/{ID}/recipe/frost_shard.json", {
    "type": "minecraft:smelting",
    "category": "misc",
    "ingredient": f"{ID}:froststone",
    "result": {"id": f"{ID}:frost_shard"},
    "experience": 0.7,
    "cookingtime": 200,
})

# --------------------------------------------------------------------------- tags
write(MOD / "data/minecraft/tags/block/mineable/pickaxe.json", {
    "values": [
        f"{ID}:voidstone",
        f"{ID}:voidstone_bricks",
        f"{ID}:void_crystal_ore",
        f"{ID}:void_crystal_block",
        f"{ID}:crystal_lamp",
        f"{ID}:emberstone",
        f"{ID}:scorched_stone",
        f"{ID}:froststone",
    ]
})
write(MOD / "data/minecraft/tags/block/mineable/shovel.json", {
    "values": [f"{ID}:void_gravel", f"{ID}:frost_gravel"]
})
# Iron tools and better harvest the crystal's ore and block.
write(MOD / "data/minecraft/tags/block/needs_iron_tool.json", {
    "values": [f"{ID}:void_crystal_ore", f"{ID}:void_crystal_block"]
})

# --------------------------------------------------------------------------- worldgen features
# None here on purpose: Horizon logs "placed features cannot run and will be
# skipped" for feature-based placement, so every pocket of the Abyss (ores,
# gravel, lamps, bricks, sprouts, lava) is grown by the datapack's material
# rule instead - see the surface.json section further down.

# --------------------------------------------------------------------------- assets
EN = {
    f"block.{ID}.voidstone": "Voidstone",
    f"block.{ID}.voidstone_bricks": "Voidstone Bricks",
    f"block.{ID}.void_gravel": "Void Gravel",
    f"block.{ID}.void_crystal_ore": "Void Crystal Ore",
    f"block.{ID}.void_crystal_block": "Block of Void Crystal",
    f"block.{ID}.crystal_lamp": "Crystal Lamp",
    f"block.{ID}.crystal_sprout": "Crystal Sprout",
    f"block.{ID}.voidshell": "Voidshell",
    f"block.{ID}.abyss_bedrock": "Abyss Bedrock",
    f"item.{ID}.void_crystal": "Void Crystal",
    f"item.{ID}.void_alloy": "Void Alloy Ingot",
    f"item.{ID}.void_alloy_pickaxe": "Void Alloy Pickaxe",
    f"item.{ID}.void_alloy_sword": "Void Alloy Sword",
    f"item.{ID}.void_alloy_axe": "Void Alloy Axe",
    f"item.{ID}.void_alloy_hoe": "Void Alloy Hoe",
    f"item.{ID}.void_alloy_shovel": "Void Alloy Shovel",
    f"item.{ID}.void_alloy_helmet": "Void Alloy Helmet",
    f"item.{ID}.void_alloy_chestplate": "Void Alloy Chestplate",
    f"item.{ID}.void_alloy_leggings": "Void Alloy Leggings",
    f"item.{ID}.void_alloy_boots": "Void Alloy Boots",
    f"item.{ID}.ember_shard": "Ember Shard",
    f"item.{ID}.frost_shard": "Frost Shard",
    f"item.{ID}.stalker_shard": "Stalker Shard",
    f"block.{ID}.emberstone": "Emberstone",
    f"block.{ID}.scorched_stone": "Scorched Stone",
    f"block.{ID}.froststone": "Froststone",
    f"block.{ID}.frost_gravel": "Frost Gravel",
    f"itemGroup.{ID}": "Underbedrock",
}
RU = {
    f"block.{ID}.voidstone": "Пустотный камень",
    f"block.{ID}.voidstone_bricks": "Пустотный кирпич",
    f"block.{ID}.void_gravel": "Пустотный гравий",
    f"block.{ID}.void_crystal_ore": "Руда пустотного кристалла",
    f"block.{ID}.void_crystal_block": "Блок пустотного кристалла",
    f"block.{ID}.crystal_lamp": "Кристальный светильник",
    f"block.{ID}.crystal_sprout": "Кристальный росток",
    f"block.{ID}.voidshell": "Панцирь пустоты",
    f"block.{ID}.abyss_bedrock": "Бедрок бездны",
    f"item.{ID}.void_crystal": "Пустотный кристалл",
    f"item.{ID}.void_alloy": "Слиток пустотного сплава",
    f"item.{ID}.void_alloy_pickaxe": "Кирка из пустотного сплава",
    f"item.{ID}.void_alloy_sword": "Меч из пустотного сплава",
    f"item.{ID}.void_alloy_axe": "Топор из пустотного сплава",
    f"item.{ID}.void_alloy_hoe": "Мотыга из пустотного сплава",
    f"item.{ID}.void_alloy_shovel": "Лопата из пустотного сплава",
    f"item.{ID}.void_alloy_helmet": "Шлем из пустотного сплава",
    f"item.{ID}.void_alloy_chestplate": "Кираса из пустотного сплава",
    f"item.{ID}.void_alloy_leggings": "Поножи из пустотного сплава",
    f"item.{ID}.void_alloy_boots": "Ботинки из пустотного сплава",
    f"item.{ID}.ember_shard": "Тлеющий осколок",
    f"item.{ID}.frost_shard": "Морозный осколок",
    f"item.{ID}.stalker_shard": "Осколок сталкера",
    f"block.{ID}.emberstone": "Тлеющий камень",
    f"block.{ID}.scorched_stone": "Выжженный камень",
    f"block.{ID}.froststone": "Морозный камень",
    f"block.{ID}.frost_gravel": "Морозный гравий",
    f"itemGroup.{ID}": "Под бедроком",
}
write(MOD / f"assets/{ID}/lang/en_us.json", EN)
write(MOD / f"assets/{ID}/lang/ru_ru.json", RU)

for name in BLOCKS:
    write(MOD / f"assets/{ID}/blockstates/{name}.json",
          {"variants": {"": {"model": f"{ID}:block/{name}"}}})
    if name == "crystal_sprout":
        write(MOD / f"assets/{ID}/models/block/{name}.json",
              {"parent": "minecraft:block/cross", "textures": {"cross": f"{ID}:block/{name}"}})
    else:
        write(MOD / f"assets/{ID}/models/block/{name}.json",
              {"parent": "minecraft:block/cube_all", "textures": {"all": f"{ID}:block/{name}"}})
    write(MOD / f"assets/{ID}/items/{name}.json",
          {"model": {"type": "minecraft:model", "model": f"{ID}:block/{name}"}})

for name, parent in (("void_crystal", "minecraft:item/generated"),
                     ("void_alloy", "minecraft:item/generated")):
    write(MOD / f"assets/{ID}/models/item/{name}.json",
          {"parent": parent, "textures": {"layer0": f"{ID}:item/{name}"}})
    write(MOD / f"assets/{ID}/items/{name}.json",
          {"model": {"type": "minecraft:model", "model": f"{ID}:item/{name}"}})
for name in ("void_alloy_pickaxe", "void_alloy_sword", "void_alloy_axe",
             "void_alloy_hoe", "void_alloy_shovel"):
    write(MOD / f"assets/{ID}/models/item/{name}.json",
          {"parent": "minecraft:item/handheld", "textures": {"layer0": f"{ID}:item/{name}"}})
    write(MOD / f"assets/{ID}/items/{name}.json",
          {"model": {"type": "minecraft:model", "model": f"{ID}:item/{name}"}})
for name in ("ember_shard", "frost_shard", "stalker_shard", "void_alloy_helmet",
             "void_alloy_chestplate", "void_alloy_leggings", "void_alloy_boots"):
    write(MOD / f"assets/{ID}/models/item/{name}.json",
          {"parent": "minecraft:item/generated", "textures": {"layer0": f"{ID}:item/{name}"}})
    write(MOD / f"assets/{ID}/items/{name}.json",
          {"model": {"type": "minecraft:model", "model": f"{ID}:item/{name}"}})

# =========================================================================== the data pack
# The pack that gives the Overworld room under its bedrock floor and generates
# the Abyss there. Installed per world (or as a feature pack); see README.md.
write(PACK / "pack.mcmeta", {
    "pack": {
        "description": "Underbedrock: the world under the bedrock floor",
        "min_format": 121,
        "max_format": 121,
    }
})

DIMENSION_TYPE = {
    # A faint ambient floor: the Abyss's layers read without torches, while
    # the surface night stays dark enough for monsters. (The sky glimpses in
    # unloaded chunks are chunk streaming, not light - they close as chunks
    # load; ambient light cannot hide them, so keep render distance up.)
    "ambient_light": 0.05,
    "attributes": {
        "minecraft:audio/ambient_sounds": {
            "mood": {
                "block_search_extent": 8,
                "offset": 2.0,
                "sound": "minecraft:ambient.cave",
                "tick_delay": 6000,
            }
        },
        "minecraft:audio/background_music": {
            "creative": {"max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.creative"},
            "default": {"max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.game"},
        },
        "minecraft:gameplay/bed_rule": {
            "can_set_spawn": "always",
            "can_sleep": "when_dark",
            "error_message": {"translate": "block.minecraft.bed.no_sleep"},
        },
        "minecraft:gameplay/nether_portal_spawns_piglin": True,
        "minecraft:gameplay/respawn_anchor_works": False,
        "minecraft:gameplay/straw_bed_rule": {
            "can_set_spawn": "never",
            "can_sleep": "when_dark",
            "destroy_on_leave": True,
            "error_message": {"translate": "block.minecraft.bed.no_sleep"},
        },
        "minecraft:visual/ambient_light_color": "#0a0a0a",
        "minecraft:visual/cloud_color": "#ccffffff",
        "minecraft:visual/cloud_height": 192.33,
        "minecraft:visual/fog_color": "#c0d8ff",
        "minecraft:visual/sky_color": "#78a7ff",
    },
    "coordinate_scale": 1.0,
    "default_clock": "minecraft:overworld",
    "has_ceiling": False,
    "has_ender_dragon_fight": False,
    "has_skylight": True,
    "height": 896,
    "infiniburn": "#minecraft:infiniburn_overworld",
    "logical_height": 384,
    "min_y": -576,
    "monster_spawn_block_light_limit": 0,
    "monster_spawn_light_level": {"type": "minecraft:uniform", "max_inclusive": 7, "min_inclusive": 0},
    "timelines": "#minecraft:in_overworld",
}
write(PACK / "data/minecraft/dimension_type/overworld.json", DIMENSION_TYPE)

write(PACK / "data/minecraft/worldgen/noise_settings/overworld.json", {
    "noise": {"min_y": -576, "height": 896},
    "default_block": "minecraft:stone",
    "default_fluid": "minecraft:water",
    # The aquifer keeps the surface world's oceans, rivers and dry caves as
    # vanilla's do. Its lava function is a flat zero: no lava pocket anywhere,
    # and nothing below y -54 is ever a fluid cell (the density is solid down
    # there), so the engine's deep lava sea has nowhere to go.
    "aquifers": {
        "barrier": 1.0,
        "exclusion": {
            "type": "minecraft:min",
            "left": {"type": "minecraft:sub", "left": -0.225, "right": "minecraft:overworld/erosion"},
            "right": {
                "type": "minecraft:max",
                "left": {"type": "minecraft:sub", "left": "minecraft:overworld/depth", "right": 0.9},
                "right": 0.0,
            },
        },
        "fluid_level_floodedness": {
            "type": "minecraft:noise",
            "noise": f"{ID}:floodedness",
            "xz_scale": 0.67,
            "y_scale": 0.71,
        },
        "fluid_level_spread": {
            "type": "minecraft:noise",
            "noise": f"{ID}:spread",
            "xz_scale": 1.0,
            "y_scale": 0.71,
        },
        "lava": 0.0,
        "surface_level": "minecraft:overworld/chunk_surface_level",
    },
    "noise_router": {
        "chunk_surface_level": "minecraft:overworld/chunk_surface_level",
        "continents": "minecraft:overworld/continents",
        "depth": "minecraft:overworld/depth",
        "erosion": "minecraft:overworld/erosion",
        "final_density": f"{ID}:final_density",
        "ridges": "minecraft:overworld/ridges",
        "temperature": "minecraft:overworld/temperature",
        "vegetation": "minecraft:overworld/vegetation",
    },
    "material_rule": f"{ID}:surface",
    "spawn_target": [],
    "sea_level": 63,
    "disable_mob_generation": False,
    "legacy_random_source": False,
})

DF = PACK / f"data/{ID}/worldgen/density_function"
write(DF / "final_density.json", {
    # Above y -54: vanilla's terrain, untouched. Below it: plain solid rock;
    # the Abyss's caves are carved out of that solid by the material rule, so
    # no fluid (and so no lava) is ever placed down there.
    "type": "minecraft:range_choice",
    "input": "minecraft:y",
    "min_inclusive": -1000000.0,
    "max_exclusive": -54.0,
    "when_in_range": 1.0,
    "when_out_of_range": "minecraft:overworld/final_density",
})
for name, octave, octaves in (("cheese", -6, 4), ("tunnels", -5, 3),
                              ("cheese_big", -8, 3), ("floodedness", -3, 1),
                              ("spread", -4, 1)):
    write(PACK / f"data/{ID}/worldgen/noise/{name}.json", {
        "base_octave": octave,
        "octave_count": octaves,
        "base_amplitude": 1.0,
    })

# Material conditions sample the NOISE registry, never density functions (a
# density name in noise_threshold fails every chunk at the surface stage -
# the void-world bug of 2026-11). So every pocket of the Abyss is a numeric
# window on the raw cheese / cheese_big / tunnels noises, and the windows are
# disjoint so a pocket is exactly one thing. Higher cheese = cave air in the
# cool layer; the hot layer carves on cheese_big alone (wide open expanse).
AIR = {"type": "minecraft:block", "result_state": "minecraft:air"}
LAVA = {"type": "minecraft:block", "result_state": "minecraft:lava"}
MR = PACK / f"data/{ID}/worldgen/material_rule"


def blk(state):
    return {"type": "minecraft:block", "result_state": state}


def y_above(anchor):
    return {"type": "minecraft:y_above", "anchor": {"absolute": anchor},
            "surface_depth_multiplier": 0, "add_stone_depth": False}


def at_least(y):
    return y_above(y - 1)


def at_most(y):
    return {"type": "minecraft:not", "invert": y_above(y)}


def when(cond, body):
    return {"type": "minecraft:condition", "if_true": cond, "then_run": body}


def yband(shallow, deep, body):
    """The band from the `shallow` end down to the `deep` one: y at or below
    the shallow anchor and at or above the deep one. The other way round
    (at_least the shallow, at_most the deep) is an empty set - the bug that
    made the whole Abyss voidshell on 2026-11-03."""
    return when(at_most(shallow), when(at_least(deep), body))


def noise_win(noise, lo, hi):
    """True while the raw noise value sits in [lo, hi)."""
    return {"type": "minecraft:noise_threshold", "noise": f"{ID}:{noise}",
            "min_threshold": lo, "max_threshold": hi, "is_3d": True}


def noise_at_least(noise, lo):
    return noise_win(noise, lo, 1000000.0)


def seq(*rules):
    return {"type": "minecraft:sequence", "sequence": list(rules)}


# The Abyss, layer by layer (the user's stack, 2026-11): hot and open under
# the gate, colder and emptier with depth; the frozen wastes below -420 host
# nothing but the stalker (the native half spawns and renders it).
HOT = yband(-73, -168, seq(
    # Wide open expanse: emberstone islands where cheese_big stays low, and
    # lava lakes in the bellies of the open space.
    when(noise_at_least("cheese_big", 0.10),
         seq(when(noise_win("cheese", -1000000.0, -0.45), LAVA), AIR)),
    # Thin lava veins where the tunnels noise crosses zero.
    when(noise_win("tunnels", -0.03, 0.03), LAVA),
    when(noise_win("cheese", 0.30, 0.36), blk(f"{ID}:scorched_stone")),
    when(noise_win("cheese", 0.36, 0.42), blk(f"{ID}:void_crystal_ore")),
    blk(f"{ID}:emberstone"),
))
INACTIVE = yband(-169, -280, seq(
    # Cooling lava veins take the tunnel sheets before the air rules do.
    when(noise_win("tunnels", -0.03, 0.03), LAVA),
    when(noise_at_least("cheese", 0.25), AIR),
    when(noise_win("tunnels", -0.09, 0.09), AIR),
    when(noise_win("cheese", -0.50, -0.44), blk(f"{ID}:void_gravel")),
    when(noise_win("cheese", 0.13, 0.16), blk(f"{ID}:void_crystal_ore")),
    blk(f"{ID}:scorched_stone"),
))
COOL = yband(-281, -420, seq(
    when(noise_at_least("cheese_big", 0.35), AIR),
    when(noise_at_least("cheese", 0.25), AIR),
    when(noise_win("tunnels", -0.09, 0.09), AIR),
    # Decor grows on the cave walls: sprouts in the shell right under the
    # cheese-air threshold, ruin bricks behind them, lamps on the wide
    # cavern walls.
    when(noise_win("cheese", 0.22, 0.25), blk(f"{ID}:crystal_sprout")),
    when(noise_win("cheese", 0.19, 0.22), blk(f"{ID}:voidstone_bricks")),
    when(noise_win("cheese_big", 0.30, 0.34), blk(f"{ID}:crystal_lamp")),
    # Ores, each its own disjoint window of the fine cheese noise.
    when(noise_win("cheese", 0.17, 0.19), blk("minecraft:deepslate_diamond_ore")),
    when(noise_win("cheese", 0.13, 0.16), blk(f"{ID}:void_crystal_ore")),
    when(noise_win("cheese", 0.09, 0.12), blk("minecraft:deepslate_iron_ore")),
    when(noise_win("cheese", 0.05, 0.08), blk("minecraft:deepslate_coal_ore")),
    when(noise_win("cheese", -0.50, -0.44), blk(f"{ID}:void_gravel")),
    blk(f"{ID}:voidstone"),
))
FROZEN = yband(-421, -564, seq(
    # Sparse caves only: the wastes are cold, empty and quiet.
    when(noise_at_least("cheese", 0.40), AIR),
    when(noise_win("cheese", -0.50, -0.44), blk(f"{ID}:frost_gravel")),
    when(noise_win("cheese", 0.33, 0.37), blk(f"{ID}:void_crystal_ore")),
    blk(f"{ID}:froststone"),
))

write(MR / "surface.json", seq(
    # The Overworld's bedrock floor, a thin jagged band at -64..-59: the
    # gradient alone is true at every depth below it, so the y_above under
    # it keeps the band where a transition belongs.
    when({"type": "minecraft:vertical_gradient",
          "random_name": f"{ID}:floor_bedrock",
          "true_at_and_below": {"absolute": -64},
          "false_at_and_above": {"absolute": -59}},
         when(at_least(-63), blk("minecraft:bedrock"))),
    # Everything below y -55: the Abyss, depth by depth.
    when(at_most(-55), seq(
        # The world's bottom: bedrock no pickaxe digs.
        when(at_most(-564), blk(f"{ID}:abyss_bedrock")),
        # The layered world itself, -73..-564.
        when(at_most(-72), seq(HOT, INACTIVE, COOL, FROZEN)),
        # The gate under the bedrock floor: voidshell, -65..-72.
        when(at_most(-65), blk(f"{ID}:voidshell")),
    )),
    # The Overworld's own deepslate gets sparse crystal pockets (y -60..-16),
    # so the pickaxe is craftable in survival long before anyone touches
    # bedrock. Everywhere else the sequence falls through to vanilla's rules.
    when(at_least(-60), when(at_most(-16),
         when(noise_win("cheese", 0.44, 0.47), blk(f"{ID}:void_crystal_ore")))),
    # Everything above: vanilla's rules, unchanged.
    "minecraft:overworld",
))

# The vanilla carvers would carve tunnels into the solid Abyss and the engine
# would fill them with its deep lava sea, so the pack replaces the three
# Overworld carvers with ones that never start. The Abyss's caves are the
# material rule's; the surface keeps vanilla's density caves.
CARVER_Y = {"type": "minecraft:constant", "value": {"absolute": -55}}
write(PACK / "data/minecraft/worldgen/carver/cave.json", {
    "type": "minecraft:cave",
    "probability": 0.0,
    "y": CARVER_Y,
    "count": {"type": "minecraft:constant", "value": 0},
    "thickness": {"type": "minecraft:constant", "value": 1.0},
    "floor_level": {"type": "minecraft:constant", "value": 0.0},
    "horizontal_radius_multiplier": {"type": "minecraft:constant", "value": 1.0},
    "vertical_radius_multiplier": {"type": "minecraft:constant", "value": 1.0},
    "room_vertical_radius_multiplier": {"type": "minecraft:constant", "value": 1.0},
})
write(PACK / "data/minecraft/worldgen/carver/cave_extra_underground.json", {
    "type": "minecraft:cave",
    "probability": 0.0,
    "y": CARVER_Y,
    "count": {"type": "minecraft:constant", "value": 0},
    "thickness": {"type": "minecraft:constant", "value": 1.0},
    "floor_level": {"type": "minecraft:constant", "value": 0.0},
    "horizontal_radius_multiplier": {"type": "minecraft:constant", "value": 1.0},
    "vertical_radius_multiplier": {"type": "minecraft:constant", "value": 1.0},
    "room_vertical_radius_multiplier": {"type": "minecraft:constant", "value": 1.0},
})
# Vanilla's own hollows, kept out of the Abyss. Structure sets have no depth
# gate, so the mineshafts (and the ancient cities, whose deep dark columns
# reach the whole stack) are switched off for this world entirely - the
# surface keeps every other structure. Amethyst geodes are a placed feature,
# and those do take a height range: the vanilla one has none, so geodes grew
# at any depth; this keeps them where vanilla puts them, above the floor.
EMPTY_SET = {
    "structures": [],
    "placement": {
        "type": "minecraft:random_spread",
        "spacing": 4096,
        "separation": 4095,
        "salt": 1024,
    },
}
write(PACK / "data/minecraft/worldgen/structure_set/mineshafts.json", EMPTY_SET)
write(PACK / "data/minecraft/worldgen/structure_set/ancient_cities.json", EMPTY_SET)
write(PACK / "data/minecraft/worldgen/placed_feature/amethyst_geode.json", {
    "feature": "minecraft:amethyst_geode",
    "placement": [{
        "type": "minecraft:height_range",
        "distribution": {
            "type": "minecraft:uniform",
            "min_inclusive": {"absolute": -58},
            "max_inclusive": {"absolute": 30},
        },
    }],
})

write(PACK / "data/minecraft/worldgen/carver/canyon.json", {
    "type": "minecraft:canyon",
    "probability": 0.0,
    "y": CARVER_Y,
    "vertical_rotation": {"type": "minecraft:constant", "value": 0.0},
    "shape": {
        "distance_factor": {"type": "minecraft:constant", "value": 1.0},
        "thickness": {"type": "minecraft:constant", "value": 1.0},
        "width_smoothness": 1,
        "horizontal_radius_factor": {"type": "minecraft:constant", "value": 1.0},
        "vertical_radius_default_factor": 0.5,
        "vertical_radius_center_factor": 4.0,
        "y_scale": {"type": "minecraft:constant", "value": 1.0},
    },
})

print("wrote the content mod and the data pack")
