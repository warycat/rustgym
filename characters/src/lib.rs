pub const CHARACTERS: &[&str] = &[
    "Mario",
    "Link",
    "Princess Zelda",
    "Princess Peach",
    "Rayman",
    "Diddy Kong",
    "Donkey Kong",
    "Spyro",
    "Sonic",
    "Banjo",
    "Pikachu",
    "Bowser",
    "Megaman",
    "Kratos",
    "Pacman",
    "Geralt of Rivia",
    "Lara Croft",
];

#[cfg(test)]
mod tests {
    use super::CHARACTERS;
    #[test]
    fn characters() {
        assert_eq!(CHARACTERS.len(), 17);
    }
}
