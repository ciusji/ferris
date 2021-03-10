//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Combinators: and_then

// `map()` was described as a chainable way to simplify `match` statements. However, using `map()` on
// a function that returns an `Option<T>` results in the nested `Option<Option<T>>`. Chaining multiple
// calls together can then become confusing. That's where another combinator called `and_then()`, know
// in some languages as flatmap, comes in.
// `and_then()` calls its function input with the wrapped value and returns the result. if the `Option`
// is `None`, then is returns `None` instead.

#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// We don't have the ingredients to make Sushi.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

// We have the recipe for everything except Cordon Bleu.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

// To make a dish, we need both the recipe and the ingredients.
// We can represent the logic with a chain of matches.
fn cook_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food)
        }
    }
}

// This can conveniently be rewritten more compactly with and_then().
fn cook_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat(food: Food, day: Day) {
    match cook_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day)
    }
}


pub fn combinator_and_then() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday );
    eat(sushi, Day::Wednesday);
}