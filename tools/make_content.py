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
authors = ["you"]
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
            "sounds": "minecraft:metal",
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
            "sounds": "minecraft:grass",
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
    # The world's bottom: unbreakable for everyone, pickaxe included.
    "abyss_bedrock": {
        "template": "minecraft:bedrock",
        "properties": {
            "hardness": -1.0,
            "explosion_resistance": 3600000.0,
            "sounds": "minecraft:stone",
        },
        "item": False,
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
        "minecraft:enchantment_glint_override": True,
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

# --------------------------------------------------------------------------- creative tab
write(MOD / f"data/{ID}/horizon/creative_tab/{ID}.json", {
    "title": f"itemGroup.{ID}",
    "icon": f"{ID}:void_alloy_pickaxe",
    "items": [
        f"{ID}:void_alloy_pickaxe",
        f"{ID}:void_alloy",
        f"{ID}:void_crystal",
        f"{ID}:void_crystal_ore",
        f"{ID}:void_crystal_block",
        f"{ID}:crystal_lamp",
        f"{ID}:crystal_sprout",
        f"{ID}:voidstone",
        f"{ID}:voidstone_bricks",
        f"{ID}:void_gravel",
        f"{ID}:voidshell",
        f"{ID}:abyss_bedrock",
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
             "crystal_lamp", "crystal_sprout", "voidshell"):
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

# --------------------------------------------------------------------------- tags
write(MOD / "data/minecraft/tags/block/mineable/pickaxe.json", {
    "values": [
        f"{ID}:voidstone",
        f"{ID}:voidstone_bricks",
        f"{ID}:void_crystal_ore",
        f"{ID}:void_crystal_block",
        f"{ID}:crystal_lamp",
    ]
})
write(MOD / "data/minecraft/tags/block/mineable/shovel.json", {
    "values": [f"{ID}:void_gravel"]
})
# Iron tools and better harvest the crystal's ore and block.
write(MOD / "data/minecraft/tags/block/needs_iron_tool.json", {
    "values": [f"{ID}:void_crystal_ore", f"{ID}:void_crystal_block"]
})

# --------------------------------------------------------------------------- worldgen features
def ore(targets, size, discard=0.0):
    return {
        "type": "minecraft:ore",
        "config": {
            "size": size,
            "discard_chance_on_air_exposure": discard,
            "targets": [{"target": t, "state": s} for t, s in targets],
        },
    }

VOID = {"predicate_type": "minecraft:block_match", "block": f"{ID}:voidstone"}
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_void_crystal_voidstone.json",
      ore([(VOID, f"{ID}:void_crystal_ore")], 8))
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_void_crystal_overworld.json", ore([
    ({"predicate_type": "minecraft:tag_match", "tag": "minecraft:deepslate_ore_replaceables"}, f"{ID}:void_crystal_ore"),
    ({"predicate_type": "minecraft:tag_match", "tag": "minecraft:stone_ore_replaceables"}, f"{ID}:void_crystal_ore"),
], 5))
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_lamp_voidstone.json",
      ore([(VOID, f"{ID}:crystal_lamp")], 3))
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_gravel_voidstone.json",
      ore([(VOID, f"{ID}:void_gravel")], 20))
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_coal_voidstone.json",
      ore([(VOID, "minecraft:coal_ore")], 12))
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_iron_voidstone.json",
      ore([(VOID, "minecraft:iron_ore")], 8))
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_diamond_voidstone.json",
      ore([(VOID, "minecraft:diamond_ore")], 6))


def placed(feature, count, y_min, y_max, rarity=None):
    modifiers = []
    if rarity:
        modifiers.append({"type": "minecraft:rarity_filter", "chance": rarity})
    modifiers += [
        {"type": "minecraft:count", "count": count},
        {"type": "minecraft:in_square"},
        {"type": "minecraft:height_range", "height": {
            "type": "minecraft:uniform",
            "min_inclusive": {"absolute": y_min},
            "max_inclusive": {"absolute": y_max},
        }},
        {"type": "minecraft:biome"},
    ]
    return {"feature": f"{ID}:{feature}", "placement": modifiers}

