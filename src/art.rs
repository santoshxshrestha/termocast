use std::collections::HashMap;

#[derive(Debug)]
pub struct AsciiArt {
    art: HashMap<String, String>,
}

impl Default for AsciiArt {
    fn default() -> Self {
        let mut art = HashMap::new();
        art.insert(
            "sunny".to_string(),
            r"
    \   /
     .-.
  ― (   ) ―
     `-'
    /   \
                "
            .to_string(),
        );
        art.insert(
            "cloudy".to_string(),
            r"
      .--.
   .-(    ).
  (___.__)__)
                "
            .to_string(),
        );
        art.insert(
            "rainy".to_string(),
            r"
      .--.
   .-(    ).
  (___.__)__)
   ' ' ' ' '
                "
            .to_string(),
        );
        art.insert(
            "stormy".to_string(),
            r"
      .--.
   .-(    ).
  (___.__)__)
   ⚡⚡⚡⚡⚡
                "
            .to_string(),
        );
        art.insert(
            "snowy".to_string(),
            r"
      .--.
   .-(    ).
  (___.__)__)
   * * * * *
                "
            .to_string(),
        );
        Self { art }
    }
}
impl AsciiArt {
    pub fn get_art(&self, condition: &str) -> &str {
        match condition {
            "Clear" => self.art.get("sunny").unwrap(),
            "Clouds" => self.art.get("cloudy").unwrap(),
            "Rain" | "Drizzle" => self.art.get("rainy").unwrap(),
            "Thunderstorm" => self.art.get("stormy").unwrap(),
            "Snow" => self.art.get("snowy").unwrap(),
            _ => "No art available for this condition",
        }
    }
}
