use bevy::{prelude::*, utils::HashMap};

#[derive(PartialEq, Copy, Clone)]
pub enum Item {
    Wood,
    Iron,
    Limestone,
    Copper,
    Coal,
    Steel,
    Titanium,
}

impl Item {
    pub fn get_tier(&self) -> u8 {
        use Item::*;
        match *self {
            Wood | Iron | Limestone | Copper | Coal => 0,
            Steel => 1,
            Titanium => 4,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ItemStack {
    item: Item,
    amount: u16,
}

#[derive(Clone)]
pub struct Recipe {
    input: Vec<ItemStack>,
    output: Vec<ItemStack>,
    seconds_to_produce: f32,
}

impl Recipe {
    pub fn contains(&self, item: ItemStack) -> bool {
        for stack in self.input.clone().iter() {
            if item.item == stack.item {
                return true;
            }
        }
        false
    }
}

pub struct Producer {
    available_recipes: HashMap<String, Recipe>,
    current_recipe: Option<Recipe>,
    production_timer: Timer,
    input: Vec<ItemStack>,
    output: Vec<ItemStack>,
}

impl Producer {
    pub fn can_accept(&self, item: ItemStack) -> bool {
        if let Some(recipe) = &self.current_recipe {
            recipe.contains(item)
        } else {
            false
        }
    }

    pub fn set_recipe(&mut self, recipe_key: String) {
        if let Some(recipe) = self.available_recipes.get(&recipe_key) {
            self.current_recipe = Some(recipe.clone());
        }
    }

    pub fn start_next(&mut self) {
        todo!()
    }
}
