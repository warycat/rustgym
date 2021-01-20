pub const CHARACTERS: &[&str] = &[
    "Banjo",
    "Bowser",
    "Ciri",
    "Diddy Kong",
    "Donkey Kong",
    "Geralt of Rivia",
    "Kratos",
    "Lara Croft",
    "Link",
    "Mario",
    "Megaman",
    "Pacman",
    "Pikachu",
    "Princess Zelda",
    "Princess Peach",
    "Rayman",
    "Sonic",
    "Spyro",
    "Yennefer",
    "Yoshi",
];

#[cfg(test)]
mod tests {
    use super::CHARACTERS;
    #[test]
    fn characters() {
        assert_eq!(CHARACTERS.len(), 17);
    }
}
