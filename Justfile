dev:
  docker compose up --build

stop:
  docker compose down

test_e2e:
  bats ./tests/e2e-reports-via-connector.bats

test_fluvio_install:
  fluvio version
  fluvio topic list
  fluvio topic create foobar
  sleep 3
  echo foo | fluvio produce foobar
  fluvio consume foobar -B -d
