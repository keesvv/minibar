use std::{env, net::Ipv4Addr};

use minibar::webhook::WebhookBody;
use minibar_ntfy::OrderFormatter;
use warp::{self, body::json, path, reply::Reply, Filter};

#[tokio::main]
async fn main() {
    let webhook_handler = path!().and(json::<WebhookBody>()).and_then(handle_webhook);

    warp::serve(webhook_handler)
        .run((Ipv4Addr::LOCALHOST, 1338))
        .await;
}

async fn handle_webhook(body: WebhookBody) -> Result<impl Reply, warp::Rejection> {
    let topic = env::var("NTFY_TOPIC").ok().expect("missing ntfy topic");
    reqwest::Client::new()
        .post(format!("https://ntfy.sh/{}", topic))
        .body(format!(
            "{who} wants {order}",
            who = body.who,
            order = OrderFormatter::new(&body.order)
        ))
        .send()
        .await
        .unwrap();
    Ok::<_, warp::Rejection>(warp::reply())
}
