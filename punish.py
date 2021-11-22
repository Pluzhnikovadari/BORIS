import subprocess
import time
import os
import sys
from datetime import datetime

def run_command(cmd, output="print", exit_on_error=False):
    p = subprocess.Popen(cmd, stdout=subprocess.PIPE,
                         stderr=subprocess.PIPE, shell=True,
                         universal_newlines=True)
    p.communicate()



if __name__ == "__main__":

    
    
    path = input("Write path: ")
    acc = input("Write acc: ")

    
    
    cmd = "cd substrate-api-client_1 && cargo +nightly-2020-10-01 run --example example_transfer {}".format(acc)
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE,
                         stderr=subprocess.PIPE, shell=True,
                         universal_newlines=True)
    
    
    while process.poll() is None:
        print(process.stdout.readline())

   
    with open('substrate-api-client_1/data.txt',
        'w') as send_data_write:
        send_data_write.write(path)
    cmd = "cd substrate-api-client_1/ && cargo +nightly-2020-10-01 run --example example_generic_extrinsic"
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE,
                         stderr=subprocess.PIPE, shell=True,
                         universal_newlines=True)
    

    with open('substrate-api-client_1/events.txt',
        'w') as send_data_write:
        send_data_write.write("1169254253000024\n")
    
