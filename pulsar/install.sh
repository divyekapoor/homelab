#!/usr/bin/env bash
sudo ln -s `pwd`/pulsar.service /usr/lib/systemd/system/
sudo systemd enable pulsar.service
sudo systemd start pulsar.service
