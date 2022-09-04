use types::Project;
use yew::prelude::*;

fn fetch_projects() -> Vec<Project> {

}

fn projects_to_html_table(projects: Vec<Project>) -> Html {
    html! {
        <table>
            <tr>
                <th>{ "Id" }</th>
                <th>{ "Name" }</th>
                <th>{ "Description" }</th>
                <th>{ "Assigned to" }</th>
                <th>{ "Created by" }</th>
                <th>{ "Created at" }</th>
            </tr>
            { for projects.iter().map(|project| html! {
                <tr>
                    <td>{ project.project_id }</td>
                    <td>{ project.project_name }</td>
                    <td>{ project.desc_short }</td>
                    <td>{ project.assigned_to }</td>
                    <td>{ project.created_by }</td>
                    <td>{ project.created_at }</td>
                </tr>
            }) }
        </table>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div>
            <h1>{ "Projects" }</h1>
            { projects_to_html_table(fetch_projects()) }
        </div>
    }
}
