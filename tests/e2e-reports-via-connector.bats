#!/usr/bin/env bats

load './bats-helpers/bats-support/load'
load './bats-helpers/bats-assert/load'

setup() {
    UUID=$(uuidgen | awk '{print tolower($0)}')
    TOPIC=${UUID}-topic

    FILE=$(mktemp)
    cp ./tests/e2e-reports-via-connector.yaml $FILE

    CONNECTOR=${UUID}-sends-data
    export VERSION=$(cat ./Connector.toml | grep "^version = " | cut -d"\"" -f2)
    IPKG_NAME="http-sink-$VERSION.ipkg"
    fluvio topic create $TOPIC

    sed -i.BAK "s/CONNECTOR/${CONNECTOR}/g" $FILE
    sed -i.BAK "s/TOPIC/${TOPIC}/g" $FILE
    sed -i.BAK "s/VERSION/${VERSION}/g" $FILE
    cat $FILE

    cdk deploy start --config $FILE
}

teardown() {
    fluvio topic delete $TOPIC
    cdk deploy shutdown --name $CONNECTOR
}

@test "e2e-reports-via-connector" {
    echo "Starting consumer on topic $TOPIC"
    echo "Using connector $CONNECTOR"
    sleep 45

    echo "Produce \"20\" on $TOPIC"
    echo 20 | fluvio produce $TOPIC
    sleep 10

    echo "Produce \"25\" on $TOPIC"
    echo 25 | fluvio produce $TOPIC

    echo "Produce \"30\" on $TOPIC"
    echo 30 | fluvio produce $TOPIC
    sleep 10

    echo "Produce \"35\" on $TOPIC"
    echo 35 | fluvio produce $TOPIC
    sleep 10

    echo "Produce \"40\" on $TOPIC"
    echo 40 | fluvio produce $TOPIC
    sleep 10

    echo "Sleep to ensure record is processed"
    sleep 45

    echo "Retrieves metrics from server"
    curl -o ./data.json http://localhost:12345/render\?target\=weather.temperature.ca.sandiego\&format\=json\&noNullPoints
    cat ./data.json

    cat ./data.json | grep "20"
    assert_success

    cat ./data.json | grep "25"
    assert_success

    cat ./data.json | grep "30"
    assert_success

    cat ./data.json | grep "35"
    assert_success

    cat ./data.json | grep "40"
    assert_success
}
