#!/usr/bin/env bash

CURDIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
. "$CURDIR"/../../../shell_env.sh


echo "create table s_distinct (a String not null);" | $MYSQL_CLIENT_CONNECT

for i in `seq 1 100`;do
	echo "insert into s_distinct values ('$i'), ('$[i+1]'), ('$[i+2]')" | $MYSQL_CLIENT_CONNECT
done

echo "select count(distinct a) from s_distinct" |  $MYSQL_CLIENT_CONNECT

echo "drop table s_distinct;" | $MYSQL_CLIENT_CONNECT
