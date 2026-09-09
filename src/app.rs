use futures::{channel::mpsc, StreamExt};
use leptos::{
    prelude::*,
    server_fn::{codec::JsonEncoding, BoxedStream, Websocket},
};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    lazy_route, Lazy, LazyRoute, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/axummhmmmm.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view={Lazy::<Home>::new()} />
                </Routes>
            </main>
        </Router>
    }
}

struct Home;

#[lazy_route]
impl LazyRoute for Home {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let (mut tx, rx) = mpsc::channel(1);
        let latest = RwSignal::new(Ok("".into()));

        if cfg!(feature = "hydrate") {
            leptos::task::spawn_local(async move {
                match echo_websocket(rx.into()).await {
                    Ok(mut messages) => {
                        while let Some(msg) = messages.next().await {
                            leptos::logging::log!("{:?}", msg);
                            latest.set(msg);
                        }
                    }
                    Err(e) => leptos::logging::warn!("{e}"),
                }
            });
        }

        let mut x = 0;
        view! {
            <h1>Simple Echo WebSocket Communication</h1>
            <input
                type="text"
                on:input:target=move |ev| {
                    x += 1;
                    let msg = ev.target().value();
                    leptos::logging::log!("In client: {} {:?}", x, msg);
                    if x % 5 == 0 {
                        let _ = tx
                            .try_send(
                                Err(
                                    ServerFnError::Registration(
                                        "Error generated from client".to_string(),
                                    ),
                                ),
                            );
                    } else {
                        let _ = tx.try_send(Ok(msg));
                    }
                }
            />
            <div>
                <ErrorBoundary fallback=|errors| {
                    view! {
                        <p>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| format!("{e:?}"))
                                    .collect::<Vec<String>>()
                                    .join(" ")
                            }}
                        </p>
                    }
                }>
                    <p>{latest}</p>
                </ErrorBoundary>
            </div>
        }
        .into_any()
    }
}

#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
async fn echo_websocket(
    input: BoxedStream<String, ServerFnError>,
) -> Result<BoxedStream<String, ServerFnError>, ServerFnError> {
    use futures::{channel::mpsc, SinkExt, StreamExt};
    let mut input = input; // FIXME :-) server fn fields should pass mut through to destructure

    // create a channel of outgoing websocket messages
    // we'll return rx, so sending a message to tx will send a message to the client via the websocket
    let (mut tx, rx) = mpsc::channel(1);

    // spawn a task to listen to the input stream of messages coming in over the websocket
    tokio::spawn(async move {
        let mut x = 0;
        while let Some(msg) = input.next().await {
            // do some work on each message, and then send our responses
            x += 1;
            println!("In server: {} {:?}", x, msg);
            if x % 3 == 0 {
                let _ = tx
                    .send(Err(ServerFnError::Registration(
                        "Error generated from server".to_string(),
                    )))
                    .await;
            } else {
                let _ = tx.send(msg.map(|msg| msg.to_ascii_uppercase())).await;
            }
        }
    });

    Ok(rx.into())
}
