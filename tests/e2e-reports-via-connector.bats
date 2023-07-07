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

    echo "Produce \"16\" on $TOPIC"
    echo 16 | fluvio produce $TOPIC

    echo "Produce \"16\" on $TOPIC"
    echo 16 | fluvio produce $TOPIC

    echo "Produce \"16\" on $TOPIC"
    echo 16 | fluvio produce $TOPIC

    echo "Produce \"16\" on $TOPIC"
    echo 16 | fluvio produce $TOPIC

    echo "Produce \"16\" on $TOPIC"
    echo 16 | fluvio produce $TOPIC

    echo "Sleep to ensure record is processed"
    sleep 65

    echo "Retrieves metrics from server"
    curl -o ./$TOPIC.json http://localhost:12345/render\?target\=weather.temperature.ca.sandiego\&format\=json\&noNullPoints
    cat ./$TOPIC.json

    cat ./$TOPIC.json | grep "16"
    assert_success
}
