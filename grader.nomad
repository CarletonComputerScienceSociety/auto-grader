job "grader8" {
  datacenters = ["dc1"]

  group "scheduler" {
    network {
      mode = "bridge"
    }

    service {
      name = "grading-scheduler"
      port = "4000"

      connect {
        sidecar_service {}
      }
    }

    task "scheduler" {
      driver = "docker"

      config {
        image = "ghcr.io/angelonfira/rust-test-server/scheduler:latest"
      }

      env {
        test = "test4"
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

      env {
        test = "test4"
      }

      resources {
        cpu    = 1000
        memory = 1000
      }

      # Allow for as many restarts as is needed, since a restart is just a
      # compilation environment getting cleaned.
      restart {
        interval = "1m"
        attempts = 100
        delay    = "0s"
        mode     = "delay"
      }

    }
  }
}
