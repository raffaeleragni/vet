#!/usr/bin/env bash
CURDIR=$(cd `dirname $BASH_SOURCE` && pwd)

export VET=$CURDIR/../target/debug/vet

if [ ! -f $VET ]; then
    cd $CURDIR
    cd ..
    cargo build
    cd $CURDIR
fi

for i in categories/*; do
    cd $i
    . ./run.sh
    cd -
done
