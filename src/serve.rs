use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::header::HeaderMap,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum MeasureRes {
    Forbidden,
    BadResquest,
    Data(Vec<u8>)
}

impl IntoResponse for MeasureRes {
    fn into_response(self) -> Response {
        match self {
            Self::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            Self::BadResquest => (StatusCode::BAD_REQUEST).into_response(),
            Self::Data(data) => (StatusCode::OK, data).into_response()
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
    let mut data: Vec<u8> = Vec::new();
    match header.get("x-openbmclapi-secret") {
        Some(secret) => {
            if secret != "secret" {
                return MeasureRes::Forbidden;
            }
            if size > 200 {
                return MeasureRes::BadResquest;
            }
            let buffer: Vec<u8> = vec![0x00, 0x66, 0xcc, 0xff];
            for _ in 0..size {
                data.extend(&buffer);
            }
            return MeasureRes::Data(data);
        }
        None => MeasureRes::Forbidden
    }
}

/// 返回文件的请求函数
/// app.get('/download/:hash(\\w+)', async (req: Request, res: Response, next: NextFunction) => {
pub async fn res_donwload(header: HeaderMap, Query(param): Query<HashMap<String, String>>, Path(hash): Path<String>) -> impl IntoResponse {
    todo!();
}
