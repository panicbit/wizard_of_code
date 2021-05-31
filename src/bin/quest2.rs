// Title: The Potion Making

fn main() {
    let mut num_potions = 0;
    let mut ingredients = Ingredients {
        snake: 799,
        beetle: 567,
        durian: 677,
    };

    num_potions += ingredients.cook_max(&INTENSITY_3);
    num_potions += ingredients.cook_max(&INTENSITY_2);
    num_potions += ingredients.cook_max(&INTENSITY_1);

    println!("The wizard can make {} potions!", num_potions);
}

struct Ingredients {
    snake: u32,
    beetle: u32,
    durian: u32,
}

impl Ingredients {
    fn cook_once(&mut self, recipe: &Ingredients) -> bool {
        let enough_snake = self.snake >= recipe.snake;
        let enough_beetle = self.beetle >= recipe.beetle;
        let enough_durian = self.durian >= recipe.durian;
        let enough_ingredients = enough_snake && enough_beetle && enough_durian;

        if enough_ingredients {
            self.snake -= recipe.snake;
            self.beetle -= recipe.beetle;
            self.durian -= recipe.durian;
        }

        enough_ingredients
    }

    fn cook_max(&mut self, recipe: &Ingredients) -> u32 {
        let mut num_cooked = 0;

        while self.cook_once(recipe) {
            num_cooked += 1;
        }

        num_cooked
    }
}

const INTENSITY_1: Ingredients = Ingredients {
    snake: 3,
    beetle: 2,
    durian: 2,
};

const INTENSITY_2: Ingredients = Ingredients {
    snake: 6,
    beetle: 3,
    durian: 4,
};

const INTENSITY_3: Ingredients = Ingredients {
    snake: 9,
    beetle: 11,
    durian: 9,
};
