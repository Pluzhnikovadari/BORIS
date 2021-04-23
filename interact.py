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

    with open('our_ip.txt', 'w'):
        pass
    cmd = "sudo cat /etc/quagga/bgpd.conf >> our_ip.txt"
    with open('our_ip.txt', 'r') as our_ip:
        run_command(cmd, "print")
        our_way = our_ip.readlines()[4:6]
    our_way = [our_way[0].split(' ')[-1][:-1],
               to_string(our_way[1].split(' ')[-1][:-1])]
    run_command(cmd, "print")
    event_get = open("substrate-api-client/events.txt", "r")
    ev = ''
    cnt = 0
    while True:
        # работа с событиями и добавление пути в quagga.
        ev = ev + event_get.readline()
        if ev and ev[-1] == '\n':
            print(datetime.now())
            ev = ev.split(", ")[-1][:-2]
            with open('/etc/quagga/bgpd.conf', "r+") as f:
                line = f.readlines()
                add = str(int(ev[1:4])) + "." + str(int(ev[4:7])) + \
                    "." + str(int(ev[7:10])) + "." + \
                    str(int(ev[10:13])) + '/' + str(int(ev[13:]))
                for i in range(len(line)):
                    """ ip prefix-list PREFIX-LIST-FROM-BGP1
                    seq 2 permit 169.254.253.0/24."""
                    if ('route-map MAP-FROM-BGP1 permit' in line[i]):
                        line = line[:i] + [' ip prefix-list \
                            PREFIX-LIST-FROM-BGP1 seq \
                            1 permit ' + add + '\n'] + ['!\n'] + line[i:]
                        break
                with open('/etc/quagga/bgpd.conf', "w") as f:
                    for el in line:
                        f.write(el)
            ev = ''
            #print(2, datetime.now())


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
                tablefile = table.readlines()[6:-2]
                updatefile = update.readlines()[6:-2]

                t = set(tablefile)
                u = set(updatefile)
                t.difference_update(u)
                for elem in t:
                    ip_path = elem.split()[1]
                    ip_path = to_string(ip_path)
                    print(datetime.now())
                    with open('substrate-api-client_1/data.txt',
                              'w') as send_data_write:
                        send_data_write.write(ip_path)
                    print(datetime.now())
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