# The Abyss lives at -73..-564; these ranges sit inside its voidstone.
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_void_crystal_underbedrock.json",
      placed("ore_void_crystal_voidstone", 14, -545, -85))
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_lamp_underbedrock.json",
      placed("ore_lamp_voidstone", 8, -520, -90))
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_gravel_underbedrock.json",
      placed("ore_gravel_voidstone", 10, -540, -85))
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_coal_underbedrock.json",
      placed("ore_coal_voidstone", 12, -540, -90))
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_iron_underbedrock.json",
      placed("ore_iron_voidstone", 8, -540, -100))
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_diamond_underbedrock.json",
      placed("ore_diamond_voidstone", 4, -545, -120))
# A few veins in the ordinary deepslate, so the first alloy needs no bedrock.
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_void_crystal_overworld.json",
      placed("ore_void_crystal_overworld", 3, -60, -16, rarity=3))

write(MOD / f"data/{ID}/neoforge/biome_modifier/underbedrock_ores.json", {
    "type": "neoforge:add_features",
    "biomes": "#minecraft:is_overworld",
    "features": [
        f"{ID}:ore_void_crystal_underbedrock",
        f"{ID}:ore_lamp_underbedrock",
        f"{ID}:ore_gravel_underbedrock",
        f"{ID}:ore_coal_underbedrock",
        f"{ID}:ore_iron_underbedrock",
        f"{ID}:ore_diamond_underbedrock",
    ],
    "step": "underground_ores",
})
write(MOD / f"data/{ID}/neoforge/biome_modifier/overworld_void_crystal.json", {
    "type": "neoforge:add_features",
    "biomes": "#minecraft:is_overworld",
    "features": f"{ID}:ore_void_crystal_overworld",
    "step": "underground_ores",
})

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
write(MOD / f"assets/{ID}/models/item/void_alloy_pickaxe.json",
      {"parent": "minecraft:item/handheld", "textures": {"layer0": f"{ID}:item/void_alloy_pickaxe"}})
write(MOD / f"assets/{ID}/items/void_alloy_pickaxe.json",
      {"model": {"type": "minecraft:model", "model": f"{ID}:item/void_alloy_pickaxe"}})

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
    "ambient_light": 0.0,
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
write(DF / "carve.json", {
    # Positive where the Abyss is hollow: cheese blobs, or spaghetti tunnels
    # where the tunnel noise crosses zero.
    "type": "minecraft:max",
    "left": {"type": "minecraft:sub", "left": f"{ID}:cheese", "right": 0.25},
    "right": {
        "type": "minecraft:sub",
        "left": 0.09,
        "right": {"type": "minecraft:abs", "input": f"{ID}:tunnels"},
    },
})
write(DF / "carve_big.json", {
    # The wide caverns of the middle depths.
    "type": "minecraft:sub",
    "left": f"{ID}:cheese_big",
    "right": 0.35,
})
write(DF / "cheese.json", {
    "type": "minecraft:noise",
    "noise": f"{ID}:cheese",
    "xz_scale": 1.0,
    "y_scale": 1.6,
})
write(DF / "tunnels.json", {
    "type": "minecraft:noise",
    "noise": f"{ID}:tunnels",
    "xz_scale": 0.9,
    "y_scale": 0.9,
})
write(DF / "cheese_big.json", {
    "type": "minecraft:noise",
    "noise": f"{ID}:cheese_big",
    "xz_scale": 0.6,
    "y_scale": 0.8,
})
for name, octave, octaves in (("cheese", -6, 4), ("tunnels", -5, 3),
                              ("cheese_big", -8, 3), ("floodedness", -3, 1),
                              ("spread", -4, 1)):
    write(PACK / f"data/{ID}/worldgen/noise/{name}.json", {
        "base_octave": octave,
        "octave_count": octaves,
        "base_amplitude": 1.0,
    })

