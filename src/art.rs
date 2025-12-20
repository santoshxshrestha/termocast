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
