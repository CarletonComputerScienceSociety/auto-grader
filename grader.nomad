job "grader2" {
  datacenters = ["dc1"]

  group "scheduler" {
    network {
      port "backend" {
        to = 4000
      }
    }

    service {
      name = "grading-scheduler"
      port = "backend"

      check {
        type     = "http"
        port     = "backend"
        path     = "/hello"
        interval = "5s"
        timeout  = "2s"

        check_restart {
          limit           = 3
          grace           = "30s"
          ignore_warnings = false
        }
      }
    }

    task "scheduler" {
      driver = "docker"

      config {
        image = "ghcr.io/angelonfira/rust-test-server/scheduler:latest"
        ports = ["backend"]
      }

      resources {
        cpu    = 100
        memory = 100
      }
    }
  }

  group "runners" {
    count = 30

    task "runner" {
      driver = "docker"

      config {
        image = "ghcr.io/angelonfira/rust-test-server/runner:latest"
      }

      resources {
        cpu    = 100
        memory = 100
      }
    }
  }
}
