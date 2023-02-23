#!/usr/bin/env bash
declare -a dim=("0" "2" "3" "4")
declare -a arr=("128" "256" "512" "1024" "2048" "4096" "8192" "16384" "32768" "65536" "131072" "262144")

make
true > tests.log

for i in "${dim[@]}"
do
    for j in "${arr[@]}"
    do
        ./randmst 1 ${j} 5 ${i} >> tests.log
        echo $'\n' >> tests.log
    done
    echo $'\n'$'\n'
done
