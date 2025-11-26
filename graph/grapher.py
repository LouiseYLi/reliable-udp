from dataclasses import dataclass

@dataclass
class Program_Log:
    sent: int
    received: int
    ignored: int # for client and server programs only
    # TODO: need delayed, dropped?
    def __init__(self):
        self.sent = 0
        self.received = 0
        self.ignored = 0

    def read_log(self, file_path: str):
        with open(file_path) as f:
            # loop through file line by line
            for line in f:
                # check if line includes "[SEND]", "[RECEIVE]", "[IGNORE]"
                # increment counters for each
                if "[SEND]" in line:
                    ++self.sent
                elif "[RECEIVE]" in line:
                    ++self.received
                elif "[IGNORE]" in line:
                    ++self.ignored
 
def main():
    client = Program_Log()
    proxy = Program_Log()
    server = Program_Log()

    client.read_log("")
    proxy.read_log("")
    server.read_log("")

