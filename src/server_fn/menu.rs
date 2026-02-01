use leptos::{
    prelude::{expect_context, ServerFnError},
    server,
};

use crate::dto::menu_dto::MenuDto;
#[cfg(feature = "ssr")]
use crate::entity::prelude::*;

#[cfg(feature = "ssr")]
use crate::entity::tab_menu;

#[cfg(feature = "ssr")]
use sea_orm::{prelude::*, Condition, QueryOrder};

#[cfg(feature = "ssr")]
use crate::state::app_state::AppState;

/**
 * 加载菜单数据
 */
#[server]
pub async fn get_menus() -> Result<Vec<MenuDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let state = expect_context::<AppState>();
        let db = state.db();

        let conditions = Condition::all();

        let menus = TabMenu::find()
            .filter(conditions)
            .order_by_desc(tab_menu::Column::Id)
            .all(db)
            .await
            .unwrap();

        let menus_dto: Vec<_> = menus
            .into_iter()
            .map(|menu| MenuDto {
                // key: user.id.to_string(),
                id: menu.id,
                menu_name: menu.menu_name,
            })
            .collect();

        Ok(menus_dto)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("get_users should only run on the server");
}
