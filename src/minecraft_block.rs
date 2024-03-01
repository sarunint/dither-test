use std::collections::HashMap;

#[derive(Clone)]
pub struct MinecraftBlock<'a> {
    pub name: &'a str,
    pub state: HashMap<&'a str, &'a str>,
}

#[derive(Clone)]
pub struct ColumnPosition {
    pub z: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct ColumnBlock<'a> {
    pub position: ColumnPosition,
    pub block: MinecraftBlock<'a>,
}

pub fn get_block_for_colour_index_hashmap<'a>() -> HashMap<i32, MinecraftBlock<'a>> {
    [
        (
            0,
            MinecraftBlock {
                name: "minecraft:grass_block",
                state: [].into(),
            },
        ),
        (
            1,
            MinecraftBlock {
                name: "minecraft:birch_planks",
                state: [].into(),
            },
        ),
        (
            2,
            MinecraftBlock {
                name: "minecraft:mushroom_stem",
                state: [].into(),
            },
        ),
        (
            3,
            MinecraftBlock {
                name: "minecraft:redstone_block",
                state: [].into(),
            },
        ),
        (
            4,
            MinecraftBlock {
                name: "minecraft:packed_ice",
                state: [].into(),
            },
        ),
        (
            5,
            MinecraftBlock {
                name: "minecraft:iron_block",
                state: [].into(),
            },
        ),
        (
            6,
            MinecraftBlock {
                name: "minecraft:oak_leaves",
                state: [("persistent", "true")].into(),
            },
        ),
        (
            7,
            MinecraftBlock {
                name: "minecraft:white_carpet",
                state: [].into(),
            },
        ),
        (
            8,
            MinecraftBlock {
                name: "minecraft:clay",
                state: [].into(),
            },
        ),
        (
            9,
            MinecraftBlock {
                name: "minecraft:dirt",
                state: [].into(),
            },
        ),
        (
            10,
            MinecraftBlock {
                name: "minecraft:cobblestone",
                state: [].into(),
            },
        ),
        (
            11,
            MinecraftBlock {
                name: "minecraft:oak_leaves",
                state: [("persistent", "true"), ("waterlogged", "true")].into(),
            },
        ),
        (
            12,
            MinecraftBlock {
                name: "minecraft:oak_planks",
                state: [].into(),
            },
        ),
        (
            13,
            MinecraftBlock {
                name: "minecraft:quartz_block",
                state: [].into(),
            },
        ),
        (
            14,
            MinecraftBlock {
                name: "minecraft:orange_carpet",
                state: [].into(),
            },
        ),
        (
            15,
            MinecraftBlock {
                name: "minecraft:magenta_carpet",
                state: [].into(),
            },
        ),
        (
            16,
            MinecraftBlock {
                name: "minecraft:light_blue_carpet",
                state: [].into(),
            },
        ),
        (
            17,
            MinecraftBlock {
                name: "minecraft:yellow_carpet",
                state: [].into(),
            },
        ),
        (
            18,
            MinecraftBlock {
                name: "minecraft:lime_carpet",
                state: [].into(),
            },
        ),
        (
            19,
            MinecraftBlock {
                name: "minecraft:pink_carpet",
                state: [].into(),
            },
        ),
        (
            20,
            MinecraftBlock {
                name: "minecraft:gray_carpet",
                state: [].into(),
            },
        ),
        (
            21,
            MinecraftBlock {
                name: "minecraft:light_gray_carpet",
                state: [].into(),
            },
        ),
        (
            22,
            MinecraftBlock {
                name: "minecraft:cyan_carpet",
                state: [].into(),
            },
        ),
        (
            23,
            MinecraftBlock {
                name: "minecraft:purple_carpet",
                state: [].into(),
            },
        ),
        (
            24,
            MinecraftBlock {
                name: "minecraft:blue_carpet",
                state: [].into(),
            },
        ),
        (
            25,
            MinecraftBlock {
                name: "minecraft:brown_carpet",
                state: [].into(),
            },
        ),
        (
            26,
            MinecraftBlock {
                name: "minecraft:green_carpet",
                state: [].into(),
            },
        ),
        (
            27,
            MinecraftBlock {
                name: "minecraft:red_carpet",
                state: [].into(),
            },
        ),
        (
            28,
            MinecraftBlock {
                name: "minecraft:black_carpet",
                state: [].into(),
            },
        ),
        (
            29,
            MinecraftBlock {
                name: "minecraft:gold_block",
                state: [].into(),
            },
        ),
        (
            30,
            MinecraftBlock {
                name: "minecraft:prismarine_bricks",
                state: [].into(),
            },
        ),
        (
            31,
            MinecraftBlock {
                name: "minecraft:lapis_block",
                state: [].into(),
            },
        ),
        (
            32,
            MinecraftBlock {
                name: "minecraft:emerald_block",
                state: [].into(),
            },
        ),
        (
            33,
            MinecraftBlock {
                name: "minecraft:spruce_planks",
                state: [].into(),
            },
        ),
        (
            34,
            MinecraftBlock {
                name: "minecraft:netherrack",
                state: [].into(),
            },
        ),
        (
            35,
            MinecraftBlock {
                name: "minecraft:white_terracotta",
                state: [].into(),
            },
        ),
        (
            36,
            MinecraftBlock {
                name: "minecraft:orange_terracotta",
                state: [].into(),
            },
        ),
        (
            37,
            MinecraftBlock {
                name: "minecraft:magenta_terracotta",
                state: [].into(),
            },
        ),
        (
            38,
            MinecraftBlock {
                name: "minecraft:light_blue_terracotta",
                state: [].into(),
            },
        ),
        (
            39,
            MinecraftBlock {
                name: "minecraft:yellow_terracotta",
                state: [].into(),
            },
        ),
        (
            40,
            MinecraftBlock {
                name: "minecraft:lime_terracotta",
                state: [].into(),
            },
        ),
        (
            41,
            MinecraftBlock {
                name: "minecraft:pink_terracotta",
                state: [].into(),
            },
        ),
        (
            42,
            MinecraftBlock {
                name: "minecraft:gray_terracotta",
                state: [].into(),
            },
        ),
        (
            43,
            MinecraftBlock {
                name: "minecraft:light_gray_terracotta",
                state: [].into(),
            },
        ),
        (
            44,
            MinecraftBlock {
                name: "minecraft:cyan_terracotta",
                state: [].into(),
            },
        ),
        (
            45,
            MinecraftBlock {
                name: "minecraft:purple_terracotta",
                state: [].into(),
            },
        ),
        (
            46,
            MinecraftBlock {
                name: "minecraft:blue_terracotta",
                state: [].into(),
            },
        ),
        (
            47,
            MinecraftBlock {
                name: "minecraft:brown_terracotta",
                state: [].into(),
            },
        ),
        (
            48,
            MinecraftBlock {
                name: "minecraft:green_terracotta",
                state: [].into(),
            },
        ),
        (
            49,
            MinecraftBlock {
                name: "minecraft:red_terracotta",
                state: [].into(),
            },
        ),
        (
            50,
            MinecraftBlock {
                name: "minecraft:black_terracotta",
                state: [].into(),
            },
        ),
        (
            51,
            MinecraftBlock {
                name: "minecraft:crimson_nylium",
                state: [].into(),
            },
        ),
        (
            52,
            MinecraftBlock {
                name: "minecraft:crimson_planks",
                state: [].into(),
            },
        ),
        (
            53,
            MinecraftBlock {
                name: "minecraft:crimson_hyphae",
                state: [].into(),
            },
        ),
        (
            54,
            MinecraftBlock {
                name: "minecraft:warped_nylium",
                state: [].into(),
            },
        ),
        (
            55,
            MinecraftBlock {
                name: "minecraft:warped_planks",
                state: [].into(),
            },
        ),
        (
            56,
            MinecraftBlock {
                name: "minecraft:warped_hyphae",
                state: [].into(),
            },
        ),
        (
            57,
            MinecraftBlock {
                name: "minecraft:warped_wart_block",
                state: [].into(),
            },
        ),
        (
            58,
            MinecraftBlock {
                name: "minecraft:deepslate",
                state: [].into(),
            },
        ),
        (
            59,
            MinecraftBlock {
                name: "minecraft:raw_iron_block",
                state: [].into(),
            },
        ),
        (
            60,
            MinecraftBlock {
                name: "minecraft:glow_lichen",
                state: [("down", "true")].into(),
            },
        ),
        (
            -1,
            MinecraftBlock {
                name: "minecraft:cobblestonoe",
                state: [].into(),
            },
        ),
    ]
    .into()
}
