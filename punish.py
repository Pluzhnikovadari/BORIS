import subprocess
import time
import os
import sys
from datetime import datetime

def run_command(cmd, output="print", exit_on_error=False):
    p = subprocess.Popen(cmd, stdout=subprocess.PIPE,
                         stderr=subprocess.PIPE, shell=True,
                         universal_newlines=True)
    o, e = p.communicate()


def to_string(ip_path):
    ip_path = ip_path.split('.')
    res = "1" + "0" * (3 - len(ip_path[0])) + ip_path[0] + \
          "0" * (3 - len(ip_path[1])) + ip_path[1] + \
          "0" * (3 - len(ip_path[2])) + ip_path[2]
    if "/" in ip_path:
        ip_path = ip_path[-1].split('/')
        res += "0" * (3 - len(ip_path[0])) + ip_path[0] + ip_path[-1]
    else:
        res += "0" * (3 - len(ip_path[3])) + ip_path[3] + '24'
    return res


def bob_func():
    print("in")
    event_get = open("substrate-api-client_1/events.txt", "r")
    address = ''
    cnt = 0
    while True:
        # работа с событиями и добавление пути в quagga.
        address = address + event_get.readline()
        if address and address[-1] == '\n':
            print("got")
            address = address.split(", ")[-1][:-2]
            with open('/etc/quagga/bgpd.conf', "r+") as f:
                line = f.readlines()
            delit = str(int(address[1:4])) + "." + str(int(address[4:7])) + \
                "." + str(int(address[7:10])) + "." + \
                str(int(address[10:13]))
            
            map_neigh = ""
        
            for i in range(len(line)):
                if (delit in line[i]):
                    line = line[:i] + line[i + 1:]
                    break
            with open('/etc/quagga/bgpd.conf', "w+") as f:
                for el in line:
                    f.write(el)
    
def alice_fun():
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
    

  if __name__ == "__main__":

    if len(sys.argv) == 1:
        print("Add description for mode")
    else:
        if sys.argv[1] == "bob":
            bob_func()
        elif sys.argv[1] == "alice":
            alice_func()
