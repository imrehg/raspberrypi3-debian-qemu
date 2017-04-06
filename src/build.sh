#!/bin/bash

# Rebuilding the cross-build helper
go build -ldflags "-w -s" -o ../bin/resin-xbuild resin-xbuild.go
