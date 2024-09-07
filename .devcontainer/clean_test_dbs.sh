#!/bin/bash

# About this script:
# You might sometimes have some leaking databases after running tests that interact 
# with the PostgreSQL database. This script deletes them all to ensure cleanliness.

PREFIX='test_db'
export PGPASSWORD=postgres
export PGUSER=postgres
export PGHOST=localhost
export PGPORT=5432

TEST_DB_LIST=$(psql -l | awk '{ print $1 }' | grep '^[a-z]' | grep -v template | grep -v postgres)
for TEST_DB in $TEST_DB_LIST ; do
    if [ $(echo $TEST_DB | sed "s%^$PREFIX%%") != $TEST_DB ]
    then
        echo "Dropping $TEST_DB"
        dropdb --if-exists $TEST_DB
    fi
done