AIR = {"type": "minecraft:block", "result_state": "minecraft:air"}
MR = PACK / f"data/{ID}/worldgen/material_rule"
write(MR / "surface.json", {
    "type": "minecraft:sequence",
    "sequence": [
        # The Overworld's bedrock floor, a thin jagged band at -64..-59: the
        # gradient alone is true at every depth below it, so the y_above under
        # it keeps the band where a transition belongs.
        {
            "type": "minecraft:condition",
            "if_true": {
                "type": "minecraft:vertical_gradient",
                "random_name": f"{ID}:floor_bedrock",
                "true_at_and_below": {"absolute": -64},
                "false_at_and_above": {"absolute": -59},
            },
            "then_run": {
                "type": "minecraft:condition",
                "if_true": {
                    "type": "minecraft:y_above",
                    "anchor": {"absolute": -64},
                    "surface_depth_multiplier": 0,
                    "add_stone_depth": False,
                },
                "then_run": {"type": "minecraft:block", "result_state": "minecraft:bedrock"},
            },
        },
        # Everything below y -55: the Abyss, depth by depth.
        {
            "type": "minecraft:condition",
            "if_true": {
                "type": "minecraft:not",
                "invert": {
                    "type": "minecraft:y_above",
                    "anchor": {"absolute": -55},
                    "surface_depth_multiplier": 0,
                    "add_stone_depth": False,
                },
            },
            "then_run": {
                "type": "minecraft:sequence",
                "sequence": [
                    # The world's bottom: bedrock no pickaxe digs.
                    {
                        "type": "minecraft:condition",
                        "if_true": {
                            "type": "minecraft:not",
                            "invert": {
                                "type": "minecraft:y_above",
                                "anchor": {"absolute": -564},
                                "surface_depth_multiplier": 0,
                                "add_stone_depth": False,
                            },
                        },
                        "then_run": {"type": "minecraft:block", "result_state": f"{ID}:abyss_bedrock"},
                    },
                    # The carved world itself.
                    {
                        "type": "minecraft:condition",
                        "if_true": {
                            "type": "minecraft:not",
                            "invert": {
                                "type": "minecraft:y_above",
                                "anchor": {"absolute": -72},
                                "surface_depth_multiplier": 0,
                                "add_stone_depth": False,
                            },
                        },
                        "then_run": {
                            "type": "minecraft:sequence",
                            "sequence": [
                                # Wide caverns in the middle depths.
                                {
                                    "type": "minecraft:condition",
                                    "if_true": {
                                        "type": "minecraft:not",
                                        "invert": {
                                            "type": "minecraft:y_above",
                                            "anchor": {"absolute": -139},
                                            "surface_depth_multiplier": 0,
                                            "add_stone_depth": False,
                                        },
                                    },
                                    "then_run": {
                                        "type": "minecraft:condition",
                                        "if_true": {
                                            "type": "minecraft:y_above",
                                            "anchor": {"absolute": -420},
                                            "surface_depth_multiplier": 0,
                                            "add_stone_depth": False,
                                        },
                                        "then_run": {
                                            "type": "minecraft:condition",
                                            "if_true": {
                                                "type": "minecraft:noise_threshold",
                                                "noise": f"{ID}:carve_big",
                                                "min_threshold": 0.0,
                                                "max_threshold": 1000000.0,
                                                "is_3d": True,
                                            },
                                            "then_run": AIR,
                                        },
                                    },
                                },
                                # Cheese caves and spaghetti tunnels.
                                {
                                    "type": "minecraft:condition",
                                    "if_true": {
                                        "type": "minecraft:noise_threshold",
                                        "noise": f"{ID}:carve",
                                        "min_threshold": 0.0,
                                        "max_threshold": 1000000.0,
                                        "is_3d": True,
                                    },
                                    "then_run": AIR,
                                },
                                {"type": "minecraft:block", "result_state": f"{ID}:voidstone"},
                            ],
                        },
                    },
                    # The gate under the bedrock floor: voidshell, -65..-72.
                    {
                        "type": "minecraft:condition",
                        "if_true": {
                            "type": "minecraft:not",
                            "invert": {
                                "type": "minecraft:y_above",
                                "anchor": {"absolute": -65},
                                "surface_depth_multiplier": 0,
                                "add_stone_depth": False,
                            },
                        },
                        "then_run": {"type": "minecraft:block", "result_state": f"{ID}:voidshell"},
                    },
                ],
            },
        },
        # Everything above: vanilla's rules, unchanged.
        "minecraft:overworld",
    ],
})

# The vanilla carvers would carve tunnels into the solid Abyss and the engine
# would fill them with its deep lava sea, so the pack replaces the three
# Overworld carvers with ones that never start. The Abyss's caves are the
# material rule's; the surface keeps vanilla's density caves.
CARVER_Y = {"type": "minecraft:constant", "value": -55}
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
