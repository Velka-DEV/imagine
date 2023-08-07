use salvo::prelude::*;
use salvo::core::fs::NamedFile;

#[handler]
pub async fn serve_file(req: &mut Request, res: &mut Response) {
    NamedFile::builder("/arealpath").attached_name("image.png").send(req.headers(), res).await;
}