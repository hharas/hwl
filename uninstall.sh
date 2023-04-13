#!/bin/bash

set -e

echo "Uninstalling hwl..."
rm /usr/local/bin/hwl

if [ $? -ne 0 ]; then
  echo "Failed to uninstall hwl from /usr/local/bin/."
  exit 1
fi

echo "Alhamdulillah! Exiting..."
