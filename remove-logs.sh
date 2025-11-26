#!/bin/bash

echo "=== Deleting client log..."
rm ./client/log.txt

echo "=== Deleting proxy log..."
rm ./proxy/log.txt

echo "=== Deleting server log..."
rm ./server/log.txt

echo "===Successfully deleted logs!==="
