import sys
import time
import matplotlib.pyplot as plt
import numpy as np
import os
from dataclasses import dataclass

@dataclass
class ProgramLog:
    def __init__(self):
        self.sent = 0
        self.received = 0
        self.ignored = 0
        self.delayed = 0
        self.dropped = 0
        self.last_position = 0
    def read_log(self, file_path: str):
        try:
            file_size = os.path.getsize(file_path)

            # if file empty, reset last_position but keep counters
            if file_size == 0:
                self.last_position = 0
                return

            with open(file_path, "r") as f:
                f.seek(self.last_position)  # move to last read position
                for line in f:
                    line = line.strip()
                    if "[SEND]" in line:
                        self.sent += 1
                    elif "[RECEIVE]" in line:
                        self.received += 1
                    elif "[IGNORE]" in line:
                        self.ignored += 1
                    elif "[DELAY]" in line:
                        self.delayed += 1
                    elif "[DROP]" in line:
                        self.dropped += 1
                self.last_position = f.tell()
        except FileNotFoundError:
            pass

if "-r" in sys.argv:
    print(os.getcwd())
    try:
        os.remove("/home/louise/BCIT/7005comp/Assignments/COMP7005-project/reliable-udp/client/log.txt")
    except FileNotFoundError:
        pass
    try:
        os.remove("/home/louise/BCIT/7005comp/Assignments/COMP7005-project/reliable-udp/proxy/log.txt")
    except FileNotFoundError:
        pass
    try:
        os.remove("/home/louise/BCIT/7005comp/Assignments/COMP7005-project/reliable-udp/server/log.txt")
    except FileNotFoundError:
        pass

client_log = ProgramLog()
proxy_log = ProgramLog()
server_log = ProgramLog()

client_file = "client/log.txt"
proxy_file = "proxy/log.txt"
server_file = "server/log.txt"

# setup bar chart
plt.ion()
fig, ax = plt.subplots()

programs = ["Client", "Proxy", "Server"]
metrics = ["Sent", "Received", "Delayed", "Dropped", "Ignored"]
n_metrics = 4
width = 0.25  
spacing = 1.5  
x = np.arange(len(programs)) * spacing

# bars
bars_sent = ax.bar(x - 2*width, [0]*len(programs), width, label="Sent", color="#b2f2bb")
bars_received = ax.bar(x - 1*width, [0]*len(programs), width, label="Received", color="#a6cee3")
bars_delayed = ax.bar(x + 0*width, [0]*len(programs), width, label="Delayed", color="#b39ddb")
bars_dropped = ax.bar(x + 1*width, [0]*len(programs), width, label="Dropped", color="#fbb4ae")
bars_ignored = ax.bar(x + 2*width, [0]*len(programs), width, label="Ignored", color="#fed976")

ax.set_xticks(x)
ax.set_xticklabels(programs)
ax.set_ylabel("Count")
ax.set_title("Live UDP Program Logs")
ax.legend()

smoothing_factor = 0.2

# live update graph loop
try:
    while True:
        # read new log entries
        client_log.read_log(client_file)
        proxy_log.read_log(proxy_file)
        server_log.read_log(server_file)

        # update bar heights
        sent_values = [client_log.sent, proxy_log.sent, server_log.sent]
        received_values = [client_log.received, proxy_log.received, server_log.received]
        delayed_values = [client_log.delayed, proxy_log.delayed, server_log.delayed]
        dropped_values = [client_log.dropped, proxy_log.dropped, server_log.dropped]
        ignored_values = [client_log.ignored, proxy_log.ignored, server_log.ignored]

        for bar, target in zip(bars_sent, sent_values):
            current = bar.get_height()
            bar.set_height(current + (target - current) * smoothing_factor)
        for bar, target in zip(bars_received, received_values):
            current = bar.get_height()
            bar.set_height(current + (target - current) * smoothing_factor)
        for bar, target in zip(bars_delayed, delayed_values):
            current = bar.get_height()
            bar.set_height(current + (target - current) * smoothing_factor)
        for bar, target in zip(bars_dropped, dropped_values):
            current = bar.get_height()
            bar.set_height(current + (target - current) * smoothing_factor)
        for bar, target in zip(bars_ignored, ignored_values):
            current = bar.get_height()
            bar.set_height(current + (target - current) * smoothing_factor)
            
        ax.set_ylim(0, max(max(sent_values + received_values)*1.2, 10))

        # --- Log to console ---
        # print(f"Sent: Client={sent_values[0]}, Proxy={sent_values[1]}, Server={sent_values[2]}")
        # print(f"Received: Client={received_values[0]}, Proxy={received_values[1]}, Server={received_values[2]}")
        # print("-"*40)

        plt.pause(0.1)
except KeyboardInterrupt:
    print("Exiting live graph.")
    plt.close()
finally:
    plt.close('all')