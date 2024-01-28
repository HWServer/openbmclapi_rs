use crate::{config::Config, utils::hash_to_filename};

use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::header::HeaderMap,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum MeasureRes {
    Forbidden,
    BadResquest,
    Data(Vec<u8>),
}

impl IntoResponse for MeasureRes {
    fn into_response(self) -> Response {
        match self {
            Self::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            Self::BadResquest => (StatusCode::BAD_REQUEST).into_response(),
            Self::Data(data) => (StatusCode::OK, data).into_response(),
        }
    }
}

/// ```ts
/// import express from 'express'
///
/// const router = express.Router()
///
/// const MeasureRoute = router
///
/// router.get('/:size(\\d+)', (req, res) => {
///   if (req.get('x-openbmclapi-secret') !== process.env.CLUSTER_SECRET) {
///     return res.sendStatus(403)
///   }
///   const size = parseInt(req.params.size, 10)
///   if (isNaN(size) || size > 200) return res.sendStatus(400)
///   const buffer = Buffer.alloc(1024 * 1024, '0066ccff', 'hex')
///   res.set('content-length', (size * 1024 * 1024).toString())
///   for (let i = 0; i < size; i++) {
///     res.write(buffer)
///   }
/// })
///
/// export default MeasureRoute
/// ```
pub async fn measure(header: HeaderMap, Path(size): Path<u32>) -> MeasureRes {
    match header.get("x-openbmclapi-secret") {
        Some(secret) => {
            if secret != "secret" {
                return MeasureRes::Forbidden;
            }
            if size > 200 {
                return MeasureRes::BadResquest;
            }
            // size -> size * mb
            let mut data: Vec<u8> = Vec::with_capacity((size * 1024 * 1024) as usize);
            data.fill(114_u8);
            return MeasureRes::Data(data);
        }
        None => MeasureRes::Forbidden,
    }
}

/// 返回文件的请求函数
/// app.get('/download/:hash(\\w+)', async (req: Request, res: Response, next: NextFunction) => {
pub async fn res_donwload(
    State(config): State<Config>,
    header: HeaderMap,
    Query(param): Query<HashMap<String, String>>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    let hash = hash.to_lowercase();
    let file_path = config.cache_dir.join(hash_to_filename(&hash));
    
    todo!();
}
