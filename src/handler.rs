use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    db,
    form::{CreateForm, CreateUnsignedForm},
    model,
    state::AppState,
};

/// 所有记录列表
pub async fn list<'a>(
    State(state): State<Arc<AppState>>,
) -> Result<Json<model::Resp<'a, Vec<model::TestResp>>>, String> {
    let conn = &*state.pool;
    let ts = db::list(conn).await.map_err(|e| e.to_string())?;
    let mut ls = Vec::with_capacity(ts.len());
    for t in ts {
        ls.push(model::TestResp::from(t));
    }
    Ok(Json(model::Resp::ok(ls)))
}

/// 通过原始数查找
pub async fn find<'a>(
    State(state): State<Arc<AppState>>,
    Path(num): Path<model::Uint>,
) -> Result<Json<model::Resp<'a, model::TestResp>>, String> {
    let conn = &*state.pool;
    let r = db::find(conn, num).await.map_err(|e| e.to_string())?;
    if r.is_none() {
        return Err("不存在的记录".to_string());
    }
    let r = model::TestResp::from(r.unwrap());
    Ok(Json(model::Resp::ok(r)))
}

/// 通过无符号数查找
pub async fn find_unsigned<'a>(
    State(state): State<Arc<AppState>>,
    Path(num): Path<u32>,
) -> Result<Json<model::Resp<'a, model::TestResp>>, String> {
    let conn = &*state.pool;
    let num = model::Uint::from(num);
    let r = db::find(conn, num).await.map_err(|e| e.to_string())?;
    if r.is_none() {
        return Err("不存在的记录".to_string());
    }
    let r = model::TestResp::from(r.unwrap());
    Ok(Json(model::Resp::ok(r)))
}

/// 通过原始数创建
pub async fn create<'a>(
    State(state): State<Arc<AppState>>,
    Json(frm): Json<CreateForm>,
) -> Result<Json<model::Resp<'a, model::TestResp>>, String> {
    let conn = &*state.pool;
    let r = db::insert(
        conn,
        &model::Test {
            num: frm.num,
            ..Default::default()
        },
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(Json(model::Resp::ok(r.into())))
}

/// 通过无符号数创建
pub async fn create_unsigned<'a>(
    State(state): State<Arc<AppState>>,
    Json(frm): Json<CreateUnsignedForm>,
) -> Result<Json<model::Resp<'a, model::TestResp>>, String> {
    let conn = &*state.pool;
    let num = model::Uint::from(frm.num);
    let r = db::insert(
        conn,
        &model::Test {
            num,
            ..Default::default()
        },
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(Json(model::Resp::ok(r.into())))
}
