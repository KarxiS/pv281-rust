use askama::Template;
use axum::{
    extract::{Multipart, Path, State},
    response::Html,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tauri::Manager;

use crate::master_service::MasterService;
use crate::slave_service::{SlaveService, SlaveStatusSnapshot};
use crate::status::MasterStatusSnapshot;

// --- STATE MANAGEMENT ---
struct AppState {
    master_service: Arc<MasterService>,
    slave_service: Arc<SlaveService>,
    #[allow(dead_code)]
    app_handle: tauri::AppHandle,
}

#[derive(Clone, serde::Serialize)]
struct Client {
    id: String,
    name: String,
    ip: String,
    status: String,
}

// --- TEMPLATES ---
#[derive(Template)]
#[template(path = "index.html")]
struct LandingTemplate;

#[derive(Template)]
#[template(path = "master.html")]
struct MasterTemplate {
    clients: Vec<Client>,
    is_running: bool,
}

#[derive(Template)]
#[template(path = "slave.html")]
struct SlaveTemplate;

#[derive(Template)]
#[template(path = "client_list.html")]
struct ClientListTemplate {
    clients: Vec<Client>,
}

// --- HANDLERS ---

async fn landing_handler() -> Html<String> {
    let template = LandingTemplate;
    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Render error: {}", e)),
    )
}

fn get_clients_from_service(service: &MasterService) -> Vec<Client> {
    service
        .get_clients()
        .into_iter()
        .map(|c| Client {
            id: c.id,
            name: c.hostname,
            ip: c.ip,
            status: c.status,
        })
        .collect()
}

async fn master_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let clients = get_clients_from_service(&state.master_service);
    let is_running = state.master_service.is_running();

    let template = MasterTemplate {
        clients,
        is_running,
    };
    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Render error: {}", e)),
    )
}

async fn slave_handler() -> Html<String> {
    let template = SlaveTemplate;
    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Render error: {}", e)),
    )
}

#[derive(Deserialize)]
struct ToggleMasterForm {
    action: String,
    mouse: Option<String>,
    keyboard: Option<String>,
}

async fn toggle_master_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<ToggleMasterForm>,
) -> Html<String> {
    let is_running_initial = state.master_service.is_running();

    if form.action == "start" {
        if is_running_initial {
            return Html(render_master_button(true));
        }

        match state.master_service.start(form.mouse, form.keyboard) {
            Ok(_) => Html(render_master_button(true)),
            Err(e) => {
                eprintln!("Failed to start master: {}", e);
                Html(render_master_button(false))
            }
        }
    } else {
        if !is_running_initial {
            return Html(render_master_button(false));
        }

        state.master_service.stop();
        Html(render_master_button(false))
    }
}

fn render_master_button(is_running: bool) -> String {
    if is_running {
        r#"<button type="submit" name="action" value="stop" class="bg-red-600 hover:bg-red-500 text-white font-bold py-2 px-4 rounded text-sm transition-colors shadow-lg shadow-red-900/50">Stop Master Process</button>"#.to_string()
    } else {
        r#"<button type="submit" name="action" value="start" class="bg-blue-600 hover:bg-blue-500 text-white font-bold py-2 px-4 rounded text-sm transition-colors shadow-lg shadow-blue-900/50">Start Master Process</button>"#.to_string()
    }
}

async fn connect_client_handler(
    State(state): State<Arc<AppState>>,
    Path(_id): Path<String>,
) -> Html<String> {
    let clients = get_clients_from_service(&state.master_service);
    let template = ClientListTemplate { clients };
    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Render error: {}", e)),
    )
}

async fn disconnect_client_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Html<String> {
    state.master_service.disconnect_client(&id);

    let clients = get_clients_from_service(&state.master_service);
    let template = ClientListTemplate { clients };
    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Render error: {}", e)),
    )
}

async fn clients_list_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let clients = get_clients_from_service(&state.master_service);
    let template = ClientListTemplate { clients };
    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Render error: {}", e)),
    )
}

#[derive(Deserialize)]
struct StartSlaveForm {
    master_ip: String,
}

async fn start_slave_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<StartSlaveForm>,
) -> Html<String> {
    match state.slave_service.start(form.master_ip) {
        Ok(_) => Html("".to_string()),
        Err(e) => Html(format!("<span class='text-red-400'>Error: {}</span>", e)),
    }
}

async fn stop_slave_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    state.slave_service.stop();
    Html("".to_string())
}

