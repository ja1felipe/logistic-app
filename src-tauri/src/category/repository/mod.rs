use anyhow::Result;
use entity::category::{self, Entity as Category};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde_json::json;

use super::data::CreateCategoryDTO;

pub async fn create(
    db: &DatabaseConnection,
    new_category: &CreateCategoryDTO,
) -> Result<category::Model, String> {
    let Ok(category) = category::ActiveModel::from_json(json!(new_category)) else {
            return Err("Failed creating a category from json".to_string());
        };

    match category.insert(db).await {
        Ok(category) => Ok(category),
        Err(error) => {
            eprint!("{:#?}", error);
            Err("Failed insert a category".to_string())
        }
    }
}

pub async fn get_all(db: &DatabaseConnection) -> Option<Vec<category::Model>> {
    let categories = Category::find().all(db).await;

    match categories {
        Ok(categories) => Some(categories),
        Err(_) => None,
    }
}

pub async fn update(
    db: &DatabaseConnection,
    id: i32,
    new_category: CreateCategoryDTO,
) -> Result<category::Model, String> {
    let Ok(ctg) = Category::find_by_id(id).one(db).await else {
        return Err(format!("Failed to find a category with id: {}", id));
    };

    let mut ctg: category::ActiveModel = ctg.unwrap().into();

    ctg.code = Set(new_category.code);
    ctg.description = Set(new_category.description);

    let Ok(ctg) = ctg.update(db).await else {
        return Err(format!("Failed to update a category with id: {}", id));
    };
    Ok(ctg)
}

pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<bool> {
    let res = Category::delete_by_id(id).exec(db).await?;
    if res.rows_affected > 0 {
        return Ok(true);
    }

    Ok(false)
}
