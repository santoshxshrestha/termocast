use std::collections::HashMap;

#[derive(Debug)]
pub struct AsciiArt {
    art: HashMap<String, String>,
}

impl Default for AsciiArt {
    fn default() -> Self {
        let mut art = HashMap::new();
        art.insert(
            "smoke".to_string(),
            r"
      (  .      )
     )           (              )
            .  '   .   '  .
  (    , )       (.   )  (   ',    )
   .' ) ( . )    ,  ( ,     )   ( .
). , ( .   (  ) ( , ')  .' (  ,    )
(_,) . ), ) _) _,')  (, ) '. )  ,. (' )
                "
            .to_string(),
        );
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
            c if c.contains("clear") => self.art.get("sunny").unwrap(),
            c if c.contains("clouds") => self.art.get("cloudy").unwrap(),
            c if c.contains("rain") || c.contains("Dirzzle") => self.art.get("rainy").unwrap(),
            c if c.contains("thunderstorm") => self.art.get("stormy").unwrap(),
            c if c.contains("snow") => self.art.get("snowy").unwrap(),
            c if c.contains("smoke") || c.contains("haze") || c.contains("fog") => {
                self.art.get("smoke").unwrap()
            }
            _ => "No art available for this condition",
        }
    }
}
