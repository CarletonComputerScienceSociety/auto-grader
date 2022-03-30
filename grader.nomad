job "grader8" {
  datacenters = ["dc1"]

  group "scheduler" {
    network {
      mode = "bridge"
      port "backend" {
        to = 4000
      }
    }

    service {
      name = "grading-scheduler"
      port = "backend"

      connect {
        sidecar_service {}
      }
    }

    task "scheduler" {
      driver = "docker"

      config {
        image = "ghcr.io/angelonfira/rust-test-server/scheduler:latest"
        ports = ["backend"]
      }

      resources {
        cpu    = 5000
        memory = 1000
      }
    }
  }

  group "runners" {
    count = 3

    network {
      mode = "bridge"
    }

      service {
        name = "grading-runner"

        connect {
          sidecar_service {
            proxy {
              upstreams {
                destination_name = "grading-scheduler"
                local_bind_port  = 4000
              }
            }
          }
        }
      }

    task "runner" {
      driver = "docker"

      config {
        image = "ghcr.io/angelonfira/rust-test-server/runner:latest"
      }

      resources {
        cpu    = 1000
        memory = 1000
      }
    }
  }
}
