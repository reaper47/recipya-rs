use diesel::prelude::*;

use repository::schema;
use schema_org::HowToTool;
use schema_org::field::RecipeToolFieldEnum;
use support::strings::extract_number;

use crate::Recipe;

/// Represents a tool in the recipe management system.
#[derive(Debug, Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::tools)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tool {
    id: i64,
    name: String,
}

/// Represents a tool being created in the recipe management system.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolForCreate {
    pub name: String,
    pub quantity: i16,
}

impl From<HowToTool> for ToolForCreate {
    fn from(value: HowToTool) -> Self {
        let s = value.name.first().cloned().unwrap_or_default();

        let quantity = extract_number(&s).unwrap_or(1);

        Self {
            name: s.replace(&quantity.to_string(), "").trim().to_string(),
            quantity: value
                .required_quantity
                .first()
                .map(|q| q.quantity())
                .unwrap_or_default(),
        }
    }
}

/// Represents a tool being inserted into the `tools` table.
#[derive(Insertable)]
#[diesel(table_name = schema::tools)]
pub(crate) struct ToolForInsert {
    pub name: String,
}

/// Represents the details of a tool used in a recipe.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolRecipe {
    pub name: String,
    pub quantity: i16,
    pub tool_order: i16,
}

impl From<&ToolForCreate> for ToolRecipe {
    fn from(tool: &ToolForCreate) -> Self {
        Self {
            name: tool.name.clone(),
            quantity: tool.quantity,
            tool_order: 1,
        }
    }
}

/// Represents the insertion of a tool-recipe relationship into the database.
#[derive(Associations, Insertable)]
#[diesel(table_name = schema::tools_recipes)]
#[diesel(belongs_to(Recipe))]
pub(crate) struct ToolRecipeForInsert {
    pub tool_id: i64,
    pub recipe_id: i64,
    pub quantity: i16,
    pub tool_order: i16,
}

impl From<&RecipeToolFieldEnum> for ToolForCreate {
    fn from(value: &RecipeToolFieldEnum) -> Self {
        match value {
            RecipeToolFieldEnum::HowToTool(tool) => Self {
                name: tool.name.first().cloned().unwrap_or_default(),
                quantity: tool
                    .required_quantity
                    .first()
                    .map(|q| q.quantity())
                    .unwrap_or_default(),
            },
            RecipeToolFieldEnum::Text(s) => match extract_number::<i16>(s) {
                Ok(n) => Self {
                    name: s.replace(&n.to_string(), "").trim().to_string(),
                    quantity: n,
                },
                Err(_) => Self {
                    name: s.clone(),
                    quantity: 0,
                },
            },
        }
    }
}
