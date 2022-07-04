#!/bin/bash

cd /home/back 
ROCKET_ADDRESS="0.0.0.0" cargo +nightly run &

cd /home/front
yarn start

