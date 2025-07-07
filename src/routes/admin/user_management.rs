use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};

use crate::{app::BACKEND_URL, directives::table::Table};

#[allow(non_snake_case)]
#[component]
pub fn UserManagement() -> impl IntoView {

    // portfolio state
    let portfolio: RwSignal<Vec<serde_json::Value>> = RwSignal::new(vec![serde_json::Value::Array(vec![])]);
    let portfolio_page: RwSignal<i32> = RwSignal::new(1);
    let portfolio_total: RwSignal<usize> = RwSignal::new(0);
    let portfolio_loading: RwSignal<bool> = RwSignal::new(false);
    let portfolio_limit = 3;

    // skills state
    let skills: RwSignal<Vec<serde_json::Value>> = RwSignal::new(vec![serde_json::Value::Array(vec![])]);

    // experience state
    let experience: RwSignal<Vec<serde_json::Value>> = RwSignal::new(vec![serde_json::Value::Array(vec![])]);

    let fetch_portfolio = move |page: i32| {
        let offset = (page - 1) * portfolio_limit;
        let url = format!(
            "{}/data/table?tablename=portfolio&offset={}&limit={}&nidkey=portfolio_id",
            BACKEND_URL,
            offset,
            portfolio_limit
        );

        spawn_local(async move {
            portfolio_loading.set(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<serde_json::Value>().await {
                    portfolio.set(data["rows"].as_array().unwrap_or(&vec![]).to_vec());
                    portfolio_total.set(data["total"].as_i64().unwrap_or(0) as usize);
                }
            }
            portfolio_loading.set(false);
        });
    };

    Effect::new(move |_| {
        fetch_portfolio(portfolio_page.get());
    });

    view! {
        <div class="user-management">
            <div class="row">
                <div class="col-12 portfolio">
                    <div class="card">
                        <div class="card-header">
                            Portfolio
                        </div>
                        <div class="card-body">
                            <Table 
                                table="portfolio".to_string() 
                                data=portfolio.clone() 
                                page=portfolio_page
                                total=portfolio_total
                                limit=portfolio_limit
                                refresh=fetch_portfolio
                                loading_data=portfolio_loading/>
                        </div>
                    </div>
                </div>
                <div class="col-md-6 experience">
                    <div class="card">
                        <div class="card-header">
                            Experience
                        </div>
                        <div class="card-body">
                            // <Table table="experience".to_string() data=data.clone()/>
                        </div>
                    </div>
                </div>
                <div class="col-md-6 skills">
                    <div class="card">
                        <div class="card-header">
                            Skills
                        </div>
                        <div class="card-body">
                            // <Table table="skills".to_string() data=data.clone()/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}