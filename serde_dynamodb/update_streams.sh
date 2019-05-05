#!/bin/sh
set -ex

echo "Cleanup previous version"
rm -rf src/dynamodbstreams
rm tests/dynamodbstreams.rs

echo "Copy dynamodb implementation"
cp -r src/dynamodb src/dynamodbstreams
cp tests/dynamodb.rs tests/dynamodbstreams.rs

echo "Update to use dynamodbstreams"
sed -e 's/rusoto_dynamodb/rusoto_dynamodbstreams/g' -i '' src/dynamodbstreams/*
sed -e 's/rusoto_dynamodb/rusoto_dynamodbstreams/g' -i '' tests/dynamodbstreams.rs
sed -e 's/serde_dynamodb::from_hashmap/serde_dynamodb::streams::from_hashmap'/g -i '' tests/dynamodbstreams.rs
sed -e 's/serde_dynamodb::to_hashmap/serde_dynamodb::streams::to_hashmap'/g -i '' tests/dynamodbstreams.rs

echo "Put notice on top of generated files"
for file in src/dynamodbstreams/* tests/dynamodbstreams.rs
do
    tmpfile=$(mktemp /tmp/rusoto_dynamodb_update.XXXXXX)
    echo "// generated file, see update_streams.sh" > $tmpfile
    echo "" >> $tmpfile
    cat $file >> $tmpfile
    mv $tmpfile $file
done
