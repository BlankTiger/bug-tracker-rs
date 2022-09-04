use gloo_console::log;
use reqwasm::http::Request;
use types::Project;
use yew::prelude::*;
use yew::use_state;

async fn fetch_projects() -> Result<Vec<Project>, Box<dyn std::error::Error>> {
    let session_id = "Q2WyN6fwVyGPUkVaWNMLKxlfFZ74LakXveKVEmy8YziE1LEhzeJH06DNcsT4p5qFU0IWG7LAQ7DgDvDTA4AGGOiM0oLWBEuFtpwd0buOIHms5+hhTcXlBvJPI7Q%3D";

    let cookies = wasm_cookies::all().unwrap();
    log!("are empty? {}", cookies.is_empty());

    let projects = Request::get("https://127.0.0.1:8082/api/projects/get_projects")
        .header("cookie", &format!("id:{}", session_id) as &str)
        .send()
        .await?
        .json::<Vec<Project>>()
        .await?;

    Ok(projects)
}

fn projects_to_html_table(projects: Option<&Vec<Project>>) -> Html {
    let table_data = match projects {
        Some(projects) => html! {
            <tbody>
                { for projects.iter().map(|project| html! {
                    <tr>
                        <td>{ &project.project_id }</td>
                        <td>{ &project.project_name }</td>
                        <td>{ &project.desc_short }</td>
                        <td>{ &project.created_at }</td>
                    </tr>
                }) }
            </tbody>
        },
        None => html! {
            <tbody>
                <tr>
                    <td>{ "No projects found" }</td>
                    <td>{ "No projects found" }</td>
                    <td>{ "No projects found" }</td>
                    <td>{ "No projects found" }</td>
                </tr>
            </tbody>
        },
    };

    html! {
        <table class="projects-table">
            <thead>
                <tr>
                    <th>{ "id" }</th>
                    <th>{ "name" }</th>
                    <th>{ "description" }</th>
                    <th>{ "created at" }</th>
                </tr>
            </thead>
            { table_data }
        </table>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = use_state(|| None);
    let projects_clone = projects.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let fetched_projects = fetch_projects().await.unwrap();
        projects.set(Some(fetched_projects));
    });

    let projects_table = projects_to_html_table(projects_clone.as_ref());

    html! {
        <div class="projects-fragment">
            { projects_table }
        </div>
    }
}
