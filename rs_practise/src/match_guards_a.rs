#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    contents: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn print_tile(tile: Tile) {
    use Tile::*;

    match tile {

        Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
            println!("The brick color is {brick:?}")
        }

        Brick(other) => println!("{other:?} brick"),

        Dirt | Grass | Sand => println!("The tile is {tile:?}"),

        Treasure(TreasureChest {
            contents: TreasureItem::Gold,
            amount,
        }) if amount > 100 => {
            println!("The treasure chest contains gold and has more than 100 items");
        }
        Water(pressure) if pressure.0 > 100 => {
            println!("The water pressure is {}", pressure.0);
        }

        _ => println!("The tile is {:?}", tile),
    }
}

pub fn match_guards_a() {
    let tile = Tile::Brick(BrickStyle::Red);
    print_tile(tile);

    let tile = Tile::Sand;
    print_tile(tile);

    let tile = Tile::Treasure(TreasureChest {
        contents: TreasureItem::Gold,
        amount: 101,
    });
    print_tile(tile);

    let tile = Tile::Water(Pressure(101));
    print_tile(tile);

    let tile = Tile::Wood;
    print_tile(tile);

    let tile = Tile::Brick(BrickStyle::Gray);
    print_tile(tile);

    let tile = Tile::Treasure(TreasureChest {
        contents: TreasureItem::SuperPower,
        amount: 100,
    });
    print_tile(tile);

    let tile = Tile::Water(Pressure(99));
    print_tile(tile);

    let tile = Tile::Brick(BrickStyle::Dungeon);
    print_tile(tile);

    let tile = Tile::Dirt;
    print_tile(tile);

    let tile = Tile::Grass;
    print_tile(tile);

    let tile = Tile::Sand;
    print_tile(tile);
}
