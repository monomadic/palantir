use site::{Parser, Renderable, Site};
use std::sync::Arc;
use warp::{path::FullPath, Filter};

#[tokio::main]
pub async fn start<R: Renderable + Sync + Send + 'static, P: Parser<R> + Sync + Send + 'static>(
    site: Site<R, P>,
) {
    let site_arc = Arc::new(site);
    let with_state = warp::any().map(move || site_arc.clone());

    let route = warp::path::full()
        .and(with_state)
        .map(|path: FullPath, site: Arc<Site<R, P>>| {
            site.get_renderer(&path.as_str().to_string())
                .expect(&format!("error requesting renderer for {}", path.as_str()))
                .render_html()
        });

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
