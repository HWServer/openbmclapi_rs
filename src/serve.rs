use axum::{extract::Path, http::header::HeaderMap, http::StatusCode, response::IntoResponse};

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
async fn measure(header: HeaderMap, Path(size): Path<u32>) -> impl IntoResponse {
    match header.get("x-openbmclapi-secret") {
        Some(secret) => {
            if secret != "secret" {
                return (StatusCode::FORBIDDEN, Vec::new());
            }
            if size > 200 {
                return (StatusCode::BAD_REQUEST, Vec::new());
            }
            let buffer: Vec<u8> = vec![0x00, 0x66, 0xcc, 0xff];
            let mut response: Vec<u8> = Vec::new();
            for _ in 0..size {
                response.extend(&buffer);
            }
            return (StatusCode::OK, response);
        }
        None => return (StatusCode::FORBIDDEN, Vec::new()),
    }
}
