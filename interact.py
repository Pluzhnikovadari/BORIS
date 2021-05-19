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
    event_get = open("substrate-api-client/events.txt", "r")
    address = ''
    cnt = 0
    while True:
        # работа с событиями и добавление пути в quagga.
        address = address + event_get.readline()
        if address and address[-1] == '\n':
            address = address.split(", ")[-1][:-2]
            with open('/etc/quagga/bgpd.conf', "r+") as f:
                line = f.readlines()
            add = str(int(address[13:16])) + "." + str(int(address[16:19])) + \
                "." + str(int(address[19:22])) + "." + \
                str(int(address[22:25])) + '/' + str(int(address[25:]))
            neighbour_ip = str(int(address[1:4])) + "." + str(int(address[4:7])) + \
                "." + str(int(address[7:10])) + "." + \
                str(int(address[10:13]))
            map_neigh = ""
            for elem in line:
                if "neighbor " + neighbour_ip + " route-map" in elem:
                    map_neigh = elem.split(" ")[-2]
                    map_neigh = map_neigh.split("-")[-1]
                    break
            count = 0
            for elem in line:
                if 'ip prefix-list PREFIX-LIST-FROM-' + \
                        map_neigh + ' seq' in elem:
                        count += 1
            for i in range(len(line)):
                if ("route-map MAP-FROM"  in line[i] and 'permit' in line[i]):
                    if "!" in line[i - 1]:
                        print("in this 1")
                        line = line[:i - 1] + [' ip prefix-list PREFIX-LIST-FROM-' + \
                            map_neigh + ' seq ' + \
                            str(count + 1) + ' permit ' + add + '\n'] + line[i - 1:]
                    else:
                        line = line[:i] + ['!\n ip prefix-list PREFIX-LIST-FROM-' + \
                            map_neigh + ' seq ' + \
                            str(count + 1) + ' permit ' + add + '\n!\n'] + line[i:]
                    break
            with open('/etc/quagga/bgpd.conf', "w+") as f:
                for el in line:
                    f.write(el)


def alice_func():
    cmd = "sudo vtysh -c 'show ip bgp' >> bgptable.txt"
    run_command(cmd, "print")
    cmd = "cd substrate-api-client_1/ && cargo +nightly-2020-10-01 \
    build --example example_generic_extrinsic"
    run_command(cmd, "print")
    while True:
        with open('bgptable_updates.txt', 'w'):
            pass

        cmd = "sudo vtysh -c 'show ip bgp' >> bgptable_updates.txt"
        run_command(cmd, "skip")

        with open('bgptable_updates.txt', 'r') as update:
            with open('bgptable.txt', 'r') as table:
                tablefile = table.readlines()
                updatefile = update.readlines()[6:-2]

                router_ip = 0

                for el in tablefile:
            		if "bgp router-id" in el:
            			router_ip = to_string(' '.split(el)[-1])
                        break


            	tablefile = tablefile[6:-2]
                t = set(tablefile)
                u = set(updatefile)
                t.difference_update(u)
                for elem in t:
                    ip_path = elem.split()[1]
                    ip_path = router_ip + to_string(ip_path)
                    with open('substrate-api-client_1/data.txt',
                              'w') as send_data_write:
                        send_data_write.write(ip_path)
                    cmd = "cd substrate-api-client_1/ && \
                    ./target/release/examples/example_generic_extrinsic"
                    run_command(cmd, "print")
            with open('bgptable.txt', 'w') as tablewrite:
                update.seek(0)
                data = update.read()
                tablewrite.write(data)


if __name__ == "__main__":

    if len(sys.argv) == 1:
        print("Add description for mode")
    else:
        if sys.argv[1] == "bob":
            bob_func()
        elif sys.argv[1] == "alice":
            alice_func()
