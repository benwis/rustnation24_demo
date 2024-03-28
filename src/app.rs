use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(); 

    view! {
        <Stylesheet id="leptos" href="/pkg/rustnation24-complete.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let update_count = create_server_action::<UpdateCount>();
    let count_resource = create_resource(move ||(update_count.version().get()), |_| get_count());
    view! {
        <h1>"Welcome to Leptos!"</h1>

        <span>
            "Count: "
            <Transition>

                {
                    let count = move || { count_resource.get().and_then(|n| n.ok()).unwrap_or(0) };
                    move || count().to_string()
                }

            </Transition>
        </span>
        <ActionForm action=update_count>
            <span>Increment By:</span>
            <input type="number" name="increment_by"/>
            <button type="submit">Update</button>
        </ActionForm>
    }
}

/// Get the Count from the server
#[server]
pub async fn get_count() -> Result<i64, ServerFnError>{
    let reader = expect_context::<crate::Count>();
    let count = *reader.read().unwrap();

    Ok(count)
}

#[server]
pub async fn update_count( increment_by: i64) -> Result<i64, ServerFnError>{
    let count = get_count().await.unwrap();

    let new_count = count+increment_by;
    let count_wrapper = expect_context::<crate::Count>();
    let mut writer = count_wrapper.write()?;
    *writer = new_count;
    Ok(new_count)
}
