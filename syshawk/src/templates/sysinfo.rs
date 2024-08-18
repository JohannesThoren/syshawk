use std::fmt::format;
use sqlx::SqlitePool;
use syshawk_templating::html::{div, h, p};
use syshawk_templating::node::Node;
use crate::database::probe::fetch_probes;
use anyhow::Result;
use rocket::form::Shareable;
use syshawk_templating::attribute::Attribute;
use crate::database::history::fetch_latest_by_id;
use crate::models::probe::Probe;
use crate::templates::progress_bar::progress_bar;

pub async fn sys_info_template(pool: &SqlitePool) -> Result<Node> {
    let probes = fetch_probes(pool).await?;
    let mut sys_info_cards = Vec::new();

    let columns = |probes: &Vec<Probe>| {
        let min = 2;
        let max = 5;
        if probes.len() < min {
            return min;
        }
        if probes.len() < max {
            return probes.len();
        }
        return max;
    };

    for probe in &probes {
        sys_info_cards.push(
            sys_info_card(probe.id.clone(), pool).await
                .class("bg-gray-500 p-5 rounded-md".to_string())
        )
    }

    Ok(
        div(sys_info_cards)
            .class(format!("grid grid-cols-{} gap-2 h-full w-full p-2 bg-slate-300", columns(&probes)))
        // .attributes(vec![
        //     Attribute { key: "hx-trigger".to_string(), value: "every 10s".to_string() },
        //     Attribute { key: "hx-get".to_string(), value: "/ssr/sysinfo".to_string() }
        // ])
    )
}

pub async fn sys_info_card(id: String, pool: &SqlitePool) -> Node {
    match fetch_latest_by_id(id.clone(), pool).await {
        Ok(r) => {
            let sys_info = serde_json::from_str::<syshawklib::system::System>(r.system_info.as_str()).unwrap();

            return div(vec![
                h(1, sys_info.hostname),
                div(vec![
                    h(2, "CPU Usage".to_string()),
                    progress_bar(sys_info.cpu.usage),
                    div(vec![
                        h(3, "Per Thread Usage".to_string()),
                        per_core_usage(sys_info.cpu.per_core_usage),
                    ]),
                ]).class("cpu-info".to_string()),
            ]).attributes(vec![
                Attribute { key: "hx-get".to_string(), value: format!("/ssr/sysinfo/{}", id.clone()) },
                Attribute { key: "hx-trigger".to_string(), value: "every 1s".to_string() },
            ]);
        }
        Err(_) => {
            return div(vec![p("server error occured".to_string())])
        }
    }
}

pub fn per_core_usage(core_usage_vec: Vec<f32>) -> Node {
    let mut bars: Vec<Node> = Vec::new();

    for (core, usage) in core_usage_vec.iter().enumerate() {
        bars.push(
            div(vec![
                p(format!("{}", core))
                    .class("w-6".to_string()),
                progress_bar(usage.clone()),
            ]).class("flex items-center".to_string())
        )
    }

    div(bars).class(format!("grid grid-cols-{} gap-5", core_usage_vec.len() / 4))
}