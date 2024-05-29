use bollard::Docker;
use bollard::container::InspectContainerOptions;
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use bollard::models::ContainerStateStatusEnum;
use bollard::models::HealthStatusEnum;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <container_name>", args[0]);
        std::process::exit(1);
    }
    let container_name = &args[1];

    check_container_health(container_name).await;
}

async fn check_container_health(container_name: &str) {
    let docker = Docker::connect_with_local_defaults().expect("Failed to connect to Docker");

    let mut attempts = 0;
    let max_attempts = 24; // 2 minutes / 5 seconds per attempt

    loop {
        match docker.inspect_container(container_name, None::<InspectContainerOptions>).await {
            Ok(container_info) => {
                if let Some(state) = container_info.state {
                    let status = state.status.unwrap_or(ContainerStateStatusEnum::EMPTY);
                    println!("Container Status: {:?}", status);

                    if let Some(health) = state.health {
                        let health_status = health.status.unwrap_or(HealthStatusEnum::EMPTY);
                        println!("Health Status: {:?}", health_status);

                        if health_status == HealthStatusEnum::HEALTHY {
                            println!("Container is healthy.");
                            return;
                        }
                    } else {
                        println!("No health information available.");
                    }
                } else {
                    println!("No state information available.");
                }
            },
            Err(e) => {
                eprintln!("Error inspecting container: {}", e);
            },
        }

        attempts += 1;
        if attempts >= max_attempts {
            eprintln!("Container did not become healthy within the timeout period.");
            std::process::exit(1);
        }

        sleep(Duration::from_secs(5)).await;
    }
}
