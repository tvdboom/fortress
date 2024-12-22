mod player;


struct Player {
    name: String,
    resources: Resources,
    weapons: Vec<Box<dyn Weapon>>,
}


struct Resources {
    bullets: u32,
    gasoline: u32,
    raw_material: u32,
}
