use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // good opportunity here to also demonstrate CSS hot-reloading for people
        // i.e., if you go and edit this CSS file the changes appear instantly
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
    let count_resource = create_resource(move || (update_count.version().get()), |_| get_count());

    let query = use_query_map();
    let initial_increment = move || {
        query
            .with(|q| q.get("increment").cloned())
            .unwrap_or_else(|| "1".to_string())
    };
    view! {
        <h1>"Welcome to Leptos!"</h1>
        // if you have `--hot-reload` running, adding something like a <p> below is a good
        // hot-reloading demo
        // <p>"Hot reloading works!"</p>

        <span>
            "Count: "
            <Transition>
                // you can render the resource directly here, because
                // Option<Result<i64, ServerFnError>> does implement IntoView
                // you could use an ErrorBoundary to catch errors from the server fn if you want
                {count_resource}
            </Transition>
        </span>
        <ActionForm action=update_count>
            <span>Increment By:</span>
            // it would be nice to have some way of having a default value, or using the previous
            // value, for the value of this input field -- that way it would persist across POSTs
            // when JS is off. Not super necessary though
            <input type="number" name="increment_by" value=initial_increment/>
            <button type="submit">Update</button>
        </ActionForm>
    }
}

/// Get the Count from the server
#[server]
pub async fn get_count() -> Result<i64, ServerFnError> {
    let reader = expect_context::<crate::Count>();
    // added ? like the update_count below, this is nice
    let count = *reader.read()?;

    Ok(count)
}

#[server]
pub async fn update_count(increment_by: i64) -> Result<i64, ServerFnError> {
    use leptos_axum::redirect;

    // unless I'm missing some other reasoning (like explaining that you can call a server fn from
    // another server fn without going out through HTTP), it seems like you can just update the
    // value in the lock directly
    let count_wrapper = expect_context::<crate::Count>();
    // nice to have the ? here throwing out to the ServerFnError
    let mut writer = count_wrapper.write()?;
    *writer += increment_by;
    redirect(&format!("/?increment={increment_by}"));
    Ok(*writer)
}
