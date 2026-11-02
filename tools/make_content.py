#!/usr/bin/env python3
"""Write every JSON/TOML file of the Underbedrock content mod and its data pack.

Run: python3 tools/make_content.py
The files are the deliverable; this script only keeps them reproducible.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MOD = ROOT / "content-mod"
PACK = ROOT / "datapack" / "underbedrock_world"

ID = "underbedrock"


def write(path: Path, data) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if isinstance(data, str):
        path.write_text(data, encoding="utf-8")
    else:
        path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


# --------------------------------------------------------------------------- manifest
write(MOD / "horizon-mod.toml", f'''id = "{ID}"
name = "Underbedrock"
version = "0.1.0"
description = "A world under the bedrock floor: voidstone caverns, etherium ore, and an enchanted-alloy pickaxe that breaks bedrock."
authors = ["you"]
# A content mod: blocks, items and mob effects from data/ and assets/, no code (API 0.2).
api = "0.2"
side = "both"
# registries: "May add blocks, items, mob effects and entity types to your worlds".
capabilities = ["registries"]
''')

# --------------------------------------------------------------------------- blocks
BLOCKS = {
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
    "etherium_ore": {
        "template": "minecraft:diamond_ore",
        "properties": {
            "hardness": 3.5,
            "explosion_resistance": 3.0,
            "requires_tool": True,
            "light": 3,
            "sounds": "minecraft:deepslate",
        },
        "item": True,
    },
    "etherium_block": {
        "template": "minecraft:netherite_block",
        "properties": {
            "hardness": 6.0,
            "explosion_resistance": 12.0,
            "requires_tool": True,
            "light": 4,
            "sounds": "minecraft:netherite_block",
        },
        "item": True,
    },
    "ether_crystal": {
        "template": "minecraft:amethyst_block",
        "properties": {
            "hardness": 1.5,
            "explosion_resistance": 1.5,
            "requires_tool": True,
            "light": 12,
            "sounds": "minecraft:amethyst_block",
        },
        "item": True,
    },
}
for name, body in BLOCKS.items():
    write(MOD / f"data/{ID}/horizon/block/{name}.json", body)

# --------------------------------------------------------------------------- items
write(MOD / f"data/{ID}/horizon/item/raw_etherium.json",
      {"template": "minecraft:raw_iron", "rarity": "uncommon"})
write(MOD / f"data/{ID}/horizon/item/etherium_ingot.json",
      {"template": "minecraft:iron_ingot", "rarity": "uncommon"})
# The pickaxe that breaks bedrock. Its tool rules replace the netherite pickaxe's:
# one rule over the whole mineable/pickaxe tag, faster and top-tier, so it also
# harvests the mod's blocks (they join that tag below).
write(MOD / f"data/{ID}/horizon/item/etherium_pickaxe.json", {
    "template": "minecraft:netherite_pickaxe",
    "rarity": "epic",
    "components": {
        "minecraft:max_damage": 2800,
        "minecraft:enchantment_glint_override": True,
        "minecraft:enchantable": {"value": 22},
        "minecraft:repairable": {"items": f"{ID}:etherium_ingot"},
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
    "icon": f"{ID}:etherium_pickaxe",
    "items": [
        f"{ID}:etherium_pickaxe",
        f"{ID}:etherium_block",
        f"{ID}:etherium_ingot",
        f"{ID}:raw_etherium",
        f"{ID}:etherium_ore",
        f"{ID}:ether_crystal",
        f"{ID}:voidstone",
        f"{ID}:voidstone_bricks",
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

for name in ("voidstone", "voidstone_bricks", "etherium_block", "ether_crystal"):
    write(MOD / f"data/{ID}/loot_table/blocks/{name}.json", drops_itself(name))
write(MOD / f"data/{ID}/loot_table/blocks/etherium_ore.json", {
    "type": "minecraft:block",
    "pools": [{
        "condition": {"type": "minecraft:survives_explosion"},
        "entries": [{"type": "minecraft:item", "name": f"{ID}:raw_etherium"}],
        "rolls": 1,
    }],
    "random_sequence": f"{ID}:blocks/etherium_ore",
})

# --------------------------------------------------------------------------- recipes
write(MOD / f"data/{ID}/recipe/etherium_ingot_from_smelting.json", {
    "type": "minecraft:smelting",
    "category": "misc",
    "ingredient": f"{ID}:raw_etherium",
    "result": {"id": f"{ID}:etherium_ingot"},
    "experience": 1.2,
    "cookingtime": 200,
})
write(MOD / f"data/{ID}/recipe/etherium_ingot_from_blasting.json", {
    "type": "minecraft:blasting",
    "category": "misc",
    "ingredient": f"{ID}:raw_etherium",
    "result": {"id": f"{ID}:etherium_ingot"},
    "experience": 1.2,
    "cookingtime": 100,
})
write(MOD / f"data/{ID}/recipe/etherium_pickaxe.json", {
    "type": "minecraft:crafting_shaped",
    "category": "equipment",
    "group": "etherium_pickaxe",
    "pattern": ["EEE", " R ", " R "],
    "key": {"E": f"{ID}:etherium_ingot", "R": "minecraft:blaze_rod"},
    "result": {"id": f"{ID}:etherium_pickaxe"},
})
write(MOD / f"data/{ID}/recipe/etherium_block.json", {
    "type": "minecraft:crafting_shaped",
    "category": "building",
    "pattern": ["EEE", "EEE", "EEE"],
    "key": {"E": f"{ID}:etherium_ingot"},
    "result": {"id": f"{ID}:etherium_block"},
})
write(MOD / f"data/{ID}/recipe/etherium_ingot_from_block.json", {
    "type": "minecraft:crafting_shapeless",
    "category": "misc",
    "ingredients": [f"{ID}:etherium_block"],
    "result": {"id": f"{ID}:etherium_ingot", "count": 9},
})
write(MOD / f"data/{ID}/recipe/voidstone_bricks.json", {
    "type": "minecraft:crafting_shaped",
    "category": "building",
    "pattern": ["VV", "VV"],
    "key": {"V": f"{ID}:voidstone"},
    "result": {"id": f"{ID}:voidstone_bricks", "count": 4},
})

# --------------------------------------------------------------------------- tags
write(MOD / "data/minecraft/tags/block/mineable/pickaxe.json", {
    "values": [
        f"{ID}:voidstone",
        f"{ID}:voidstone_bricks",
        f"{ID}:etherium_ore",
        f"{ID}:etherium_block",
        f"{ID}:ether_crystal",
    ]
})
# Iron tools and better harvest the alloy's ore and block.
write(MOD / "data/minecraft/tags/block/needs_iron_tool.json", {
    "values": [f"{ID}:etherium_ore", f"{ID}:etherium_block"]
})

# --------------------------------------------------------------------------- worldgen features
def ore(name, targets, size, discard=0.0):
    return {
        "type": "minecraft:ore",
        "config": {
            "size": size,
            "discard_chance_on_air_exposure": discard,
            "targets": [{"target": t, "state": {"Name": s}} for t, s in targets],
        },
    }

VOID = {"predicate_type": "minecraft:block_match", "block": f"{ID}:voidstone"}
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_etherium_voidstone.json",
      ore("etherium", [(VOID, f"{ID}:etherium_ore")], 9))
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_etherium_overworld.json", ore("etherium", [
    ({"predicate_type": "minecraft:tag_match", "tag": "minecraft:deepslate_ore_replaceables"}, f"{ID}:etherium_ore"),
    ({"predicate_type": "minecraft:tag_match", "tag": "minecraft:stone_ore_replaceables"}, f"{ID}:etherium_ore"),
], 5))
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_crystal_voidstone.json",
      ore("crystal", [(VOID, f"{ID}:ether_crystal")], 6))
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_diamond_voidstone.json",
      ore("diamond", [(VOID, "minecraft:diamond_ore")], 6))
write(MOD / f"data/{ID}/worldgen/configured_feature/ore_iron_voidstone.json",
      ore("iron", [(VOID, "minecraft:iron_ore")], 8))


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

# The world under the bedrock floor lives at -576..-64; these ranges sit inside it.
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_etherium_underbedrock.json",
      placed("ore_etherium_voidstone", 14, -540, -96))
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_crystal_underbedrock.json",
      placed("ore_crystal_voidstone", 10, -520, -90))
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_diamond_underbedrock.json",
      placed("ore_diamond_voidstone", 4, -540, -120))
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_iron_underbedrock.json",
      placed("ore_iron_voidstone", 10, -540, -100))
# A few veins in the ordinary deepslate, so the first alloy needs no bedrock.
write(MOD / f"data/{ID}/worldgen/placed_feature/ore_etherium_overworld.json",
      placed("ore_etherium_overworld", 3, -60, -16, rarity=3))

write(MOD / f"data/{ID}/neoforge/biome_modifier/underbedrock_ores.json", {
    "type": "neoforge:add_features",
    "biomes": "#minecraft:is_overworld",
    "features": [
        f"{ID}:ore_etherium_underbedrock",
        f"{ID}:ore_crystal_underbedrock",
        f"{ID}:ore_diamond_underbedrock",
        f"{ID}:ore_iron_underbedrock",
    ],
    "step": "underground_ores",
})
write(MOD / f"data/{ID}/neoforge/biome_modifier/overworld_etherium.json", {
    "type": "neoforge:add_features",
    "biomes": "#minecraft:is_overworld",
    "features": f"{ID}:ore_etherium_overworld",
    "step": "underground_ores",
})

# --------------------------------------------------------------------------- assets
EN = {
    f"block.{ID}.voidstone": "Voidstone",
    f"block.{ID}.voidstone_bricks": "Voidstone Bricks",
    f"block.{ID}.etherium_ore": "Etherium Ore",
    f"block.{ID}.etherium_block": "Block of Etherium",
    f"block.{ID}.ether_crystal": "Ether Crystal",
    f"item.{ID}.raw_etherium": "Raw Etherium",
    f"item.{ID}.etherium_ingot": "Enchanted Alloy Ingot",
    f"item.{ID}.etherium_pickaxe": "Enchanted Alloy Pickaxe",
    f"itemGroup.{ID}": "Underbedrock",
}
RU = {
    f"block.{ID}.voidstone": "Пустотный камень",
    f"block.{ID}.voidstone_bricks": "Пустотный кирпич",
    f"block.{ID}.etherium_ore": "Эфириумовая руда",
    f"block.{ID}.etherium_block": "Эфириумовый блок",
    f"block.{ID}.ether_crystal": "Эфирный кристалл",
    f"item.{ID}.raw_etherium": "Необработанный эфириум",
    f"item.{ID}.etherium_ingot": "Слиток зачарованного сплава",
    f"item.{ID}.etherium_pickaxe": "Кирка из зачарованного сплава",
    f"itemGroup.{ID}": "Подбедрок",
}
write(MOD / f"assets/{ID}/lang/en_us.json", EN)
write(MOD / f"assets/{ID}/lang/ru_ru.json", RU)

for name in BLOCKS:
    write(MOD / f"assets/{ID}/blockstates/{name}.json",
          {"variants": {"": {"model": f"{ID}:block/{name}"}}})
    write(MOD / f"assets/{ID}/models/block/{name}.json",
          {"parent": "minecraft:block/cube_all", "textures": {"all": f"{ID}:block/{name}"}})
    write(MOD / f"assets/{ID}/items/{name}.json",
          {"model": {"type": "minecraft:model", "model": f"{ID}:block/{name}"}})

for name, parent in (("raw_etherium", "minecraft:item/generated"),
                     ("etherium_ingot", "minecraft:item/generated")):
    write(MOD / f"assets/{ID}/models/item/{name}.json",
          {"parent": parent, "textures": {"layer0": f"{ID}:item/{name}"}})
    write(MOD / f"assets/{ID}/items/{name}.json",
          {"model": {"type": "minecraft:model", "model": f"{ID}:item/{name}"}})
write(MOD / f"assets/{ID}/models/item/etherium_pickaxe.json",
      {"parent": "minecraft:item/handheld", "textures": {"layer0": f"{ID}:item/etherium_pickaxe"}})
write(MOD / f"assets/{ID}/items/etherium_pickaxe.json",
      {"model": {"type": "minecraft:model", "model": f"{ID}:item/etherium_pickaxe"}})

# =========================================================================== the data pack
# The pack that gives the Overworld room under its bedrock floor and generates
# the world there. Installed per world (or as a feature pack); see README.md.
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
    # Above the bedrock floor: vanilla's terrain, untouched. Below it: the mod's.
    "type": "minecraft:range_choice",
    "input": "minecraft:y",
    "min_inclusive": -1000000.0,
    "max_exclusive": -64.0,
    "when_in_range": f"{ID}:layer",
    "when_out_of_range": "minecraft:overworld/final_density",
})
write(DF / "layer.json", {
    # Solid where the cheese noise is low, hollow where it is high; the ramps
    # keep a stone floor at the bottom of the world and a stone ceiling under
    # the bedrock floor.
    "type": "minecraft:add",
    "left": {"type": "minecraft:max", "left": f"{ID}:floor_ramp", "right": f"{ID}:ceiling_ramp"},
    "right": {"type": "minecraft:sub", "left": -0.25, "right": f"{ID}:cheese"},
})
write(DF / "floor_ramp.json", {
    "type": "minecraft:gradient",
    "axis": "y",
    "from_coordinate": -576,
    "from_value": 4.0,
    "to_coordinate": -552,
    "to_value": 0.0,
})
write(DF / "ceiling_ramp.json", {
    "type": "minecraft:gradient",
    "axis": "y",
    "from_coordinate": -96,
    "from_value": 0.0,
    "to_coordinate": -64,
    "to_value": 4.0,
})
write(DF / "cheese.json", {
    "type": "minecraft:noise",
    "noise": f"{ID}:cheese",
    "xz_scale": 1.0,
    "y_scale": 2.0,
})
write(PACK / f"data/{ID}/worldgen/noise/cheese.json", {
    "base_octave": -7,
    "octave_count": 4,
    "base_amplitude": 1.0,
})

MR = PACK / f"data/{ID}/worldgen/material_rule"
write(MR / "surface.json", {
    "type": "minecraft:sequence",
    "sequence": [
        # The bottom of the taller world gets the floor bedrock would give it.
        {
            "type": "minecraft:condition",
            "if_true": {
                "type": "minecraft:vertical_gradient",
                "random_name": f"{ID}:bottom_bedrock",
                "true_at_and_below": {"absolute": -576},
                "false_at_and_above": {"absolute": -571},
            },
            "then_run": {"type": "minecraft:block", "result_state": {"Name": "minecraft:bedrock"}},
        },
        # The Overworld keeps its bedrock floor at -64..-59: vanilla's own floor
        # rule now sits at the level's bottom, so the old floor is placed here.
        {
            "type": "minecraft:condition",
            "if_true": {
                "type": "minecraft:vertical_gradient",
                "random_name": f"{ID}:floor_bedrock",
                "true_at_and_below": {"absolute": -64},
                "false_at_and_above": {"absolute": -59},
            },
            "then_run": {"type": "minecraft:block", "result_state": {"Name": "minecraft:bedrock"}},
        },
        # Below the floor: the mod's own world.
        {
            "type": "minecraft:condition",
            "if_true": {
                "type": "minecraft:not",
                "invert": {
                    "type": "minecraft:y_above",
                    "anchor": {"absolute": -64},
                    "surface_depth_multiplier": 0,
                    "add_stone_depth": False,
                },
            },
            "then_run": f"{ID}:underbedrock",
        },
        # Everything above: vanilla's rules, unchanged.
        "minecraft:overworld",
    ],
})
write(MR / "underbedrock.json",
      {"type": "minecraft:block", "result_state": {"Name": f"{ID}:voidstone"}})

print("wrote the content mod and the data pack")
