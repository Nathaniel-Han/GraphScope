#!/bin/bash

export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/apsara/alicpp/built/gcc-4.9.2/gcc-4.9.2/lib64/:/apsara/alicpp/built/gcc-4.9.2/openssl-1.0.2a/lib/:/usr/local/hadoop-2.8.4/lib/native/:/usr/local/jdk1.8.0_191/jre/lib/amd64/server/:/usr/local/hadoop-2.8.4/lib/native/:/usr/local/lib64

ROOT_DIR=$PWD

echo $1
object_id=$1
echo $2
server_id=$2
echo $3
export VINEYARD_IPC_SOCKET=$3
echo $4
export worker_num=$4


for worker_id in {1..worker_num}
do
  server_id=$worker_id+1
  echo "Start worker $server_id..."
  mkdir -p /home/maxgraph/logs/executor/executor_${object_id}_${word_id}

  export LOG_DIRS=/home/maxgraph/logs/executor/executor_${object_id}_${word_id}

  rm -rf $ROOT_DIR/deploy/local/executor.vineyard.properties
  cp $ROOT_DIR/deploy/local/executor.vineyard.properties.bak $ROOT_DIR/deploy/local/executor.vineyard.properties_${worker_id}
  sed -i "s/VINEYARD_OBJECT_ID/$object_id/g" $ROOT_DIR/deploy/local/executor.vineyard.properties_${worker_id}
  sed -i "s/WORKER_NUM/$worker_num/g" $ROOT_DIR/deploy/local/executor.vineyard.properties_${worker_id}

  inner_config=$ROOT_DIR/deploy/local/executor.vineyard.properties_${worker_id}

  export flag="maxgraph"$object_id"executor"
  #export VINEYARD_IPC_SOCKET=/tmp/vineyard.sock.1617013756979

  RUST_BACKTRACE=full $ROOT_DIR/build/0.0.1-SNAPSHOT/bin/executor --config $inner_config $flag $server_id 1>> $LOG_DIRS/maxgraph-executor.out 2>> $LOG_DIRS/maxgraph-executor.err &

  echo $! > $ROOT_DIR/executor_${object_id}_${server_id}.pid
done