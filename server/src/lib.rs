use site::{Parser, Renderable, Site};
use std::sync::Arc;
use warp::{path::FullPath, Filter};

// can we abstract this so we don't use Site, just traits?

#[tokio::main]
pub async fn start<R: Renderable + Sync + Send + 'static, P: Parser<R> + Sync + Send + 'static>(
    site: Site<R, P>,
    host: Option<std::net::SocketAddr>,
) {
    let site_arc = Arc::new(site);
    let with_state = warp::any().map(move || site_arc.clone());

    let route = warp::path::full()
        .and(with_state)
        .map(|path: FullPath, site: Arc<Site<R, P>>| site.render_html(&path.as_str().to_string()));

    warp::serve(route)
        .run(host.unwrap_or(([127, 0, 0, 1], 3030).into()))
        .await;
}
