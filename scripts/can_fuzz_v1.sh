#!/bin/bash

set -xe 
CANID=$((100 + $RANDOM % 100 ))
VAL=01

while [ 1 ]
do
	cansend $1 ${CANID}#1212121212121212
	CANID=$((100 + $RANDOM % 100 ))
done
