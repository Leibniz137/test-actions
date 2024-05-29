use bollard::Docker;
use bollard::container::InspectContainerOptions;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <container_name>", args[0]);
        std::process::exit(1);
    }
    let container_name = &args[1];

    get_container_status(container_name).await;
}

async fn get_container_status(container_name: &str) {
    let docker = Docker::connect_with_local_defaults().expect("Failed to connect to Docker");

    match docker.inspect_container(container_name, None::<InspectContainerOptions>).await {
        Ok(container_info) => {
            let state = container_info.state.unwrap();
            let status = match state.status {
                Some(status_enum) => format!("{:?}", status_enum),
                None => "unknown".to_string(),
            };
            println!("Container ID: {}", container_info.id.unwrap_or_default());
            println!("Container Status: {}", status);

            if let Some(health) = state.health {
                let health_status = match health.status {
                    Some(health_enum) => format!("{:?}", health_enum),
                    None => "unknown".to_string(),
                };
                println!("Health Status: {}", health_status);
            }
        },
        Err(e) => {
            eprintln!("Error inspecting container: {}", e);
        },
    }
}
