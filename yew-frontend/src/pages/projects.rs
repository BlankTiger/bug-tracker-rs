use reqwasm::http::Request;
use types::Project;
// use wasm_cookies::get;
use yew::prelude::*;
use yew::use_state;

async fn fetch_projects() -> Result<Vec<Project>, Box<dyn std::error::Error>> {
    // let cookies = wasm_cookies::all().unwrap();
    // console::log_1(&JsString::from(format!("{:?}", &cookies)));
    // let session_id = cookies.get("id").unwrap();

    let session_id = "CM90Kp66HViJH0fz3R6mA55aBzGMa%2FtlIJRIyhv27CkiayObY5QfJES7mXEuVJl%2FUqmSdX5BYRGmhP4ay0m41japMGHIZ0ZHQzKdi00vhhrelIn4cpcSn3uJvR0%3D";

    // let session_id = match get("id") {
    //     Some(session_id) => session_id,
    //     None => Err("No session id found")?,
    // };

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
            { for projects.iter().map(|project| html! {
                <tr>
                    <td>{ &project.project_id }</td>
                    <td>{ &project.project_name }</td>
                    <td>{ &project.desc_short }</td>
                    <td>{ &project.created_at }</td>
                </tr>
            }) }
        },
        None => html! {
            <tr>
                <td>{ "No projects found" }</td>
                <td>{ "No projects found" }</td>
                <td>{ "No projects found" }</td>
                <td>{ "No projects found" }</td>
            </tr>
        },
    };

    html! {
        <table>
            <tr>
                <th>{ "Id" }</th>
                <th>{ "Name" }</th>
                <th>{ "Description" }</th>
                <th>{ "Created at" }</th>
            </tr>
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
        <div>
            <h1>{ "Projects" }</h1>
            { projects_table }
        </div>
    }
}
