#!/usr/bin/env bash
CURDIR=$(cd `dirname $BASH_SOURCE` && pwd)

cd $CURDIR
cd ..
cargo build
cd $CURDIR

export VET=$CURDIR/target/debug/vet
for i in categories/*; do
    . $i/run.sh
done
