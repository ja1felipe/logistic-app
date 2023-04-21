use super::data::{CreateItemDTO, UpdateItemDTO};
use anyhow::Result;
use entity::item::{self, Entity as Item};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use serde_json::json;

pub async fn create(
    new_item: &CreateItemDTO,
    db: &DatabaseConnection,
) -> Result<item::Model, String> {
    let Ok(item) = item::ActiveModel::from_json(json!(new_item)) else {
        return Err("Failed to create item".to_string());
    };

    match item.insert(db).await {
        Ok(item) => Ok(item),
        Err(error) => {
            eprintln!("{error}");
            Err("Failed to insert item".to_string())
        }
    }
}

pub async fn get_by_category(
    code: &str,
    db: &DatabaseConnection,
) -> Result<Vec<item::Model>, String> {
    let Ok(items) = Item::find().filter(item::Column::Category.eq(code)).order_by_asc(item::Column::Description).all(db).await else {
        return Err("Failed to find items with category code {code}".to_string());
    };

    Ok(items)
}

pub async fn get_by_id(id: i32, db: &DatabaseConnection) -> Result<item::Model, String> {
    let Ok(item) = Item::find_by_id(id).one(db).await else {
        return Err("Failed to find item with id: {id}".to_string());
    };

    match item {
        Some(item) => Ok(item),
        None => Err("None item finded with id: {id}".to_string()),
    }
}

pub async fn get_by_description(
    description: &str,
    db: &DatabaseConnection,
) -> Result<Vec<item::Model>, String> {
    let Ok(items) = Item::find().filter(item::Column::Description.contains(description)).order_by_asc(item::Column::Description).all(db).await else {
        return Err("Failed to find items with description: {description}".to_string());
    };

    Ok(items)
}

pub async fn update(
    id: i32,
    new_item: UpdateItemDTO,
    db: &DatabaseConnection,
) -> Result<item::Model, String> {
    let Ok(item) = Item::find_by_id(id).one(db).await else {
        return Err("Failed to find item with id: {id}".to_string());
    };

    let Some(item) = item else {
        return Err("None item finded with id: {id}".to_string())
    };

    let mut item: item::ActiveModel = item.into();

    item.category = Set(new_item.category);
    item.description = Set(new_item.description);
    item.total_qtd = Set(new_item.total_qtd);

    let Ok(item_up) = item.update(db).await else {
        return Err(format!("Failed to update a category with id: {}", id));
    };

    Ok(item_up)
}
