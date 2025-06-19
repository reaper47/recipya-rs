use lru::LruCache;

use crate::data::ViewRecipe;

/// The definition of a key in the cache.
pub type RecipeCacheKey = (i64, i64); // (user_id, recipe_id)

/// The type of the LRU cache used to cache user recipes.
pub type RecipeCache = LruCache<RecipeCacheKey, ViewRecipe>;