async fn send_file_handler(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Html<String> {
    if !state.master_service.is_running() {
        return Html("<span class='text-red-400'>Master is not running</span>".to_string());
    }

    let field = match multipart.next_field().await {
        Ok(Some(field)) => field,
        Ok(None) => return Html("<span class='text-red-400'>No file</span>".to_string()),
        Err(e) => return Html(format!("<span class='text-red-400'>{}</span>", e)),
    };

    let bytes = match field.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => return Html(format!("<span class='text-red-400'>{}</span>", e)),
    };

    let path = std::env::temp_dir().join("kmf-upload.bin");
    if let Err(e) = tokio::fs::write(&path, &bytes).await {
        return Html(format!("<span class='text-red-400'>{}</span>", e));
    }

    match state
        .master_service
        .send_file(path.to_string_lossy().to_string())
    {
        Ok(()) => Html("<span class='text-green-400'>File sent</span>".to_string()),
        Err(e) => Html(format!("<span class='text-red-400'>{}</span>", e)),
    }
}

async fn slave_status_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let status: SlaveStatusSnapshot = state.slave_service.get_status();
    if !status.running {
        if let Some(err) = status.last_error {
            return Html(format!(
                "<span class='text-red-400'>Disconnected: {}</span>",
                err
            ));
        }
        return Html("<span class='text-gray-400'>Idle</span>".to_string());
    }

    if status.connecting {
        return Html("<span class='text-yellow-300'>Connecting...</span>".to_string());
    }

    if status.connected {
        Html("<span class='text-green-400'>Connected</span>".to_string())
    } else {
        Html("<span class='text-red-400'>Disconnected</span>".to_string())
    }
}

async fn status_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let clients = get_clients_from_service(&state.master_service);
    let count = clients.len();
    let status: MasterStatusSnapshot = state.master_service.get_status();

    if !status.running {
        return Html("<span class='text-red-400'>Master Stopped</span>".to_string());
    }

    let mode_label = if status.remote_mode { "SLAVE" } else { "LOCAL" };

    if status.calibration_mode {
        Html(format!(
            "<div class='text-sm text-yellow-300'>\
                <div class='font-semibold'>Calibration mode</div>\
                <div>1) Cursor forced to top-left.</div>\
                <div>2) Move to bottom-right.</div>\
                <div>3) Press 'c' to set size.</div>\
                <div class='mt-1 text-gray-300'>Live cursor: x={}, y={}</div>\
                <div class='text-gray-400 text-xs'>Clients: {}</div>\
            </div>",
            status.cursor_x, status.cursor_y, count
        ))
    } else {
        Html(format!(
            "<div class='text-sm text-green-300'>\
                <div class='font-semibold'>Master active ({})</div>\
                <div>Screen: {} × {}</div>\
                <div>Cursor: x={}, y={}</div>\
                <div class='text-gray-400 text-xs'>Clients: {}</div>\
            </div>",
            mode_label,
            status.master_width,
            status.master_height,
            status.cursor_x,
            status.cursor_y,
            count
        ))
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            let state = Arc::new(AppState {
                master_service: Arc::new(MasterService::new()),
                slave_service: Arc::new(SlaveService::new()),
                app_handle,
            });

            let app_state = state.clone();

            tauri::async_runtime::spawn(async move {
                let router = Router::new()
                    .route("/", get(landing_handler))
                    .route("/master", get(master_handler))
                    .route("/slave", get(slave_handler))
                    .route("/api/toggle_master", post(toggle_master_handler))
                    .route("/api/connect/:id", post(connect_client_handler))
                    .route("/api/disconnect/:id", post(disconnect_client_handler))
                    .route("/api/clients", get(clients_list_handler))
                    .route("/api/start_slave", post(start_slave_handler))
                    .route("/api/stop_slave", post(stop_slave_handler))
                    .route("/api/send_file", post(send_file_handler))
                    .route("/api/status", get(status_handler))
                    .route("/api/slave_status", get(slave_status_handler))
                    .with_state(app_state);

                let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
                    .await
                    .expect("Failed to bind UI server to 127.0.0.1:3000");
                axum::serve(listener, router)
                    .await
                    .expect("Failed to start UI server");
            });

            let main_window = app
                .get_webview_window("main")
                .expect("Failed to get main window");
            let _ = main_window
                .navigate(tauri::Url::parse("http://127.0.0.1:3000").expect("Invalid UI URL"));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
